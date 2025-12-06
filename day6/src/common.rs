use anyhow::{Context, Result, bail};
use regex::Regex;
use std::ops::{Add, Mul};

type HomeworkOperator = fn(u64, u64) -> u64;

struct Homework {
    numbers: Vec<u64>,
    operator: HomeworkOperator,
}

impl Homework {
    fn do_homework(&self) -> u64 {
        // cloned() returns an iterator that clones all the elements
        self.numbers.iter().cloned().reduce(self.operator).unwrap()
    }
}

pub struct Homeworks {
    homeworks: Vec<Homework>,
}

impl Homeworks {
    fn line_to_operators(line: &str) -> Result<Vec<HomeworkOperator>> {
        let re = Regex::new(r"[+*]")?;

        let operators: Vec<Result<HomeworkOperator>> = re
            .find_iter(line)
            .map(|m| {
                Ok(match m.as_str() {
                    "+" => u64::add,
                    "*" => u64::mul,
                    op => bail!("Invalid operator {op}"),
                })
            })
            .collect();

        // Get the first Err() or Ok(Vec<HomeworkOperator>)
        operators.into_iter().collect()
    }

    fn line_to_numbers(line: &str) -> Result<Vec<u64>> {
        let re = Regex::new(r"\d+")?;
        let numbers: Vec<Result<u64>> = re
            .find_iter(line)
            .map(|m| m.as_str().parse().context("Could not parse number"))
            .collect();

        // Get the first Err() or Ok(Vec<u64>)
        numbers.into_iter().collect()
    }

    pub fn new(lines: Vec<String>) -> Self {
        let mut homework_numbers: Vec<Vec<u64>> = Vec::new();

        let mut iter = lines.iter();
        // Parse and save numbers
        for line in iter.by_ref().take(lines.len() - 1) {
            let numbers = Self::line_to_numbers(line).unwrap();
            for (i, number) in numbers.iter().enumerate() {
                match homework_numbers.get_mut(i) {
                    Some(homework) => homework.push(*number),
                    None => homework_numbers.push(vec![*number]),
                }
            }
        }

        // Parse operators
        let operators = Self::line_to_operators(iter.next().unwrap()).unwrap();

        let homeworks = homework_numbers
            .into_iter()
            .zip(operators)
            .map(|(numbers, operator)| Homework { numbers, operator })
            .collect();

        Self { homeworks }
    }

    fn transpose(lines: Vec<String>) -> Vec<String> {
        let tmp: Vec<Vec<char>> = lines.iter().map(|s| s.chars().rev().collect()).collect();
        let height = lines.len();
        let width = lines.first().unwrap().len();
        let mut new_lines = Vec::new();
        for x in 0..width {
            let mut chars = Vec::new();
            for y in 0..height {
                chars.push(*tmp.get(y).unwrap().get(x).unwrap());
            }
            new_lines.push(chars.iter().collect());
        }

        new_lines
    }

    pub fn new_pt2(mut lines: Vec<String>) -> Self {
        // Pop off last line and parse operators
        let operators: Vec<HomeworkOperator> = Self::line_to_operators(&lines.pop().unwrap())
            .unwrap()
            .into_iter()
            .rev()
            .collect();

        // Transpose lines ("-90 deg") and parse homework numbers
        let lines = Self::transpose(lines);
        let mut homework_numbers: Vec<Vec<u64>> = Vec::new();
        let mut numbers: Vec<u64> = Vec::new();
        for line in lines.iter() {
            let line = line.trim();
            if line.is_empty() {
                homework_numbers.push(numbers);
                numbers = Vec::new();
                continue;
            }
            numbers.push(line.parse().unwrap());
        }
        homework_numbers.push(numbers); // Catch last group

        // Create homeworks
        let homeworks = homework_numbers
            .into_iter()
            .zip(operators)
            .map(|(numbers, operator)| Homework { numbers, operator })
            .collect();

        Self { homeworks }
    }

    pub fn do_homeworks(&self, verify: Option<Vec<u64>>) -> u64 {
        let done_homeworks: Vec<u64> = self
            .homeworks
            .iter()
            .map(|homework| homework.do_homework())
            .collect();

        if let Some(v) = verify {
            assert_eq!(done_homeworks, v);
        }

        done_homeworks.iter().sum()
    }
}
