use anyhow::{Context, Ok, Result};
use regex::Regex;
use std::{ops::Range, path::PathBuf};

fn get_invalid_ids(range: Range<u64>, advanced: bool) -> Vec<u64> {
    let mut invalid_ids = Vec::new();

    for number in range {
        let number_str = number.to_string();
        let len = number_str.len();

        if !advanced {
            if len % 2 == 1 {
                continue;
            }

            // TODO: Possible to compare the int directly by dividing with 10^len/2?
            if &number_str[..len / 2] == &number_str[len / 2..] {
                invalid_ids.push(number);
            }
            continue;
        }

        // Try slice sizes where len % slice_size == 0
        for slice_size in 1..(len / 2 + 1) {
            if len % slice_size == 1 {
                continue;
            }

            let slice = &number_str[0..slice_size];
            // TODO: Can this be verified without allocating memory with repeat()?
            if slice.repeat(len / slice_size) == number_str {
                invalid_ids.push(number);
                break;
            }
        }
    }

    return invalid_ids;
}

fn parse_range(range_str: &str) -> Result<Range<u64>> {
    let re = Regex::new(r"(\d+)-(\d+)")?;

    let caps = re
        .captures(range_str)
        .context("No regex match for {range_str}")?;
    let low: u64 = caps
        .get(1)
        .context("No capture group 1")?
        .as_str()
        .parse::<u64>()?;
    let high: u64 = caps
        .get(2)
        .context("No capture group 2")?
        .as_str()
        .parse::<u64>()?;

    Ok(low..high + 1)
}

pub fn run_task_on_file(path: PathBuf, advanced: bool) -> u64 {
    let line = utils::read_lines(path).unwrap().remove(0);

    let ranges_strs: Vec<&str> = line.split(',').collect();
    let ranges: Vec<Range<u64>> = ranges_strs
        .into_iter()
        .map(|range_str| parse_range(range_str).unwrap())
        .collect();

    let mut invalid_ids: Vec<u64> = Vec::new();
    ranges
        .into_iter()
        .for_each(|range| invalid_ids.append(&mut get_invalid_ids(range, advanced)));

    invalid_ids.into_iter().sum()
}
