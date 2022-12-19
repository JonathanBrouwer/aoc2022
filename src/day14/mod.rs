use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::HashSet;

fn part1(inp: &str) -> usize {
    let mut state = build_initial(inp);

    let air = state.iter().map(|p| p.1).max().unwrap() + 1;

    for count in 0.. {
        let mut pos = (500, 0);

        loop {
            if pos.1 == air {
                return count;
            }
            if !state.contains(&(pos.0, pos.1 + 1)) {
                pos = (pos.0, pos.1 + 1);
                continue;
            }
            if !state.contains(&(pos.0 - 1, pos.1 + 1)) {
                pos = (pos.0 - 1, pos.1 + 1);
                continue;
            }
            if !state.contains(&(pos.0 + 1, pos.1 + 1)) {
                pos = (pos.0 + 1, pos.1 + 1);
                continue;
            }
            state.insert(pos);
            break;
        }
    }

    return 0;
}

fn part2(inp: &str) -> usize {
    let mut state = build_initial(inp);

    let ground = state.iter().map(|p| p.1).max().unwrap() + 1;

    for count in 0.. {
        let mut pos = (500, 0);

        loop {
            if pos.1 == ground {
                state.insert(pos);
                break;
            }
            if !state.contains(&(pos.0, pos.1 + 1)) {
                pos = (pos.0, pos.1 + 1);
                continue;
            }
            if !state.contains(&(pos.0 - 1, pos.1 + 1)) {
                pos = (pos.0 - 1, pos.1 + 1);
                continue;
            }
            if !state.contains(&(pos.0 + 1, pos.1 + 1)) {
                pos = (pos.0 + 1, pos.1 + 1);
                continue;
            }
            if pos == (500, 0) {
                return count + 1;
            }
            state.insert(pos);
            break;
        }
    }

    return 0;
}

fn build_initial(inp: &str) -> HashSet<(usize, usize)> {
    let mut state = HashSet::new();
    for input in parse_input(inp) {
        for (from, to) in input.tuple_windows() {
            if from.0 == to.0 {
                for y in min(from.1, to.1)..=max(from.1, to.1) {
                    state.insert((from.0, y));
                }
            } else if from.1 == to.1 {
                for x in min(from.0, to.0)..=max(from.0, to.0) {
                    state.insert((x, from.1));
                }
            } else {
                unreachable!()
            }
        }
    }
    state
}

fn parse_input(inp: &str) -> impl Iterator<Item = impl Iterator<Item = (usize, usize)> + '_> + '_ {
    inp.lines().map(|l| {
        l.split(" -> ").map(|p| {
            let (x, y) = p.split_once(",").unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"));
        assert_eq!(24, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(737, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example1"));
        assert_eq!(93, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(28145, result);
    }
}
