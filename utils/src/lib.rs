use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use std::fs::read_to_string;
use anyhow::Result;

/// Input file type
pub enum InputType {
    Regular,
    Example,
    Custom,
}

/// Benchmark a function
/// 
/// The result is automatically printed
pub fn benchmark(fun: fn() -> (), iterations: Option<u32>) {
    let mut duration_tot = Duration::new(0,0);
    let n: u32 = iterations.unwrap_or(1);

    for _ in 0..n {
        let timer = Instant::now();
        fun();
        duration_tot += timer.elapsed();
    }

    let avg_time = duration_tot / n;
    println!("Average time: {:.2?} (n={})", avg_time, n);
}

/// Get all lines in a file
pub fn read_lines<P>(path: P) -> Result<Vec<String>>
where P: AsRef<Path>
{
    Ok(read_to_string(path)?.lines().map(String::from).collect())
}

/// Get all lines in a file (as double ended queue)
pub fn read_lines_deque<P>(path: P) -> Result<VecDeque<String>>
where P: AsRef<Path>
{
    Ok(read_to_string(path)?.lines().map(String::from).collect())
}

/// Get input file path
pub fn get_input_path(day: u32, task: u32, input_type: InputType, number: Option<u32>) -> PathBuf {
    let input_type_str = match input_type {
        InputType::Regular => "",
        InputType::Example => "example_",
        InputType::Custom => "custom_",
    };

    let number_str = match number {
        Some(nbr) => format!("_{nbr}"),
        None => "".to_owned(),
    };

    PathBuf::from(format!("input/{day}_{task}_{input_type_str}input{number_str}.txt"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_lines() -> Result<()> {
        let mut path = std::env::temp_dir();
        path.push("blah12345.txt");
        
        // Test no such file
        assert!(read_lines(&path).is_err());
        
        // Test real file
        let lines = vec!("Line1", "Line2", "Line3");
        std::fs::write(&path, lines.join("\n"))?;

        let read_lines = read_lines(&path)?;
        assert_eq!(lines, read_lines);

        std::fs::remove_file(&path)?;

        Ok(())
    }

    #[test]
    fn test_get_input_path() {
        assert_eq!(get_input_path(1, 1, InputType::Regular, None), PathBuf::from("input/1_1_input.txt"));
        assert_eq!(get_input_path(3, 2, InputType::Regular, None), PathBuf::from("input/3_2_input.txt"));
        assert_eq!(get_input_path(1, 1, InputType::Example, None), PathBuf::from("input/1_1_example_input.txt"));
        assert_eq!(get_input_path(1, 1, InputType::Custom, None), PathBuf::from("input/1_1_custom_input.txt"));
        assert_eq!(get_input_path(1, 1, InputType::Regular, Some(2)), PathBuf::from("input/1_1_input_2.txt"));
    }
}
