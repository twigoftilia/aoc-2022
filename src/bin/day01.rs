use aoc_2022::vec_of_integer_vec;

static DAY_1_INPUT: &str = include_str!(r"../../inputs/day01.txt");

fn main() {
    let v = vec_of_integer_vec(DAY_1_INPUT);
    let d1_1 = solve_first(&v);
    println!("Day 1\n first puzzle: {}", d1_1.unwrap());
    let d1_2 = solve_second(&v);
    println!(" second puzzle: {}", d1_2.unwrap());
}

fn solve_first(numbers: &Vec<Vec<i32>>) -> Result<i32, &'static str> {
    Ok(numbers.iter().map(|v| v.iter().sum::<i32>()).max().unwrap())
}

fn solve_second(numbers: &Vec<Vec<i32>>) -> Result<i32, &'static str> {
    let mut cals: Vec<i32> = numbers.iter().map(|v| v.iter().sum::<i32>()).collect();
    cals.sort_unstable();
    let sum: i32 = cals.iter().rev().take(3).sum();
    return Ok(sum);
}

#[cfg(test)]
mod tests {
    static DAY_1_EXAMPLE: &str = include_str!(r"../../inputs/day01_example.txt");

    use super::*;
    #[test]
    fn first_solution() {
        assert_eq!(
            solve_first(&vec_of_integer_vec(DAY_1_INPUT)).unwrap(),
            70296
        );
    }

    #[test]
    fn second_solution() {
        assert_eq!(
            solve_second(&vec_of_integer_vec(DAY_1_INPUT)).unwrap(),
            205381
        );
    }
    #[test]
    fn example_second() {
        assert_eq!(
            solve_second(&vec_of_integer_vec(DAY_1_EXAMPLE)).unwrap(),
            45000
        );
    }
}
