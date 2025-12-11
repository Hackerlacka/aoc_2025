use log::info;
use utils::{InputType, get_input_path};

use crate::common::DeviceMap;

fn run_example() {
    let path = get_input_path(11, 2, InputType::Example, None);
    let lines = utils::read_lines(path).unwrap();

    let device_map = DeviceMap::new(lines);
    let paths_svr_to_out = device_map.find_paths_from_x_to_out("svr", Some(vec!["dac", "fft"]));
    assert_eq!(paths_svr_to_out, 2);
}

pub fn run_task() {
    run_example();

    let path = get_input_path(11, 1, InputType::Regular, None);
    let lines = utils::read_lines(path).unwrap();

    let device_map = DeviceMap::new(lines);
    let paths_svr_to_out = device_map.find_paths_from_x_to_out("svr", Some(vec!["dac", "fft"]));
    info!("Paths svr -> out: {paths_svr_to_out}");
}
