use std::cmp::Ordering;
use itertools::Itertools;

fn part1(inp: &str) -> String {
    let (mut cs, is) = parse_input(inp);

    for (c, f, t) in is {
        let (f, t) = take_two(&mut cs[..], f-1, t-1);
        t.extend(f.drain(f.len()-c .. ).rev());
    }

    cs.into_iter().map(|v| *v.last().unwrap() as char).collect()
}

fn part2(inp: &str) -> String {
    let (mut cs, is) = parse_input(inp);

    for (c, f, t) in is {
        let (f, t) = take_two(&mut cs[..], f-1, t-1);
        t.extend(f.drain(f.len()-c .. ));
    }

    cs.into_iter().map(|v| *v.last().unwrap() as char).collect()
}

fn take_two<T>(s: &mut [T], a: usize, b: usize) -> (&mut T, &mut T) {
    match a.cmp(&b) {
        Ordering::Less => {
            let (r1, r2) = s.split_at_mut(b);
            (&mut r1[a], &mut r2[0])
        }
        Ordering::Equal => panic!("Can't get two mutable references to the same element."),
        Ordering::Greater => {
            let (r1, r2) = s.split_at_mut(a);
            (&mut r2[0], &mut r1[b])
        }
    }
}

fn parse_input(inp: &str) -> (Vec<Vec<u8>>, impl Iterator<Item=(usize, usize, usize)> + '_) {
    let (part1, part2) = inp.split_once("\n\n").unwrap();

    let mut p1lines = part1.lines().rev();
    let mut part1 = vec![Vec::new(); (p1lines.next().unwrap().len() + 2) / 4];
    p1lines.for_each(|l| {
        l.as_bytes().into_iter().chunks(4).into_iter().enumerate().map(|(i, mut c)| (i, *c.nth(1).unwrap())).filter(|(_, c)| *c != b' ').for_each(|(i, c)| {
            part1[i].push(c);
        })
    });

    let part2 = part2.lines().map(|l| {
        let (_, p1, _, p2, _, p3) = l.split_ascii_whitespace().collect_tuple().unwrap();
        (p1.parse::<usize>().unwrap(), p2.parse::<usize>().unwrap(), p3.parse::<usize>().unwrap())
    });

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
            let result = part1(include_str!("example1"));
            assert_eq!(String::from("CMZ"), result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(String::from("ZSQVCCJLL"), result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example1"));
        assert_eq!(String::from("MCD"), result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(String::from("QZFJRWHGS"), result);
    }
}


