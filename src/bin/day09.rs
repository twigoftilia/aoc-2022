use std::{
    collections::{BTreeMap, BTreeSet},
    iter::Skip,
};

use aoc_2022::Direction;
use nalgebra::partial_max;

static DAY_NO: &str = "09";
static INPUT: &str = include_str!("../../inputs/day09.txt");
#[cfg(test)]
static EXAMPLE: &str = include_str!(r"../../inputs/day09_example.txt");

fn main() {
    let input = &Instruction::list_from_input(INPUT);
    let r1 = solve_first(&input);
    println!("Day {}\n first puzzle: {}", DAY_NO, r1);
    // let r2 = solve_second(&input);
    // println!(" second puzzle: {}", r2);
}

fn solve_first(input: &Vec<Instruction>) -> usize {
    let mut head = (0, 0);
    let mut tail = (0, 0);

    let mut tail_vistits = BTreeMap::new();

    tail_vistits.insert(0, BTreeSet::from([0]));

    for i in input {
        dbg!(i);

        for step in 0..i.steps {
            let last_head = head.clone();
            let last_tail = tail.clone();

            match i.dir {
                Dir::U => head.1 -= 1,
                Dir::D => head.1 += 1,
                Dir::L => head.0 -= 1,
                Dir::R => head.0 += 1,
            }
            //            dbg!(&head);

            let should_tail_move: bool =
                ((head.0 - tail.0) as i32).abs() > 1 || ((head.1 - tail.1) as i32).abs() > 1;

            if !should_tail_move {
                continue;
            }

            // ....
            // ....
            // ....
            // ....

            //   0123
            // 0 ....
            // 1 .T.H
            // 2 ....
            // 3 .H.T

            if head.0 - tail.0 > 1 {
                tail.0 += 1;
            } else if head.0 - tail.0 < -1 {
                tail.0 -= 1;
            }

            if head.1 - tail.1 > 1 {
                tail.1 += 1;
            } else if head.1 - tail.1 < -1 {
                tail.1 -= 1;
            }

            // Move in from diagonal position?
            if last_head.0 != last_tail.0 && last_head.1 != last_tail.1 {
                match i.dir {
                    Dir::U | Dir::D => tail.0 = head.0,
                    Dir::L | Dir::R => tail.1 = head.1,
                }
            }

            dbg!(tail);

            let mut res = tail_vistits.get_mut(&tail.0);

            match res {
                Some(v) => {
                    v.insert(tail.1);
                }
                None => {
                    tail_vistits.insert(tail.0, BTreeSet::from([tail.1]));
                }
            }
        }
    }
    let sum = tail_vistits
        .values()
        .map(|y_in_x_pos| y_in_x_pos.iter().count())
        .sum();

    sum
}

#[derive(Debug)]
pub enum Dir {
    U,
    D,
    L,
    R,
}

impl Dir {
    pub const VALUES: [Self; 4] = [Self::U, Self::D, Self::L, Self::R];

    fn parse(dirname: &str) -> Dir {
        match dirname {
            "U" => Dir::U,
            "D" => Dir::D,
            "L" => Dir::L,
            "R" => Dir::R,
            _ => {
                panic!("omg omg omg!");
            }
        }
    }
}

#[derive(Debug)]
struct Instruction {
    dir: Dir,
    steps: usize,
}

impl Instruction {
    fn list_from_input(input: &str) -> Vec<Instruction> {
        let x = input
            .lines()
            .map(|l| {
                let mut i = l.split(" ");
                Instruction {
                    dir: Dir::parse(i.next().unwrap()),
                    steps: i.next().unwrap().parse::<usize>().unwrap(),
                }
            })
            .collect();
        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_example() {
        assert_eq!(solve_first(&Instruction::list_from_input(EXAMPLE)), 13);
    }

    // #[test]
    // fn first() {
    //     assert_eq!(solve_first(&Instruction::from_input(INPUT)), 1703);
    // }

    // #[test]
    // fn second_example() {
    //     assert_eq!(solve_second(&Instruction::from_input(EXAMPLE)), 8);
    // }

    // #[test]
    // fn second() {
    //     assert_eq!(solve_second(&Instruction::from_input(INPUT)), 496650);
    // }
}
