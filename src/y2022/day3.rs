use std::collections::{HashSet, BTreeSet};

fn get_priority(c: &char) -> i32 {
    let c_int:i32 = *c as i32;
    if c.is_ascii_lowercase() {
        return c_int - 96;
    }
    else if c.is_ascii_uppercase() {
        return c_int - 38;
        
    }
    else {
        panic!();
    }
}

pub fn part1() -> () {
    let input = include_str!("../../inputs/y2022/d3_input.txt");
    let mut dups: Vec<i32> = Vec::new();
    input.split('\n').into_iter().for_each(|line| {
        let (pocket1, pocket2) = line.split_at(line.len() / 2); 
        if pocket1.len() == pocket2.len() {
            println!("R: {:?} = p1: {:?} + p2 {:?}", line, pocket1, pocket2);
            let mut p1_set = BTreeSet::new();
            pocket1.chars().for_each(|c| {p1_set.insert(get_priority(&c));});

            pocket2.chars().find(|c| {
                let p = get_priority(c);

                let found  = match p1_set.get(&p) {
                    Some(_) =>  {
                        dups.push(p);
                        true
                    }
                    None => false
                };
                found
            });
        }
    });
    let total: i32 = dups.iter().sum();
    println!("Part 1 Sum={:?}", total);
}

pub fn part2() -> () {
    let input = include_str!("../../inputs/y2022/d3_input.txt");
    let mut elf1_set: HashSet<i32> = HashSet::new();
    let mut elf2_set: HashSet<i32> = HashSet::new();
    let mut elf3_set: HashSet<i32> = HashSet::new();
    let mut dups: Vec<i32> = Vec::new();
    input.split('\n').into_iter().enumerate().for_each(|(idx ,line)| {
        match idx % 3 {
            0 => {
                elf1_set.clear();
                line.chars().for_each(|c| {elf1_set.insert(get_priority(&c));});
            },
            1 => {
                elf2_set.clear();
                line.chars().for_each(|c| {elf2_set.insert(get_priority(&c));});
            },
            2 => {
                elf3_set.clear();
                line.chars().for_each(|c| {elf3_set.insert(get_priority(&c));});
                
                let mut intersection = elf1_set.intersection(&elf2_set);
                
                //locale the first element in intersection that's present in elf3 set
                if let Some(badge) = intersection.find_map(|e| {
                    if let Some(_) = elf3_set.get(e) { Some(e) }
                    else { None } 
                }) {
                    dups.push(*badge);
                };

            },
            _ => panic!()
        }
    });
    let total: i32 = dups.iter().sum();
    println!("Part 2 Sum={:?}", total);
}