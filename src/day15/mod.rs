use std::collections::HashSet;

fn part1(inp: &str, row: isize) -> usize {
    let input = parse_input(inp);
    let mut set = HashSet::new();
    let mut bcs = HashSet::new();
    for ((sx, sy), (bx, by)) in input {
        let d = (bx.abs_diff(sx)) + (by.abs_diff(sy));

        if sy.abs_diff(row) > d {
            continue;
        }
        let width = (d - sy.abs_diff(row)) as isize;
        for x in sx - width..=sx + width {
            set.insert(x);
        }

        if by == row {
            bcs.insert(by);
        }
    }

    set.difference(&bcs).count()
}

fn part2(inp: &str, border: isize) -> usize {
    let mut input: Vec<_> = parse_input(inp).collect();
    input.sort_by_key(|k| k.0 .0);

    for row in 0..=border {
        let mut max = 0isize;

        for &((sx, sy), (bx, by)) in &input {
            let d = (bx.abs_diff(sx)) + (by.abs_diff(sy));
            let dy = sy.abs_diff(row);
            if dy > d {
                continue;
            }
            let width = (d - dy) as isize;
            let xmin = sx - width;
            let xmax = sx + width;
            if xmin - 1 <= max {
                max = max.max(xmax);
            }
        }

        if max < border {
            return ((max + 1) * 4000000 + row) as usize;
        }
    }

    unreachable!()
}

fn parse_input(inp: &str) -> impl Iterator<Item = ((isize, isize), (isize, isize))> + '_ {
    inp.lines().map(|line| {
        let (p1, p2) = line.split_once(": closest beacon is at x=").unwrap();
        let p1 = p1.strip_prefix("Sensor at x=").unwrap();
        let (p1a, p1b) = p1.split_once(", y=").unwrap();
        let (p2a, p2b) = p2.split_once(", y=").unwrap();
        (
            (p1a.parse().unwrap(), p1b.parse().unwrap()),
            (p2a.parse().unwrap(), p2b.parse().unwrap()),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"), 10);
        assert_eq!(26, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"), 2000000);
        println!("Part 1: {}", result);
        assert_eq!(4919281, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example1"), 20);
        assert_eq!(56000011, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"), 4000000);
        println!("Part 2: {}", result);
        assert_eq!(12630143363767, result);
        // too low 543264948865
        //
    }
}
