use itertools::Itertools;

fn part1(inp: &str) -> usize {
    parse_input(inp)
        .filter(|&(a, b, c, d)| (a <= c && d <= b) || (c <= a && b <= d))
        .count()
}

fn part2(inp: &str) -> usize {
    parse_input(inp)
        .filter(|&(a, b, c, d)| c <= b && a <= d)
        .count()
}

fn parse_input(inp: &str) -> impl Iterator<Item = (usize, usize, usize, usize)> + '_ {
    inp.split(&['\n', ',', '-'])
        .map(|n| n.parse().unwrap())
        .tuples()
}

fn oneliner(inp: &str) {
    print!(
        "{:?}",
        inp.split(['\n', ',', '-'])
            .map(|n| n.parse::<u64>().unwrap())
            .tuples()
            .fold((0, 0), |s, (a, b, c, d)| (
                s.0 + (a <= c && d <= b || c <= a && b <= d) as u64,
                s.1 + (c <= b && a <= d) as u64
            ))
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"));
        assert_eq!(2, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(494, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example1"));
        assert_eq!(4, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(833, result);
    }

    #[test]
    fn test_oneliner() {
        oneliner(include_str!("input"))
    }
}
