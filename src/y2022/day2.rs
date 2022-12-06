use std::borrow::Borrow;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use std::collections::HashMap;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn part1() -> () {
    let mut game_score = HashMap::new();
    game_score.insert("A X", 3u32 + 1);
    game_score.insert("A Y", 6u32 + 2);
    game_score.insert("A Z", 0u32 + 3);
    game_score.insert("B X", 0u32 + 1);
    game_score.insert("B Y", 3u32 + 2);
    game_score.insert("B Z", 6u32 + 3);
    game_score.insert("C X", 6u32 + 1);
    game_score.insert("C Y", 0u32 + 2);
    game_score.insert("C Z", 3u32 + 3);

    let mut total_score: u32 = 0;
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./inputs/y2022/d2_input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            match line {
                Ok(line_str) => {

                    if let Some(score) = game_score.get(&line_str.borrow()) {
                        total_score += *score;
                    }
                }
                Err(_) => (),
            }
        }
    }
    println!("Total score: {:#?}", total_score);
}

pub fn part2() -> () {
    let mut game_score = HashMap::new();
    game_score.insert("A A", 3u32 + 1);
    game_score.insert("A B", 6u32 + 2);
    game_score.insert("A C", 0u32 + 3);
    game_score.insert("B A", 0u32 + 1);
    game_score.insert("B B", 3u32 + 2);
    game_score.insert("B C", 6u32 + 3);
    game_score.insert("C A", 6u32 + 1);
    game_score.insert("C B", 0u32 + 2);
    game_score.insert("C C", 3u32 + 3);

    let mut game_move = HashMap::new();
    game_move.insert("A X", "A C");
    game_move.insert("A Y", "A A");
    game_move.insert("A Z", "A B");
    game_move.insert("B X", "B A");
    game_move.insert("B Y", "B B");
    game_move.insert("B Z", "B C");
    game_move.insert("C X", "C B");
    game_move.insert("C Y", "C C");
    game_move.insert("C Z", "C A");

    let mut total_score: u32 = 0;
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./inputs/y2022/d2_input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            match line {
                Ok(line_str) => {
                    if let Some(&gmove) = game_move.get(&line_str.borrow()) {
                        if let Some(score) = game_score.get(gmove) {
                            println!("{:#?} -> {:#?}={:#?}", &line_str, gmove, score);
                            total_score += *score;
                        }
                    }
                }
                Err(_) => (),
            }
        }
    }
    println!("Total score: {:#?}", total_score);
}