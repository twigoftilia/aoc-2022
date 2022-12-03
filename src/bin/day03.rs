use array_tool::vec::Intersect;

static DAY_NO: &str = "03";
static INPUT: &str = include_str!("../../inputs/day03.txt");
fn main() {
    let r1 = solve_first(INPUT);
    println!("Day {}\n first puzzle: {}", DAY_NO, r1.unwrap());
    let r2 = solve_second(INPUT);
    println!(" second puzzle: {}", r2.unwrap());
}

fn solve_first(input: &str) -> Result<i32, &'static str> {
    let res = input
        .lines()
        .map(|line| {
            let len = line.len() / 2;
            let x1 = line[..len].as_bytes();
            let x2 = line[len..len * 2].as_bytes();

            for c1 in x1 {
                for c2 in x2 {
                    if c1 == c2 {
                        return score_item(*c1);
                    }
                }
            }
            panic!("OMG! Shouldn't be here 😞");
        })
        .sum();
    Ok(res)
}

fn solve_second(input: &str) -> Result<i32, &'static str> {
    let mut score = 0;
    let mut line_iter = input.lines();

    'line_loop: loop {
        for _elf_in_group in 0..3 {
            let w = line_iter.next();
            if w == None {
                break 'line_loop;
            }
            let x1 = w.unwrap().as_bytes();
            let x2 = line_iter.next().unwrap().as_bytes();
            let x3 = line_iter.next().unwrap().as_bytes();

            let i = x3.to_vec().intersect(x1.to_vec().intersect(x2.to_vec()));
            // assert!(i.len() == 1);

            score += score_item(i[0]);
        }
    }

    Ok(score)
}

fn score_item(c: u8) -> i32 {
    if c > 'Z' as u8 {
        return c as i32 - 'a' as i32 + 1;
    }
    c as i32 - 'A' as i32 + 27
}

#[cfg(test)]
mod tests {
    static EXAMPLE: &str = include_str!(r"../../inputs/day03_example.txt");

    use super::*;

    #[test]
    fn first_example() {
        assert_eq!(solve_first(EXAMPLE).unwrap(), 157);
    }

    #[test]
    fn first() {
        assert_eq!(solve_first(INPUT).unwrap(), 8515);
    }

    #[test]
    fn second_example() {
        assert_eq!(solve_second(EXAMPLE).unwrap(), 70);
    }

    #[test]
    fn second() {
        assert_eq!(solve_second(INPUT).unwrap(), 2434);
    }
}
