use crate::read_input_world;
use std::convert::TryFrom;
use Cell::*;

type ChangeFn = fn(&World, usize, usize) -> bool;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Cell {
    Empty,
    Occupied,
    Floor,
}

impl Cell {
    fn flip(&mut self) {
        *self = match self {
            Empty => Occupied,
            Occupied => Empty,
            Floor => Floor,
        };
    }
}

#[derive(Clone)]
struct World {
    cells: Vec<Vec<Cell>>,
    rows: usize,
    cols: usize,
}

impl World {
    fn from_file(f: &str) -> World {
        let cells = read_input_world(
            f,
            &vec![('L', Empty), ('.', Floor)].into_iter().collect(),
        );
        Self {
            rows: cells.len(),
            cols: cells[0].len(),
            cells,
        }
    }

    fn change_cell_part1(&self, row: usize, col: usize) -> bool {
        let neighbors = &self.neighbors(row, col);
        match self.cells[row][col] {
            Occupied => count_occupied(neighbors) >= 4,
            Empty => count_occupied(neighbors) == 0,
            Floor => false,
        }
    }

    fn change_cell_part2(&self, row: usize, col: usize) -> bool {
        let visible = self.occupied_visible(row, col);
        match self.cells[row][col] {
            Occupied => visible >= 5,
            Empty => visible == 0,
            Floor => false,
        }
    }

    fn neighbors(&self, row: usize, col: usize) -> Vec<Cell> {
        let row = isize::try_from(row).unwrap();
        let col = isize::try_from(col).unwrap();
        vec![
            (row - 1, col - 1),
            (row - 1, col),
            (row - 1, col + 1),
            (row, col - 1),
            (row, col + 1),
            (row + 1, col - 1),
            (row + 1, col),
            (row + 1, col + 1),
        ]
        .iter()
        .filter(|(x, y)| {
            *x >= 0 && *x < self.rows as isize && *y >= 0 && *y < self.cols as isize
        })
        .map(|&(x, y)| self.cells[x as usize][y as usize])
        .collect()
    }

    fn occupied_visible(&self, row: usize, col: usize) -> usize {
        [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .iter()
        .filter(|&&step| self.occupied_in_direction(step, row, col))
        .count()
    }

    fn occupied_in_direction(
        &self,
        step: (isize, isize),
        row: usize,
        col: usize,
    ) -> bool {
        let mut res = false;
        let mut row = isize::try_from(row).unwrap() + step.0;
        let mut col = isize::try_from(col).unwrap() + step.1;
        while !res
            && row >= 0
            && row < self.rows as isize
            && col >= 0
            && col < self.cols as isize
            && self.cells[row as usize][col as usize] != Empty
        {
            res = self.cells[row as usize][col as usize] == Occupied;
            row += step.0;
            col += step.1;
        }
        res
    }

    fn step_once(&mut self, func: ChangeFn) -> bool {
        let mut changed = false;
        let snapshot = self.clone();
        for row in 0..self.rows {
            for col in 0..self.cols {
                if func(&snapshot, row, col) {
                    self.cells[row][col].flip();
                    changed = true;
                }
            }
        }
        changed
    }

    fn step(&mut self, func: ChangeFn) {
        let mut changed = true;
        while changed {
            changed = self.step_once(func);
        }
    }

    fn _as_string(&self) -> String {
        let mut s = String::new();
        for row in 0..self.rows {
            for col in 0..self.cols {
                match self.cells[row][col] {
                    Floor => s.push('.'),
                    Empty => s.push('L'),
                    Occupied => s.push('#'),
                }
            }
            s.push('\n');
        }
        s
    }
}

fn count_occupied(s: &Vec<Cell>) -> usize {
    s.iter().filter(|&&x| x == Occupied).count()
}

fn solve_with_change_fn(f: ChangeFn) -> usize {
    let mut w = World::from_file("../input/day11");
    w.step(f);
    w.cells.iter().map(count_occupied).sum::<usize>()
}

pub fn a() -> String {
    solve_with_change_fn(World::change_cell_part1).to_string()
}

pub fn b() -> String {
    solve_with_change_fn(World::change_cell_part2).to_string()
}
