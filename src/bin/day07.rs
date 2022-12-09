use std::collections::BTreeMap;

static DAY_NO: &str = "07";
static INPUT: &str = include_str!("../../inputs/day07.txt");
#[cfg(test)]
static EXAMPLE: &str = include_str!(r"../../inputs/day07_example.txt");

fn main() {
    let (r1, r2) = solve_both(&Dirs::parse(INPUT));
    println!("Day {}\n first puzzle: {}", DAY_NO, r1);
    println!(" second puzzle: {}", r2);
}
fn solve_both(dirs: &Dirs) -> (usize, usize) {
    let mut fact = Fact {
        dirs: dirs,
        task_sum: 0,
        sizes: vec![],
    };

    fact.call(&dirs.root.clone());
    fact.sizes.sort();
    let du = fact.sizes[fact.sizes.len() - 1];
    let free = 70000000 - du;
    let needed = 30000000 - free;
    let mut suitable_dirs_size = 0;

    for dirsize in fact.sizes {
        if dirsize >= needed {
            suitable_dirs_size = dirsize;
            break;
        }
    }
    (fact.task_sum, suitable_dirs_size)
}

struct Fact<'a> {
    dirs: &'a Dirs,
    task_sum: usize,
    sizes: Vec<usize>,
}

impl Fact<'_> {
    fn call(&mut self, path: &DirPath) -> usize {
        let d = self.dirs.dirs.get(&path).unwrap();

        let s = d.own_size;
        let mut sum_inc_subdirs = s;

        d.sub_dirs.iter().for_each(|p| {
            sum_inc_subdirs += self.call(p);
        });

        if sum_inc_subdirs <= 100000 {
            self.task_sum += sum_inc_subdirs;
        }
        self.sizes.push(sum_inc_subdirs);
        sum_inc_subdirs
    }
}

#[derive(Eq, PartialEq, Debug, Hash, Default, Clone, Ord, PartialOrd)]
struct DirPath {
    path: Vec<String>,
}

#[derive(PartialEq, Eq, Hash, Debug, Default)]
struct Dir {
    name: String,
    own_size: usize,
    sub_dirs: Vec<DirPath>,
}

#[derive(PartialEq, Eq, Hash, Debug, Default)]
struct Dirs {
    root: DirPath,
    dirs: BTreeMap<DirPath, Dir>,
}

impl Dirs {
    fn parse(i: &str) -> Dirs {
        let mut dirs = Dirs::default();
        let mut root_path = DirPath::default();
        root_path.path.push(String::from("root"));
        dirs.root = root_path.clone();
        let mut root = Dir::default();
        root.name = String::from("root");
        dirs.dirs.insert(root_path.clone(), root);
        let mut current_path = root_path.clone();

        i.lines().for_each(|line| {
            let v: Vec<&str> = line.split(" ").collect();

            // cd command
            if v.len() == 3 {
                if v[2] == ".." {
                    let mut path = current_path.clone();
                    if path.path.len() > 1 {
                        path.path.pop();
                    }
                    current_path = path;
                } else if v[2] == "/" {
                    current_path = root_path.clone();
                } else {
                    let mut new_dir = Dir::default();
                    new_dir.name = String::from(v[2]);
                    let mut path = current_path.clone();
                    path.path.push(new_dir.name.clone());

                    dirs.dirs
                        .get_mut(&current_path)
                        .unwrap()
                        .sub_dirs
                        .push(path.clone());

                    current_path = path.clone();

                    dirs.dirs.insert(path, new_dir);
                }
            } else if v.len() == 2 {
                if v[1].starts_with("ls") {
                } else {
                    // assume data
                    if v[0].starts_with("dir") {
                        // ignore
                    } else if v[0].len() > 0 {
                        let v = v[0].parse::<usize>();
                        if let Ok(i) = v {
                            {
                                dirs.dirs.get_mut(&current_path).unwrap().own_size += i;
                            }
                        }
                    }
                }
            } else {
                panic!("Oh no, not 2 or 3");
            }
        });
        dirs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let (a, b) = solve_both(&Dirs::parse(EXAMPLE));
        assert_eq!(a, 95437);
        assert_eq!(b, 24933642);
    }

    #[test]
    fn solve() {
        let (a, b) = solve_both(&Dirs::parse(INPUT));
        assert_eq!(a, 1555642);
        assert_eq!(b, 5974547);
    }
}
