use log::info;
use utils::{InputType, get_input_path};

use crate::common::Manifold;

fn run_example() {
    let path = get_input_path(7, 1, InputType::Example, None);
    let lines = utils::read_lines(path).unwrap();

    let mut manifold = Manifold::new(lines);
    let timelines = manifold.tachyon_beam_split_count(true);
    assert_eq!(timelines, 40);
}

pub fn run_task() {
    run_example();

    let path = get_input_path(7, 1, InputType::Regular, None);
    let lines = utils::read_lines(path).unwrap();

    let mut manifold = Manifold::new(lines);
    let timelines = manifold.tachyon_beam_split_count(true);
    assert_eq!(timelines, 16716444407407);
    info!("Timelines: {timelines}");
}
