use crate::read_input_world;
use std::collections::HashSet;
use std::hash::Hash;

const ROUNDS: usize = 6;

trait Cell<T> {
    fn cell_from_xy(x: isize, y: isize) -> T;
    fn neighbors(&self) -> HashSet<T>;
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Cell3D(isize, isize, isize);

#[derive(PartialEq, Eq, Hash, Clone)]
struct Cell4D(isize, isize, isize, isize);

impl Cell<Cell3D> for Cell3D {
    fn cell_from_xy(x: isize, y: isize) -> Self {
        Self(x, y, 0)
    }

    fn neighbors(&self) -> HashSet<Self> {
        let mut v = HashSet::new();
        for x in self.0 - 1..=self.0 + 1 {
            for y in self.1 - 1..=self.1 + 1 {
                for z in self.2 - 1..=self.2 + 1 {
                    v.insert(Cell3D(x, y, z));
                }
            }
        }
        v.remove(self);
        v
    }
}

impl Cell<Cell4D> for Cell4D {
    fn cell_from_xy(x: isize, y: isize) -> Self {
        Self(x, y, 0, 0)
    }

    fn neighbors(&self) -> HashSet<Self> {
        let mut v = HashSet::new();
        for x in self.0 - 1..=self.0 + 1 {
            for y in self.1 - 1..=self.1 + 1 {
                for z in self.2 - 1..=self.2 + 1 {
                    for w in self.3 - 1..=self.3 + 1 {
                        v.insert(Cell4D(x, y, z, w));
                    }
                }
            }
        }
        v.remove(self);
        v
    }
}

fn activated<T: Eq + Hash + Cell<T>>(c: &T, active: &HashSet<T>) -> bool {
    let active_neighbors =
        c.neighbors().iter().filter(|x| active.contains(x)).count();
    if active.contains(c) {
        active_neighbors == 2 || active_neighbors == 3
    } else {
        active_neighbors == 3
    }
}

fn step<T: Eq + Hash + Clone + Cell<T>>(active: &HashSet<T>) -> HashSet<T> {
    active
        .iter()
        .fold(HashSet::new(), |acc, x| {
            acc.union(&x.neighbors()).cloned().collect()
        })
        .union(active)
        .filter(|x| activated(*x, active))
        .cloned()
        .collect()
}

fn read_input<T: Eq + Hash + Cell<T>>(f: &str) -> HashSet<T> {
    let mut active: HashSet<T> = HashSet::new();
    let legend = [('#', true), ('.', false)].iter().cloned().collect();
    let world = read_input_world(f, &legend);
    for (y, line) in world.iter().enumerate() {
        for (x, cell) in line.iter().enumerate() {
            if *cell {
                active.insert(T::cell_from_xy(x as isize, y as isize));
            }
        }
    }
    active
}

fn run<T: Eq + Hash + Clone + Cell<T>>() -> usize {
    let mut active: HashSet<T> = read_input("../input/day17");
    for _ in 0..ROUNDS {
        active = step(&active);
    }
    active.iter().count()
}

pub fn a() -> String {
    run::<Cell3D>().to_string()
}

pub fn b() -> String {
    run::<Cell4D>().to_string()
}
