use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;

fn part1(inp: &str) -> usize {
    let input = parse_input(inp);
    let input = &input;
    let start = input
        .iter()
        .enumerate()
        .map(|l| {
            l.1.iter()
                .enumerate()
                .find(|c| *c.1 == 'S')
                .map(|p| (l.0, p.0))
        })
        .flatten()
        .next()
        .unwrap();

    let (v, _) = dijkstra::<(usize, usize), _>(
        [(start, 0)].into_iter(),
        |p| input[p.0][p.1] == 'E',
        |&p| {
            nbs(p, input.len(), input[0].len())
                .filter(move |nb| {
                    let o = match input[p.0][p.1] {
                        'S' => 0,
                        'E' => 25,
                        c => c as usize - 'a' as usize,
                    };
                    let n = match input[nb.0][nb.1] {
                        'S' => 0,
                        'E' => 25,
                        c => c as usize - 'a' as usize,
                    };
                    o > n || (n as usize - o as usize) <= 1
                })
                .map(|p| (p, 1))
        },
    )
    .unwrap();

    v
}

fn part2(inp: &str) -> usize {
    let input = parse_input(inp);
    let input = &input;

    let starts = input
        .iter()
        .enumerate()
        .map(|l| {
            l.1.iter()
                .enumerate()
                .find(|c| *c.1 == 'S' || *c.1 == 'a')
                .map(|p| (l.0, p.0))
        })
        .flatten();

    let (v, _) = dijkstra::<(usize, usize), _>(
        starts.map(|s| (s, 0)),
        |p| input[p.0][p.1] == 'E',
        |&p| {
            nbs(p, input.len(), input[0].len())
                .filter(move |nb| {
                    let o = match input[p.0][p.1] {
                        'S' => 0,
                        'E' => 25,
                        c => c as usize - 'a' as usize,
                    };
                    let n = match input[nb.0][nb.1] {
                        'S' => 0,
                        'E' => 25,
                        c => c as usize - 'a' as usize,
                    };
                    o > n || (n as usize - o as usize) <= 1
                })
                .map(|p| (p, 1))
        },
    )
    .unwrap();
    v
}

fn nbs((px, py): (usize, usize), n: usize, m: usize) -> impl Iterator<Item = (usize, usize)> {
    let mut nbs = Vec::new();
    if px != 0 {
        nbs.push((px - 1, py));
    }
    if py != 0 {
        nbs.push((px, py - 1));
    }
    if px != n - 1 {
        nbs.push((px + 1, py));
    }
    if py != m - 1 {
        nbs.push((px, py + 1));
    }
    nbs.into_iter()
}

fn parse_input(inp: &str) -> Vec<Vec<char>> {
    inp.lines().map(|line| line.chars().collect()).collect()
}

fn dijkstra<N: Eq + Hash + Clone + Copy, IN: IntoIterator<Item = (N, usize)>>(
    starts: impl Iterator<Item = (N, usize)>,
    is_end: impl Fn(&N) -> bool,
    nbs: impl Fn(&N) -> IN,
) -> Option<(usize, Vec<N>)> {
    #[derive(Eq, PartialEq)]
    struct State<N: Eq + Clone>(N, usize);
    impl<N: Eq + Clone> PartialOrd<Self> for State<N> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    impl<N: Eq + Clone> Ord for State<N> {
        fn cmp(&self, other: &Self) -> Ordering {
            self.1.cmp(&other.1).reverse()
        }
    }

    let mut distances = HashMap::new();
    let mut heap = BinaryHeap::new();

    for (start, startd) in starts {
        distances.insert(start, (startd, None, false));
        heap.push(State(start, startd));
    }

    while let Some(State(n, c)) = heap.pop() {
        //If we found something shorter or equally good, continue
        let (_, _, visited) = distances.get_mut(&n).unwrap();
        if *visited {
            continue;
        }
        *visited = true;

        //Found goal
        if is_end(&n) {
            // Collect path
            let mut path = vec![n];
            while let Some(next) = distances[path.last().unwrap()].1 {
                path.push(next)
            }
            path.reverse();

            return Some((c, path));
        }

        // Iterate over neighbours
        for (nb, nb_cost) in nbs(&n) {
            let alt = c + nb_cost;

            // Get entry for distance of this neighbour
            match distances.entry(nb) {
                // If node is encountered, update cost if better
                Entry::Occupied(mut cost_old) => {
                    let (cost_old, prev, _) = cost_old.get_mut();
                    if *cost_old > alt {
                        *cost_old = alt;
                        heap.push(State(nb, alt));
                        *prev = Some(n);
                    }
                }
                // If node is not encountered, store it
                Entry::Vacant(cost_old) => {
                    cost_old.insert((alt, Some(n), false));
                    heap.push(State(nb, alt));
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"));
        assert_eq!(31, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(350, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example1"));
        assert_eq!(29, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(349, result);
    }
}
