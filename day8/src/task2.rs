use log::info;
use utils::{InputType, get_input_path};

use crate::common::JunctionBoxes;

fn run_example() {
    let path = get_input_path(8, 1, InputType::Example, None);
    let lines = utils::read_lines(&path).unwrap();

    let mut junction_boxes = JunctionBoxes::new(lines);
    let circuits = junction_boxes.get_circuits(None);
    assert_eq!(circuits.len(), 1);

    let (b1, b2) = circuits.first().unwrap().get_last();
    let product = b1.get_x_product(&b2);
    assert_eq!(product, 25272);
}

pub fn run_task() {
    run_example();

    let path = get_input_path(8, 1, InputType::Regular, None);
    let lines = utils::read_lines(&path).unwrap();

    let mut junction_boxes = JunctionBoxes::new(lines);
    let circuits = junction_boxes.get_circuits(None);
    assert_eq!(circuits.len(), 1);

    let (b1, b2) = circuits.first().unwrap().get_last();
    let product = b1.get_x_product(&b2);
    info!("Product: {product}");
    assert_eq!(product, 6934702555);
}
