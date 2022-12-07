use crate::day7::Input::{CdIn, CdOut, LsDir, LsFile, LsHeader};

fn part1(inp: &str) -> usize {
    let mut result = 0;

    iterate_tree(inp, |v| {
        if v <= 100000 {
            result += v;
        }
    });

    result
}

fn part2(inp: &str) -> usize {
    let mut used = 0;
    iterate_tree(inp, |v| used = v);
    let needed = 30000000 + used - 70000000;

    let mut result = usize::MAX;
    iterate_tree(inp, |v| {
        if v >= needed {
            result = result.min(v);
        }
    });

    result
}

fn iterate_tree<'a>(inp: &str, mut f: impl FnMut(usize)) {
    let mut sum = vec![];
    for input in parse_input(inp) {
        match input {
            CdIn(_) => {
                sum.push(0);
            }
            CdOut => {
                let v = sum.pop().unwrap();
                f(v);
                *sum.last_mut().unwrap() += v;
            }
            LsHeader() => {}
            LsFile(v, _) => {
                *sum.last_mut().unwrap() += v;
            }
            LsDir(_) => {}
        }
    }

    while let Some(v) = sum.pop() {
        f(v);
        if !sum.is_empty() {
            *sum.last_mut().unwrap() += v;
        }
    }
}

enum Input<'a> {
    CdIn(&'a str),
    CdOut,
    LsHeader(),
    LsFile(usize, &'a str),
    LsDir(&'a str),
}

fn parse_input(inp: &str) -> impl Iterator<Item=Input> {
    inp.lines().map(|s| {
        if s == "$ ls" {
            LsHeader()
        } else if s == "$ cd .." {
            CdOut
        } else if s.starts_with("$ cd ") {
            CdIn(&s[5..])
        } else if s.starts_with("dir ") {
            let s = s.split_once(" ").unwrap();
            LsDir(s.1)
        }else{
            let s = s.split_once(" ").unwrap();
            LsFile(s.0.parse().unwrap(), s.1)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"));
        assert_eq!(95437, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(1141028, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example1"));
        assert_eq!(24933642, result);
    }

    #[test]
    fn test_part2_real() {
        // 9725484 too high
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(8278005, result);
    }
}
