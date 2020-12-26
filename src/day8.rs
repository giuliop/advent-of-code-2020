use ::std::collections::HashSet;
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

type Parameter = isize;
type Instruction = (Op, Parameter);

struct Computer {
    pc: usize,
    acc: Parameter,
    program: Vec<Instruction>,
}

impl Computer {
    fn from_program(s: &str) -> Result<Self, CompilationError> {
        let program = Self::compile(s)?;
        //println!("{:?}", program);
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
                    words.next().unwrap().parse::<Parameter>()?,
                ))
            })
            .collect()
    }

    fn step(&mut self) {
        let (op, p) = self.program[self.pc];
        self.pc += 1;
        self.execute(op, p);
    }

    fn execute(&mut self, op: Op, p: Parameter) {
        match op {
            Op::Acc => self.acc += p,
            Op::Jmp => {
                self.pc -= 1;
                self.pc_add(p)
            }
            Op::Nop => (),
        }
    }

    fn pc_add(&mut self, p: Parameter) {
        if p >= 0 {
            self.pc += usize::try_from(p).unwrap();
        } else {
            self.pc -= usize::try_from(-p).unwrap();
        }
    }
}

pub fn a() -> String {
    let mut computer = Computer::from_program(
        fs::read_to_string("../input/day8")
            .expect("error reading file")
            .trim(),
    )
    .unwrap();
    let mut seen = HashSet::new();
    while !seen.contains(&computer.pc) {
        seen.insert(computer.pc);
        computer.step();
    }
    computer.acc.to_string()
}

pub fn b() -> String {
    "".to_string()
}
