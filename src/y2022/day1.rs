use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn part1() -> String {
    let mut result = String::new();
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./inputs/y2022/d1_input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut max_cal: u32 = 0;
        let mut max_cal_index: u32 = 0;
        let mut current_index: u32 = 1;
        let mut current_cal_sum: u32 = 0;
        for line in lines {
            match line {
                Ok(line_str) => {
                    if let "" = line_str.as_str() {
                        if current_cal_sum > max_cal {
                            max_cal = current_cal_sum;
                            max_cal_index = current_index;
                        }
                        current_index += 1;
                        current_cal_sum = 0;
                    } else {
                        if let Ok(cal) = &line_str.parse() {
                            current_cal_sum += cal;
                        }
                        else {
                            println!("Error");
                        }
                    }
                }
                Err(_) => (),
            }
        }
        // do checks for last elf
        if current_cal_sum > max_cal {
            max_cal = current_cal_sum;
            max_cal_index = current_index;
        }
        result = format!("elf[{}]={}", max_cal_index, max_cal);
    }
    result
}



pub fn part2() -> String {
    let result = String::new();
    let mut elfs_cal_map: Vec<(u32, u32)> = Vec::new();
    let mut current_index: u32 = 1;
    let mut current_cal_sum: u32 = 0;
    if let Ok(lines) = read_lines("./inputs/y2022/d1_input.txt") {
        println!("Error");
        lines.into_iter().for_each(|l| {
            if let Ok(line) = l {
                match line.as_str() {
                    "" => {
                        elfs_cal_map.push((current_cal_sum, current_index));
                        current_index += 1;
                        current_cal_sum = 0;
                    },
                    _ => {
                        if let Ok(cal) = line.as_str().parse::<u32>() {
                            current_cal_sum += cal;
                        }
                        else {
                            println!("Error");
                        }
                    }
                }
            }
            else {
                println!("Error");
            }
        });
        elfs_cal_map.push((current_cal_sum, current_index));
    }
    else {
        println!("Error");
    }

    elfs_cal_map.sort_by(|(a_cal, _), (b_cal, _)| b_cal.cmp(a_cal));
    let mut sum_top_3: u32 = 0;
    for (cal, idx) in elfs_cal_map.get(0..3).unwrap() {
        println!("Cal {:#?} index {:#?}", cal, idx);
        sum_top_3 += cal;
    }
    
    println!("Top 3 cal sum {:#?}", sum_top_3);
    result
}