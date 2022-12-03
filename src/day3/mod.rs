fn part1(inp: &str) -> usize {
    let input = parse_input(inp);

    input.map(|l| {
        let a = &l.as_bytes()[..l.len() / 2];
        let b = &l.as_bytes()[l.len() / 2..];
        let c = b.iter().find(|c| a.contains(c)).unwrap();
        code(*c)
    }).sum()
}

fn part2(inp: &str) -> usize {
    let input = parse_input(inp);

    input.array_chunks::<3>()

    return 0;
}

fn parse_input(inp: &str) -> impl Iterator<Item=&str> {
    inp.lines()
}

fn code(c: u8) -> usize {
    (match c {
        b'a'..=b'z' => c - b'a' + 1,
        b'A'..=b'Z' => c - b'A' + 27,
        _ => unreachable!(),
    }) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        assert_eq!(code(b'B'), 28);
        assert_eq!(code(b'b'), 2);
        let result = part1(include_str!("example1"));
        assert_eq!(157, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(7581, result);
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


