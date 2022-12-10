use aoc_2022::Direction;

static DAY_NO: &str = "08";
static INPUT: &str = include_str!("../../inputs/day08.txt");
#[cfg(test)]
static EXAMPLE: &str = include_str!(r"../../inputs/day08_example.txt");

// not an very elgant / cleaned up implementations
fn main() {
    let input = &ForestMap::from_input(INPUT);
    let r1 = solve_first(&input);
    println!("Day {}\n first puzzle: {}", DAY_NO, r1);
    let r2 = solve_second(&input);
    println!(" second puzzle: {}", r2);
}

fn solve_first(input: &ForestMap) -> usize {
    input.visible_from_out_of_forest()
}

fn solve_second(input: &ForestMap) -> usize {
    input.best_scenic_score()
}

#[derive(Debug)]
struct ForestMap {
    tree: Vec<u8>,
    rows: usize,
    cols: usize,
}

impl ForestMap {
    fn from_input(input: &str) -> ForestMap {
        let mut forest_map = ForestMap {
            tree: Vec::new(),
            cols: 0,
            rows: 0,
        };
        forest_map.rows = input.lines().count();
        input.lines().enumerate().for_each(|(i, s)| {
            if i == 0 {
                forest_map.cols = s.len();
            }
            s.chars()
                .for_each(|s| forest_map.tree.push(s.to_string().parse().unwrap()));
        });
        forest_map
    }

    fn tree_at(&self, x: usize, y: usize) -> u8 {
        self.tree[x + y * self.cols]
    }

    fn best_scenic_score(&self) -> usize {
        let mut max_score = 0;
        for x in 1..(self.cols - 1) {
            for y in 1..(self.rows - 1) {
                let mut score = 1;
                let candidate_tree = self.tree_at(x, y);
                for dir in Direction::VALUES {
                    let tree_row = self.tree_row(x, y, &dir);
                    let mut point = 0;

                    for tree in tree_row {
                        point += 1;
                        if tree >= candidate_tree {
                            break;
                        }
                    }
                    score *= point;
                }
                max_score = max_score.max(score);
            }
        }
        max_score
    }

    fn tree_row(&self, x: usize, y: usize, direction: &Direction) -> Vec<u8> {
        let mut v = vec![];

        let mut x_next_in_row = x;
        let mut y_next_in_row = y;
        'break_out: loop {
            match direction {
                Direction::North => {
                    if y_next_in_row > 0 {
                        y_next_in_row -= 1;
                    } else {
                        break 'break_out;
                    }
                }
                Direction::South => {
                    if y_next_in_row < (self.rows - 1) {
                        y_next_in_row += 1;
                    } else {
                        break 'break_out;
                    }
                }

                Direction::West => {
                    if x_next_in_row > 0 {
                        x_next_in_row -= 1;
                    } else {
                        break 'break_out;
                    }
                }
                Direction::East => {
                    if x_next_in_row < (self.cols - 1) {
                        x_next_in_row += 1;
                    } else {
                        break 'break_out;
                    }
                }
            }
            v.push(self.tree_at(x_next_in_row, y_next_in_row));
        }
        v
    }

    fn visible_from_out_of_forest(&self) -> usize {
        let mut visible_inside = vec![false; self.rows * self.cols];

        let mut acc_visible = 2 * self.cols + 2 * self.rows - 4; // note must be at least 2 rows and 2 cols;

        let x_inner = 1..=(self.cols - 2);
        let y_inner = 1..=(self.cols - 2);

        // Hideous logic copies instead of matrix. Very unelegant, but who cares (but me)...
        for x in x_inner.clone() {
            let mut top_down_vision_stopped = false;
            let mut down_top_vision_stopped = false;
            let mut top_down_vision_max = self.tree_at(x, 0);
            let mut down_top_vision_max = self.tree_at(x, self.rows - 1);

            for y in y_inner.clone() {
                if top_down_vision_stopped && down_top_vision_stopped {
                    continue;
                }

                let current_tree = self.tree_at(x, y);

                if !top_down_vision_stopped {
                    if current_tree > self.tree_at(x, y - 1).max(top_down_vision_max) {
                        visible_inside[x + y * self.cols] = true;
                    }
                    if current_tree == 9 {
                        top_down_vision_stopped = true;
                    }
                    top_down_vision_max = top_down_vision_max.max(current_tree);
                }

                // down to up:
                let y_low = self.rows - 1 - y;

                let current_tree = self.tree_at(x, y_low);
                if !down_top_vision_stopped {
                    if current_tree > self.tree_at(x, y_low + 1).max(down_top_vision_max) {
                        visible_inside[x + y_low * self.cols] = true;
                    }
                    if current_tree == 9 {
                        down_top_vision_stopped = true;
                    }
                    down_top_vision_max = down_top_vision_max.max(current_tree);
                }
            }
        }

        for y in y_inner {
            let mut left_rigth_vision_stopped = false;
            let mut right_left_vision_stopped = false;
            let mut left_rigth_vision_max = self.tree_at(0, y);
            let mut right_left_vision_max = self.tree_at(self.cols - 1, y);
            for x in x_inner.clone() {
                if left_rigth_vision_stopped && right_left_vision_stopped {
                    continue;
                }

                let current_tree = self.tree_at(x, y);

                if !left_rigth_vision_stopped {
                    if current_tree > self.tree_at(x - 1, y).max(left_rigth_vision_max) {
                        visible_inside[x + y * self.cols] = true;
                    }
                    if current_tree == 9 {
                        left_rigth_vision_stopped = true;
                    }
                    left_rigth_vision_max = left_rigth_vision_max.max(current_tree);
                }

                // right to left
                let x_right = self.cols - 1 - x;

                let current_tree = self.tree_at(x_right, y);

                if !right_left_vision_stopped {
                    if current_tree > self.tree_at(x_right + 1, y).max(right_left_vision_max) {
                        visible_inside[x_right + y * self.cols] = true;
                    }
                    if current_tree == 9 {
                        right_left_vision_stopped = true;
                    }
                    right_left_vision_max = right_left_vision_max.max(current_tree);
                }
            }
        }

        acc_visible += visible_inside
            .iter()
            .filter(|s| {
                if **s {
                    return true;
                }
                false
            })
            .count();

        acc_visible
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_example() {
        assert_eq!(solve_first(&ForestMap::from_input(EXAMPLE)), 21);
    }

    #[test]
    fn first() {
        assert_eq!(solve_first(&ForestMap::from_input(INPUT)), 1703);
    }

    #[test]
    fn second_example() {
        assert_eq!(solve_second(&ForestMap::from_input(EXAMPLE)), 8);
    }

    #[test]
    fn second() {
        assert_eq!(solve_second(&ForestMap::from_input(INPUT)), 496650);
    }
}
