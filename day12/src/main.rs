mod task1;
mod task2;

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    //task1::run_task();
    utils::benchmark(task1::run_task, None);
    //task2::run_task();
    //utils::benchmark(task2::run_task, None);
}
