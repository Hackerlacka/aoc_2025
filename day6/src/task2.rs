use crate::common::Homeworks;
use log::info;
use utils::{InputType, get_input_path};

fn run_example() {
    let path = get_input_path(6, 1, InputType::Example, None);
    let lines = utils::read_lines(path).unwrap();

    let homeworks = Homeworks::new_pt2(lines);
    let done_homeworks = homeworks.do_homeworks(Some(vec![1058, 3253600, 625, 8544]));
    assert_eq!(done_homeworks, 3263827);
}

pub fn run_task() {
    run_example();

    let path = get_input_path(6, 1, InputType::Regular, None);
    let lines = utils::read_lines(path).unwrap();

    let homeworks = Homeworks::new_pt2(lines);
    let done_homeworks = homeworks.do_homeworks(None);
    assert_eq!(done_homeworks, 9695042567249);
    info!("Result: {done_homeworks}")
}
