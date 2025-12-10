use log::info;
use utils::{InputType, get_input_path};

use crate::common::Floor;

fn run_example() {
    let path = get_input_path(9, 1, InputType::Example, None);
    let lines = utils::read_lines(path).unwrap();
    let floor = Floor::new(lines);
    let area = floor.find_largest_rectangle_area();
    assert_eq!(area, 50);
}

pub fn run_task() {
    run_example();

    let path = get_input_path(9, 1, InputType::Regular, None);
    let lines = utils::read_lines(path).unwrap();
    let floor = Floor::new(lines);
    let area = floor.find_largest_rectangle_area();
    info!("Area: {area}");
}
