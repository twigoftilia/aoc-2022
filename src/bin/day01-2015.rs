static DAY_NO: &str = "01-2015";

static INPUT: &str = include_str!(r"../../inputs/day01-2015.txt");

// #[cfg(test)]
// static EXAMPLE: &str = include_str!(r"../../inputs/day04_example.txt");
fn main() {
    let r1 = solve_first(INPUT);
    println!("Day {}\n first puzzle: {}", DAY_NO, r1);
    // let r2 = solve_second(&pairs);
    // println!(" second puzzle: {}", r2);
}

fn solve_first(input: &str) -> i32 {
    let mut floor: i32 = 0;

    input.chars().for_each(|tecken| match tecken {
        '(' => {
            floor += 1;
        }
        ')' => {
            floor -= 1;
        }
        _ => {
            dbg!(" FEEL ");
        }
    });

    floor
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn first_example() {
    //     assert_eq!(solve_first(&parse_to_ranges(EXAMPLE)), 2);
    // }

    // #[test]
    // fn first() {
    //     assert_eq!(solve_first(&parse_to_ranges(INPUT)), 450);
    // }

    // #[test]
    // fn second_example() {
    //     assert_eq!(solve_second(&parse_to_ranges(EXAMPLE)), 4);
    // }

    // #[test]
    // fn second() {
    //     assert_eq!(solve_second(&parse_to_ranges(INPUT)), 837);
    // }
}
