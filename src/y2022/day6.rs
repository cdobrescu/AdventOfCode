use std::{collections::{VecDeque, HashSet}};

const INPUT: &str = include_str!("../../inputs/y2022/d6_input.txt");
// const INPUT: &str = include_str!("../../inputs/y2022/d6_test.txt");

pub fn part1() -> () {
    let mut buffer: VecDeque<char> = VecDeque::new();
    let mut cursor = 0;
    let marker_len = 4; 
    INPUT.chars().for_each(|c| {
        cursor += 1;
        buffer.push_back(c);
        if buffer.len() > marker_len {
            buffer.pop_front();
        }
        if buffer.len() == marker_len {
            //convert buffer to set and check is the length doesn't change
            let v = buffer.iter().map(|c1| c1.to_owned()).collect::<HashSet<char>>();
            if v.len() == marker_len
            {
                println!("Found at {} {:?}", cursor, v);
            }
        }

    });

}

pub fn part2() -> () {
    let mut buffer: VecDeque<char> = VecDeque::new();
    let mut cursor = 0;
    let marker_len = 14; 
    INPUT.chars().for_each(|c| {
        cursor += 1;
        buffer.push_back(c);
        if buffer.len() > marker_len {
            buffer.pop_front();
        }
        if buffer.len() == marker_len {
            let v = buffer.iter().map(|c1| c1.to_owned()).collect::<HashSet<char>>();
            if v.len() == marker_len
            {
                println!("Found at {} {:?}", cursor, v);
            }
        }
    });
}