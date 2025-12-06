use crate::common::Homeworks;
use log::info;
use utils::{InputType, get_input_path};

fn run_example() {
    let path = get_input_path(6, 1, InputType::Example, None);
    let lines = utils::read_lines(path).unwrap();

    let homeworks = Homeworks::new(lines);
    let done_homeworks = homeworks.do_homeworks(Some(vec![33210, 490, 4243455, 401]));
    assert_eq!(done_homeworks, 4277556);
}

pub fn run_task() {
    run_example();

    let path = get_input_path(6, 1, InputType::Regular, None);
    let lines = utils::read_lines(path).unwrap();

    let homeworks = Homeworks::new(lines);
    let done_homeworks = homeworks.do_homeworks(None);
    assert_eq!(done_homeworks, 5060053676136);
    info!("Result: {done_homeworks}")
}
