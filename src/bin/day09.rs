use std::collections::{BTreeMap, BTreeSet};

static DAY_NO: &str = "09";
static INPUT: &str = include_str!("../../inputs/day09.txt");
#[cfg(test)]
static EXAMPLE: &str = include_str!(r"../../inputs/day09_example.txt");
#[cfg(test)]
static EXAMPLE_2: &str = include_str!(r"../../inputs/day09_example_2.txt");

fn main() {
    let input = &Instruction::list_from_input(INPUT);
    let r1 = solve_both(&input, 2);
    println!("Day {}\n first puzzle: {}", DAY_NO, r1);
    let r2 = solve_both(&input, 10);
    println!(" second puzzle: {}", r2);
}

// fn solve_first_old(input: &Vec<Instruction>) -> usize {
//     let mut head = (0, 0);
//     let mut tail = (0, 0);

//     let mut tail_vistits = BTreeMap::new();

//     tail_vistits.insert(0, BTreeSet::from([0]));

//     for i in input {
//         //        dbg!(i);

//         for _step in 0..i.steps {
//             //  let last_tail = tail.clone();

//             match i.dir {
//                 Dir::U => head.1 -= 1,
//                 Dir::D => head.1 += 1,
//                 Dir::L => head.0 -= 1,
//                 Dir::R => head.0 += 1,
//             }

//             let tail_update_option = nearest_tail_move_step(&head, &tail, &i.dir);

//             match tail_update_option {
//                 Some(pos) => {
//                     tail = pos;
//                 }
//                 None => {}
//             }

//             let res = tail_vistits.get_mut(&tail.0);

//             match res {
//                 Some(v) => {
//                     v.insert(tail.1);
//                 }
//                 None => {
//                     tail_vistits.insert(tail.0, BTreeSet::from([tail.1]));
//                 }
//             }
//         }
//     }
//     let sum = tail_vistits
//         .values()
//         .map(|y_in_x_pos| y_in_x_pos.iter().count())
//         .sum();

//     sum
// }

fn solve_both(input: &Vec<Instruction>, knots: usize) -> usize {
    let mut rope: Vec<Pos> = vec![(0, 0); knots];
    let rope_length = rope.len();
    let mut tail_vistits = BTreeMap::new();
    tail_vistits.insert(0, BTreeSet::from([0]));
    for i in input {
        for _step in 0..i.steps {
            match i.dir {
                Dir::U => rope[0].1 -= 1,
                Dir::D => rope[0].1 += 1,
                Dir::L => rope[0].0 -= 1,
                Dir::R => rope[0].0 += 1,
            }
            for follwer_no in 1..rope_length {
                let tail_update_option =
                    nearest_tail_move_step(&rope[follwer_no - 1], &rope[follwer_no]);

                match tail_update_option {
                    Some(pos) => {
                        rope[follwer_no] = pos;
                    }
                    None => {}
                }
            }

            let res = tail_vistits.get_mut(&rope[rope_length - 1].0);

            match res {
                Some(v) => {
                    v.insert(rope[rope_length - 1].1);
                }
                None => {
                    tail_vistits.insert(
                        rope[rope_length - 1].0,
                        BTreeSet::from([rope[rope.len() - 1].1]),
                    );
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

fn nearest_tail_move_step(head: &Pos, tail: &Pos) -> Option<Pos> {
    let mut new_tail = tail.clone();

    //  should_tail_move at all
    if ((head.0 - tail.0) as i32).abs() <= 1 && ((head.1 - tail.1) as i32).abs() <= 1 {
        return None;
    }
    // up or down
    if head.0 == tail.0 || head.1 == tail.1 {
        if head.0 - tail.0 > 1 {
            new_tail.0 += 1;
        } else if head.0 - tail.0 < -1 {
            new_tail.0 -= 1;
        }

        if head.1 - tail.1 > 1 {
            new_tail.1 += 1;
        } else if head.1 - tail.1 < -1 {
            new_tail.1 -= 1;
        }
    } else {
        if head.0 > tail.0 {
            new_tail.0 += 1;
        } else {
            new_tail.0 -= 1;
        }

        if head.1 > tail.1 {
            new_tail.1 += 1;
        } else {
            new_tail.1 -= 1;
        }
    }
    Some(new_tail)
}
type Pos = (i32, i32);

#[derive(Debug)]
pub enum Dir {
    U,
    D,
    L,
    R,
}

impl Dir {
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
        assert_eq!(solve_both(&Instruction::list_from_input(EXAMPLE), 2), 13);
    }

    #[test]
    fn first() {
        assert_eq!(solve_both(&Instruction::list_from_input(INPUT), 2), 6190);
    }

    #[test]
    fn second_example_1() {
        assert_eq!(solve_both(&Instruction::list_from_input(EXAMPLE), 10), 1);
    }

    #[test]
    fn second_example_2() {
        assert_eq!(solve_both(&Instruction::list_from_input(EXAMPLE_2), 10), 36);
    }

    #[test]
    fn second() {
        assert_eq!(solve_both(&Instruction::list_from_input(INPUT), 10), 2516);
    }
}
