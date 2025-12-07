use log::info;
use utils::{InputType, get_input_path};

use crate::common::Manifold;

fn run_example() {
    let path = get_input_path(7, 1, InputType::Example, None);
    let lines = utils::read_lines(path).unwrap();

    let mut manifold = Manifold::new(lines);
    let beam_split_count = manifold.tachyon_beam_split_count(false);
    assert_eq!(beam_split_count, 21);
}

pub fn run_task() {
    run_example();

    let path = get_input_path(7, 1, InputType::Regular, None);
    let lines = utils::read_lines(path).unwrap();

    let mut manifold = Manifold::new(lines);
    let beam_split_count = manifold.tachyon_beam_split_count(false);
    assert_eq!(beam_split_count, 1585);
    info!("Split count: {beam_split_count}");
}
