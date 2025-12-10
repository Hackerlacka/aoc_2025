use anyhow::{Result, bail};
use log::{debug, info};
use regex::Regex;
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
    fn new(a: &Tile, b: &Tile) -> Self {
        if a.x == b.x {
            Edge::X(std::cmp::min(a.y, b.y)..=std::cmp::max(a.y, b.y), a.x)
        } else if a.y == b.y {
            Edge::Y(std::cmp::min(a.x, b.x)..=std::cmp::max(a.x, b.x), a.y)
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
        // TODO: Does it make sense to instead check a whole line? But how?

        // If point is on line, then it is "inside"
        if self.edges.iter().any(|edge| edge.contains_point(&x, &y)) {
            //info!("A: x={x} y={y} is inside (on edge)");
            return true;
        }

        // TODO: Otherwise count how many lines we pass in some direction that is not 0, and check if odd or even number
        let mut direction_counter: HashMap<Direction, u64> = HashMap::new();
        self.edges.iter().for_each(|edge| {
            if let Some(direction) = edge.would_intersect(&x, &y) {
                match direction_counter.get_mut(&direction) {
                    Some(counter) => *counter += 1,
                    None => _ = direction_counter.insert(direction, 1),
                }
            }
        });

        if direction_counter.values().any(|v| v % 2 == 1) {
            //info!("B: x={x} y={y} is inside");
            // if x == 2 && y == 1 {
            //     info!("direction_counter: {direction_counter:?}");
            //     info!("edges: {:?}", self.edges);
            // }
            return true;
        }

        if direction_counter.values().any(|v| *v != 0 && v % 2 == 0) {
            return false;
        }

        panic!("Point does not cross any line!")
    }

    fn is_rectangle_inside(&self, t1: &Tile, t2: &Tile) -> bool {
        // "Point in polygon" problem
        // Check if corners and edges of rectangle are inside or not
        // (Check edges to cover an U-shaped edge.)
        // TODO: Could become many tiles to check per rectangle? Check edges first? Or some cross pattern

        // Redundant but might speed up things?
        if !(self.is_point_inside(t1.x, t2.y) && self.is_point_inside(t2.x, t1.y)) {
            return false;
        }

        let min_x = std::cmp::min(t1.x, t2.x);
        let min_y = std::cmp::min(t1.y, t2.y);
        let max_x = std::cmp::max(t1.x, t2.x);
        let max_y = std::cmp::max(t1.y, t2.y);

        // let combinations = (max_x - min_x + 1) * 2 + (max_y - min_y + 1) * 2;
        // info!("Combinations: {combinations}");

        if (min_x..=max_x)
            .any(|x| !self.is_point_inside(x, min_y) || !self.is_point_inside(x, max_y))
        {
            return false;
        }

        if (min_y..=max_y)
            .any(|y| !self.is_point_inside(min_x, y) || !self.is_point_inside(max_x, y))
        {
            return false;
        }

        true

        //self.is_point_inside(t1.x, t2.y) && self.is_point_inside(t2.x, t1.y)
    }

    pub fn find_largest_rectangle_area_inside(&self) -> isize {
        let mut max_area = 0;
        let mut rectangles_inside: u64 = 0;
        let loop_max = self.tiles.iter().len();
        for (i, tile) in self.tiles.iter().enumerate() {
            for other_tile in self.tiles.iter().skip(i + 1) {
                let area = tile.calculate_area(other_tile);
                if area <= max_area {
                    // Area is fast to calculate, so skip if area is smaller than max area this far
                    continue;
                }

                if !Self::is_rectangle_inside(self, tile, other_tile) {
                    continue;
                }
                rectangles_inside += 1;
                // if rectangles_inside % 100 == 0 {
                //     info!("rectangles_inside: {rectangles_inside}");
                // }
                info!("({i}/{loop_max}): rectangles_inside: {rectangles_inside}");
                //info!("Area of {tile:?} x {other_tile:?} is {area}");
                max_area = area;
            }
        }

        info!("Rectangles inside: {rectangles_inside}");

        max_area
    }
}
