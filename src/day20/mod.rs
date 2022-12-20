use itertools::Itertools;

fn part1(inp: &str) -> isize {
    let data = parse_input(inp).collect_vec();
    let mut data = data.iter().enumerate().map(|(i, n)| {
        let prev = if i == 0 { data.len() - 1 } else { i - 1 };
        let next = (i + 1) % data.len();
        (*n, prev, next)
    }).collect_vec();

    for i in 0..data.len() {
        let d = data[i].0;
        if d < 0 {
            for _ in 0..d.abs() as usize % (data.len() - 1) {
                let l = data[i].1;
                let r = data[i].2;
                let ll = data[l].1;

                data[ll].2 = i;
                data[i].1 = ll;
                data[i].2 = l;
                data[l].1 = i;
                data[l].2 = r;
                data[r].1 = l;
            }
        } else if d > 0 {
            for _ in 0..d.abs() as usize % (data.len() - 1) {
                let l = data[i].1;
                let r = data[i].2;
                let rr = data[r].2;

                data[l].2 = r;
                data[r].1 = l;
                data[r].2 = i;
                data[i].1 = r;
                data[i].2 = rr;
                data[rr].1 = i;
            }
        }

    }

    let mut i = data.iter().find_position(|v| v.0 == 0).unwrap().0;
    let mut sum = 0;
    for _ in 0..3 {
        for _ in 0..1000 {
            i = data[i].2;
        }
        sum += data[i].0;
    }

    return sum;
}

fn part2(inp: &str) -> isize {
    let data = parse_input(inp).collect_vec();
    let mut data = data.iter().enumerate().map(|(i, n)| {
        let prev = if i == 0 { data.len() - 1 } else { i - 1 };
        let next = (i + 1) % data.len();

        let n = *n * 811589153;

        (n, prev, next)
    }).collect_vec();

    for _ in 0..10 {
        for i in 0..data.len() {
            let d = data[i].0;
            if d < 0 {
                for _ in 0..d.abs() as usize % (data.len() - 1) {
                    let l = data[i].1;
                    let r = data[i].2;
                    let ll = data[l].1;

                    data[ll].2 = i;
                    data[i].1 = ll;
                    data[i].2 = l;
                    data[l].1 = i;
                    data[l].2 = r;
                    data[r].1 = l;
                }
            } else if d > 0 {
                for _ in 0..d.abs() as usize % (data.len() - 1) {
                    let l = data[i].1;
                    let r = data[i].2;
                    let rr = data[r].2;

                    data[l].2 = r;
                    data[r].1 = l;
                    data[r].2 = i;
                    data[i].1 = r;
                    data[i].2 = rr;
                    data[rr].1 = i;
                }
            }
        }
    }

    let mut i = data.iter().find_position(|v| v.0 == 0).unwrap().0;
    let mut sum = 0;
    for _ in 0..3 {
        for _ in 0..1000 {
            i = data[i].2;
        }
        sum += data[i].0;
    }

    return sum;
}

fn parse_input(inp: &str) -> impl Iterator<Item=isize> + '_ {
    inp.lines().map(|n| n.parse().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"));
        assert_eq!(3, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(13967, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example1"));
        assert_eq!(1623178306, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(1790365671518, result);
    }
}
