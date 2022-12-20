use itertools::Itertools;
use std::collections::HashSet;

pub fn part1(inp: &str) -> u32 {
    parse_input(inp)
        .enumerate()
        .map(|(bid, b)| {
            let max_ore = [
                b.ore_robot_cost,
                b.clay_robot_cost,
                b.obsidian_robot_cost.0,
                b.geode_robot_cost.0,
            ]
            .into_iter()
            .max()
            .unwrap();
            let max_clay = b.obsidian_robot_cost.1;
            let max_obs = b.geode_robot_cost.1;

            let mut states = HashSet::new();
            states.insert(State([(0, 1), (0, 0), (0, 0), (0, 0)]));

            for i in 0..24 {
                println!("{}", i);
                let mut new_states = HashSet::new();

                for state in states {
                    let state = state.0;
                    let upd_state = state.map(|p| (p.0 + p.1, p.1));

                    if state[0].0 >= b.ore_robot_cost {
                        new_states.insert(State([
                            (upd_state[0].0 - b.ore_robot_cost, upd_state[0].1 + 1),
                            upd_state[1],
                            upd_state[2],
                            upd_state[3],
                        ]));
                    }
                    if state[0].0 >= b.clay_robot_cost {
                        new_states.insert(State([
                            (upd_state[0].0 - b.clay_robot_cost, upd_state[0].1),
                            (upd_state[1].0, upd_state[1].1 + 1),
                            upd_state[2],
                            upd_state[3],
                        ]));
                    }
                    if state[0].0 >= b.obsidian_robot_cost.0
                        && state[1].0 >= b.obsidian_robot_cost.1
                    {
                        new_states.insert(State([
                            (upd_state[0].0 - b.obsidian_robot_cost.0, upd_state[0].1),
                            (upd_state[1].0 - b.obsidian_robot_cost.1, upd_state[1].1),
                            (upd_state[2].0, upd_state[2].1 + 1),
                            upd_state[3],
                        ]));
                    }
                    if state[0].0 >= b.geode_robot_cost.0 && state[2].0 >= b.geode_robot_cost.1 {
                        new_states.insert(State([
                            (upd_state[0].0 - b.geode_robot_cost.0, upd_state[0].1),
                            upd_state[1],
                            (upd_state[2].0 - b.geode_robot_cost.1, upd_state[2].1),
                            (upd_state[3].0, upd_state[3].1 + 1),
                        ]));
                    }

                    if state[0].0 < max_ore || state[1].0 < max_clay || state[2].0 < max_obs {
                        new_states.insert(State(upd_state));
                    }
                }

                states = new_states;
            }

            let v = (bid + 1) as u32 * states.into_iter().map(|s| s.0[3].0).max().unwrap();
            println!("{}", v);
            v
        })
        .sum()
}

//ore clay obs geod
//(count, rbt_count)
#[derive(Hash, Eq, PartialEq)]
struct State([(u32, u32); 4]);

impl State {
    fn gte(&self, other: &Self) -> bool {
        self.0.iter().zip(other.0.iter()).all(|(a, b)| a >= b)
    }
}

pub fn part2(inp: &str) -> u32 {
    parse_input(inp)
        .enumerate()
        .take(3)
        .map(|(bid, b)| {
            let mut states = HashSet::new();
            states.insert(State([(0, 1), (0, 0), (0, 0), (0, 0)]));

            for i in 0..32 {
                println!("{}", i);
                let mut new_states = HashSet::new();

                for state in states {
                    let state = state.0;
                    let upd_state = state.map(|p| (p.0 + p.1, p.1));

                    if state[0].0 >= b.ore_robot_cost {
                        new_states.insert(State([
                            (upd_state[0].0 - b.ore_robot_cost, upd_state[0].1 + 1),
                            upd_state[1],
                            upd_state[2],
                            upd_state[3],
                        ]));
                    }
                    if state[0].0 >= b.clay_robot_cost {
                        new_states.insert(State([
                            (upd_state[0].0 - b.clay_robot_cost, upd_state[0].1),
                            (upd_state[1].0, upd_state[1].1 + 1),
                            upd_state[2],
                            upd_state[3],
                        ]));
                    }
                    if state[0].0 >= b.obsidian_robot_cost.0
                        && state[1].0 >= b.obsidian_robot_cost.1
                    {
                        new_states.insert(State([
                            (upd_state[0].0 - b.obsidian_robot_cost.0, upd_state[0].1),
                            (upd_state[1].0 - b.obsidian_robot_cost.1, upd_state[1].1),
                            (upd_state[2].0, upd_state[2].1 + 1),
                            upd_state[3],
                        ]));
                    }
                    if state[0].0 >= b.geode_robot_cost.0 && state[2].0 >= b.geode_robot_cost.1 {
                        new_states.insert(State([
                            (upd_state[0].0 - b.geode_robot_cost.0, upd_state[0].1),
                            upd_state[1],
                            (upd_state[2].0 - b.geode_robot_cost.1, upd_state[2].1),
                            (upd_state[3].0, upd_state[3].1 + 1),
                        ]));
                    }

                    new_states.insert(State(upd_state));
                }

                let mins_left = 31 - i;
                let min_geodes = new_states
                    .iter()
                    .map(|s| s.0[3].0 + s.0[3].1 * mins_left)
                    .max()
                    .unwrap();

                states = new_states
                    .into_iter()
                    .filter(|s| {
                        s.0[3].0 + s.0[3].1 * mins_left + (mins_left * (mins_left - 1) / 2)
                            >= min_geodes
                    })
                    .collect();
            }

            states.into_iter().map(|s| s.0[3].0).max().unwrap()
        })
        .product()
}

struct Blueprint {
    ore_robot_cost: u32,             // ore
    clay_robot_cost: u32,            // ore
    obsidian_robot_cost: (u32, u32), // ore,clay
    geode_robot_cost: (u32, u32),    // ore,obsidian
}

fn parse_input<'a>(inp: &'a str) -> impl Iterator<Item = Blueprint> + 'a {
    inp.lines().map(|line| {
        let (a, b, c, d, e, f) = line
            .split(" ")
            .map(|n| n.parse::<u32>())
            .flatten()
            .collect_tuple()
            .unwrap();
        Blueprint {
            ore_robot_cost: a,
            clay_robot_cost: b,
            obsidian_robot_cost: (c, d),
            geode_robot_cost: (e, f),
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"));
        assert_eq!(33, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(1266, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example1"));
        assert_eq!(3472, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(5800, result);
    }
}
