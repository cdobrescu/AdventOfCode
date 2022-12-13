use std::{collections::HashSet};

const INPUT: &str = include_str!("../../inputs/y2022/d9_input.txt");
// const INPUT: &str = include_str!("../../inputs/y2022/d9_test.txt");
// const INPUT: &str = include_str!("../../inputs/y2022/d9_test2.txt");
const ROPE_LENGTH: usize = 10;
fn dump_rope(rope: &[(i32, i32); ROPE_LENGTH]) -> String {
    let mut rope_str = "".to_string();

    for i in 0..ROPE_LENGTH {
        let knot = rope.get(i).unwrap();
        rope_str = format!("{} {}:({},{})", rope_str, i, knot.0, knot.1);
        
    }
    rope_str
}
pub fn part1() -> String {
    let mut moves: Vec<(char, usize)> = Vec::new();
    let lines = INPUT.split('\n').into_iter();
    for l in lines {
        match l.split(" ").into_iter().collect::<Vec<&str>>()[..]{
            [direction, count] => {
                moves.push((
                    direction.chars().nth(0).unwrap().to_owned(), 
                    count.parse::<usize>().unwrap()
                ));
            },
            _ => {panic!()}

        }
    }
    let mut trail: Vec<(i32, i32)> = Vec::new();

    let mut rope: [(i32, i32); ROPE_LENGTH] = [(0, 0);ROPE_LENGTH];
    trail.push((0,0));

    let mut x_range = (0, 0);
    let mut y_range = (0, 0);

    let mut move_counter = 0;

    for (direction, count) in moves {
        for _ in 0..count {
            move_counter += 1;
            // move the head
            {
                let mut leader = rope.get_mut(0).unwrap();
                let prev_pos = leader.clone();
                match direction {
                    'R' => {
                        leader.0 += 1;
                    },
                    'L' => {
                        leader.0 -= 1;
                    }
                    'U' => {
                        leader.1 += 1;
                    },
                    'D' => {
                        leader.1 -= 1;
                    },
                    _ => panic!()
                }
                if leader.0 > x_range.1 { x_range.1 = leader.0;}
                if leader.0 < x_range.0 { x_range.0 = leader.0;}
                if leader.1 > y_range.1 { y_range.1 = leader.1;}
                if leader.1 < y_range.0 { y_range.0 = leader.1;}
                // println!("Leader {}: {:?} -> {:?}",direction, prev_pos, leader)
            }
            
            let prev_tail = rope.last().unwrap().clone();
            
            for i in 1..ROPE_LENGTH {
                
                let leader = rope.get(i-1).unwrap().clone();
                let mut follower = rope.get_mut(i).unwrap();
                let prev_pos = follower.clone();
                
                let delta_x = leader.0 - follower.0;
                let delta_y = leader.1 - follower.1;
                
                // if i == ROPE_LENGTH - 1 {
                //     println!("Leader={:?}, Follower={:?}  delta_x={},delta_y={}",leader, follower, delta_x, delta_y);
                // }
                if (delta_x.abs() + delta_y.abs()) == 2 {
                    if delta_x == 2 {
                        follower.0 += 1;
                    }
                    else if delta_x == -2 {
                        follower.0 -= 1;
                    }
                    else if delta_y == 2 {
                        follower.1 += 1;
                    }
                    else if delta_y == -2 {
                        follower.1 -= 1;
                    }
                } else if (delta_x.abs() + delta_y.abs()) == 3 {
                    if delta_x == 2 {
                        follower.0 += 1;
                        follower.1 += delta_y;
                    }
                    else if delta_x == -2 {
                        follower.0 -= 1;
                        follower.1 += delta_y;
                    }
                    else if delta_y == 2 {
                        follower.1 += 1;
                        follower.0 += delta_x;
                    }
                    else if delta_y == -2 {
                        follower.1 -= 1;
                        follower.0 += delta_x;
                    }
                } else if (delta_x.abs() + delta_y.abs()) == 4 {
                    if delta_x == 2 {
                        follower.0 += 1;
                    } else {
                        follower.0 -= 1;
                    }
                    if delta_y == 2 {
                        follower.1 += 1;
                    } else {
                        follower.1 -= 1;
                    }
                }
                // if i == ROPE_LENGTH - 1 {
                //     println!("Follower {}: {:?} -> {:?}",direction, prev_pos, follower);
                // }
                if (prev_pos.0 - follower.0).abs() > 1 || (prev_pos.1 - follower.1).abs() > 1 {
                    println!("Follower {}: {:?} -> {:?}",direction, prev_pos, follower);
                    println!("Move{} {} rope={} ",move_counter, direction, dump_rope(&rope));
                    println!("i={}", i);
                    panic!();
                }
            }

            let current_tail = rope.last().unwrap().clone();
            if prev_tail != current_tail {
                trail.push(current_tail);
            }
            if (prev_tail.0 - current_tail.0).abs() > 1 ||(prev_tail.1 - current_tail.1).abs() > 1 {
                panic!();
            }
            println!("Move{} {} rope={} ",move_counter, direction, dump_rope(&rope));
        }

    }
    
    println!("Total Range X={}", x_range.1 - x_range.0 + 1);
    println!("Total Range y={}", y_range.1 - y_range.0 + 1);
    
    println!("Trail= {:?}", trail);
    let trail_set: HashSet<(i32, i32)> = trail.into_iter().collect();
    format!("Trailset length={}", trail_set.len())
}

pub fn part2() -> String {
    let mut moves: Vec<(char, usize)> = Vec::new();
    let lines = INPUT.split('\n').into_iter();
    for l in lines {
        match l.split(" ").into_iter().collect::<Vec<&str>>()[..]{
            [direction, count] => {
                moves.push((
                    direction.chars().nth(0).unwrap().to_owned(), 
                    count.parse::<usize>().unwrap()
                ));
            },
            _ => {panic!()}

        }
    }
    format!("TODO!")
}