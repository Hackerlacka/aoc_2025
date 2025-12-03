use log::info;
use utils::{InputType, get_input_path};

use crate::common::run_task_on_file;

fn run_example() {
    let path = get_input_path(3, 1, InputType::Example, None);
    let (max_joltage, max_joltage_sum) = run_task_on_file(&path, 12);

    assert_eq!(
        max_joltage,
        vec![987654321111, 811111111119, 434234234278, 888911112111]
    );
    assert_eq!(max_joltage_sum, 3121910778619);
}

pub fn run_task() {
    run_example();

    let path = get_input_path(3, 1, InputType::Regular, None);
    let (_, max_joltage_sum) = run_task_on_file(&path, 12);

    info!("Max joltage sum: {max_joltage_sum}");
}
