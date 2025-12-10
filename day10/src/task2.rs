use crate::common::Machines;
use log::info;
use utils::{InputType, get_input_path};

fn run_example() {
    let path = get_input_path(10, 1, InputType::Example, None);
    let lines = utils::read_lines(path).unwrap();
    let machines = Machines::new(lines);
    let fewest_button_presses_joltage = machines.find_fewest_button_presses_joltage();
    assert_eq!(fewest_button_presses_joltage, vec![10, 12, 11]);
    let sum: usize = fewest_button_presses_joltage.iter().sum();
    assert_eq!(sum, 33)
}

pub fn run_task() {
    run_example();

    // let path = get_input_path(10, 1, InputType::Regular, None);
    // let lines = utils::read_lines(path).unwrap();
    // let machines = Machines::new(lines);
    // let fewest_button_presses_joltage = machines.find_fewest_button_presses_joltage();
    // let sum: usize = fewest_button_presses_joltage.iter().sum();
    // info!("Sum: {sum}");
}
