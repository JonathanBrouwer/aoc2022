use crate::day17::Dir::{Left, Right};
use crate::day17::Shape::*;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

fn part1(inp: &str) -> usize {
    let mut input = parse_input(inp).cycle();

    let mut board: Vec<[bool; 7]> = vec![[false; 7]; 4];
    let mut start_height = 3;
    let mut pos = (3, 2);
    let mut shape = A;

    let mut count = 0;
    while count < 2022 {
        println!();
        // Move lr
        match input.next().unwrap() {
            Left if pos.1 != 0 && tiles(&shape, (pos.0, pos.1 - 1)).all(|t| !board[t.0][t.1]) => {
                pos.1 -= 1;
            }
            Right
                if pos.1 + shape.width() < 7
                    && tiles(&shape, (pos.0, pos.1 + 1)).all(|t| !board[t.0][t.1]) =>
            {
                pos.1 += 1;
            }
            _ => {}
        }

        // Move down
        if pos.0 + 1 - shape.height() != 0
            && tiles(&shape, (pos.0 - 1, pos.1)).all(|t| !board[t.0][t.1])
        {
            pos.0 -= 1;
        } else {
            tiles(&shape, (pos.0, pos.1)).for_each(|t| board[t.0][t.1] = true);
            shape = shape.next();
            start_height = start_height.max(pos.0 + 3);
            pos = (start_height + shape.height(), 2);
            while board.len() < start_height + shape.height() + 10 {
                board.push([false; 7]);
            }
            count += 1;
        }
    }

    return start_height - 2;
}

pub fn part2(inp: &str) -> usize {
    let input: Vec<_> = parse_input(inp).collect();

    let mut board: Vec<[bool; 7]> = vec![[false; 7]; 4];
    let mut start_height = 3;
    let mut extra_score = 0;
    let mut pos = (3, 2);
    let mut shape = A;
    let mut signatures: HashMap<_, (usize, usize)> = HashMap::new();
    let mut count = 0;
    let mut i = 0;

    while count < 1000000000000 {
        for _ in 0.. {
            // Move lr
            match input[i % input.len()] {
                Left if pos.1 != 0
                    && tiles(&shape, (pos.0, pos.1 - 1)).all(|t| !board[t.0][t.1]) =>
                {
                    pos.1 -= 1;
                }
                Right
                    if pos.1 + shape.width() < 7
                        && tiles(&shape, (pos.0, pos.1 + 1)).all(|t| !board[t.0][t.1]) =>
                {
                    pos.1 += 1;
                }
                _ => {}
            }
            i += 1;

            // Move down
            if pos.0 + 1 - shape.height() != 0
                && tiles(&shape, (pos.0 - 1, pos.1)).all(|t| !board[t.0][t.1])
            {
                pos.0 -= 1;
            } else {
                break;
            }
        }

        tiles(&shape, (pos.0, pos.1)).for_each(|t| board[t.0][t.1] = true);
        shape = shape.next();
        start_height = start_height.max(pos.0 + 3);
        pos = (start_height + shape.height(), 2);
        while board.len() < start_height + shape.height() + 1 {
            board.push([false; 7]);
        }
        count += 1;

        if extra_score == 0 && count > 10 && i % input.len() == 0 {
            let score = start_height - 2;

            let sig = signature(&board);
            match signatures.entry(sig) {
                Entry::Occupied(e) => {
                    let (pc, ps) = *e.get();

                    let d_score = score - ps;
                    let d_count = count - pc;

                    let iters = (1000000000000 - count) / d_count - 1;
                    extra_score = iters * d_score;
                    count += iters * d_count;
                    assert!(count <= 1000000000000);
                }
                Entry::Vacant(e) => {
                    e.insert((count, score));
                }
            }
        }
    }

    extra_score + start_height - 2
}

fn signature(board: &Vec<[bool; 7]>) -> [usize; 7] {
    let h: [usize; 7] = (0..7)
        .map(|x| (0..board.len()).rev().find(|y| board[*y][x]).unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    h.map(|v| v - h.iter().min().unwrap())
}

// fn signature(board: &Vec<[bool; 7]>) -> Vec<[bool; 7]> {
//     let mut res = Vec::new();
//     let mut row = [true; 7];
//     let mut y = board.len();
//     while row.iter().any(|b| *b) && y > 0 {
//         let mut next = row;
//         for x in 0..7 {
//             if board[y-1][x] {
//                 next[x] = false;
//             }
//         }
//
//         print!("");
//         // Scan right
//         for x in 0..6 {
//             if next[x] && !board[y-1][x+1] {
//                 next[x+1] = true;
//             }
//         }
//
//         // Scan left
//         for x in (1..7).rev() {
//             if next[x] && !board[y-1][x-1] {
//                 next[x-1] = true;
//             }
//         }
//
//         res.push(next);
//         row = next;
//         y -= 1;
//     }
//     print!("");
//     res
// }

#[derive(Copy, Clone, Eq, PartialEq)]
enum Shape {
    A,
    B,
    C,
    D,
    E,
}

impl Shape {
    fn next(&self) -> Shape {
        match self {
            A => B,
            B => C,
            C => D,
            D => E,
            E => A,
        }
    }

    fn width(&self) -> usize {
        match self {
            A => 4,
            B => 3,
            C => 3,
            D => 1,
            E => 2,
        }
    }

    fn height(&self) -> usize {
        match self {
            A => 1,
            B => 3,
            C => 3,
            D => 4,
            E => 2,
        }
    }
}

const A_POS: [(usize, usize); 4] = [(0, 0), (0, 1), (0, 2), (0, 3)];
const B_POS: [(usize, usize); 5] = [(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)];
const C_POS: [(usize, usize); 5] = [(0, 2), (1, 2), (2, 0), (2, 1), (2, 2)];
const D_POS: [(usize, usize); 4] = [(0, 0), (1, 0), (2, 0), (3, 0)];
const E_POS: [(usize, usize); 4] = [(0, 0), (0, 1), (1, 0), (1, 1)];

fn tiles(shape: &Shape, coord: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    match shape {
        A => A_POS[..].iter(), //vec!,
        B => B_POS[..].iter(), //vec![(coord.0,coord.1+1) , (coord.0+1,coord.1) , (coord.0+1,coord.1+1) , (coord.0+1,coord.1+2) , (coord.0+2,coord.1+1)],
        C => C_POS[..].iter(), //vec![(coord.0,coord.1+2) , (coord.0+1,coord.1+2) , (coord.0+2,coord.1) , (coord.0+2,coord.1+1) , (coord.0+2,coord.1+2)],
        D => D_POS[..].iter(), //vec![(coord.0,coord.1) , (coord.0+1,coord.1) , (coord.0+2,coord.1) , (coord.0+3,coord.1)],
        E => E_POS[..].iter(), //vec![(coord.0,coord.1) , (coord.0,coord.1+1) , (coord.0+1,coord.1) , (coord.0+1,coord.1+1)],
    }
    .map(move |&(x, y)| (coord.0 - x, coord.1 + y))
}

enum Dir {
    Left,
    Right,
}

fn parse_input(inp: &str) -> impl Iterator<Item = Dir> + '_ + Clone {
    inp.bytes().map(|b| match b {
        b'>' => Right,
        b'<' => Left,
        _ => unreachable!(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"));
        assert_eq!(3068, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(3069, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example1"));
        assert_eq!(1514285714288, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(1523167155404, result);
        //         1523174183274
    }
}
