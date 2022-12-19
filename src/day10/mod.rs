use crate::day10::Instr::{Add, Noop};

fn part1(inp: &str) -> usize {
    let input = parse_input(inp);
    let mut state = 1;
    let mut cycles = 0;
    let mut next_milestone = 20;
    let mut result = 0;

    for instr in input {
        cycles += instr.cycles();
        while cycles >= next_milestone {
            result += next_milestone * state;
            next_milestone += 40;
        }
        if let Add(d) = instr {
            state += d;
        }
    }

    result as usize
}

fn part2(inp: &str) {
    let input = parse_input(inp);

    let mut state: isize = 1;
    let mut i: isize = 0;

    let mut output = String::new();
    for instr in input {
        for _ in 0..instr.cycles() {
            output.push(if i.abs_diff(state) <= 1 { '#' } else { ' ' });
            i += 1;
            i %= 40;
        }
        if let Add(d) = instr {
            state += d;
        }
    }

    for y in 0..6 {
        println!("{}", &output[y * 40..(y + 1) * 40]);
    }
}

enum Instr {
    Noop,
    Add(isize),
}

impl Instr {
    fn cycles(&self) -> isize {
        match self {
            Noop => 1,
            Add(_) => 2,
        }
    }
}

fn parse_input(inp: &str) -> impl Iterator<Item = Instr> + '_ {
    inp.lines().map(|l| {
        if l == "noop" {
            Noop
        } else {
            Add(l[5..].parse().unwrap())
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"));
        assert_eq!(13140, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(14040, result);
    }

    #[test]
    fn test_part2_real() {
        part2(include_str!("input"));
    }
}
