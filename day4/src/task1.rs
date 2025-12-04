use crate::common::PaperRollMap;
use log::info;
use std::path::PathBuf;
use utils::{InputType, get_input_path};

fn run_task_on_file(path: &PathBuf) -> usize {
    let lines = utils::read_lines(path).unwrap();
    let map = PaperRollMap::new(lines);
    map.accessible_paper_rolls()
}

fn run_example() {
    let path = get_input_path(4, 1, InputType::Example, None);
    let accessible_paper_rolls = run_task_on_file(&path);
    assert_eq!(accessible_paper_rolls, 13);
}

pub fn run_task() {
    run_example();

    let path = get_input_path(4, 1, InputType::Regular, None);
    let accessible_paper_rolls = run_task_on_file(&path);
    info!("Accessible paper rolls: {accessible_paper_rolls}")
}
