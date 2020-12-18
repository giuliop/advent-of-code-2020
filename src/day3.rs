use crate::read_input_world;

// x,y
type Pos = (usize, usize);

type Cells = Vec<Cell>;

#[derive(Copy, Clone)]
enum Cell {
    Tree,
    Empty,
}

struct World {
    cells: Vec<Cells>,
    rows: usize,
    cols: usize,
}

impl World {
    //constructor
    fn build(w: Vec<Cells>) -> Self {
        World {
            rows: w.len(),
            cols: w[0].len(),
            cells: w,
        }
    }

    fn lookup(&self, pos: Pos) -> Cell {
        let (mut x, y) = pos;
        x = x % self.cols;
        self.cells[y][x]
    }
}

fn build_world() -> World {
    let legend = [('#', Cell::Tree), ('.', Cell::Empty)]
        .iter()
        .cloned()
        .collect();

    World::build(read_input_world("../input/day3", &legend))
}

fn trees_in_path(world: &World, right_step: usize, down_step: usize) -> usize {
    let (mut x, mut y): Pos = (0, 0);
    let mut res = 0;

    while y < world.rows {
        if let Cell::Tree = world.lookup((x, y)) {
            res += 1;
        }
        x += right_step;
        y += down_step;
    }
    res
}

pub fn a() -> String {
    let world = build_world();
    let res = trees_in_path(&world, 3, 1);
    res.to_string()
}

pub fn b() -> String {
    let world = build_world();
    let res = trees_in_path(&world, 1, 1)
        * trees_in_path(&world, 3, 1)
        * trees_in_path(&world, 5, 1)
        * trees_in_path(&world, 7, 1)
        * trees_in_path(&world, 1, 2);
    res.to_string()
}
