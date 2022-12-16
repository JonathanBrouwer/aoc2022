use std::collections::{HashMap, HashSet};
use itertools::Itertools;

fn part1(inp: &str) -> usize {
    let input = parse_input(inp).collect_vec();

    let map: HashMap<(HashSet<&str>, &str), usize> = HashMap::new();


    return 0;
}

fn part2(inp: &str) -> usize {
    let input = parse_input(inp);

    return 0;
}

fn parse_input(inp: &str) -> impl Iterator<Item=(&str, usize, Vec<&str>)> {
    inp.lines().map(|line| {
        let name = &line[6..8];
        let (flow, valves) = line[23..].split_once("; tunnels lead to valves ").or(line[23..].split_once("; tunnel leads to valve ")).unwrap();
        (name, flow.parse().unwrap(), valves.split(", ").collect())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"));
        assert_eq!(0, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(0, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example1"));
        assert_eq!(0, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(0, result);
    }
}
