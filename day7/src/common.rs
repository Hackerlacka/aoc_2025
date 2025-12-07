use anyhow::{Result, bail};

#[derive(Debug)]
enum Location {
    Start,
    Empty,
    Splitter,
    TachyonBeam(u64),
}

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::TachyonBeam(_), Self::TachyonBeam(_)) => true, // Ignore the number inside the beam when comparing
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl Location {
    fn new(c: char) -> Result<Self> {
        Ok(match c {
            'S' => Self::Start,
            '.' => Self::Empty,
            '^' => Self::Splitter,
            '|' => Self::TachyonBeam(1), // Should not be any of these on the diagram already
            _ => bail!("Invalid location type: {c}"),
        })
    }
}

pub struct Manifold {
    diagram: Vec<Vec<Location>>,
}

impl Manifold {
    pub fn new(lines: Vec<String>) -> Self {
        let diagram = lines
            .into_iter()
            .map(|line| line.chars().map(|c| Location::new(c).unwrap()).collect())
            .collect();
        Manifold { diagram }
    }

    fn are_all_same_kind(
        diagram: &Vec<Vec<Location>>,
        kind: &Location,
        locations: Vec<(usize, usize)>,
    ) -> bool {
        locations.into_iter().all(|locations| {
            let y = locations.0;
            let x = locations.1;
            let c = diagram.get(y).unwrap().get(x).unwrap();

            c == kind
        })
    }

    fn find_splitters_with_beams(&self) -> u64 {
        let mut count = 0;

        let height = self.diagram.len();
        let width = self.diagram.first().unwrap().len();

        for y in 0..height {
            for x in 0..width {
                let c = self.diagram.get(y).unwrap().get(x).unwrap();
                // Check if c is splitter and it has 1 beam above and 2 to the sides it
                if c == &Location::Splitter
                    && Self::are_all_same_kind(
                        &self.diagram,
                        &Location::TachyonBeam(0),
                        vec![(y - 1, x), (y, x - 1), (y, x + 1)],
                    )
                {
                    count += 1;
                }
            }
        }

        count
    }

    fn fine_timelines(&self) -> u64 {
        self.diagram
            .last()
            .unwrap()
            .iter()
            .map(|location| match location {
                Location::TachyonBeam(i) => *i,
                _ => 0,
            })
            .sum()
    }

    pub fn tachyon_beam_split_count(&mut self, timelines: bool) -> u64 {
        let height = self.diagram.len();
        let width = self.diagram.first().unwrap().len();

        for y in 0..height {
            for x in 0..width {
                let c = self.diagram.get(y).unwrap().get(x).unwrap();
                if c == &Location::TachyonBeam(0) || c == &Location::Start {
                    // TODO: Can do this in a better way?
                    let beam_count = match *c {
                        Location::TachyonBeam(i) => i,
                        Location::Start => 1,
                        _ => panic!(),
                    };
                    // Send beam downwards
                    if y + 1 == height {
                        break;
                    }

                    let c_below = self.diagram.get_mut(y + 1).unwrap().get_mut(x).unwrap();
                    match *c_below {
                        Location::TachyonBeam(i) => {
                            // Merge tachyon beams
                            *c_below = Location::TachyonBeam(i + beam_count)
                        }
                        Location::Empty => *c_below = Location::TachyonBeam(beam_count),
                        Location::Splitter => {
                            // TODO: Would be nice to break out this code
                            if let Some(c_below_left) =
                                self.diagram.get_mut(y + 1).unwrap().get_mut(x - 1)
                            {
                                match *c_below_left {
                                    Location::TachyonBeam(i) => {
                                        *c_below_left = Location::TachyonBeam(i + beam_count)
                                    }
                                    Location::Empty => {
                                        *c_below_left = Location::TachyonBeam(beam_count)
                                    }
                                    Location::Splitter => panic!("Splitter next to splitter!"),
                                    _ => {
                                        panic!(
                                            "Did not expect {c_below_left:?} at y: {}, x: {}",
                                            y + 1,
                                            x - 1
                                        )
                                    }
                                }
                            }
                            if let Some(c_below_right) =
                                self.diagram.get_mut(y + 1).unwrap().get_mut(x + 1)
                            {
                                match *c_below_right {
                                    Location::TachyonBeam(i) => {
                                        *c_below_right = Location::TachyonBeam(i + beam_count)
                                    }
                                    Location::Empty => {
                                        *c_below_right = Location::TachyonBeam(beam_count)
                                    }
                                    Location::Splitter => panic!("Splitter next to splitter!"),
                                    _ => {
                                        panic!(
                                            "Did not expect {c_below_right:?} at y: {}, x: {}",
                                            y + 1,
                                            x + 1
                                        )
                                    }
                                }
                            }
                        }
                        _ => panic!("Did not expect {c_below:?} at y: {}, {x}: x", y + 1),
                    }
                }
            }
        }

        if timelines {
            self.fine_timelines()
        } else {
            self.find_splitters_with_beams()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_location_partial_eq() {
        assert_eq!(&Location::Empty, &Location::Empty);
        assert_eq!(&Location::TachyonBeam(5), &Location::TachyonBeam(1));
        assert_ne!(&Location::Empty, &Location::TachyonBeam(1));
    }
}
