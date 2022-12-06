use itertools::Itertools;

fn part1(inp: &str) -> usize {
    parse_input(inp).windows(4).find_position(|w| w.iter().unique().count() == 4).unwrap().0 + 4
}

fn part2(inp: &str) -> usize {
    parse_input(inp).windows(14).find_position(|w| w.iter().unique().count() == 14).unwrap().0 + 14
}

fn parse_input(inp: &str) -> &[u8] {
    inp.as_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_exs() {
        assert_eq!(part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(1042, result);
    }

    #[test]
    fn test_part2_ex1() {
        assert_eq!(part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(2980, result);
    }
}
