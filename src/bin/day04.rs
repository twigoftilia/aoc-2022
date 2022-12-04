use std::ops::RangeInclusive;

static DAY_NO: &str = "04";
static INPUT: &str = include_str!("../../inputs/day04.txt");
#[cfg(test)]
static EXAMPLE: &str = include_str!(r"../../inputs/day04_example.txt");
fn main() {
    let pairs = parse_to_ranges(INPUT);

    let r1 = solve_first(&pairs);
    println!("Day {}\n first puzzle: {}", DAY_NO, r1);
    let r2 = solve_second(&pairs);
    println!(" second puzzle: {}", r2);
}

fn solve_first(pairs: &Vec<(RangeInclusive<usize>, RangeInclusive<usize>)>) -> usize {
    pairs
        .iter()
        .filter(|s| one_overlaps_the_other_fully(&s.0, &s.1))
        .count()
}

fn solve_second(pairs: &Vec<(RangeInclusive<usize>, RangeInclusive<usize>)>) -> usize {
    pairs.iter().filter(|s| intersects(&s.0, &s.1)).count()
}

fn parse_to_ranges(input: &str) -> Vec<(RangeInclusive<usize>, RangeInclusive<usize>)> {
    input
        .lines()
        .map(|line| {
            //            dbg!(line);
            let edges = line
                .split(|c| c == ',' || c == '-')
                .map(|s| s.trim().parse().expect("Not an integer"))
                .collect::<Vec<usize>>();
            (edges[0]..=edges[1], edges[2]..=edges[3])
        })
        .collect()
}

fn intersects(range_a: &RangeInclusive<usize>, range_b: &RangeInclusive<usize>) -> bool {
    if range_a.contains(range_b.start())
        || range_a.contains(range_b.end())
        || range_b.contains(range_a.start())
        || range_b.contains(range_a.end())
    {
        return true;
    }
    false
}

fn one_overlaps_the_other_fully(
    range_a: &RangeInclusive<usize>,
    range_b: &RangeInclusive<usize>,
) -> bool {
    if range_a.contains(range_b.start()) && range_a.contains(range_b.end())
        || range_b.contains(range_a.start()) && range_b.contains(range_a.end())
    {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_example() {
        assert_eq!(solve_first(&parse_to_ranges(EXAMPLE)), 2);
    }

    #[test]
    fn first() {
        assert_eq!(solve_first(&parse_to_ranges(INPUT)), 450);
    }

    #[test]
    fn second_example() {
        assert_eq!(solve_second(&parse_to_ranges(EXAMPLE)), 4);
    }

    #[test]
    fn second() {
        assert_eq!(solve_second(&parse_to_ranges(INPUT)), 837);
    }
}
