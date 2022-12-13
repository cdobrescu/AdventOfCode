use std::collections::HashMap;

const INPUT: &str = include_str!("../../inputs/y2022/d7_input.txt");
// const INPUT: &str = include_str!("../../inputs/y2022/d7_test.txt");
const MAX_SIZE: u32 = 100000;

const TOTAL_DISK_SIZE: u32 = 70000000;
const NEEDED_SPACE: u32 = 30000000;





pub fn part1() -> String {
    //   let mut pwd = Vec::new();
    let mut pwd: Vec<String> = Vec::new();
    let lines = INPUT.split('\n').into_iter();

    let mut size_map: HashMap<String, u32> = HashMap::new();

    for l in lines {
        // println!("line {}", l);
        match l.split(' ').into_iter().collect::<Vec<&str>>()[..] {
            ["$", "cd", dir] => {
                // change dir command
                match dir{
                    ".." => {
                        // we want to add the total size of the child dir to the parent dir
                        let child_size = size_map.get_mut(&pwd.join("").to_string()).unwrap().clone();
                        pwd.pop();
                        let parent_size = size_map.get_mut(&pwd.join("").to_string()).unwrap();
                        *parent_size += child_size;
                    },
                    "/" => {
                        pwd.push("/".to_string());
                    }
                    _ => {
                        pwd.push(format!("{}/",dir));
                    }
                }
            },
            ["$", "ls"] => {
                // ls command
                size_map.insert(pwd.join("").to_string(), 0);
            },
            ["dir", _] => {
                // ls output - dir 
            },
            [f_s, _] => {
                    let current_size = size_map.get_mut(&pwd.join("").to_string()).unwrap();
                    *current_size += f_s.parse::<u32>().unwrap();

            },
            _ => {
                    println!("Unrecognized");   
            }
        }
    }
    println!("pwd={:?}", pwd);
    // we want to update the size of the paren dirs
    // so we emulate "cd .." until we end up in "/"
    while pwd.len() > 1 {
        let child_size = size_map.get_mut(&pwd.join("").to_string()).unwrap().clone();
        pwd.pop();
        let parent_size = size_map.get_mut(&pwd.join("").to_string()).unwrap();
        *parent_size += child_size;
    }


    let mut total_sum = 0;
    for (k, v ) in &size_map {

        println!("{}={}", k, v);
        if *v <= MAX_SIZE {
            total_sum += v;
        }
        
    }
    println!("pwd={:?}", pwd);
    let total_usage = size_map.get(&"/".to_string()).unwrap();
    println!("total_usage={}", total_usage);
    let needed_space = NEEDED_SPACE - (TOTAL_DISK_SIZE - total_usage);
    println!("needed_space={}", needed_space);
    let mut dir_delete = *total_usage;
    for (k, v ) in &size_map {
        if *v >= needed_space && *v < dir_delete {
            dir_delete = *v;
            println!("{}={}", k, v);
        }
    }
    println!("dir_delete (part 2)={}", dir_delete);
    format!("total_sum (part 1)={}", total_sum)
}
