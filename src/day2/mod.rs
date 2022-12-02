fn part1(inp: &str) -> usize {
    parse_input(inp).map(|(opponent, you)| {
        // Calculate points for what you play
        let s1 = you as usize - 'W' as usize;
        // Calculate points for result
        let s2 = ((you as usize - opponent as usize - 1) % 3) * 3;
        s1 + s2
    }).sum::<usize>()
}

fn part2(inp: &str) -> usize {
    parse_input(inp).map(|(opponent, result)| {
        // Calculate points for result
        let s1 = (result as usize - 'X' as usize) * 3;
        // Calculate points for what you play
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


