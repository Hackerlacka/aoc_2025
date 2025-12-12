use log::{debug, info};
use std::{
    clone,
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
};

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
    fn compress(devices: &mut HashMap<String, Device>) {
        // TODO: Loses the path information :D
        let not_optimize = ["you", "svr", "dac", "fft"];
        while let Some(single_destination_identifier) = devices
            .iter()
            .find(|(identifier, device)| {
                !not_optimize.contains(&identifier.as_str()) && device.connections.len() == 1
            })
            .map(|(identifier, _)| identifier.to_owned())
        {
            let device = devices.remove(&single_destination_identifier).unwrap();
            devices
                .iter_mut()
                .filter(|(_, d)| d.connections.contains(&device.identifier))
                .for_each(|(_, d)| {
                    d.connections = d
                        .connections
                        .iter()
                        .filter(|s| s != &&device.identifier)
                        .cloned()
                        .collect();

                    let next_dest = device.connections.first().unwrap().clone();
                    if !d.connections.contains(&next_dest) {
                        d.connections.push(next_dest);
                    }
                });
        }
    }

    pub fn new(lines: Vec<String>) -> Self {
        let mut devices: HashMap<String, Device> = lines
            .iter()
            .map(|line| {
                let device = Device::new(line);
                (device.identifier.clone(), device)
            })
            .collect();
        // let len_before = devices.len();
        // Self::compress(&mut devices);
        // let len_after: usize = devices.len();
        // info!(
        //     "Removed/compressed: {}/{}",
        //     len_before - len_after,
        //     len_before
        // );
        //info!("Devices: {devices:?}");
        DeviceMap { devices }
    }

    pub fn find_paths_from_x_to_out(&self, x: &str, must_have: Option<Vec<&str>>) -> usize {
        // Never visit the same node twice, in one path?
        // Avoid loops that never lead to out
        // Should be no loops that can actually go to out? Because then there would be infinite combinations right?
        let mut devices_to_check = vec![self.devices.get(x).unwrap()];
        //let mut device_entering_paths: HashMap<String, Vec<Vec<&str>>> = HashMap::new();
        let mut device_entering_paths: HashMap<String, Vec<Vec<&str>>> = self
            .devices
            .keys()
            .map(|identifier| (identifier.clone(), Vec::new()))
            .collect();
        while let Some(device) = devices_to_check.pop() {
            // let mut hasher = DefaultHasher::new();
            // devices_to_check.hash(&mut hasher);
            // info!("Checking {}, {:x}", device.identifier, hasher.finish());

            //info!("Devices to check: {}", devices_to_check.len());
            let entering_paths = device_entering_paths.remove(&device.identifier).unwrap();

            for device_next in device.connections.iter() {
                // Stop if next node is out or self
                // TODO: Remove or part again?
                if device_next == "out" || device_next == &device.identifier {
                    // TODO: Could also be solved by adding a "out" device to the device map
                    continue;
                }

                let next_device_entering_path = device_entering_paths.get_mut(device_next).unwrap();

                let mut will_visit_next = false;
                if entering_paths.is_empty() {
                    next_device_entering_path.push(vec![&device.identifier]);
                    will_visit_next = true;
                } else {
                    for mut path in entering_paths.iter().cloned() {
                        // Protect against loops
                        if path.contains(&device.identifier.as_str()) {
                            continue;
                        }

                        path.push(&device.identifier);
                        if next_device_entering_path.contains(&path) {
                            continue;
                        }

                        next_device_entering_path.push(path);
                        will_visit_next = true;
                    }
                }

                if will_visit_next {
                    debug!("Adding {device_next} to devices to check");
                    let next_dev = self.devices.get(device_next).unwrap();
                    if !devices_to_check.contains(&next_dev) {
                        devices_to_check.push(next_dev);
                    }

                    // TODO: Revert to just devices_to_check.push(next_dev);?
                }
            }

            // Re-insert device entering paths
            device_entering_paths.insert(device.identifier.clone(), entering_paths);
        }

        // info!("Device entering paths:");
        // for device_entering_path in device_entering_paths.iter() {
        //     info!("{device_entering_path:?}")
        // }

        self.devices
            .iter()
            .filter(|(_, device)| device.connections.contains(&"out".to_owned()))
            .map(|(identifier, _)| {
                device_entering_paths
                    .remove(identifier)
                    .unwrap_or_default()
                    .iter()
                    .filter(|a| {
                        // TODO: Why do we have to clone here?
                        if let Some(must) = must_have.clone() {
                            return must.iter().all(|m| a.contains(m));
                        }
                        true
                    })
                    .count()
            })
            .sum()
    }
}
