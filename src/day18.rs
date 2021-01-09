use std::fs;
use std::ops::{Add, Mul};

type Operation = fn(usize, usize) -> usize;

#[derive(Debug, Clone)]
enum Token {
    Num(usize),
    Add,
    Mul,
    Expr(Vec<Token>),
}

fn solve_a(tokens: &[Token]) -> usize {
    if tokens.is_empty() {
        return 0;
    }
    let mut reg: usize = 0;
    let mut op: Operation = Add::add;

    for t in tokens {
        match t {
            Token::Add => op = Add::add,
            Token::Mul => op = Mul::mul,
            Token::Num(x) => reg = (op)(reg, *x),
            Token::Expr(ts) => reg = (op)(reg, solve_a(&ts)),
        }
    }
    reg
}

fn op_from(t: &Token) -> Operation {
    match t {
        Token::Mul => Mul::mul,
        Token::Add => Add::add,
        _ => unreachable!(),
    }
}

fn solve_b(tokens: &[Token]) -> usize {
    match tokens.len() {
        0 => 0,
        1 => match &tokens[0] {
            Token::Num(x) => *x,
            Token::Expr(ts) => solve_b(&ts),
            _ => unreachable!(),
        },
        3 => (op_from(&tokens[1]))(solve_b(&tokens[0..=0]), solve_b(&tokens[2..=2])),
        n if n & 1 != 0 => match &tokens[1] {
            Token::Mul => solve_b(&tokens[0..=0]) * solve_b(&tokens[2..]),
            Token::Add => solve_b(&{
                let mut v = vec![Token::Num(
                    solve_b(&tokens[0..1]) + solve_b(&tokens[2..3]),
                )];
                v.extend_from_slice(&tokens[3..]);
                v
            }),
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
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

fn tokenize(s: &str) -> Vec<Token> {
    let mut res = Vec::new();
    let mut idx = 0;
    while idx < s.len() {
        let start = idx;
        let token = match s[start..].chars().next().unwrap() {
            '(' => {
                idx += match_paren(&s[start..]) + 1;
                Token::Expr(tokenize(&s[start + 1..idx - 1]))
            }
            '0'..='9' => {
                idx += s[start..]
                    .find(|x: char| !x.is_numeric())
                    .unwrap_or(s[start..].len())
                    + 1;
                Token::Num(s[start..idx - 1].parse::<usize>().unwrap())
            }
            '+' => {
                idx += 2;
                Token::Add
            }
            '*' => {
                idx += 2;
                Token::Mul
            }
            ' ' | '\n' => {
                idx += 1;
                continue;
            }
            _ => unreachable!(),
        };
        res.push(token);
    }
    res
}

pub fn a() -> String {
    fs::read_to_string("../input/day18")
        .expect("error reading file")
        .lines()
        .map(|x| solve_a(&tokenize(x)))
        .sum::<usize>()
        .to_string()
}

pub fn b() -> String {
    fs::read_to_string("../input/day18")
        .expect("error reading file")
        .lines()
        .map(|x| solve_b(&tokenize(x)))
        .sum::<usize>()
        .to_string()
}
