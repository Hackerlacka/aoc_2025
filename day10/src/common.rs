use anyhow::{Context, Result, bail};
use log::debug;
use regex::Regex;

#[derive(Debug)]
struct Machine {
    wanted_indicator_lights: Vec<bool>,
    button_wiring: Vec<Vec<usize>>,
    wanted_joltage: Vec<usize>,
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
            wanted_joltage,
        }
    }

    fn press_button(
        buttons_pressed: &mut usize,
        allowed_button_presses: usize,
        button_wiring: &mut Vec<Vec<usize>>,
        indicator_lights: &mut Vec<bool>,
        wanted_indicator_lights: &Vec<bool>,
    ) -> bool {
        // Press button
        for i in 0..button_wiring.len() {
            let button = button_wiring.remove(i);
            for j in button.iter().copied() {
                indicator_lights[j] = !indicator_lights[j];
            }

            *buttons_pressed += 1;
            if indicator_lights == wanted_indicator_lights {
                return true;
            }

            if *buttons_pressed == allowed_button_presses {
                // Reset and try next button
                *buttons_pressed -= 1;
                for j in button.iter().copied() {
                    indicator_lights[j] = !indicator_lights[j];
                }
                button_wiring.insert(i, button);
                continue;
            }

            if Self::press_button(
                buttons_pressed,
                allowed_button_presses,
                button_wiring,
                indicator_lights,
                wanted_indicator_lights,
            ) {
                return true;
            }

            // Reset and try next button
            *buttons_pressed -= 1;
            for j in button.iter().copied() {
                indicator_lights[j] = !indicator_lights[j];
            }
            button_wiring.insert(i, button);
        }

        false
    }

    fn find_fewest_button_presses(&self) -> usize {
        // Pressing the same button twice cancels out the effect
        let mut allowed_button_presses = 0;
        let mut button_wiring = self.button_wiring.clone();
        while allowed_button_presses <= self.button_wiring.len() {
            allowed_button_presses += 1;

            let mut buttons_pressed = 0;
            let mut indicator_lights = Vec::new();
            (0..self.wanted_indicator_lights.len()).for_each(|_| indicator_lights.push(false));

            if Self::press_button(
                &mut buttons_pressed,
                allowed_button_presses,
                &mut button_wiring,
                &mut indicator_lights,
                &self.wanted_indicator_lights,
            ) {
                return buttons_pressed;
            }
        }

        panic!("No solution!")
    }

    fn some_recursive_func() {
        // goal_joltage: Vec<usize>

        // Pop 1 button
        // min_joltage = min(joltage, vec<joltages_affected>)
        // Check range 0..=min_joltage
        // joltage = ...
        // Check if joltage == goal_joltage -> return!
        // Check so all conditions are ok, else return
        // Call some_recursive_func() (if there are more buttons left!)
    }

    fn find_fewest_button_presses_joltage(&self) -> usize {
        todo!()
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
