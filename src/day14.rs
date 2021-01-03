use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::ops::BitAnd;
use std::ops::BitOr;

const MASK_LEN: usize = 36;

pub fn a() -> String {
    let mut computer = Computer::new();
    computer.run(read_program());
    computer.memory.values().sum::<usize>().to_string()
}

#[derive(Debug)]
enum Cmd {
    Mem(usize, usize),
    Mask(String),
}

// The type for bitwise operators AND , OR
type MaskFn = fn(usize, usize) -> usize;

#[derive(Default)]
struct Computer {
    memory: HashMap<usize, usize>,
    // bitwise operator, mask value
    mask: Vec<(MaskFn, usize)>,
}

impl Computer {
    fn new() -> Self {
        Default::default()
    }

    fn run(&mut self, cmds: Vec<Cmd>) {
        cmds.iter().for_each(|cmd| match cmd {
            Cmd::Mem(addr, val) => self.set_memory(*addr, *val),
            Cmd::Mask(m) => self.set_mask(m),
        })
    }

    fn set_mask(&mut self, m: &str) {
        let mut mask: Vec<(MaskFn, usize)> = Vec::new();
        m.chars().enumerate().for_each(|(i, c)| match c {
            'X' => (),
            '1' => mask.push((BitOr::bitor, 1 << (MASK_LEN - 1 - i))),
            '0' => mask.push((BitAnd::bitand, !(1 << (MASK_LEN - 1 - i)))),
            _ => unreachable!(),
        });
        self.mask = mask;
    }

    fn set_memory(&mut self, addr: usize, val: usize) {
        let masked_val = self.mask.iter().fold(val, |acc, (op, m)| op(acc, *m));
        self.memory.insert(addr, masked_val);
    }
}

fn read_program() -> Vec<Cmd> {
    lazy_static! {
        static ref RE_MEM: Regex = Regex::new(r"mem\[(\d*)\] = (\d*)").unwrap();
        static ref RE_MASK: Regex = Regex::new(r"mask = ([01X]*)").unwrap();
    }
    fs::read_to_string("../input/day14")
        .expect("error reading file")
        .lines()
        .filter_map(|s| match s {
            s if RE_MEM.is_match(s) => {
                let caps = RE_MEM.captures(s).unwrap();
                Some(Cmd::Mem(caps[1].parse().unwrap(), caps[2].parse().unwrap()))
            }
            s if RE_MASK.is_match(s) => {
                let caps = RE_MASK.captures(s).unwrap();
                Some(Cmd::Mask(caps[1].to_string()))
            }
            _ => None,
        })
        .collect()
}

pub fn b() -> String {
    let mut computer = ComputerV2::new();
    computer.run(read_program());
    computer.memory.values().sum::<usize>().to_string()
}

#[derive(Default)]
struct ComputerV2 {
    memory: HashMap<usize, usize>,
    // 1,0: bitwise operator, mask value
    mask: Vec<(MaskFn, usize)>,
    // X: bit position
    mask_x: Vec<usize>,
}

impl ComputerV2 {
    fn new() -> Self {
        Default::default()
    }

    fn run(&mut self, cmds: Vec<Cmd>) {
        cmds.iter().for_each(|cmd| match cmd {
            Cmd::Mem(addr, val) => self.set_memory(*addr, *val),
            Cmd::Mask(m) => self.set_mask(m),
        })
    }

    fn set_mask(&mut self, m: &str) {
        let mut mask: Vec<(MaskFn, usize)> = Vec::new();
        let mut mask_x: Vec<usize> = Vec::new();
        m.chars().enumerate().for_each(|(i, c)| match c {
            'X' => mask_x.push(MASK_LEN - 1 - i),
            '1' => mask.push((BitOr::bitor, 1 << (MASK_LEN - 1 - i))),
            '0' => (),
            _ => unreachable!(),
        });
        self.mask = mask;
        self.mask_x = mask_x;
    }

    fn set_memory(&mut self, addr: usize, val: usize) {
        let masked_addr_before_x =
            self.mask.iter().fold(addr, |acc, (op, m)| op(acc, *m));
        let mut addresses: Vec<usize> = vec![masked_addr_before_x];
        for m in &self.mask_x {
            addresses = addresses
                .iter()
                .flat_map(|a| vec![a | (1 << m), a & !(1 << m)])
                .collect()
        }
        for a in addresses {
            self.memory.insert(a, val);
        }
    }
}
