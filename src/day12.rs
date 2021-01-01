use crate::read_input;
use std::convert::TryFrom;
use std::str::FromStr;
use Cmd::*;
use Dir::*;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Dir {
    N = 0,
    E = 90,
    S = 180,
    W = 270,
}

impl From<isize> for Dir {
    fn from(n: isize) -> Self {
        match n {
            0 => N,
            90 => E,
            180 => S,
            270 => W,
            _ => unreachable!(),
        }
    }
}

impl Dir {
    fn steps_by(&self) -> (isize, isize) {
        match self {
            N => (0, -1),
            S => (0, 1),
            E => (1, 0),
            W => (-1, 0),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Cmd {
    Move(Dir, isize),
    F(isize),
    R(isize),
    L(isize),
}

impl FromStr for Cmd {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("Empty string !".to_string());
        }
        let p: isize = s[1..].parse::<isize>().expect("ParseIntError !");

        Ok(match &s[..1] {
            "N" => Move(N, p),
            "S" => Move(S, p),
            "W" => Move(W, p),
            "E" => Move(E, p),
            "F" => F(p),
            "R" => R(p),
            "L" => L(p),
            _ => return Err("Unrecognized command !".to_string()),
        })
    }
}

// x, y are measure from origin 0,0 as east, south coordinates
struct Ship {
    dir: Dir,
    x: isize,
    y: isize,
}

impl Ship {
    fn new() -> Self {
        Self { dir: E, x: 0, y: 0 }
    }

    fn do_cmd(&mut self, cmd: Cmd) {
        match cmd {
            Move(dir, steps) => self.move_to(dir, steps),
            F(steps) => self.move_to(self.dir, steps),
            R(_) | L(_) => self.turn(cmd),
        }
    }

    fn move_to(&mut self, dir: Dir, steps: isize) {
        let (steps_x, steps_y) = dir.steps_by();
        self.x += steps_x * steps;
        self.y += steps_y * steps;
    }

    fn turn(&mut self, c: Cmd) {
        let new_dir = match c {
            R(degrees) => (self.dir as isize + degrees) % 360,
            L(degrees) => (self.dir as isize - degrees + 360) % 360,
            _ => unreachable!(),
        };
        self.dir = Dir::try_from(new_dir).unwrap();
    }

    fn distance(&self) -> usize {
        usize::try_from(self.x.abs() + self.y.abs()).unwrap()
    }
}

pub fn a() -> String {
    let cmds = read_input::<Cmd>("../input/day12");
    let mut ship = Ship::new();
    cmds.iter().for_each(|c| ship.do_cmd(*c));
    ship.distance().to_string()
}

struct Waypoint {
    x: isize,
    y: isize,
    ship: Ship,
}

impl Waypoint {
    fn new() -> Self {
        let ship = Ship::new();
        Self {
            x: ship.x + 10,
            y: ship.y - 1,
            ship,
        }
    }

    fn do_cmd(&mut self, cmd: Cmd) {
        match cmd {
            Move(dir, steps) => self.move_to(dir, steps),
            R(_) | L(_) => self.turn(cmd),
            F(steps) => {
                let delta_x = self.x - self.ship.x;
                let delta_y = self.y - self.ship.y;
                self.ship.x += delta_x * steps;
                self.ship.y += delta_y * steps;
                self.x = self.ship.x + delta_x;
                self.y = self.ship.y + delta_y;
            }
        }
    }

    fn move_to(&mut self, dir: Dir, steps: isize) {
        let (steps_x, steps_y) = dir.steps_by();
        self.x += steps_x * steps;
        self.y += steps_y * steps;
    }

    fn turn(&mut self, c: Cmd) {
        let degrees = match c {
            R(degrees) => degrees,
            L(degrees) => 360 - degrees,
            _ => unreachable!(),
        };
        let (x, y): (isize, isize) = match degrees {
            0 | 360 => (self.x, self.y),
            90 => (
                self.ship.x + (self.ship.y - self.y),
                self.ship.y + (self.x - self.ship.x),
            ),
            180 => (
                self.ship.x - (self.x - self.ship.x),
                self.ship.y + (self.ship.y - self.y),
            ),
            270 => (
                self.ship.x - (self.ship.y - self.y),
                self.ship.y - (self.x - self.ship.x),
            ),
            _ => unreachable!(),
        };

        self.x = x;
        self.y = y;
    }
}

pub fn b() -> String {
    let cmds = read_input::<Cmd>("../input/day12");
    let mut waypoint = Waypoint::new();
    cmds.iter().for_each(|c| waypoint.do_cmd(*c));
    waypoint.ship.distance().to_string()
}
