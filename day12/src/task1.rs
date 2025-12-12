use std::usize;

use log::info;
use utils::{InputType, get_input_path};

#[derive(Debug)]
struct Shape {
    parts: Vec<(usize, usize)>,
    variant_parts: Vec<Vec<(usize, usize)>>,
}

impl Shape {
    const VARIANTS: usize = 8; // 4 rotations * 2 for flips

    fn flip(parts: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
        // 00 10 20     20 10 00
        // 01 11 21 --> 21 11 01
        // 02 12 22     22 12 02
        parts
            .iter()
            .map(|part| match part {
                (0, 0) => (2, 0),
                (1, 0) => (1, 0),
                (2, 0) => (0, 0),
                (0, 1) => (2, 1),
                (1, 1) => (1, 1),
                (2, 1) => (0, 1),
                (0, 2) => (2, 2),
                (1, 2) => (1, 2),
                (2, 2) => (0, 2),
                _ => panic!(),
            })
            .collect()
    }

    fn rotate_90_deg(parts: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
        // 00 10 20     02 01 00
        // 01 11 21 --> 12 11 10
        // 02 12 22     22 21 20
        parts
            .iter()
            .map(|part| match part {
                (0, 0) => (0, 2),
                (1, 0) => (0, 1),
                (2, 0) => (0, 0),
                (0, 1) => (1, 2),
                (1, 1) => (1, 1),
                (2, 1) => (1, 0),
                (0, 2) => (2, 2),
                (1, 2) => (2, 1),
                (2, 2) => (2, 0),
                _ => panic!(),
            })
            .collect()
    }

    fn create_variants(parts: &Vec<(usize, usize)>) -> Vec<Vec<(usize, usize)>> {
        let mut variants = Vec::new();
        let mut tmp_parts = parts.clone();
        for _ in 0..4 {
            let rotated_parts = Self::rotate_90_deg(&tmp_parts);
            let flipped_parts = Self::flip(&rotated_parts);

            tmp_parts = rotated_parts.clone();

            variants.push(rotated_parts);
            variants.push(flipped_parts);
        }
        variants
    }

    fn new(lines: &Vec<&String>) -> Self {
        let parts = lines
            .iter()
            .enumerate()
            .map(|(length, line)| {
                let vec: Vec<(usize, usize)> = line
                    .char_indices()
                    .filter_map(|(width, c)| {
                        if c == '#' {
                            Some((width, length))
                        } else {
                            None
                        }
                    })
                    .collect();
                vec
            })
            .reduce(|acc, e| {
                // TODO: Silly to clone here...
                let mut acc_clone = acc.clone();
                acc_clone.append(&mut e.clone());
                acc_clone
            })
            .unwrap();

        let variant_parts = Self::create_variants(&parts);

        Shape {
            parts,
            variant_parts,
        }
    }

    fn area(&self) -> usize {
        self.parts.len()
    }
}

#[derive(Debug)]

struct Region {
    width: usize,
    length: usize,
    shape_counts: Vec<usize>,
}

impl Region {
    fn new(line: &str) -> Self {
        let mut colon_split = line.split(":");
        let mut width_length_split = colon_split.next().unwrap().split("x");
        let width = width_length_split.next().unwrap().parse().unwrap();
        let length = width_length_split.next().unwrap().parse().unwrap();

        let shape_counts = colon_split
            .next()
            .unwrap()
            .trim()
            .split(" ")
            .map(|s| s.parse().unwrap())
            .collect();

        Region {
            width,
            length,
            shape_counts,
        }
    }

    fn place_shape(
        place: bool,
        shape: &Shape,
        variant: usize,
        x: usize,
        y: usize,
        board: &mut Vec<Vec<bool>>,
    ) {
        shape.variant_parts[variant]
            .iter()
            .for_each(|(part_x, part_y)| board[y + *part_y][x + *part_x] = place);
    }

    fn can_place_shape(
        shape: &Shape,
        variant: usize,
        x: usize,
        y: usize,
        width: usize,
        length: usize,
        board: &Vec<Vec<bool>>,
    ) -> bool {
        !shape.variant_parts[variant].iter().any(|(part_x, part_y)| {
            let res_y = y + *part_y;
            let res_x = x + *part_x;
            res_y >= length || res_x >= width || board[res_y][res_x]
        })
    }

    fn place_next_shape(&mut self, shapes: &Vec<Shape>, board: &mut Vec<Vec<bool>>) -> bool {
        let index = match self.shape_counts.iter().position(|counts| *counts > 0) {
            Some(i) => i,
            None => return true, // Found solution, return!
        };

        self.shape_counts[index] -= 1;
        let shape = &shapes[index];
        //info!("Shape counts={:?}", self.shape_counts);

        // Try all possible positions
        for x in 0..self.width {
            for y in 0..self.length {
                // Try all possible rotations and flips
                for variant_index in 0..Shape::VARIANTS {
                    //info!("Trying x={x}, y={y}, variant_index={variant_index}");
                    if !Self::can_place_shape(
                        shape,
                        variant_index,
                        x,
                        y,
                        self.width,
                        self.length,
                        board,
                    ) {
                        continue;
                    }

                    Self::place_shape(true, shape, variant_index, x, y, board);
                    if self.place_next_shape(shapes, board) {
                        return true;
                    }
                    Self::place_shape(false, shape, variant_index, x, y, board);
                }
            }
        }

        self.shape_counts[index] += 1;
        false
    }

    fn does_all_shapes_fit(&mut self, shapes: &Vec<Shape>) -> bool {
        info!("Region: {self:?}");
        let mut board = vec![vec![false; self.width]; self.length];
        self.place_next_shape(shapes, &mut board)
    }

    fn is_area_enough(&self, shapes: &Vec<Shape>) -> bool {
        let area_req: usize = self
            .shape_counts
            .iter()
            .enumerate()
            .map(|(i, count)| shapes[i].area() * *count)
            .sum();
        let area_avail = self.width * self.length;
        area_avail >= area_req
    }
}

#[derive(Debug)]
struct Task {
    shapes: Vec<Shape>,
    regions: Vec<Region>,
}

impl Task {
    fn new(lines: Vec<String>) -> Self {
        let switch_index = lines.iter().position(|line| line.contains("x")).unwrap();

        let mut shapes = Vec::new();
        let mut shape_lines = Vec::new();
        for line in lines.iter().take(switch_index) {
            if !line.is_empty() {
                if !line.contains(":") {
                    // Skip line with number e.g. "0:"
                    shape_lines.push(line);
                }
                continue;
            }
            shapes.push(Shape::new(&shape_lines));
            shape_lines.clear();
        }

        let region_lines = lines.iter().skip(switch_index);
        let regions: Vec<Region> = region_lines.map(|line| Region::new(line)).collect();

        // for shape in shapes.iter() {
        //     info!("Shape: {shape:?}");
        // }
        // for region in regions.iter() {
        //     info!("Region: {region:?}");
        // }

        Task { shapes, regions }
    }

    fn simple_area_check_region_remove(shapes: &Vec<Shape>, regions: &mut Vec<Region>) {
        // Seems to remove ~40% of real input regions!
        info!("Regions before trim: {}", regions.len());
        regions.retain(|region| region.is_area_enough(shapes));
        info!("Regions after trim: {}", regions.len());
    }

    fn regions_that_fit_all(&mut self) -> usize {
        Self::simple_area_check_region_remove(&self.shapes, &mut self.regions);

        self.regions
            .retain_mut(|region| region.does_all_shapes_fit(&self.shapes));

        self.regions.len()
    }
}

fn run_custom() {
    let path = get_input_path(12, 1, InputType::Custom, Some(1));
    let lines = utils::read_lines(path).unwrap();

    let mut task = Task::new(lines);
    let regions_that_fit_all = task.regions_that_fit_all();
    assert_eq!(regions_that_fit_all, 2);
}

fn run_example() {
    let path = get_input_path(12, 1, InputType::Example, None);
    let lines = utils::read_lines(path).unwrap();

    // TODO: Solution is slow if shapes do not fit!

    let mut task = Task::new(lines);
    let regions_that_fit_all = task.regions_that_fit_all();
    assert_eq!(regions_that_fit_all, 2);
}

pub fn run_task() {
    run_custom();
    run_example();

    let path = get_input_path(12, 1, InputType::Regular, None);
    let lines = utils::read_lines(path).unwrap();

    let mut task = Task::new(lines);
    let regions_that_fit_all = task.regions_that_fit_all();
    assert_eq!(regions_that_fit_all, 599);
    info!("Regions that fit all shapes: {regions_that_fit_all}");
}
