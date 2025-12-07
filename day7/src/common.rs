use anyhow::{Result, bail};

#[derive(Debug, PartialEq)]
enum Location {
    Start,
    Empty,
    Splitter,
    TachyonBeam(u64),
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
        location_kind: &Location,
        all_coordinates: Vec<(usize, usize)>,
    ) -> bool {
        all_coordinates
            .into_iter()
            .all(|coordinates: (usize, usize)| {
                let y = coordinates.0;
                let x = coordinates.1;
                let c = &diagram[y][x];

                // Ignores the number inside the TachyonBeam
                matches!(
                    (c, location_kind),
                    (&Location::TachyonBeam(_), &Location::TachyonBeam(_))
                )
            })
    }

    fn find_splitters_with_beams(&self) -> u64 {
        let mut count = 0;

        let height = self.diagram.len();
        let width = self.diagram.first().unwrap().len();

        for y in 0..height {
            for x in 0..width {
                let c = &self.diagram[y][x];
                // Check if c is splitter and it has 1 beam above and 2 to the sides it
                if c == &Location::Splitter
                    && Self::are_all_same_kind(
                        &self.diagram,
                        // It feels a bit silly to create a new Beam here when
                        // we just want to compare the type. I have looked at
                        // std::mem::discriminant, but it still does not fully
                        // avoid this...
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

    fn find_timelines(&self) -> u64 {
        // Simply count the numbers inside the tachyon beams on the last line
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

    fn check_left_right(&mut self, (y, x): (usize, usize), beam_count: u64) {
        let c_left_right = match self.diagram[y].get_mut(x) {
            Some(c_left_right) => c_left_right,
            None => return,
        };

        match *c_left_right {
            Location::TachyonBeam(i) => *c_left_right = Location::TachyonBeam(i + beam_count),
            Location::Empty => *c_left_right = Location::TachyonBeam(beam_count),
            Location::Splitter => panic!("Splitter next to splitter!"),
            _ => {
                panic!("Did not expect {c_left_right:?} at y: {y}, x: {x}",)
            }
        }
    }

    pub fn tachyon_beam_split_count(&mut self, timelines: bool) -> u64 {
        let height = self.diagram.len();
        let width = self.diagram.first().unwrap().len();

        for y in 0..height {
            for x in 0..width {
                let c = &self.diagram[y][x];
                let beam_count = match *c {
                    Location::TachyonBeam(i) => i,
                    Location::Start => 1,
                    _ => continue,
                };

                // Check if we have reached the bottom
                if y + 1 == height {
                    break;
                }

                // Send the beam downwards
                let c_below = &mut self.diagram[y + 1][x];
                match *c_below {
                    Location::TachyonBeam(i) => *c_below = Location::TachyonBeam(i + beam_count),
                    Location::Empty => *c_below = Location::TachyonBeam(beam_count),
                    Location::Splitter => {
                        self.check_left_right((y + 1, x - 1), beam_count);
                        self.check_left_right((y + 1, x + 1), beam_count);
                    }
                    _ => panic!("Did not expect {c_below:?} at y: {}, {x}: x", y + 1),
                }
            }
        }

        if timelines {
            self.find_timelines()
        } else {
            self.find_splitters_with_beams()
        }
    }
}
