use crate::read_input_world;
use std::collections::HashSet;

const ROUNDS: usize = 6;

type Cell = (isize, isize, isize);
type ActiveCells = HashSet<Cell>;

fn neighbors(c: &Cell) -> HashSet<Cell> {
    let mut v = HashSet::new();
    for x in c.0 - 1..=c.0 + 1 {
        for y in c.1 - 1..=c.1 + 1 {
            for z in c.2 - 1..=c.2 + 1 {
                v.insert((x, y, z));
            }
        }
    }
    v.remove(&c);
    v
}

fn activated(c: &Cell, active: &ActiveCells) -> bool {
    let active_neighbors =
        neighbors(c).iter().filter(|x| active.contains(x)).count();
    if active.contains(c) {
        active_neighbors == 2 || active_neighbors == 3
    } else {
        active_neighbors == 3
    }
}

fn step(active: &ActiveCells) -> ActiveCells {
    active
        .iter()
        .fold(HashSet::new(), |acc, x| {
            acc.union(&neighbors(x)).cloned().collect()
        })
        .union(active)
        .filter(|x| activated(*x, active))
        .cloned()
        .collect()
}

fn read_input(f: &str) -> ActiveCells {
    let mut active: ActiveCells = HashSet::new();
    let legend = [('#', true), ('.', false)].iter().cloned().collect();
    let world = read_input_world(f, &legend);
    for (y, line) in world.iter().enumerate() {
        for (x, cell) in line.iter().enumerate() {
            if *cell {
                active.insert((x as isize, y as isize, 0));
            }
        }
    }
    active
}

pub fn a() -> String {
    let mut active = read_input("../input/day17");
    for _ in 0..ROUNDS {
        //print_world((-1, -1, -1), (3, 3, 1), &active);
        active = step(&active);
        //print_world((-1, -1, -1), (3, 3, 1), &active);
    }
    active.iter().count().to_string()
}

pub fn b() -> String {
    "".to_string()
}

fn _print_world(
    from: (isize, isize, isize),
    to: (isize, isize, isize),
    active: &ActiveCells,
) {
    for z in from.2..=to.2 {
        println!("z: {}", z);
        for y in from.1..=to.1 {
            for x in from.0..=to.0 {
                let cell = if active.contains(&(x, y, z)) {
                    "#"
                } else {
                    "."
                };
                print!("{}", cell);
            }
            println!();
        }
        println!();
    }
}
