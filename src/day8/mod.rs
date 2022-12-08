fn part1(inp: &str) -> usize {
    let x = parse_input(inp);
    let n = x.len();
    let mut marked = vec![vec![false; n]; n];

    let l = (0..n).map(|prim| scan(n, (prim, 0), (0, 1)));
    let r = (0..n).map(|prim| scan(n, (prim, n-1), (0, -1)));
    let t = (0..n).map(|prim| scan(n, (0, prim), (1, 0)));
    let b = (0..n).map(|prim| scan(n, (n-1, prim), (-1, 0)));

    for mut scan in l.chain(r).chain(t).chain(b) {
        let pos0 = scan.next().unwrap();
        marked[pos0.0][pos0.1] = true;
        let mut max = x[pos0.0][pos0.1];
        for pos in scan {
            let v = x[pos.0][pos.1];
            if v > max {
                max = v;
                marked[pos.0][pos.1] = true;
            }
        }
    }

    marked
        .into_iter()
        .map(|r| r.into_iter().filter(|b| *b).count())
        .sum()
}

fn part2(inp: &str) -> usize {
    let input = parse_input(inp);
    let input = &input;
    let n = input.len();

    (0..n).map(move |x| {
        (0..n).map(move |y| {
            [(0,1), (0, -1), (1, 0), (-1, 0)].into_iter().map(|dir| {
                let v0 = input[x][y];
                let mut count = 0;
                for (xs, ys) in scan(n, (x, y), dir).skip(1) {
                    count += 1;
                    if input[xs][ys] >= v0 {
                        break;
                    }
                }
                count
            }).product()
        })
    }).flatten().max().unwrap()
}

pub fn scan(
    n: usize,
    start: (usize, usize),
    dir: (isize, isize),
) -> impl Iterator<Item=(usize, usize)> {
    (0..)
        .map(move |i| (start.0 as isize + i * dir.0, start.1 as isize + i * dir.1))
        .take_while(move |&(x, y)| x >= 0 && y >= 0 && x < n as isize && y < n as isize)
        .map(|(x, y)| (x as usize, y as usize))
}

fn parse_input(inp: &str) -> Vec<Vec<usize>> {
    inp.lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"));
        assert_eq!(21, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(1700, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example1"));
        assert_eq!(8, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(470596, result);
    }
}
