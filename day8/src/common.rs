use anyhow::{Context, Ok, Result};
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Circuit {
    links: Vec<(JunctionBox, JunctionBox)>,
}

impl Circuit {
    fn new() -> Self {
        Circuit { links: Vec::new() }
    }

    fn add_link(&mut self, link: (JunctionBox, JunctionBox)) {
        self.links.push(link);
    }

    fn contains(&self, b: &JunctionBox) -> bool {
        self.links.iter().any(|(b1, b2)| b1 == b || b2 == b)
    }

    fn fuse_circuit(&mut self, mut other: Self) {
        self.links.append(&mut other.links);
    }

    fn get_junction_boxes(&self) -> usize {
        self.links.len() + 1
    }

    pub fn get_last(&self) -> (JunctionBox, JunctionBox) {
        self.links.last().unwrap().clone()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct JunctionBox {
    x: i64,
    y: i64,
    z: i64,
}

impl JunctionBox {
    fn new(line: &str) -> Result<Self> {
        let re = Regex::new(r"(\d+),(\d+),(\d+)")?;

        let caps = re.captures(line).context("No regex match for {line}")?;
        let x: i64 = caps
            .get(1)
            .context("No capture group 1")?
            .as_str()
            .parse::<i64>()?;
        let y: i64 = caps
            .get(2)
            .context("No capture group 2")?
            .as_str()
            .parse::<i64>()?;
        let z: i64 = caps
            .get(3)
            .context("No capture group 3")?
            .as_str()
            .parse::<i64>()?;

        Ok(JunctionBox { x, y, z })
    }

    fn straight_line_distance(&self, other: &JunctionBox) -> i64 {
        // Problematic to hash f64, and i64 seems to be enough for the data
        ((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)).isqrt()
    }

    pub fn get_x_product(&self, other: &Self) -> i64 {
        self.x * other.x
    }
}

pub struct JunctionBoxes {
    boxes: Vec<JunctionBox>,
    distances: HashMap<i64, Vec<(JunctionBox, JunctionBox)>>, // TODO: Work with lifetimes/references?
}

impl JunctionBoxes {
    fn calculate_distances(
        boxes: &Vec<JunctionBox>,
    ) -> HashMap<i64, Vec<(JunctionBox, JunctionBox)>> {
        let mut distances: HashMap<i64, Vec<(JunctionBox, JunctionBox)>> = HashMap::new();
        for (i, b) in boxes.iter().enumerate() {
            for b_other in boxes.iter().skip(i + 1) {
                let distance = b.straight_line_distance(b_other);
                match distances.get_mut(&distance) {
                    Some(vec) => vec.push((b.clone(), b_other.clone())),
                    None => _ = distances.insert(distance, vec![(b.clone(), b_other.clone())]),
                }
            }
        }

        distances
    }

    pub fn new(lines: Vec<String>) -> Self {
        let boxes = lines
            .iter()
            .map(|line| JunctionBox::new(line).unwrap())
            .collect();
        let distances = Self::calculate_distances(&boxes);
        JunctionBoxes { boxes, distances }
    }

    fn connect_jboxes(circuits: &mut Vec<Circuit>, pair: (JunctionBox, JunctionBox)) {
        // Check if they are part of any circuits
        let i1 = {
            circuits
                .iter()
                .enumerate()
                .find(|(_, circuit)| circuit.contains(&pair.0))
                .map(|(i, _)| i)
        };

        let i2 = {
            circuits
                .iter()
                .enumerate()
                .find(|(_, circuit)| circuit.contains(&pair.1))
                .map(|(i, _)| i)
        };

        if let Some(i1) = i1
            && let Some(i2) = i2
        {
            if i1 == i2 {
                // They are already part of the same circuit
                return;
            }

            let i_min = std::cmp::min(i1, i2);
            let i_max = std::cmp::max(i1, i2);
            let c_max_i = circuits.remove(i_max);
            let c_min_i = circuits.get_mut(i_min).unwrap();
            c_min_i.fuse_circuit(c_max_i);
            c_min_i.add_link(pair);
        } else if i1.is_some() || i2.is_some() {
            let i = i1.or(i2).unwrap();
            let circuit = circuits.get_mut(i).unwrap();
            circuit.add_link(pair);
        } else {
            // Create new circuit
            let mut circuit = Circuit::new();
            circuit.add_link(pair);
            circuits.push(circuit);
        }
    }

    pub fn get_circuits(&mut self, connections: Option<usize>) -> Vec<Circuit> {
        let mut circuits: Vec<Circuit> = Vec::new();
        let mut distances_sorted: Vec<i64> = self.distances.keys().copied().collect();
        distances_sorted.sort();
        let mut connections_created = 0;
        'outer: for distance in distances_sorted.iter() {
            let pairs = self.distances.remove(distance).unwrap();
            // There could be more than one pair with the same distance
            for pair in pairs.into_iter() {
                Self::connect_jboxes(&mut circuits, pair);
                if let Some(connections_val) = connections {
                    // Connections are considered "created" even if they are part of the same circuit
                    connections_created += 1;
                    if connections_created == connections_val {
                        break 'outer;
                    }
                } else if circuits.len() == 1
                    && circuits.first().unwrap().get_junction_boxes() == self.boxes.len()
                {
                    // Only for optimization reasons
                    break 'outer;
                }
            }
        }

        // Could add remainder as 'single' circuits, but not required

        circuits
    }

    pub fn get_product_3_largest_circuits(circuits: &Vec<Circuit>) -> usize {
        let mut jbox_cnt_per_circ: Vec<usize> = circuits
            .iter()
            .map(|circuit| circuit.get_junction_boxes())
            .collect();
        jbox_cnt_per_circ.sort();

        assert!(jbox_cnt_per_circ.len() >= 3);
        jbox_cnt_per_circ
            .into_iter()
            .rev()
            .take(3)
            .reduce(|acc, e| acc * e)
            .unwrap()
    }
}
