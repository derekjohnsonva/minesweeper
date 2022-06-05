
use std::collections::HashSet;
use std::fmt::Display;
use crate::random::random_range;

pub type Position = (usize, usize);
pub enum OpenResult {
    Mine,
    NoMine(u8),
}

#[derive(Debug)]
pub struct Minesweeper {
    width: usize,
    height: usize,
    open_fields: HashSet<Position>,
    mines: HashSet<Position>,
    flagged_fields: HashSet<Position>,
    lost: bool,
}

impl Display for Minesweeper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = (x, y);
                if !self.open_fields.contains(&pos) {
                    if self.lost && self.mines.contains(&pos) {
                        write!(f, "💣")?;
                    } else if self.flagged_fields.contains(&pos) {
                        write!(f, "🚩")?;
                    } else {
                        write!(f, "🟪")?;
                    }
                } else if self.mines.contains(&pos) {
                    write!(f, "💣")?;
                } else {
                    let mine_count = self.neighboring_mines(pos);
                    if mine_count > 0 {
                        write!(f, "{}", self.neighboring_mines(pos))?;
                    } else {
                        f.write_str("⬜")?;
                    }
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}


impl Minesweeper {
    pub fn new(width: usize, height: usize, mine_count: usize) -> Minesweeper {
        Minesweeper {
            width,
            height,
            open_fields: HashSet::new(),
            mines: {
                let mut mines = HashSet::new();
                while mines.len() < mine_count {
                    mines.insert((random_range(0, width), random_range(0, height)));
                }
                mines
            },
            flagged_fields: HashSet::new(),
            lost: false,
        }
    }

    pub fn neighbors(&self, (x, y): Position) -> impl Iterator<Item = Position> {
        let width = self.width;
        let height = self.height;
        (x.max(1) - 1..=(x + 1).min(width - 1))
            .flat_map(move |i| (y.max(1) - 1..=y.min(height - 1) + 1).map(move |j| (i, j)))
            .filter(move |& pos| pos != (x, y))
    }

    pub fn neighboring_mines(&self, (x, y): Position) -> u8 {
        self.neighbors((x, y))
            .filter(|&pos| self.mines.contains(&pos))
            .count() as u8
    }

    pub fn open(&mut self, position: Position) -> Option<OpenResult> {
        if  self.open_fields.contains(&position) {
            let mine_count = self.neighboring_mines(position);
            let flag_count = self
                .neighbors(position)
                .filter(|neighbor| self.flagged_fields.contains(neighbor))
                .count() as u8;
            if mine_count == flag_count {
                for neighbor in self.neighbors(position) {
                    if !self.flagged_fields.contains(&neighbor)
                        && !self.open_fields.contains(&neighbor)
                    {
                        self.open(neighbor);
                    }
                }
            }
            return None
        }
        if self.lost || self.flagged_fields.contains(&position) 
        {
            return None;
        }

        self.open_fields.insert(position);
        let is_mine = self.mines.contains(&position);
        if is_mine {
            self.lost = true;

            Some(OpenResult::Mine)
        } else {
            let mine_count = self.neighboring_mines(position);
            if mine_count == 0 {
                for neighbor in self.neighbors(position) {
                    if !self.flagged_fields.contains(&neighbor){
                        self.open(neighbor);
                    }
                }
            }
            Some(OpenResult::NoMine(0))
        }
    }

    pub fn toggle_flag(&mut self, pos: Position) {
        if self.lost 
            || self.open_fields.contains(&pos) 
        {
            return;
        }
        if self.flagged_fields.contains(&pos) {
            self.flagged_fields.remove(&pos);
        } else {
            self.flagged_fields.insert(pos);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_minesweeper_test() {
        let num_mines = 1;
        let ms = Minesweeper::new(10, 10, num_mines);
        assert!(ms.mines.len() == num_mines);
    }

    #[test]
    fn test_display() {
        let mut ms = Minesweeper::new(10, 10, 10);
        ms.open((5, 5));
        ms.toggle_flag((6,6));
        println!("{}", ms);
    }
}
