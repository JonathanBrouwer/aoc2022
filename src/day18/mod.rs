use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

fn part1(inp: &str) -> usize {
    let input = parse_input(inp).map(|p| (p.0 + 1, p.1 + 1, p.2 + 1));
    let mut set = HashSet::new();
    input.for_each(|p| {
        set.insert(p);
    });
    set.iter()
        .map(|p| nbs(*p).filter(|nb| !set.contains(nb)).count())
        .sum()
}

fn nbs(p: (usize, usize, usize)) -> impl Iterator<Item = (usize, usize, usize)> {
    [
        (p.0 + 1, p.1, p.2),
        (p.0 - 1, p.1, p.2),
        (p.0, p.1 + 1, p.2),
        (p.0, p.1 - 1, p.2),
        (p.0, p.1, p.2 + 1),
        (p.0, p.1, p.2 - 1),
    ]
    .into_iter()
}

fn part2(inp: &str) -> usize {
    let input = parse_input(inp).map(|p| (p.0 + 2, p.1 + 2, p.2 + 2));
    let mut set = HashSet::new();
    input.for_each(|p| {
        set.insert(p);
    });

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((1, 1, 1));
    let mut sum = 0;

    while let Some(p) = queue.pop_front() {
        for nb in nbs(p) {
            if nb.0 == 0 || nb.1 == 0 || nb.2 == 0 || nb.0 == 50 || nb.1 == 50 || nb.2 == 50 {
                continue;
            }
            if set.contains(&nb) {
                sum += 1;
            } else if !visited.contains(&nb) {
                queue.push_back(nb);
                visited.insert(nb);
            }
        }
    }

    sum
}

fn parse_input(inp: &str) -> impl Iterator<Item = (usize, usize, usize)> + '_ {
    inp.lines().map(|l| {
        l.split(",")
            .map(|n| n.parse().unwrap())
            .collect_tuple()
            .unwrap()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"));
        assert_eq!(64, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(4322, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example1"));
        assert_eq!(58, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(2516, result);
    }
}
