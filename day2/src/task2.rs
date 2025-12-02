use crate::common::run_task_on_file;

use log::info;
use utils::{InputType, get_input_path};

fn run_example() {
    let path = get_input_path(2, 1, InputType::Example, None);
    let sum = run_task_on_file(path, true);
    assert_eq!(sum, 4174379265);
}

pub fn run_task() {
    run_example();

    let path = get_input_path(2, 1, InputType::Regular, None);
    let sum = run_task_on_file(path, true);
    info!("Sum is {sum}");

    assert_eq!(sum, 31578210022);
}
