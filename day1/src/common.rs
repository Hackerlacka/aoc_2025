use anyhow::{Error, Ok, anyhow, bail};
use log::debug;

#[derive(Debug)]
pub enum Rotation {
    Left(i16),
    Right(i16),
}

impl TryFrom<String> for Rotation {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() < 2 {
            bail!("Line is too short!");
        }

        let direction = &value[0..1];
        let clicks = value[1..].parse::<i16>()?;

        match direction {
            "L" => Ok(Self::Left(clicks)),
            "R" => Ok(Self::Right(clicks)),
            c => bail!("Invalid first char: {c}"),
        }
    }
}

pub struct Safe {
    dial: i16,
    special_method: bool,
}

impl Safe {
    const DEFAULT_DIAL: i16 = 50;
    const MAX_DIAL: i16 = 99;
    const DIAL_CNT: i16 = Self::MAX_DIAL + 1; // 0 to MAX_DIAL

    pub fn new(special_method: bool) -> Self {
        Safe {
            dial: Self::DEFAULT_DIAL,
            special_method,
        }
    }

    fn turn(&mut self, rotation: Rotation) -> u32 {
        let clicks = match rotation {
            Rotation::Left(clicks) => -clicks,
            Rotation::Right(clicks) => clicks,
        };

        let mut zero_passed_cnt = (clicks / Self::DIAL_CNT).abs();
        let remainder_clicks = clicks % Self::DIAL_CNT;

        if self.dial != 0 && !(1..Self::DIAL_CNT).contains(&(self.dial + remainder_clicks)) {
            zero_passed_cnt += 1;
        }

        self.dial = (self.dial + clicks).rem_euclid(Self::DIAL_CNT);

        if self.special_method {
            zero_passed_cnt.try_into().unwrap()
        } else if self.dial == 0 {
            1
        } else {
            0
        }
    }

    pub fn find_password(&mut self, lines: Vec<String>) -> u32 {
        let rotations: Vec<Rotation> = lines
            .into_iter()
            .map(|line| line.try_into().unwrap())
            .collect();

        debug!("Rotations: {rotations:?}");

        let mut zeroes = 0;
        for rotation in rotations {
            zeroes += self.turn(rotation);
        }

        zeroes
    }
}
