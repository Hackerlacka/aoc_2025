use std::path::PathBuf;

struct BatteryBank {
    line: String,
}

impl BatteryBank {
    fn new(line: &str) -> BatteryBank {
        BatteryBank {
            line: line.to_owned(),
        }
    }

    fn find_max_digit(slice: &[char]) -> (usize, char) {
        let mut max = None;

        for (i, c) in slice.iter().enumerate() {
            if max.is_none() {
                max = Some((i, *c));
                continue;
            }

            if let Some((_, max_c)) = max
                && &max_c < c
            {
                max = Some((i, *c));
            }
        }

        return max.unwrap();
    }

    fn get_largest_joltage(&self, batteries_per_bank: usize) -> u64 {
        let mut joltages: Vec<char> = Vec::new();
        let chars: Vec<char> = self.line.chars().collect();
        let mut start_index = 0;
        let mut end_index = chars.len() - batteries_per_bank; // Inclusive

        // Limit which characters/digits to look at for each battery
        for _ in 0..batteries_per_bank {
            let slice = &chars[start_index..(end_index + 1)];
            let (slice_index, c) = Self::find_max_digit(slice);

            start_index += slice_index + 1;
            joltages.push(c);
            end_index += 1;
        }

        let max_joltage_str: String = joltages.iter().collect();

        return max_joltage_str.parse().unwrap();
    }
}

pub fn run_task_on_file(path: &PathBuf, batteries_per_bank: usize) -> (Vec<u64>, u64) {
    let lines = utils::read_lines(path).unwrap();

    let battery_banks: Vec<BatteryBank> =
        lines.iter().map(|s| BatteryBank::new(s.as_str())).collect();
    let max_joltage: Vec<u64> = battery_banks
        .iter()
        .map(|b| b.get_largest_joltage(batteries_per_bank))
        .collect();

    let max_joltage_sum = max_joltage.iter().sum();

    (max_joltage, max_joltage_sum)
}
