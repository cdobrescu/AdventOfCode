
const INPUT: &str = include_str!("../../inputs/y2022/d4_input.txt");
pub fn part1() -> () {
    let mut total: u32 = 0;
    INPUT.split('\n').into_iter().for_each(|line| {
        let pair_vec = line.split(",").collect::<Vec<&str>>();
        if pair_vec.len() == 2 {
            let (p1_str, p2_str) = (pair_vec[0], pair_vec[1]);
            let p1 = match p1_str.split("-").collect::<Vec<&str>>()[..] {
                [a, b] => (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()),
                _ => { panic!(); }
            };
            let p2 = match p2_str.split("-").collect::<Vec<&str>>()[..] {
                [a, b] => (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()),
                _ => { panic!(); }
            };
            if p1.0 <= p2.0 && p1.1 >= p2.1 {
                println!("P1={:?}, P2={:?}", p1, p2);
                total += 1;
            } 
            else if p2.0 <= p1.0 && p2.1 >= p1.1 {
                println!("P1={:?}, P2={:?}", p1, p2);
                total += 1;
            }
        } 
    });
    // let total: i32 = dups.iter().sum();
    println!("Part 1 total={:?}", total);
}

pub fn part2() -> () {
    let mut total: u32 = 0;
    INPUT.split('\n').into_iter().for_each(|line| {
        let pair_vec = line.split(",").collect::<Vec<&str>>();
        if pair_vec.len() == 2 {
            let (p1_str, p2_str) = (pair_vec[0], pair_vec[1]);
            let p1 = match p1_str.split("-").collect::<Vec<&str>>()[..] {
                [a, b] => (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()),
                _ => { panic!(); }
            };
            let p2 = match p2_str.split("-").collect::<Vec<&str>>()[..] {
                [a, b] => (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()),
                _ => { panic!(); }
            };
            if p1.0 <= p2.0 && p1.1 >= p2.0 {
                println!("P1={:?}, P2={:?}", p1, p2);
                total += 1;
            } else if p2.0 <= p1.0 && p2.1 >= p1.0 {
                println!("P1={:?}, P2={:?}", p1, p2);
                total += 1;
            } 

        }
    });
    // let total: i32 = dups.iter().sum();
    println!("Part 1 total={:?}", total);
}