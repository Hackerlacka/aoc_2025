use log::info;
use utils::{InputType, get_input_path};

use crate::common::DeviceMap;

fn run_example() {
    let path = get_input_path(11, 1, InputType::Example, None);
    let lines = utils::read_lines(path).unwrap();

    let device_map = DeviceMap::new(lines);
    let paths_you_to_out = device_map.find_paths_from_x_to_out("you", None);
    assert_eq!(paths_you_to_out, 5);
}

pub fn run_task() {
    run_example();

    let path = get_input_path(11, 1, InputType::Regular, None);
    let lines = utils::read_lines(path).unwrap();

    let device_map = DeviceMap::new(lines);
    let paths_you_to_out = device_map.find_paths_from_x_to_out("you", None);
    assert_eq!(paths_you_to_out, 497);
    info!("Paths you -> out: {paths_you_to_out}");
}
