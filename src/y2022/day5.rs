use std::{borrow::{Borrow}};

// const INPUT: &str = include_str!("../../inputs/y2022/d5_input.txt");
const INPUT: &str = include_str!("../../inputs/y2022/d5_test.txt");

#[derive(Eq, PartialEq)]
enum ParseStatus {
    // An `enum` may either be `unit-like`,
    StackMap,
    SpacksParsed
}



fn parse_stacks(stacks: &mut Vec<Vec<char>>, stacks_str: &Vec<&str>) -> () {
    //locate the longest line and compute the number of cols
    let max_len = stacks_str.iter()
        .map(|&line| line.len()).max().unwrap();
    
        let col_count = (max_len + 1) / 4; 

    for _ in 0..col_count {
        stacks.push(Vec::new());
    }

    println!("Col count = {:?}", (max_len + 1) / 4);
    
    stacks_str.iter().rev().for_each(|&line| {
        println!("Line {:?}", line);
        
        for col in 0..col_count {
            let cursor = col * 4;
            if let Some(col_data) = line.get(cursor..(cursor+3)) {
                match col_data.chars().nth(1) {
                    Some(' ') => {
                        // println!("Col {:?} empty", col);
                        
                    },
                    Some(c) => {
                        // println!("Col {:?} = {:?}", col, c);
                        let col_vec: &mut Vec<char> = stacks.get_mut(col).unwrap();
                        col_vec.push(c);
                    },
                    None => { break;}
                }

            }
        }
    });
}


fn parse_move(line: &str) -> Option<(usize, usize, usize)> {
    let splits = line.split_whitespace().collect::<Vec<&str>>();
    if splits.len() == 5 {
        if let [_, n, _, x, _, y] = splits[..] {
            return Some((n.parse::<usize>().unwrap(), 
                x.parse::<usize>().unwrap(),
                y.parse::<usize>().unwrap()));
        };
    }
    None
}

fn do_move(stacks: &mut Vec<Vec<char>>, stack_move: &(usize, usize, usize)) {

    let &(count,src,dst) = stack_move;
    let src_stack = stacks.get_mut(src - 1).unwrap();
    let src_len = src_stack.len();
    let mut buffer: Vec<char> = src_stack.split_off(src_len - count);
    assert!(buffer.len() == count);
    println!("buffer {:?}", buffer);
    buffer.reverse(); // part1

    stacks.get_mut(dst - 1).unwrap().append(&mut buffer);
}

fn do_move_part2(stacks: &mut Vec<Vec<char>>, stack_move: &(usize, usize, usize)) {

    let &(count,src,dst) = stack_move;
    let src_stack = stacks.get_mut(src - 1).unwrap();
    let src_len = src_stack.len();
    let mut buffer: Vec<char> = src_stack.split_off(src_len - count);

    assert!(buffer.len() == count);
    println!("buffer {:?}", buffer);


    stacks.get_mut(dst - 1).unwrap().append(&mut buffer);
}

fn dump_stacks(stacks: &Vec<Vec<char>>) -> () {
    stacks.into_iter().enumerate().for_each(|(i, s)| {
        print!("S{}: ", i+1);
        for c in s {
            print!("{} ", c);
        }
        println!();
    });
}

pub fn part1() -> () {
    let mut stacks : Vec<Vec<char>> = Vec::new();
    let mut stacks_str: Vec<&str> = Vec::new(); 
    let mut parsing_status = ParseStatus::StackMap;
    INPUT.split('\n').into_iter().for_each(|line| {

        if parsing_status == ParseStatus::StackMap {
            if line.contains('['){
                stacks_str.push(line)
            } else if line.len() == 0
            {
                parse_stacks(&mut stacks, stacks_str.borrow());
                // println!("Stacks: {:#?}", stacks);
                parsing_status = ParseStatus::SpacksParsed;
                dump_stacks(&stacks);
            }
        } else {
            if let Some(stack_move) = parse_move(line) {
                do_move(&mut stacks, &stack_move);
                println!("Move: {:#?}", line);
                dump_stacks(&stacks);
            }
        }
    });
    for stack in stacks {
        print!("{}", stack.last().unwrap());
    }

}

pub fn part2() -> () {
    let mut stacks : Vec<Vec<char>> = Vec::new();
    let mut stacks_str: Vec<&str> = Vec::new(); 
    let mut parsing_status = ParseStatus::StackMap;
    INPUT.split('\n').into_iter().for_each(|line| {

        if parsing_status == ParseStatus::StackMap {
            if line.contains('['){
                stacks_str.push(line)
            } else if line.len() == 0
            {
                parse_stacks(&mut stacks, stacks_str.borrow());
                // println!("Stacks: {:#?}", stacks);
                parsing_status = ParseStatus::SpacksParsed;
                dump_stacks(&stacks);
            }
        } else {
            if let Some(stack_move) = parse_move(line) {
                do_move_part2(&mut stacks, &stack_move);
                println!("Move: {:#?}", line);
                dump_stacks(&stacks);
            }
        }
    });
    for stack in stacks {
        print!("{}", stack.last().unwrap());
    }
    
}