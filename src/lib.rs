#![allow(dead_code)]

pub static FAIL_STRING: &str = "Could not solve task";
pub static PARSE_FAIL_STRING: &str = "Parsing error";

// Returns a vector of ints, taken from an aoc provided input file (one integer per row)
pub fn integer_rows_to_vector(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| line.trim().parse().expect("Not an integer"))
        .collect()
}

// Returns a vector of trimmed strings with row content
pub fn rows_to_vector(input: &str) -> Vec<&str> {
    input.lines().map(|line| line.trim()).collect()
}

// Returns a vector of vectors of ints from input. Empty spaces gives new vector
// i.e. 2022 day01
pub fn vec_of_integer_vec(input: &str) -> Vec<Vec<i32>> {
    let mut outer = vec![];
    let mut new = true;
    input.lines().for_each(|line| {
        if line.is_empty() {
            new = true;
        } else {
            if new {
                outer.push(vec![]);
                new = false;
            }
            outer
                .last_mut()
                .unwrap()
                .push(line.trim().parse().expect("Not an integer"));
        };
    });
    outer
}

fn test_inclusive_range(str: &str, min: &i32, max: &i32) -> bool {
    //    dbg!(str);
    let i = str.parse::<i32>();
    if i.is_ok() {
        let i = i.unwrap();
        return i.ge(min) && i.le(max);
    }
    false
}
