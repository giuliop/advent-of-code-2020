use std::collections::HashSet;
use std::convert::TryFrom;
use std::fs;
use std::num::ParseIntError;

#[derive(Copy, Clone, Debug)]
enum Op {
    Nop,
    Acc,
    Jmp,
}

fn op_from(s: &str) -> Result<Op, CompilationError> {
    match s {
        "nop" => Ok(Op::Nop),
        "acc" => Ok(Op::Acc),
        "jmp" => Ok(Op::Jmp),
        _ => Err(CompilationError::UnknownOpCode(s.to_string())),
    }
}

#[derive(Debug)]
enum CompilationError {
    UnknownOpCode(String),
    UnknownParameter(ParseIntError),
}

impl From<ParseIntError> for CompilationError {
    fn from(error: ParseIntError) -> Self {
        Self::UnknownParameter(error)
    }
}

type Bitsize = isize;
type Instruction = (Op, Bitsize);

#[derive(Debug)]
enum State {
    InfiniteLoop,
    Exited,
    ComputationError(String),
}

struct Computer {
    pc: usize,
    acc: Bitsize,
    program: Vec<Instruction>,
}

impl Computer {
    fn from_program(s: &str) -> Result<Self, CompilationError> {
        let program = Self::compile(s)?;
        Ok(Self {
            pc: 0,
            acc: 0,
            program,
        })
    }

    fn compile(s: &str) -> Result<Vec<Instruction>, CompilationError> {
        s.split('\n')
            .map(|x| {
                let mut words = x.split(' ');
                Ok((
                    op_from(words.next().unwrap_or_default())?,
                    words.next().unwrap().parse::<Bitsize>()?,
                ))
            })
            .collect()
    }

    fn step(&mut self) {
        let (op, p) = self.program[self.pc];
        self.execute(op, p);
    }

    fn execute(&mut self, op: Op, p: Bitsize) {
        match op {
            Op::Acc => self.acc += p,
            Op::Jmp => {
                self.pc_add(p);
                return;
            }
            Op::Nop => (),
        }
        self.pc += 1;
    }

    fn pc_add(&mut self, p: Bitsize) {
        if p >= 0 {
            self.pc += usize::try_from(p).unwrap();
        } else {
            self.pc -= usize::try_from(-p).unwrap();
        }
    }

    fn run(&mut self) -> (State, Bitsize) {
        let mut seen = HashSet::new();
        loop {
            match self.pc {
                x if x > self.program.len() => {
                    return (
                        State::ComputationError(
                            "Program Counter too large!".to_string(),
                        ),
                        self.acc,
                    )
                }
                x if x == self.program.len() => return (State::Exited, self.acc),
                x if seen.contains(&x) => return (State::InfiniteLoop, self.acc),
                _ => {
                    seen.insert(self.pc);
                    self.step()
                }
            }
        }
    }
    fn reset(&mut self) {
        self.pc = 0;
        self.acc = 0;
    }
}

fn load_input() -> Computer {
    Computer::from_program(
        fs::read_to_string("../input/day8")
            .expect("error reading file")
            .trim(),
    )
    .unwrap()
}

pub fn a() -> String {
    let mut computer = load_input();
    let (_, out) = computer.run();
    out.to_string()
}

pub fn b() -> String {
    let mut computer = load_input();
    for i in 0..computer.program.len() {
        let original_op = computer.program[i].0;
        {
            let op = &mut computer.program[i].0;
            match op {
                Op::Jmp => *op = Op::Nop,
                Op::Nop => *op = Op::Jmp,
                Op::Acc => (),
            }
        }
        let (state, out) = computer.run();
        match state {
            State::Exited => return out.to_string(),
            _ => {
                computer.program[i].0 = original_op;
                computer.reset();
            }
        }
    }
    "No answer found".to_string()
}
