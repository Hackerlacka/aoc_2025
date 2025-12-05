use log::info;
use std::path::PathBuf;
use utils::{InputType, get_input_path};

use crate::common::Database;

fn run_task_on_file(path: &PathBuf) -> Vec<u64> {
    let lines = utils::read_lines(path).unwrap();

    let db = Database::new(lines);

    db.find_fresh_ids()
}

fn run_example() {
    let path = get_input_path(5, 1, InputType::Example, None);
    let fresh_ids = run_task_on_file(&path);
    assert_eq!(fresh_ids, vec![5, 11, 17]);

    let fresh_ids_count = fresh_ids.len();
    assert_eq!(fresh_ids_count, 3);
}

pub fn run_task() {
    run_example();

    let path = get_input_path(5, 1, InputType::Regular, None);
    let fresh_ids = run_task_on_file(&path);

    let fresh_ids_count = fresh_ids.len();
    info!("Fresh ids count: {fresh_ids_count}");
    assert_eq!(fresh_ids_count, 789);
}
