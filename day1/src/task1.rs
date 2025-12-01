use utils::{InputType, get_input_path, read_lines};

use crate::common::Safe;

fn run_example() {
    let path = get_input_path(1, 1, InputType::Example, None);
    let lines = read_lines(path).unwrap();

    let mut safe = Safe::new(false);
    let password = safe.find_password(lines);

    assert_eq!(password, 3);
}

pub fn run_task() {
    run_example();

    let path = get_input_path(1, 1, InputType::Regular, None);
    let lines = read_lines(path).unwrap();

    let mut safe = Safe::new(false);
    let password = safe.find_password(lines);

    assert_eq!(password, 1029);

    println!("Password: {password}");
}
