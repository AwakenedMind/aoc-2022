use std::{cell::RefCell, collections::HashMap, rc::Rc};

// Linux Commands for Fhs
#[derive(Debug, Clone)]
enum Cmd {
    NavToChildDir,
    NavToParentDir,
    NavToRootDir,
    PrintCurDir,
    PrintChildDir,
    PrintChildFile,
    Unknown,
}

/// Linux File Hierarchy System
#[derive(Debug, Clone)]
struct Fhs {
    cur_path: Rc<RefCell<String>>,
}

/// Memory Utility for Fhs
#[derive(Debug, Clone)]
struct MemUtil {
    all_dirs: Rc<RefCell<HashMap<String, i32>>>,
}

#[derive(Debug, Clone)]
struct CmdRes(Cmd, Option<i32>, Option<String>);

impl Fhs {
    // Create a new Fhs
    pub fn new() -> Fhs {
        Fhs {
            cur_path: Rc::new(RefCell::new(ROOT_DIR.to_owned())),
        }
    }

    pub fn parse_cmd(cmd: &str) -> CmdRes {
        // split the command into tokens and spaces and match the first token deeply nested match! no regex lol
        let vec_result = cmd.split(" ").map(String::from).collect::<Vec<_>>();

        // results
        let cmd_result: Cmd;
        let cmd_result_string: Option<String>;
        let cmd_result_size: Option<i32>;

        let first_el = vec_result[0].clone();
        let second_el = vec_result[1].clone();

        match first_el.as_str() {
            // sys shell cmd
            "$" => {
                // match the next token's first two chars
                match second_el.as_str() {
                    "ls" => {
                        cmd_result = Cmd::PrintCurDir;
                        cmd_result_size = None;
                        cmd_result_string = None;
                    }
                    "cd" => {
                        let third_el = vec_result[2].clone();

                        // match the third token's length
                        match third_el.len() {
                            1 => match third_el.as_str() {
                                // navigate to root dir
                                "/" => {
                                    cmd_result = Cmd::NavToRootDir;
                                    cmd_result_size = None;
                                    cmd_result_string = Some(third_el);
                                }
                                // navigate to some child dir T (single char)
                                _ => {
                                    cmd_result = Cmd::NavToChildDir;
                                    cmd_result_size = None;
                                    cmd_result_string = Some(third_el);
                                }
                            },
                            2 => match third_el.as_str() {
                                // navigate to parent dir
                                ".." => {
                                    cmd_result = Cmd::NavToParentDir;
                                    cmd_result_size = None;
                                    cmd_result_string = Some(third_el);
                                }
                                // navigate to child dir (two chars)
                                _ => {
                                    cmd_result = Cmd::NavToChildDir;
                                    cmd_result_size = None;
                                    cmd_result_string = Some(third_el.to_owned());
                                }
                            },
                            _ => {
                                // navigate to child dir (multiple chars)
                                cmd_result = Cmd::NavToChildDir;
                                cmd_result_size = None;
                                cmd_result_string = Some(third_el.to_owned());
                            }
                        }
                    }
                    // some other command that cant be parsed
                    _ => {
                        cmd_result = Cmd::Unknown;
                        cmd_result_size = None;
                        cmd_result_string = None;
                    }
                }
            }
            "dir" => {
                // name of the dir
                cmd_result = Cmd::PrintChildDir;
                cmd_result_size = None;
                cmd_result_string = Some(second_el);
            }
            // possibly a file size
            _ => match first_el.parse::<i32>() {
                Ok(_) => {
                    cmd_result = Cmd::PrintChildFile;
                    cmd_result_size = Some(first_el.parse::<i32>().unwrap());
                    cmd_result_string = Some(second_el);
                }
                Err(_) => {
                    cmd_result = Cmd::Unknown;
                    cmd_result_size = None;
                    cmd_result_string = None;
                }
            },
        };

        CmdRes(cmd_result, cmd_result_size, cmd_result_string)
    }
}

const ROOT_DIR: &'static str = "/";

pub fn solve(data: &str) -> (i32, i32) {
    // init os with root dir
    let os = Fhs::new();

    // init memory util
    let mem_util = MemUtil {
        all_dirs: Rc::new(RefCell::new(HashMap::new())),
    };

    // split file into line commands and collect into a vector
    let lines = data
        .lines()
        .map(|line| line.split("/n").collect())
        .collect::<Vec<String>>();

    // begin building the hashmap of dir:size
    lines.iter().for_each(|s| {
        // parse the command returing the cmd, size, and name of the file/dir
        let CmdRes(cmd, size, name) = Fhs::parse_cmd(s);

        // cur os path
        let cur_path = os.cur_path.as_ref().borrow_mut().clone();

        // ref to the hashmap
        let mut mem_util_ref = mem_util.all_dirs.as_ref().borrow_mut();

        match cmd {
            Cmd::NavToChildDir => {
                // re-calc the size of the current os path using previous paths splitting on "/"
                let new_size = cur_path
                    .split("/")
                    .map(|s| {
                        println!("s: {:?}", s);
                        let mut size = 0;
                        let key = format!("/{}", s);

                        if s.len() > 0 {
                            if let Some(s) = mem_util_ref.get(&key) {
                                size = *s;
                            }
                        }
                        size
                    })
                    .sum::<i32>();

                // update the size of the current path only if the new size is greater than 0
                if new_size > 0 {
                    mem_util_ref.insert(cur_path, new_size);
                }

                // current os path clone for mutability
                let cur_path = os.cur_path.as_ref().borrow_mut().clone();

                // new dir name we just parsed
                let new_dir = name.clone().unwrap();

                // update the current os path with the new dir we just parsed n
                if cur_path == ROOT_DIR {
                    let new_path = format!("/{}/", new_dir);

                    *os.cur_path.as_ref().borrow_mut() = new_path;
                    mem_util_ref.insert(format!("/{}/", name.as_ref().unwrap()), 0);
                } else {
                    // new paths need an ending slash
                    let new_path = format!("{}{}/", cur_path, name.unwrap());

                    *os.cur_path.as_ref().borrow_mut() = new_path;
                    mem_util_ref.insert(format!("{}{}/", cur_path, new_dir), 0);
                }
            }
            Cmd::NavToParentDir => {
                // remove last two slashes from cur_path to get the parent dir the cur_path
                let updated_path = cur_path
                    .split("/")
                    .take(cur_path.split("/").count() - 2)
                    .collect::<Vec<&str>>()
                    .join("/")
                    + "/";
                *os.cur_path.as_ref().borrow_mut() = updated_path;
            }
            Cmd::NavToRootDir => {
                *os.cur_path.as_ref().borrow_mut() = ROOT_DIR.to_owned();
            }
            // do nothing here
            Cmd::PrintCurDir | Cmd::PrintChildDir => (),
            Cmd::PrintChildFile => {
                let path: String;

                // create the new os_path for the directory we just parsed with the intention of adding it to the hashmap later
                if cur_path == ROOT_DIR {
                    path = format!("/{}", name.unwrap());
                } else {
                    path = format!("{}/{}", cur_path, name.unwrap());
                }

                // current directory
                let cur_dir = path
                    .split("/")
                    .take(path.split("/").count() - 2)
                    .collect::<Vec<&str>>()
                    .join("/")
                    + "/";

                // update the size of all nested dirs in the current os path since we can derive the prev keys from the current os path
                mem_util_ref
                    .clone()
                    .keys()
                    .filter(|k| {
                        println!("NESTED DIR k: {:?}", k);
                        cur_dir.contains(*k) && *k != &cur_dir
                    })
                    .collect::<Vec<_>>()
                    .iter()
                    .for_each(|k| {
                        let cur_size = mem_util_ref.get_mut(*k).unwrap();

                        // update file dir size with new size we just parsed
                        *cur_size += size.unwrap();
                    });

                // update the size of the current dir using the size we just parsed
                let mut cur_size = *mem_util_ref.get_mut(&cur_dir).unwrap_or(&mut 0);

                cur_size += size.unwrap();

                mem_util_ref.insert(cur_dir.to_owned(), cur_size);
            }
            Cmd::Unknown => (),
        }
    });

    let mem_util_ref = mem_util.all_dirs.as_ref().borrow_mut();

    let part1_result = solve_part_1(mem_util_ref.clone());
    let part2_result = solve_part_2(mem_util_ref.clone());

    (part1_result, part2_result)
}

fn solve_part_1(hm: HashMap<String, i32>) -> i32 {
    hm.iter()
        .collect::<Vec<(&String, &i32)>>()
        .iter_mut()
        .fold(0, |acc, x| {
            if *x.1 <= 100000 {
                return acc + *x.1;
            }
            acc
        })
}

fn solve_part_2(hm: HashMap<String, i32>) -> i32 {
    let total_used_disk_space = *hm.get("/").unwrap();

    let remaining_os_space = 70000000 - total_used_disk_space;

    let mut needed_os_space = 30000000 - remaining_os_space;

    let mut dirs = hm.into_values().collect::<Vec<i32>>();

    *dirs
        .iter_mut()
        .filter(|x| *x > &mut needed_os_space)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::advent_of_code::AdventOfCodeInput;

    #[test]
    fn d7_aoc_input_part1() {
        let aoc_input = AdventOfCodeInput::get_input(7);

        let answer = super::solve(&aoc_input.inp);

        assert_eq!(answer.0, 1443806);
    }

    #[test]
    fn d7_aoc_input_part2() {
        let aoc_input = AdventOfCodeInput::get_input(7);

        let answer = super::solve(&aoc_input.inp);

        assert_eq!(answer.1, 942298);
    }
}
