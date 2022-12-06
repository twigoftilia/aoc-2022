use std::collections::BTreeSet;

static DAY_NO: &str = "06";
static INPUT: &str = include_str!("../../inputs/day06.txt");
// #[cfg(test)]
// static EXAMPLE: &str = include_str!(r"../../inputs/day06_example.txt");

fn main() {
    let i = to_byte_array(INPUT);
    let r1 = solve_first(i);
    println!("Day {}\n first puzzle: {}", DAY_NO, r1);
    let r2 = solve_second(&i);
    println!(" second puzzle: {}", r2);
}

fn solve_first(x1: &[u8]) -> usize {
    solve(x1, 4)
}

fn solve_second(x1: &[u8]) -> usize {
    solve(x1, 14)
}

fn solve(x1: &[u8], window_length: usize) -> usize {
    //  dbg!(x1);

    x1.windows(window_length)
        .map(|s| s.iter().collect::<BTreeSet<&u8>>())
        .enumerate()
        .find_map(|(n, s)| {
            if s.len() == window_length {
                return Some(n + window_length);
            }
            None
        })
        .unwrap()

    //  panic!("PanicAngst");
}

// fn solve_first_try(x1: &[u8], window_length: usize) -> usize {
//     //  dbg!(x1);
//     'window: for window_end in window_length..x1.len() {
//         let s = &x1[(window_end - window_length)..window_end];

//         for i in 0..s.len() {
//             for j in (i + 1)..(s.len()) {
//                 if s[i] == s[j] {
//                     continue 'window;
//                 }
//             }
//         }
//         return window_end;
//     }
//     panic!("PanicAngst");
// }

fn to_byte_array(i: &str) -> &[u8] {
    i.lines().take(1).next().unwrap().as_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_example() {
        assert_eq!(
            solve_first(to_byte_array("mjqjpqmgbljsphdztnvjfqwrcgsmlb")),
            7
        );

        assert_eq!(
            solve_first(to_byte_array("bvwbjplbgvbhsrlpgdmjqwftvncz")),
            5
        );

        assert_eq!(
            solve_first(to_byte_array("nppdvjthqldpwncqszvftbrmjlhg")),
            6
        );
        assert_eq!(
            solve_first(to_byte_array("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")),
            10
        );

        assert_eq!(
            solve_first(to_byte_array("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")),
            11
        );
    }

    #[test]
    fn first() {
        assert_eq!(solve_first(to_byte_array(INPUT)), 1538);
    }

    #[test]
    fn second_example() {
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(
            solve_second(to_byte_array("mjqjpqmgbljsphdztnvjfqwrcgsmlb")),
            19
        );

        assert_eq!(
            solve_second(to_byte_array("bvwbjplbgvbhsrlpgdmjqwftvncz")),
            23
        );

        assert_eq!(
            solve_second(to_byte_array("nppdvjthqldpwncqszvftbrmjlhg")),
            23
        );

        assert_eq!(
            solve_second(to_byte_array("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")),
            29
        );

        assert_eq!(
            solve_second(to_byte_array("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")),
            26
        );
    }

    #[test]
    fn second() {
        assert_eq!(solve_second(to_byte_array(INPUT)), 2315);
    }
}
