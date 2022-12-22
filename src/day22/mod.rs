use std::iter;
use itertools::Itertools;

fn part1(inp: &str) -> usize {
    let input = parse_input(inp);

    return 0;
}

fn part2(inp: &str) -> usize {
    let input = parse_input(inp);

    return 0;
}

enum X{
    L,
    R,
    M(usize)
}

fn parse_input(inp: &str) -> (Vec<(usize, Vec<bool>)>, impl Iterator<Item=X> + '_) {
    let (map, mut path) = inp.split_once("\n\n").unwrap();

    let map = map.lines().map(|line: &str| {
        let start = line.bytes().take_while(|c| *c == b' ').count();
        (start, line[start..].bytes().map(|b| b == b'#').collect())
    }).collect();

        path.chars().batching(|it| {

            todo!()
        })
    )
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
