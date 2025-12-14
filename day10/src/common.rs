use std::collections::{HashMap, HashSet};

use anyhow::{Context, Result, bail};
use log::{debug, info};
use regex::Regex;

#[derive(Debug)]
struct Machine {
    wanted_indicator_lights: Vec<bool>,
    button_wiring: Vec<Vec<usize>>,
    wanted_joltage_levels: Vec<usize>,
}

impl Machine {
    fn parse_indicator_lights(line: &str) -> Result<Vec<bool>> {
        let re = Regex::new(r"[.#]")?;
        let indicator_lights: Vec<Result<bool>> = re
            .find_iter(line)
            .map(|m| {
                Ok(match m.as_str() {
                    "." => false,
                    "#" => true,
                    s => bail!("Unexpected string: {s}"),
                })
            })
            .collect();

        // Get the first Err() or Ok(Vec<bool>)
        indicator_lights.into_iter().collect()
    }

    fn parse_button_wiring(line: &str) -> Result<Vec<Vec<usize>>> {
        let re = Regex::new(r"\([^\s()]+\)")?;
        let button_wiring = re
            .find_iter(line)
            .map(|m| {
                let filtered_str = m.as_str().replace("(", "").replace(")", "");

                filtered_str
                    .split(',')
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect()
            })
            .collect();

        Ok(button_wiring)
    }

    fn parse_joltage(line: &str) -> Result<Vec<usize>> {
        let re = Regex::new(r"\{[^\s()]+\}")?;
        let joltage: Vec<Result<usize>> = re
            .find(line)
            .context("No match")?
            .as_str()
            .replace("{", "")
            .replace("}", "")
            .split(',')
            .map(|s| s.parse::<usize>().context("Parsing failed"))
            .collect();

        joltage.into_iter().collect()
    }

    fn new(line: &str) -> Self {
        let wanted_indicator_lights = Self::parse_indicator_lights(line).unwrap();
        let button_wiring = Self::parse_button_wiring(line).unwrap();
        let wanted_joltage = Self::parse_joltage(line).unwrap();

        Machine {
            wanted_indicator_lights,
            button_wiring,
            wanted_joltage_levels: wanted_joltage,
        }
    }

    fn try_button(
        button_index: usize,
        button_wiring: &Vec<Vec<usize>>,
        indicator_lights: &mut Vec<bool>,
        wanted_indicator_lights: &Vec<bool>,
        path: &mut Vec<usize>,
        solutions: &mut Vec<Vec<usize>>,
    ) {
        // Pressing the same button twice cancels out the effect
        for press in [true, false] {
            // Press button
            let button = &button_wiring.get(button_index).unwrap();
            if press {
                path.push(button_index);
                for j in button.iter().copied() {
                    indicator_lights[j] = !indicator_lights[j];
                }
            }

            if indicator_lights == wanted_indicator_lights {
                solutions.push(path.clone());
            }

            if button_index + 1 < button_wiring.len() {
                Self::try_button(
                    button_index + 1,
                    button_wiring,
                    indicator_lights,
                    wanted_indicator_lights,
                    path,
                    solutions,
                );
            }

            if press {
                // Reset
                path.pop();
                for j in button.iter().copied() {
                    indicator_lights[j] = !indicator_lights[j];
                }
            }
        }
    }

    fn find_solutions(&self, wanted_indicator_lights: &Vec<bool>) -> Vec<Vec<usize>> {
        let mut indicator_lights = vec![false; wanted_indicator_lights.len()];
        let mut path = Vec::new();
        let mut solutions = Vec::new();

        Self::try_button(
            0,
            &self.button_wiring,
            &mut indicator_lights,
            wanted_indicator_lights,
            &mut path,
            &mut solutions,
        );

        solutions
    }

    fn find_fewest_button_presses(&self) -> usize {
        let solutions = self.find_solutions(&self.wanted_indicator_lights);
        solutions
            .iter()
            .map(|solution| solution.len())
            .min()
            .unwrap()
    }

    fn joltage_levels_to_indicator_lights(joltage_levels: &[usize]) -> Vec<bool> {
        joltage_levels
            .iter()
            .map(|joltage| joltage % 2 == 1)
            .collect()
    }

    fn apply_buttons_and_half(
        wanted_joltage_levels: &Vec<usize>,
        solution: &[usize],
        button_wiring: &Vec<Vec<usize>>,
    ) -> Option<Vec<usize>> {
        let mut new_joltage_levels = wanted_joltage_levels.clone();

        // Subtract button presses from joltage level
        for button_index in solution.iter() {
            let button = &button_wiring[*button_index];
            for joltage_index in button {
                if new_joltage_levels[*joltage_index] == 0 {
                    // Joltage level would become negative!
                    return None;
                }
                new_joltage_levels[*joltage_index] -= 1;
            }
        }

        // Half every joltage level
        new_joltage_levels
            .iter_mut()
            .for_each(|joltage_level| *joltage_level /= 2);

        Some(new_joltage_levels)
    }

    fn find_fewest_button_presses_joltage_rec(
        &self,
        wanted_joltage_levels: &Vec<usize>,
        cache: &mut HashMap<Vec<usize>, Option<usize>>,
    ) -> Option<usize> {
        if let Some(min_button_presses) = cache.get(wanted_joltage_levels) {
            return *min_button_presses;
        }

        let mut min_button_presses = None;
        let wanted_indicator_lights =
            Self::joltage_levels_to_indicator_lights(wanted_joltage_levels);
        let solutions = self.find_solutions(&wanted_indicator_lights);
        for solution in solutions.iter() {
            let mut new_button_presses = solution.len();
            if let Some(new_joltage_levels) =
                Self::apply_buttons_and_half(wanted_joltage_levels, solution, &self.button_wiring)
            {
                // Check if all levels are 0
                if !new_joltage_levels
                    .iter()
                    .all(|joltage_level| *joltage_level == 0)
                {
                    match self.find_fewest_button_presses_joltage_rec(&new_joltage_levels, cache) {
                        Some(tmp_min_button_presses) => {
                            new_button_presses += 2 * tmp_min_button_presses
                        }
                        None => continue,
                    }
                }

                if min_button_presses.is_none() {
                    min_button_presses = Some(new_button_presses);
                } else {
                    min_button_presses =
                        std::cmp::min(min_button_presses, Some(new_button_presses));
                }
            }
            // Else case means joltage levels became invalid (negative)
        }

        cache.insert(wanted_joltage_levels.clone(), min_button_presses);

        min_button_presses
    }

    fn find_fewest_button_presses_joltage(&self) -> usize {
        let wanted_joltage_levels = &self.wanted_joltage_levels;
        let mut cache = HashMap::new();
        self.find_fewest_button_presses_joltage_rec(wanted_joltage_levels, &mut cache)
            .unwrap()
    }
}

pub struct Machines {
    machines: Vec<Machine>,
}

impl Machines {
    pub fn new(lines: Vec<String>) -> Self {
        let machines = lines.iter().map(|line| Machine::new(line)).collect();
        debug!("Machines: {machines:?}");
        Self { machines }
    }

    pub fn find_fewest_button_presses(&self) -> Vec<usize> {
        self.machines
            .iter()
            .map(|m| m.find_fewest_button_presses())
            .collect()
    }

    pub fn find_fewest_button_presses_joltage(&self) -> Vec<usize> {
        self.machines
            .iter()
            .map(|m| m.find_fewest_button_presses_joltage())
            .collect()
    }
}
