use crate::day13::Input::{Int, List};
use itertools::Itertools;
use std::cmp::Ordering;

fn part1(inp: &str) -> usize {
    let input = parse_input(inp);
    input
        .tuples()
        .enumerate()
        .filter(|(_, (l, r))| l < r)
        .map(|(i, _)| i + 1)
        .sum()
}

fn part2(inp: &str) -> usize {
    let mut input = parse_input(inp).collect_vec();
    let d1 = List(vec![List(vec![Int(2)])]);
    let d2 = List(vec![List(vec![Int(6)])]);
    input.push(d1.clone());
    input.push(d2.clone());
    input.sort();

    let v1 = input.iter().enumerate().find(|(_, x)| **x == d1).unwrap().0 + 1;
    let v2 = input.iter().enumerate().find(|(_, x)| **x == d2).unwrap().0 + 1;
    v1 * v2
}

#[derive(Eq, PartialEq, Clone)]
enum Input {
    Int(usize),
    List(Vec<Input>),
}

impl PartialOrd for Input {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Input {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Int(x), Int(y)) => x.cmp(y),
            (Int(x), y @ List(_)) => List(vec![Int(*x)]).cmp(y),
            (x @ List(_), Int(y)) => x.cmp(&List(vec![Int(*y)])),
            (List(x), List(y)) => x.cmp(y),
        }
    }
}

fn parse_line(mut l: &str) -> (Input, &str) {
    if l.chars().next().unwrap() == '[' {
        l = &l[1..];
        let mut res = Vec::new();
        while l.chars().next().unwrap() != ']' {
            let (v, rest) = parse_line(l);
            res.push(v);
            l = rest;
            if l.chars().next().unwrap() == ',' {
                l = &l[1..];
            }
        }
        (List(res), &l[1..])
    } else {
        let (n, rest) = l.split_at(l.find([',', ']']).unwrap());
        (Int(n.parse().unwrap()), rest)
    }
}

fn parse_input(inp: &str) -> impl Iterator<Item = Input> + '_ {
    inp.lines()
        .filter(|l| !l.is_empty())
        .map(|l| parse_line(l).0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"));
        assert_eq!(13, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(5623, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example1"));
        assert_eq!(140, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(20570, result);
    }
}
