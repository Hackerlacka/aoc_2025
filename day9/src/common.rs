use anyhow::{Result, bail};
use log::{debug, info};
use regex::Regex;
use std::cmp::{max, min};
use std::{
    collections::HashMap,
    ops::{Add, RangeInclusive},
};

#[derive(Debug, Clone)]
struct Tile {
    x: isize,
    y: isize,
}

impl Tile {
    fn new(line: &str) -> Result<Self> {
        let re = Regex::new(r"(\d+),(\d+)")?;
        if let Some((_, [x, y])) = re.captures(line).map(|caps| caps.extract()) {
            return Ok(Tile {
                x: x.parse()?,
                y: y.parse()?,
            });
        }
        bail!("Failed to map line {line} to Tile");
    }

    fn calculate_area(&self, other: &Self) -> isize {
        // Add 1 to compensate for that coordinates are used
        (self.x - other.x).abs().add(1) * (self.y - other.y).abs().add(1)
    }

    fn get_min_and_max(t1: &Tile, t2: &Tile) -> ((isize, isize), (isize, isize)) {
        let min_x = min(t1.x, t2.x);
        let min_y = min(t1.y, t2.y);
        let max_x = max(t1.x, t2.x);
        let max_y = max(t1.y, t2.y);

        ((min_x, min_y), (max_x, max_y))
    }
}

#[derive(Hash, PartialEq, Eq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
enum Edge {
    X(RangeInclusive<isize>, isize),
    Y(RangeInclusive<isize>, isize),
}

impl Edge {
    fn new(t1: &Tile, t2: &Tile) -> Self {
        let ((min_x, min_y), (max_x, max_y)) = Tile::get_min_and_max(t1, t2);

        if t1.x == t2.x {
            Edge::X(min_y..=max_y, t1.x)
        } else if t1.y == t2.y {
            Edge::Y(min_x..=max_x, t1.y)
        } else {
            panic!();
        }
    }

    fn contains_point(&self, x: &isize, y: &isize) -> bool {
        match self {
            Edge::X(y_range, ex) => ex == x && y_range.contains(y),
            Edge::Y(x_range, ey) => ey == y && x_range.contains(x),
        }
    }

    fn would_intersect(&self, x: &isize, y: &isize) -> Option<Direction> {
        // x and y should not be on the line at this point
        match self {
            Edge::X(y_range, ex) => {
                if !y_range.contains(y) || x == ex {
                    // Ignore those on same x coordinate
                    return None;
                }
                let direction = if x > ex {
                    Direction::Left
                } else {
                    Direction::Right
                };
                Some(direction)
            }
            Edge::Y(x_range, ey) => {
                if !x_range.contains(x) || y == ey {
                    // Ignore those on same y coordinate
                    return None;
                }
                let direction = if y > ey {
                    Direction::Up
                } else {
                    Direction::Down
                };
                Some(direction)
            }
        }
    }
}

pub struct Floor {
    tiles: Vec<Tile>,
    edges: Vec<Edge>,
}

impl Floor {
    pub fn new(lines: Vec<String>) -> Self {
        let tiles: Vec<Tile> = lines.iter().map(|line| Tile::new(line).unwrap()).collect();
        let mut edges: Vec<Edge> = tiles
            .iter()
            .take(tiles.len() - 1)
            .zip(tiles.iter().skip(1))
            .map(|(t1, t2)| Edge::new(t1, t2))
            .collect();
        edges.push(Edge::new(tiles.last().unwrap(), tiles.first().unwrap()));

        Floor { tiles, edges }
    }

    pub fn find_largest_rectangle_area(&self) -> isize {
        let mut max_area = 0;

        for (i, tile) in self.tiles.iter().enumerate() {
            for other_tile in self.tiles.iter().skip(i + 1) {
                let area = tile.calculate_area(other_tile);
                debug!("Area of {tile:?} x {other_tile:?} is {area}");
                if area > max_area {
                    max_area = area;
                }
            }
        }

        max_area
    }

    fn is_point_inside(&self, x: isize, y: isize) -> bool {
        // If point is on a edge (line), then it is "inside"
        if self.edges.iter().any(|edge| edge.contains_point(&x, &y)) {
            return true;
        }

        // Check how many lines we cross in each direction
        let mut direction_counter: HashMap<Direction, u64> = HashMap::new();
        self.edges.iter().for_each(|edge| {
            if let Some(direction) = edge.would_intersect(&x, &y) {
                match direction_counter.get_mut(&direction) {
                    Some(counter) => *counter += 1,
                    None => _ = direction_counter.insert(direction, 1),
                }
            }
        });

        // TODO: There seem to be a problem at e.g. 7x7 in the example, where counters for up is 3 and right is 2
        info!("x={x}, y={y}");
        info!("direction_counter: {direction_counter:?}");
        if direction_counter.values().any(|v| v % 2 == 1) {
            return true;
        }

        if direction_counter.values().any(|v| *v != 0 && v % 2 == 0) {
            return false;
        }

        panic!("Point does not cross any line!")
    }

    fn is_rectangle_inside(&self, t1: &Tile, t2: &Tile) -> bool {
        // "Point in polygon" problem
        // Check if edges (all sides) of rectangle are inside or not

        // TODO: Redundant but might speed up things?
        if !(self.is_point_inside(t1.x, t2.y) && self.is_point_inside(t2.x, t1.y)) {
            return false;
        }

        // Find rectangle corners
        let ((min_x, min_y), (max_x, max_y)) = Tile::get_min_and_max(t1, t2);

        // Check edges along x-axis
        if (min_x..=max_x)
            .any(|x| !self.is_point_inside(x, min_y) || !self.is_point_inside(x, max_y))
        {
            return false;
        }

        // Check edges along y-axis
        if (min_y..=max_y)
            .any(|y| !self.is_point_inside(min_x, y) || !self.is_point_inside(max_x, y))
        {
            return false;
        }

        // All edges (sides) are inside!
        true
    }

    pub fn find_largest_rectangle_area_inside(&self) -> isize {
        let mut max_area_found = 0;
        let loop_max = self.tiles.iter().len();

        for (i, tile) in self.tiles.iter().enumerate() {
            info!("({i}/{loop_max})");
            for other_tile in self.tiles.iter().skip(i + 1) {
                let area = tile.calculate_area(other_tile);
                if area <= max_area_found {
                    // Area is fast to calculate, so skip if area is smaller than max area this far
                    continue;
                }

                if !Self::is_rectangle_inside(self, tile, other_tile) {
                    continue;
                }

                max_area_found = area;
            }
        }

        max_area_found
    }
}
