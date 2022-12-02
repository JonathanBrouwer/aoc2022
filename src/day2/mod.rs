fn part1(inp: &str) -> usize {
    parse_input(inp).map(|(opponent, you)| {
        // a b c rock paper scic
        // x y z rock paper scic
        let s1 = match you {
            'X' => 1,
            'Y' => 2,
            'Z' => 3,
            _ => unreachable!()
        };
        let s2 = match ((you as u8 - 'X' as u8) + 3 - (opponent as u8 - 'A' as u8)) % 3 {
            0 => 3,
            1 => 6,
            2 => 0,
            _ => unreachable!()
        };
        s1 + s2
    }).sum::<usize>()
}

fn part2(inp: &str) -> usize {
    parse_input(inp).map(|(opponent, result)| {
        let s1 = match result {
            'X' => 0,
            'Y' => 3,
            'Z' => 6,
            _ => unreachable!()
        };
        let s2 = ((opponent as usize + result as usize - 1) % 3) + 1;
        s1 + s2
    }).sum::<usize>()
}

fn parse_input(inp: &str) -> impl Iterator<Item = (char, char)> + '_ {
    inp.lines().map(|l| {
        let (s1, s2) = l.split_once(" ").unwrap();
        (s1.chars().next().unwrap(), s2.chars().next().unwrap())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"));
        assert_eq!(15, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(8933, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example1"));
        assert_eq!(12, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(11998, result);
    }
}


