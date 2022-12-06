use std::{collections::{VecDeque, HashSet}};

const INPUT: &str = include_str!("../../inputs/y2022/d6_input.txt");
// const INPUT: &str = include_str!("../../inputs/y2022/d6_test.txt");

pub fn part1() -> String {
    let mut buffer: VecDeque<char> = VecDeque::new();
    let marker_len = 4; 
    let (index, _) = INPUT.chars().enumerate().find(|(_,c)| {
        buffer.push_back(*c);
        if buffer.len() < marker_len {
            return false
        }
        if buffer.len() > marker_len {
            buffer.pop_front();
        }
        if buffer.len() == marker_len {
            //convert buffer to set and check is the length doesn't change
            let v = buffer.iter().map(|c1| c1.to_owned()).collect::<HashSet<char>>();
            return v.len() == marker_len
        }
        false
    }).unwrap();
    format!("Found at {} {:?}", index + 1, buffer)
}

pub fn part2() -> String {
    let mut buffer: VecDeque<char> = VecDeque::new();
    let marker_len = 14; 
    let (index, _) = INPUT.chars().enumerate().find(|(_,c)| {
        buffer.push_back(*c);
        if buffer.len() < marker_len {
            return false
        }
        if buffer.len() > marker_len {
            buffer.pop_front();
        }
        if buffer.len() == marker_len {
            //convert buffer to set and check is the length doesn't change
            let v = buffer.iter().map(|c1| c1.to_owned()).collect::<HashSet<char>>();
            return v.len() == marker_len
        }
        false
    }).unwrap();
    format!("Found at {} {:?}", index + 1, buffer)
}