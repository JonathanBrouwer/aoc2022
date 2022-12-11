use std::collections::VecDeque;
use itertools::Itertools;
use max_n::max_n_trait::MaxN;

fn part1(inp: &str) -> usize {
    let mut input = parse_input(inp);

    for _ in 0..20 {
        for i in 0..input.len() {
            while let Some(v) = input[i].items.pop_front() {
                let v = input[i].op.apply(v) / 3;
                if v % input[i].test_divisibility == 0 {
                    let test_true = input[i].test_true;
                    input[test_true].items.push_back(v);
                } else {
                    let test_false = input[i].test_false;
                    input[test_false].items.push_back(v);
                }
                input[i].counter += 1;
            }
        }
    }

    input.into_iter().map(|m| m.counter).max_n::<2>().iter().product()
}

fn part2(inp: &str) -> usize {
    let mut input = parse_input(inp);
    let modulo: usize = input.iter().map(|m| m.test_divisibility).product();

    for _ in 0..10000 {
        for i in 0..input.len() {
            while let Some(v) = input[i].items.pop_front() {
                let v = input[i].op.apply(v) % modulo;
                if v % input[i].test_divisibility == 0 {
                    let test_true = input[i].test_true;
                    input[test_true].items.push_back(v);
                } else {
                    let test_false = input[i].test_false;
                    input[test_false].items.push_back(v);
                }
                input[i].counter += 1;
            }
        }
    }

    input.into_iter().map(|m| m.counter).max_n::<2>().iter().product()
}

enum Operation {
    Add(usize),
    Mul(usize),
    Square,
}

impl Operation {
    fn apply(&self, x: usize) -> usize {
        match self {
            Operation::Add(v) => x+v,
            Operation::Mul(v) => x*v,
            Operation::Square => x*x,
        }
    }
}

struct Monkey {
    items: VecDeque<usize>,
    op: Operation,
    test_divisibility: usize,
    test_true: usize,
    test_false: usize,
    counter: usize,
}

fn parse_input(inp: &str) -> Vec<Monkey> {
    inp.lines().batching(|lines| {
        let _ = lines.next()?;
        let items = lines.next().unwrap()[18..].split(", ").map(|v| v.parse().unwrap()).collect();
        let op = match &lines.next().unwrap()[19..] {
            "old * old" => Operation::Square,
            s if s.starts_with("old + ") => Operation::Add(s[6..].parse().unwrap()),
            s if s.starts_with("old * ") => Operation::Mul(s[6..].parse().unwrap()),
            _ => unreachable!()
        };
        let test_divisibility = lines.next().unwrap()[21..].parse().unwrap();
        let test_true = lines.next().unwrap()[29..].parse().unwrap();
        let test_false = lines.next().unwrap()[30..].parse().unwrap();
        let _ = lines.next();


        Some(Monkey { items, op, test_divisibility, test_true, test_false, counter: 0 })
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"));
        assert_eq!(10605, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(117624, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example1"));
        assert_eq!(2713310158, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(16792940265, result);
    }
}
