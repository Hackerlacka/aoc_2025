use anyhow::{Context, Result, anyhow};
use regex::Regex;
use std::ops::Range;

pub struct Database {
    fresh_id_ranges: Vec<Range<u64>>,
    ids: Vec<u64>,
}

impl Database {
    fn parse_range_incl(range_str: &str) -> Result<Range<u64>> {
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

    fn parse_id(line: &str) -> Result<u64> {
        line.parse()
            .map_err(|err| Err(anyhow!("Failed to parse line: {err}")).unwrap())
    }

    pub fn new(lines: Vec<String>) -> Self {
        let mut fresh_id_ranges = Vec::new();
        let mut ids = Vec::new();

        let mut iter = lines.iter();

        while let Some(line) = iter.next() {
            if line.is_empty() {
                break;
            }
            fresh_id_ranges.push(Self::parse_range_incl(line).unwrap());
        }

        while let Some(line) = iter.next() {
            ids.push(Self::parse_id(line).unwrap());
        }

        Database {
            fresh_id_ranges,
            ids,
        }
    }

    pub fn find_fresh_ids(&self) -> Vec<u64> {
        let mut fresh_ids = Vec::new();

        for id in self.ids.iter() {
            for fresh_range in self.fresh_id_ranges.iter() {
                if fresh_range.contains(id) {
                    fresh_ids.push(*id);
                    break;
                }
            }
        }

        fresh_ids
    }

    pub fn find_fresh_ids_tot(&self) -> u64 {
        let ranges: Vec<Range<u64>> = Vec::new();

        for range in self.fresh_id_ranges.iter() {}

        todo!()
    }
}
