static DAY_NO: &str = "02";
static INPUT: &str = include_str!("../../inputs/day02.txt");

// 1 - for Rock (A,X)
// 2 - for Paper (B,Y)
// 3 - for Scissors (C,Z)
// part 2:
// X - you need to lose
// Y - you need to end the round in a draw
// Z - you need to win

fn main() {
    let r1 = solve_first(INPUT);
    println!("Day {}\n first puzzle: {}", DAY_NO, r1.unwrap());
    let r2 = solve_second(INPUT);
    println!(" second puzzle: {}", r2.unwrap());
}

fn solve_first(input: &str) -> Result<i32, &'static str> {
    let i = d2_input(input);
    let mut my_score =0;

    i.iter().for_each(| round| {
        my_score += my_round_score(round[0], round[1]);
    });

    Ok(my_score)
}

fn solve_second(input: &str) -> Result<i32, &'static str> {
    let  i = d2_input(input); 
    let mut my_score = 0;
    i.iter().for_each(| round| {
        let p1_gest = round[0];
        let code = round[1];

        let my_gest =  
            if code == 1 {     // X loose
                match p1_gest {
                    1 => { 3 }
                    2 => { 1 }
                    _ => { 2 }
                }
            } else if code == 3 { // win
                match p1_gest {
                    1 => { 2 }
                    2 => { 3 }
                    _ => { 1 }
                }               
            } else { // Draw, use same
                p1_gest
            };
           my_score += my_round_score(p1_gest, my_gest);
    });

    Ok(my_score)
}

pub fn my_round_score(p1_gesture: i32, my_gesture: i32) -> i32 {
    let mut my_score = my_gesture;
    if p1_gesture == my_gesture {
        my_score  += 3;
    } else if my_gesture == 1 && p1_gesture == 3 || my_gesture == 2 && p1_gesture == 1 || my_gesture == 3 && p1_gesture == 2  {
        my_score +=6;
    }
    my_score
}

pub fn d2_input(input: &str) ->Vec<Vec<i32>> {
    input
        .lines()
        .map(|lines| {
           let x = lines.split(" ").
           
           fold(vec![], |mut a,c|  {
                match c {
                    "A" | "X" => a.push(1),
                    "B" | "Y" => a.push(2),
                    "C" | "Z" => a.push(3),
                    _ => {panic!("Input error")}
                }
                a
            }
        );
        x
    }
    )
            .collect()
}

#[cfg(test)]
mod tests {
    static EXAMPLE: &str = include_str!(r"../../inputs/day02_example.txt");

    use super::*;
    #[test]
    fn first() {
        assert_eq!(
            solve_first(INPUT).unwrap(),
            12276
        );
    }

    #[test]
    fn first_example() {
        assert_eq!(
            solve_first(EXAMPLE).unwrap(),
            15
        );
    }

    #[test]
    fn second() {
        assert_eq!(
            solve_second(INPUT).unwrap(),
            9975
        );
    }

    #[test]
    fn second_example() {
        assert_eq!(
            solve_second(EXAMPLE).unwrap(),
            12
        );
    }
}