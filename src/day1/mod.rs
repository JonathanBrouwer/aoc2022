use std::cmp::Reverse;
use itertools::Itertools;

fn part1(inp: &str) -> usize {
    parse_input(inp).map(|v| v.sum()).max().unwrap()
}

fn part2(inp: &str) -> usize {
    parse_input(inp).map(|n| n.sum::<usize>()).sorted_by_key(|n| Reverse(*n)).take(3).sum()
}

fn parse_input(inp: &str) -> impl Iterator<Item=impl Iterator<Item=usize> + '_> + '_ {
    inp.split("\n\n").map(|i| i.split("\n").map(|n| n.parse().unwrap()))
}







#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"));
        assert_eq!(24000, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(72017, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example1"));
        assert_eq!(45000, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(212520, result);
    }
}


