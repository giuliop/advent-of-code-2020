use std::fs;
use std::mem;
use std::ops::{Add, Mul};

type Operation = fn(usize, usize) -> usize;
type ParseFn = fn(&[Token]) -> Expr;

#[derive(Debug)]
struct Expr {
    lhs: Operand,
    op: Operation,
    rhs: Operand,
}

#[derive(Debug)]
enum Operand {
    Num(usize),
    Expr(Box<Expr>),
}

#[derive(Debug)]
enum Token {
    Num(usize),
    Add,
    Mul,
    Expr(Vec<Token>),
}

impl Expr {
    fn from_str(s: &str, parse_fn: ParseFn) -> Self {
        parse_fn(&tokenize(s)[..])
    }

    fn solve(&self) -> usize {
        let lhs = match &self.lhs {
            Operand::Num(n) => *n,
            Operand::Expr(boxed) => boxed.solve(),
        };
        let rhs = match &self.rhs {
            Operand::Num(n) => *n,
            Operand::Expr(boxed) => boxed.solve(),
        };
        (self.op)(lhs, rhs)
    }
}

fn parse_advanced(tokens: &[Token]) -> Expr {
    let mut tokens = tokens;
    if tokens.len() == 1 {
        match &tokens[0] {
            Token::Expr(ts) => tokens = &ts[..],
            _ => unreachable!(),
        }
    }
    let lhs: &[Token];
    let rhs: &[Token];
    let op: Operation;
    match tokens
        .iter()
        .position(|x| mem::discriminant(x) == mem::discriminant(&Token::Mul))
    {
        Some(idx) => {
            lhs = &tokens[..idx];
            op = Mul::mul;
            rhs = &tokens[idx + 1..];
        }
        None => {
            let len = tokens.len();
            lhs = &tokens[..len - 2];
            op = Add::add;
            rhs = &tokens[len - 1..];
        }
    }

    let lhs = if lhs.len() == 1 {
        if let Token::Num(n) = lhs[0] {
            Operand::Num(n)
        } else {
            Operand::Expr(Box::new(parse_advanced(lhs)))
        }
    } else {
        Operand::Expr(Box::new(parse_advanced(lhs)))
    };

    let rhs = if rhs.len() == 1 {
        if let Token::Num(n) = rhs[0] {
            Operand::Num(n)
        } else {
            Operand::Expr(Box::new(parse_advanced(rhs)))
        }
    } else {
        Operand::Expr(Box::new(parse_advanced(rhs)))
    };

    Expr { lhs, op, rhs }
}

fn parse(tokens: &[Token]) -> Expr {
    let mut tokens = tokens;
    if tokens.len() == 1 {
        match &tokens[0] {
            Token::Expr(ts) => tokens = &ts[..],
            _ => unreachable!(),
        }
    }
    let len = tokens.len();
    let lhs = &tokens[..len - 2];
    let lhs = if lhs.len() == 1 {
        if let Token::Num(n) = lhs[0] {
            Operand::Num(n)
        } else {
            Operand::Expr(Box::new(parse(lhs)))
        }
    } else {
        Operand::Expr(Box::new(parse(lhs)))
    };
    let op = match &tokens[len - 2] {
        Token::Add => Add::add,
        Token::Mul => Mul::mul,
        _ => unreachable!(),
    };
    let rhs = &tokens[len - 1..];
    let rhs = if let Token::Num(n) = rhs[0] {
        Operand::Num(n)
    } else {
        Operand::Expr(Box::new(parse(rhs)))
    };

    Expr { lhs, op, rhs }
}

// return the position of closing matching parenthesis
fn match_paren(s: &str) -> usize {
    let mut s = s[1..].chars();
    let mut balance = 1;
    let mut idx = 0;
    while balance != 0 {
        match s.next().unwrap() {
            '(' => balance += 1,
            ')' => balance -= 1,
            _ => (),
        }
        idx += 1;
    }
    idx
}

// Take string and index to start from and return next token and update index
// at next unconsumed char
fn next_token(s: &str, idx: &mut usize) -> Option<Token> {
    let start = *idx;
    if s.len() <= start {
        return None;
    }
    match s[start..].chars().next().unwrap_or('\n') {
        '(' => {
            *idx += match_paren(&s[start..]) + 1;
            Some(Token::Expr(tokenize(&s[start + 1..*idx - 1])))
        }
        '0'..='9' => {
            *idx += s[start..]
                .find(|x: char| !x.is_numeric())
                .unwrap_or(s[start..].len())
                + 1;
            Some(Token::Num(s[start..*idx - 1].parse::<usize>().unwrap()))
        }
        '+' => {
            *idx += 2;
            Some(Token::Add)
        }
        '*' => {
            *idx += 2;
            Some(Token::Mul)
        }
        ' ' => {
            *idx += 1;
            next_token(s, idx)
        }
        '\n' => {
            *idx += 1;
            None
        }
        _ => unreachable!(),
    }
}

fn tokenize(s: &str) -> Vec<Token> {
    let mut res = Vec::new();
    let mut idx = 0;
    while let Some(token) = next_token(s, &mut idx) {
        res.push(token);
    }
    res
}

pub fn a() -> String {
    fs::read_to_string("../input/day18")
        .expect("error reading file")
        .lines()
        .map(|x| Expr::from_str(x, parse).solve())
        .sum::<usize>()
        .to_string()
}

pub fn b() -> String {
    fs::read_to_string("../input/day18")
        .expect("error reading file")
        .lines()
        .map(|x| Expr::from_str(x, parse_advanced).solve())
        .sum::<usize>()
        .to_string()
}

pub fn _debug() -> String {
    let tokens: Vec<Vec<Token>> = fs::read_to_string("../input/test")
        .expect("error reading file")
        .lines()
        .map(|x| tokenize(x))
        .collect();
    //dbg!(&tokens);

    let ast: Vec<Expr> = tokens.iter().map(|x| parse_advanced(x)).collect();
    //dbg!(&ast);

    ast.iter().map(|x| x.solve()).sum::<usize>().to_string()
}
