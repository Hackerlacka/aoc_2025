use anyhow::{Context, Result};
use log::debug;
use regex::Regex;
use std::ops::RangeInclusive;

pub struct Database {
    fresh_id_ranges: Vec<RangeInclusive<u64>>,
    ids: Vec<u64>,
}

impl Database {
    fn parse_range_incl(range_str: &str) -> Result<RangeInclusive<u64>> {
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

        Ok(low..=high)
    }

    fn parse_id(line: &str) -> Result<u64> {
        line.parse().context("Failed to parse line")
    }

    pub fn new(lines: Vec<String>) -> Self {
        let mut fresh_id_ranges = Vec::new();
        let mut ids = Vec::new();

        let mut iter = lines.iter();
        for line in iter.by_ref() {
            if line.is_empty() {
                break;
            }
            fresh_id_ranges.push(Self::parse_range_incl(line).unwrap());
        }

        for line in iter {
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

    fn fuse_range(
        r1: &RangeInclusive<u64>,
        r2: &RangeInclusive<u64>,
    ) -> Option<RangeInclusive<u64>> {
        if (r2.contains(r1.end()) && r1.contains(r2.start())) // r1_s r2_s/r1_e r2_e
            || (r1.contains(r2.end()) && r2.contains(r1.start())) // r2_s r1_s/r2_e r1_e
            || (r1.contains(r2.start()) && r1.contains(r2.end())) // r2 contained inside r1
            || (r2.contains(r1.start()) && r2.contains(r1.end())) // r1 contained inside r2
            || (r1.end() + 1 == *r2.start() || r2.end() + 1 == *r1.start())
        // r1r2 or r2r1
        {
            Some(*r1.start().min(r2.start())..=*r1.end().max(r2.end()))
        } else {
            None
        }
    }

    pub fn find_fresh_ids_tot(&self) -> u64 {
        let mut ranges: Vec<RangeInclusive<u64>> = Vec::new();
        let mut fresh_id_ranges = self.fresh_id_ranges.clone();

        'outer: while let Some(range) = fresh_id_ranges.pop() {
            // Look for overlap against other ranges
            for i in 0..fresh_id_ranges.len() {
                let tmp_range = fresh_id_ranges.get(i).unwrap();
                if let Some(new_range) = Self::fuse_range(&range, tmp_range) {
                    // Overlap found
                    fresh_id_ranges.remove(i);
                    fresh_id_ranges.push(new_range);
                    continue 'outer;
                }
            }

            // No overlap found
            ranges.push(range);
        }

        debug!("ranges: {ranges:?}");
        ranges
            .iter()
            .map(|range| 1 + range.end() - range.start())
            .sum()
    }
}
