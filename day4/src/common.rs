use anyhow::{Context, Result, anyhow};
use log::debug;

#[derive(PartialEq, Clone)]
enum Item {
    PaperRoll,
    None,
}

impl TryFrom<char> for Item {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self> {
        match value {
            '@' => Ok(Self::PaperRoll),
            '.' => Ok(Self::None),
            _ => Err(anyhow!("Cannot parse {} into Item", value)),
        }
    }
}

pub struct PaperRollMap {
    map: Vec<Vec<Item>>,
}

impl PaperRollMap {
    const MAX_NEARBY_ITEMS: u8 = 4; // Lesser than this

    pub fn new(lines: Vec<String>) -> Self {
        let mut map = Vec::new();

        for line in lines {
            let vec: Vec<Item> = line
                .chars()
                .into_iter()
                .map(|c| Item::try_from(c).unwrap())
                .collect();
            map.push(vec);
        }

        Self { map }
    }

    fn get_item(map: &Vec<Vec<Item>>, y: isize, x: isize) -> Result<Item> {
        // Filter out negative numbers
        let y = usize::try_from(y)?;
        let x = usize::try_from(x)?;

        let item = map
            .get(y)
            .context("y {y} outside range")?
            .get(x)
            .context("x {x} outside range")?;

        Ok(item.to_owned())
    }

    fn get_nearby_item_count(map: &Vec<Vec<Item>>, item_y: usize, item_x: usize) -> u8 {
        let item_y = isize::try_from(item_y).unwrap();
        let item_x = isize::try_from(item_x).unwrap();

        let mut item_count = 0;

        for y in -1..2 {
            for x in -1..2 {
                if y == 0 && x == 0 {
                    continue;
                }
                if Self::get_item(map, item_y + y, item_x + x)
                    .is_ok_and(|item| item == Item::PaperRoll)
                {
                    item_count += 1;
                }
            }
        }

        return item_count;
    }

    fn is_paper_roll(map: &Vec<Vec<Item>>, y: usize, x: usize) -> bool {
        Self::get_item(
            map,
            isize::try_from(y).unwrap(), // TODO: Handle better?
            isize::try_from(x).unwrap(),
        )
        .is_ok_and(|item| item == Item::PaperRoll)
    }

    fn accessible_paper_rolls_inner(&self) -> Vec<(usize, usize)> {
        let mut accessible_rolls = Vec::new();

        let height = self.map.len();
        let width = self.map.get(0).unwrap().len();

        for y in 0..height {
            for x in 0..width {
                if Self::is_paper_roll(&self.map, y, x)
                    && Self::get_nearby_item_count(&self.map, y, x) < Self::MAX_NEARBY_ITEMS
                {
                    accessible_rolls.push((y, x));
                    debug!("y {y}, x {x} is accessible");
                }
            }
        }

        return accessible_rolls;
    }

    pub fn accessible_paper_rolls(&self) -> usize {
        self.accessible_paper_rolls_inner().len()
    }

    fn remove_paper_rolls(&mut self, remove: Vec<(usize, usize)>) {
        for (y, x) in remove {
            if let Some(element) = self.map.get_mut(y).unwrap().get_mut(x) {
                *element = Item::None;
            }
        }
    }

    pub fn accessible_paper_rolls_with_remove(&mut self) -> usize {
        let mut tot_accessible_paper_rolls = 0;
        loop {
            let accessible_paper_rolls = self.accessible_paper_rolls_inner();
            if accessible_paper_rolls.len() == 0 {
                break;
            }

            tot_accessible_paper_rolls += accessible_paper_rolls.len();
            self.remove_paper_rolls(accessible_paper_rolls);
        }

        return tot_accessible_paper_rolls;
    }
}
