use log::info;
use utils::{InputType, get_input_path};

use crate::common::JunctionBoxes;

fn run_example() {
    let path = get_input_path(8, 1, InputType::Example, None);
    let lines = utils::read_lines(&path).unwrap();

    let mut junction_boxes = JunctionBoxes::new(lines);
    let circuits = junction_boxes.get_circuits(Some(10));

    let product = JunctionBoxes::get_product_3_largest_circuits(&circuits);
    assert_eq!(product, 40);
}

pub fn run_task() {
    run_example();

    let path = get_input_path(8, 1, InputType::Regular, None);
    let lines = utils::read_lines(&path).unwrap();

    let mut junction_boxes = JunctionBoxes::new(lines);
    let circuits = junction_boxes.get_circuits(Some(1000));

    let product = JunctionBoxes::get_product_3_largest_circuits(&circuits);
    info!("Product: {product}");
    assert_eq!(product, 175500);
}
