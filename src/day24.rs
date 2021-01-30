use std::collections::HashSet;
use std::fs;
use Dir::*;

enum Dir {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct Tile {
    x: isize,
    y: isize,
    z: isize,
}

impl Tile {
    fn move_to(&self, dir: &Dir) -> Tile {
        let (dx, dy, dz) = match dir {
            E => (1, -1, 0),
            W => (-1, 1, 0),
            NE => (1, 0, -1),
            SW => (-1, 0, 1),
            NW => (0, 1, -1),
            SE => (0, -1, 1),
        };
        Tile {
            x: self.x + dx,
            y: self.y + dy,
            z: self.z + dz,
        }
    }

    fn new(x: isize, y: isize, z: isize) -> Tile {
        Tile { x, y, z }
    }

    fn neighbors_and_me(&self) -> Vec<Tile> {
        let mut res = self.neighbors();
        res.push(*self);
        res
    }

    fn neighbors(&self) -> Vec<Tile> {
        [E, W, NE, NW, SE, SW]
            .iter()
            .map(|dir| self.move_to(dir))
            .collect()
    }
}

struct TileSet(HashSet<Tile>);

impl TileSet {
    fn new() -> Self {
        TileSet(HashSet::new())
    }

    fn flip_tile(&mut self, tile: Tile) {
        if !self.0.remove(&tile) {
            self.0.insert(tile);
        }
    }

    fn flip_from(&mut self, dirs: Vec<Dir>) {
        let start = Tile::new(0, 0, 0);
        let end = dirs.iter().fold(start, |tile, dir| tile.move_to(dir));
        self.flip_tile(end);
    }

    fn is_black(&self, tile: &Tile) -> bool {
        self.0.contains(tile)
    }

    fn count_black_neighbors(&self, tile: &Tile) -> usize {
        tile.neighbors().iter().filter(|x| self.is_black(x)).count()
    }

    fn daily_flip(&self) -> Self {
        TileSet(
            self.0
                .iter()
                .flat_map(|tile| tile.neighbors_and_me())
                .collect::<HashSet<_>>()
                .iter()
                .filter(|tile| {
                    let black_neighbors = self.count_black_neighbors(tile);
                    (self.is_black(tile)
                        && (black_neighbors == 1 || black_neighbors == 2))
                        || (!self.is_black(tile) && black_neighbors == 2)
                })
                .copied()
                .collect(),
        )
    }

    fn count_blacks(&self) -> usize {
        self.0.len()
    }
}

fn dirs_from_str(s: &str) -> Vec<Dir> {
    let mut dirs = Vec::new();
    let mut current = ' ';
    for c in s.chars() {
        match c {
            'e' if current == ' ' => dirs.push(E),
            'e' => {
                if current == 'n' {
                    dirs.push(NE);
                } else {
                    dirs.push(SE)
                }
                current = ' ';
            }
            'w' if current == ' ' => dirs.push(W),
            'w' => {
                if current == 'n' {
                    dirs.push(NW)
                } else {
                    dirs.push(SW)
                }
                current = ' ';
            }
            'n' | 's' => current = c,
            _ => unreachable!(),
        }
    }
    dirs
}

fn read_input(path: &str) -> Vec<Vec<Dir>> {
    fs::read_to_string(path)
        .expect("Error reading file")
        .lines()
        .map(|x| dirs_from_str(x))
        .collect()
}

pub fn a() -> String {
    let moves = read_input("../input/day24");
    let mut tiles = TileSet::new();

    for steps in moves {
        tiles.flip_from(steps);
    }

    tiles.count_blacks().to_string()
}

pub fn b() -> String {
    let moves = read_input("../input/day24");
    let mut tiles = TileSet::new();

    for steps in moves {
        tiles.flip_from(steps);
    }

    for _day in 0..100 {
        tiles = tiles.daily_flip()
    }

    tiles.count_blacks().to_string()
}
