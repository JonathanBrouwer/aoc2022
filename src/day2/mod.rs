pub fn part1(inp: &str) -> usize {
    parse_input(inp).map(|(opponent, you)| {
        // Calculate points for what you play
        let s1 = you - b'W';
        // Calculate points for result
        let s2 = ((you - opponent - 1) % 3) * 3;
        (s1 + s2) as usize
    }).sum::<usize>()
}

fn part2(inp: &str) -> usize {
    parse_input(inp).map(|(opponent, result)| {
        // Calculate points for result
        let s1 = (result - b'X') * 3;
        // Calculate points for what you play
        let s2 = ((opponent + result - 1) % 3) + 1;
        (s1 + s2) as usize
    }).sum::<usize>()
}

fn parse_input(inp: &str) -> impl Iterator<Item = (u8, u8)> + '_ {
    inp.lines().map(|l| {
        (l.as_bytes()[0], l.as_bytes()[2])
    })
}

fn oneliner(inp: &str) {
    println!("{}", inp.lines().map(|l| { ((l.as_bytes()[2] - b'X') * 3 + ((l.as_bytes()[0] + l.as_bytes()[2] - 1) % 3) + 1) as usize }).sum::<usize>())
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

    #[test]
    fn test_oneliner() {
        oneliner(include_str!("input"));
    }
}


