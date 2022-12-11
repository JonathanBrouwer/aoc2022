use std::collections::HashSet;

fn part1(inp: &str) -> usize {
    simulate::<2>(inp)
}

fn part2(inp: &str) -> usize {
    simulate::<10>(inp)
}

fn simulate<const N: usize>(inp: &str) -> usize {
    let input: Vec<_> = parse_input(inp).collect();

    let mut been = HashSet::new();
    been.insert((0, 0));

    let mut rope = [(0i32, 0i32); N];

    for (dir, count) in input {
        let (dx, dy) = match dir {
            b'R' => (1,0),
            b'L' => (-1, 0),
            b'U' => (0, 1),
            b'D' => (0, -1),
            _ => unreachable!()
        };

        for _ in 0..count {
            rope[0] = (rope[0].0 + dx, rope[0].1 + dy);

            for i in 1..N {
                if rope[i-1].0.abs_diff(rope[i].0) >= 2 || rope[i-1].1.abs_diff(rope[i].1) >= 2 {
                    rope[i].0 += (rope[i-1].0 - rope[i].0).signum();
                    rope[i].1 += (rope[i-1].1 - rope[i].1).signum();
                } else {
                    break;
                }
                if i == N-1 {
                    been.insert(rope[N-1]);
                }
            }
        }
    }

    been.len()
}

fn parse_input(inp: &str) -> impl Iterator<Item=(u8, usize)> + '_ {
    inp.lines().map(|l| {
        (l.as_bytes()[0], l[2..].parse().unwrap())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"));
        assert_eq!(13, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(6367, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example2"));
        assert_eq!(36, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(2536, result);
    }
}
