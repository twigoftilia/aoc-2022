use std::collections::HashMap;

static DAY_NO: &str = "05";
static INPUT: &str = include_str!("../../inputs/day05.txt");
#[cfg(test)]
static EXAMPLE: &str = include_str!(r"../../inputs/day05_example.txt");

fn main() {
    let d5_input = &D5Input::from_input(INPUT);
    let r1 = solve_first(&d5_input);
    println!("Day {}\n first puzzle: {}", DAY_NO, r1);
    let r2 = solve_second(&d5_input);
    println!(" second puzzle: {}", r2);
}

fn solve_first(d5_input: &D5Input) -> String {
    solve(d5_input, false)
}

fn solve_second(d5_input: &D5Input) -> String {
    solve(d5_input, true)
}

fn solve(d5_input: &D5Input, lift_all_on_one_using_cratemover9001: bool) -> String {
    let mut local_stacks = d5_input.initial_stacks.clone();

    d5_input.moves.iter().for_each(|crane_lift| {
        let dd = local_stacks.stacks.get_mut(&crane_lift.1).unwrap();

        let mut splitted = dd.split_off(dd.len() - crane_lift.0);

        if lift_all_on_one_using_cratemover9001 {
            splitted.reverse();
        }

        for _i in 0..splitted.len() {
            local_stacks.put_container_id(splitted.pop().unwrap(), crane_lift.2, false);
        }
    });
    local_stacks.top_crates()
}

#[derive(Clone, Debug)]
struct ContainerStacks {
    stacks: HashMap<usize, Vec<char>>,
}
struct D5Input {
    initial_stacks: ContainerStacks,
    moves: Vec<(usize, usize, usize)>,
}

impl ContainerStacks {
    fn put_container_id(&mut self, container_id: char, stack_no: usize, under: bool) {
        let stack = self.stacks.get_mut(&stack_no);
        if stack.is_none() {
            let new_stack = vec![container_id];
            self.stacks.insert(stack_no, new_stack);
        } else {
            if under {
                stack.unwrap().insert(0, container_id);
            } else {
                stack.unwrap().push(container_id);
            }
        }
    }

    fn top_crates(&self) -> String {
        let mut keys: Vec<usize> = self.stacks.keys().map(|k| *k).collect();
        keys.sort();

        let res = keys
            .iter()
            .map(|k| *self.stacks.get(k).unwrap().last().unwrap())
            .collect::<Vec<char>>();
        res.into_iter().collect::<String>()
    }

    fn new_empty() -> ContainerStacks {
        ContainerStacks {
            stacks: HashMap::new(),
        }
    }
}

impl D5Input {
    fn from_input(input: &str) -> D5Input {
        let mut d5 = D5Input {
            initial_stacks: ContainerStacks::new_empty(),
            moves: vec![],
        };

        input.lines().for_each(|s| {
            if s.starts_with("[") || s.starts_with("  ") {
                let cs = s.chars();

                cs.skip(1)
                    .enumerate()
                    .step_by(4)
                    .filter(|c| c.1 != ' ')
                    .for_each(|c| {
                        d5.initial_stacks.put_container_id(c.1, c.0 / 4, true);
                    });
            } else if s.starts_with("move") {
                let moves: Vec<usize> = s
                    .split(" ")
                    .skip(1)
                    .step_by(2)
                    .map(|s2| s2.parse::<usize>().expect("Not an integer"))
                    .collect();
                assert!(moves.len() == 3);
                d5.moves.push((moves[0], moves[1] - 1, moves[2] - 1));
            }
        });

        d5
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_example() {
        assert_eq!(solve_first(&D5Input::from_input(EXAMPLE)), "CMZ");
    }

    #[test]
    fn first() {
        assert_eq!(solve_first(&D5Input::from_input(INPUT)), "PTWLTDSJV");
    }

    #[test]
    fn second_example() {
        assert_eq!(solve_second(&D5Input::from_input(EXAMPLE)), "MCD");
    }

    #[test]
    fn second() {
        assert_eq!(solve_second(&D5Input::from_input(INPUT)), "WZMFVGGZP");
    }
}
