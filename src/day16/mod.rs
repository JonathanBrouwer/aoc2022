use std::collections::HashMap;

pub fn part1(inp: &str) -> usize {
    let input: HashMap<&str, _> = parse_input(inp).map(|(k, v1, v2)| (k, (v1, v2))).collect();

    let mut map: HashMap<(Vec<&str>, &str), usize> = HashMap::new();
    map.insert((Vec::new(), "AA"), 0);

    let eval = |v: &Vec<&str>| -> usize { v.iter().map(|v| input[v].0).sum() };

    for _ in 0..30 {
        let mut new_map = HashMap::new();

        for ((mut visited, current), value) in map {
            let score = eval(&visited);
            // Move
            for nb in &input.get(current).unwrap().1 {
                let x = new_map.entry((visited.clone(), *nb)).or_insert(0);
                *x = (*x).max(value + score);
            }

            // Open
            if !visited.contains(&current) && input[current].0 != 0 {
                visited.push(current);
                visited.sort();
                let x = new_map.entry((visited, current)).or_insert(0);
                *x = (*x).max(value + score);
            }
        }

        map = new_map;
    }

    let best = map.iter().max_by_key(|v| *v.1).unwrap();

    return *best.1;
}

pub fn part2(inp: &str) -> usize {
    let input: Vec<(&str, _)> = parse_input(inp).map(|(k, v1, v2)| (k, (v1, v2))).collect();
    let start = input.iter().enumerate().find(|v| v.1 .0 == "AA").unwrap().0;
    let input: Vec<(usize, Vec<usize>)> = input
        .iter()
        .enumerate()
        .map(|(_, (_, v))| {
            (
                v.0,
                v.1.iter()
                    .map(|v| input.iter().enumerate().find(|w| w.1 .0 == *v).unwrap().0)
                    .collect(),
            )
        })
        .collect();

    let mut map: HashMap<(usize, usize, usize), usize> = HashMap::new();
    map.insert((0, start, start), 0);

    let eval = |v: &usize| -> usize {
        let mut sum = 0;
        for i in 0..input.len() {
            if v & (1 << i) != 0 {
                sum += input[i].0
            }
        }
        sum
    };

    for i in 0..26 {
        println!("{}", i);
        let mut new_map = HashMap::new();

        for ((visited, c1, c2), value) in map {
            let score = eval(&visited);

            // Move c1
            for nb1 in &input.get(c1).unwrap().1 {
                //Move c2
                for nb2 in &input.get(c2).unwrap().1 {
                    let x = new_map.entry((visited.clone(), *nb1, *nb2)).or_insert(0);
                    *x = (*x).max(value + score);
                }

                //Open c2
                if (visited & (1 << c2) == 0) && input[c2].0 != 0 {
                    let x = new_map.entry((visited | (1 << c2), *nb1, c2)).or_insert(0);
                    *x = (*x).max(value + score);
                }
            }

            //Open c1
            if (visited & (1 << c1) == 0) && input[c1].0 != 0 {
                //Move c2
                for nb2 in &input.get(c2).unwrap().1 {
                    let x = new_map.entry((visited | (1 << c1), c1, *nb2)).or_insert(0);
                    *x = (*x).max(value + score);
                }

                //Open c2
                if (visited & (1 << c2) == 0) && input[c2].0 != 0 {
                    let x = new_map
                        .entry((visited | (1 << c1) | (1 << c2), c1, c2))
                        .or_insert(0);
                    *x = (*x).max(value + score);
                }
            }
        }

        map = new_map;
    }

    let best = map.iter().max_by_key(|v| *v.1).unwrap();

    println!("BEST: {}", best.1);
    return *best.1;
}

fn parse_input(inp: &str) -> impl Iterator<Item = (&str, usize, Vec<&str>)> {
    inp.lines().map(|line| {
        let name = &line[6..8];
        let (flow, valves) = line[23..]
            .split_once("; tunnels lead to valves ")
            .or(line[23..].split_once("; tunnel leads to valve "))
            .unwrap();
        (name, flow.parse().unwrap(), valves.split(", ").collect())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"));
        assert_eq!(1651, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(1460, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example1"));
        assert_eq!(1707, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(2117, result);
    }
}
