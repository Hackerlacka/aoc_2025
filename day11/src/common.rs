use log::info;
use std::{collections::HashMap, hash::Hash};

#[derive(Debug, PartialEq, Hash)]
struct Device {
    identifier: String,
    connections: Vec<String>,
}

impl Device {
    fn new(line: &str) -> Self {
        let mut colon_split = line.split(":");
        let identifier = colon_split.next().unwrap().to_owned();
        let space_split = colon_split.next().unwrap().trim().split(" ");
        let connections = space_split.into_iter().map(|s| s.to_owned()).collect();
        Device {
            identifier,
            connections,
        }
    }
}

pub struct DeviceMap {
    devices: HashMap<String, Device>,
}

impl DeviceMap {
    const OUT_DEVICE: &str = "out";

    pub fn new(lines: Vec<String>) -> Self {
        let mut devices: HashMap<String, Device> = lines
            .iter()
            .map(|line| {
                let device = Device::new(line);
                (device.identifier.clone(), device)
            })
            .collect();

        // Create an "out" device for convenience
        devices.insert(
            Self::OUT_DEVICE.to_owned(),
            Device {
                identifier: Self::OUT_DEVICE.to_owned(),
                connections: vec![],
            },
        );

        DeviceMap { devices }
    }

    /// Checks which must haves are present in paths and combines them to a string like "dac,fft"
    fn must_haves_present_as_str(
        path: &Vec<&Device>,
        must_have_devices: &Vec<&Device>,
    ) -> Option<String> {
        path.iter()
            .filter(|device| must_have_devices.contains(device))
            .map(|device| device.identifier.to_owned())
            .reduce(|acc, e| acc + "," + &e)
    }

    /// Save data about how many solutions can be found after a device given which "must haves" are present
    /// in the path this far
    fn register_device_data(
        device: &Device,
        path: &Vec<&Device>,
        solutions_increase: usize,
        device_data: &mut HashMap<String, HashMap<Option<String>, usize>>,
        must_have_devices: &Vec<&Device>,
    ) {
        let data = device_data.get_mut(&device.identifier).unwrap();
        let must_haves_present = Self::must_haves_present_as_str(path, must_have_devices);
        if data.contains_key(&must_haves_present) {
            return;
        }

        data.insert(must_haves_present, solutions_increase);
    }

    /// Check if next device needs to be visited or not depending on if there is
    /// information about how many solutions there are from that node and forward given
    /// which "must haves" are present in the path this far
    fn need_to_check_device(
        next_device: &Device,
        path: &Vec<&Device>,
        must_have_devices: &Vec<&Device>,
        device_data: &mut HashMap<String, HashMap<Option<String>, usize>>,
        solutions: &mut usize,
    ) -> bool {
        let data_next = device_data.get_mut(&next_device.identifier).unwrap();
        let must_haves_present = Self::must_haves_present_as_str(path, must_have_devices);
        if let Some(next_solutions) = data_next.get(&must_haves_present) {
            *solutions += *next_solutions;
            return false;
        }

        true
    }

    fn try_device<'a>(
        device: &'a Device,
        path: &mut Vec<&'a Device>,
        devices: &'a HashMap<String, Device>,
        must_have_devices: &Option<Vec<&Device>>,
        device_data: &mut HashMap<String, HashMap<Option<String>, usize>>,
        solutions: &mut usize,
    ) {
        path.push(device);
        let solutions_before = *solutions;

        device
            .connections
            .iter()
            .map(|next_device_identifier| &devices[next_device_identifier])
            .for_each(|next_device| {
                if next_device.identifier == Self::OUT_DEVICE {
                    // Found a potential solution
                    if let Some(must_have) = must_have_devices {
                        if must_have
                            .iter()
                            .all(|must_device_identifier| path.contains(must_device_identifier))
                        {
                            *solutions += 1;
                        }
                    } else {
                        *solutions += 1;
                    }
                    return;
                } else if path.contains(&next_device) {
                    info!("Rejected loop");
                    return;
                }

                if must_have_devices.is_none()
                    || Self::need_to_check_device(
                        next_device,
                        path,
                        must_have_devices.as_ref().unwrap(),
                        device_data,
                        solutions,
                    )
                {
                    Self::try_device(
                        next_device,
                        path,
                        devices,
                        must_have_devices,
                        device_data,
                        solutions,
                    );
                }
            });

        if must_have_devices.is_some() {
            let solutions_increase = *solutions - solutions_before;
            Self::register_device_data(
                device,
                path,
                solutions_increase,
                device_data,
                must_have_devices.as_ref().unwrap(),
            );
        }

        _ = path.pop();
    }

    fn create_device_data(&self) -> HashMap<String, HashMap<Option<String>, usize>> {
        self.devices
            .keys()
            .map(|device_identifier| (device_identifier.clone(), HashMap::new()))
            .collect()
    }

    fn get_must_have_devices(&self, must_have: Option<Vec<&str>>) -> Option<Vec<&Device>> {
        must_have.map(|must_have_devices_indentifiers| {
            must_have_devices_indentifiers
                .iter()
                .map(|must_have_device_identifier| {
                    &self.devices[must_have_device_identifier.to_owned()]
                })
                .collect()
        })
    }

    pub fn find_paths_from_x_to_out(&self, x: &str, must_have: Option<Vec<&str>>) -> usize {
        let start_device = &self.devices[x];
        let must_have_devices = self.get_must_have_devices(must_have);
        let mut device_data = self.create_device_data();
        let mut path = Vec::new();
        let mut solutions = 0;
        Self::try_device(
            start_device,
            &mut path,
            &self.devices,
            &must_have_devices,
            &mut device_data,
            &mut solutions,
        );

        solutions
    }
}
