use log::{debug, info};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
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
    pub fn new(lines: Vec<String>) -> Self {
        let devices = lines
            .iter()
            .map(|line| {
                let device = Device::new(line);
                (device.identifier.clone(), device)
            })
            .collect();
        DeviceMap { devices }
    }

    pub fn find_paths_from_x_to_out(&self, x: &str, must_have: Option<Vec<&str>>) -> usize {
        // Never visit the same node twice, in one path?
        // Avoid loops that never lead to out
        // Should be no loops that can actually go to out? Because then there would be infinite combinations right?
        let mut devices_to_check = vec![self.devices.get(x).unwrap()];
        let mut device_entering_paths: HashMap<String, Vec<Vec<&str>>> = HashMap::new();
        while let Some(device) = devices_to_check.pop() {
            debug!("Checking {}", device.identifier);
            info!("Devices to check: {}", devices_to_check.len());
            let entering_paths = device_entering_paths
                .remove(&device.identifier)
                .unwrap_or_default();

            for device_next in device.connections.iter() {
                // TODO: Remove or part again?
                if device_next == "out" || device_next == &device.identifier {
                    // TODO: Could also be solved by adding a "out" device to the device map
                    continue;
                }

                if !device_entering_paths.contains_key(device_next) {
                    device_entering_paths.insert(device_next.clone(), Vec::new());
                }
                let next_device_entering_path = device_entering_paths.get_mut(device_next).unwrap();

                let mut will_visit = false;
                if entering_paths.is_empty() {
                    next_device_entering_path.push(vec![&device.identifier]);
                    will_visit = true;
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
                        will_visit = true;
                    }
                }

                if will_visit {
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
