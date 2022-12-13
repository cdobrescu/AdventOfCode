use std::{usize, collections::HashMap};

const INPUT: &str = include_str!("../../inputs/y2022/d8_input.txt");
// const INPUT: &str = include_str!("../../inputs/y2022/d8_test.txt");

pub fn part1() -> String {
    let mut forest: Vec<Vec<usize>> = Vec::new();
    let lines = INPUT.split('\n').into_iter();
    for l in lines {
        let mut row : Vec<usize> = Vec::new();
        for t in l.chars(){
            row.push(t.to_digit(10).unwrap() as usize);
        }
        forest.push(row);
    }
    
    let max_row = forest.len() -1 ;
    let max_col = forest.get(0).unwrap().len() -1 ;
    
    // initialize visibility map. All treez are visible from all sides
    let mut v_map: HashMap<(usize, usize), (bool, bool, bool, bool)> =  HashMap::new();
    for i in 0..forest.len() {
        for j in 0..forest.get(0).unwrap().len(){
            v_map.insert((i,j), (true, true , true, true));
        }
    }

    for i in 0..=max_row {
        let mut max:usize = 0;
        for j in 0..=max_col {
            let mut v_item = v_map.get_mut(&(i, j)).unwrap(); 
            let height = forest.get(i).unwrap().get(j).unwrap();
            if j == 0 || j == max_col || max < *height {
                max = *height;
                continue;
            }
            else if max >= *height {
                    v_item.0 = false;
            }
        }
    }
    
    //rotate forest 90 degreee clockwise
    for j in 0..=max_col {
        let mut max:usize = 0;
        for i in (0..=max_row).rev() {
            let mut v_item = v_map.get_mut(&(i, j)).unwrap(); 
            let height = forest.get(i).unwrap().get(j).unwrap();
            if j == 0 || j == max_col || max < *height {
                max = *height;
                continue;
            }
            else if max >= *height {
                    v_item.1 = false;
            }
        }
    }

    //rotate forest 90 degreee clockwise
    for i in (0..=max_row).rev() {
        let mut max:usize = 0;
        for j in (0..=max_col).rev() {
            let mut v_item = v_map.get_mut(&(i, j)).unwrap(); 
            let height = forest.get(i).unwrap().get(j).unwrap();
            if j == 0 || j == max_col || max < *height {
                max = *height;
                continue;
            }
            else if max >= *height {
                    v_item.2 = false;
            }
        }
    }

    //rotate forest 90 degreee clockwise
    for j in (0..=max_col).rev() {
        let mut max:usize = 0;
        for i in 0..=max_row {
            let mut v_item = v_map.get_mut(&(i, j)).unwrap(); 
            let height = forest.get(i).unwrap().get(j).unwrap();
            if j == 0 || j == max_col || i == 0 || i == max_row || max < *height {
                max = *height;
                continue;
            }
            else if max >= *height {
                    v_item.3 = false;
            }
        }
    }
    

    let mut visible_count = 0;
    for i in 0..forest.len() {
        for j in 0..forest.get(0).unwrap().len(){
            let v_item = v_map.get(&(i, j)).unwrap();
            if v_item.0 || v_item.1 || v_item.2 || v_item.3 {
                visible_count += 1;
            }
            else {
                println!("Tree [{},{}] invisible", i , j);
            }
        }
    }
    // for i in 0..forest.len() {
    //     println!();
    //     for j in 0..forest.get(0).unwrap().len(){
    //         print!("{} {:?} ",  forest.get(i).unwrap().get(j).unwrap(), v_map.get(&(i,j)).unwrap());
    //     }
    // }

    format!("Visible trees = {}", visible_count)

}

pub fn part2() -> String {
    let mut forest: Vec<Vec<usize>> = Vec::new();
    let lines = INPUT.split('\n').into_iter();
    for l in lines {
        let mut row : Vec<usize> = Vec::new();
        for t in l.chars(){
            row.push(t.to_digit(10).unwrap() as usize);
        }
        forest.push(row);
    }
    let max_row = forest.len() -1 ;
    let max_col = forest.get(0).unwrap().len() -1 ;

    // initialize visibility map. All treez are visible from all sides
    let mut range_map: HashMap<(usize, usize), (usize, usize, usize, usize)> =  HashMap::new();
    for i in 0..forest.len() {
        for j in 0..forest.get(0).unwrap().len(){
            if i == 0 || i == max_row || j == 0 || j == max_col {
                range_map.insert((i,j), (0, 0, 0, 0));
            }
            else {
                range_map.insert((i,j), (1, 1, 1, 1));
            }
        }
    }

    for i in 0..=max_row {
        for j in 0..=max_col {
            if i == 0 || i == max_row || j == 0 || j == max_col {
                continue
            }

            let mut range_item = range_map.get_mut(&(i, j)).unwrap();
            let height = forest.get(i).unwrap().get(j).unwrap();
            //measure left
            for left in (0..j).rev() {
                let neighbour_height = forest.get(i).unwrap().get(left).unwrap();
                if height > neighbour_height && range_item.0 < (j - 0) {
                    range_item.0 += 1;
                }
                else {
                    break
                }
            }
            //measure right
            for right in ( j + 1 )..=max_col {
                let neighbour_height = forest.get(i).unwrap().get(right).unwrap();
                if height > neighbour_height && range_item.1 < (max_col - j) {
                    range_item.1 += 1;
                } 
                else {
                    break
                }
            }
            //measure top
            for top in (0..i).rev() {
                let neighbour_height = forest.get(top).unwrap().get(j).unwrap();
                if height > neighbour_height && range_item.2 < (i - 0) {
                    range_item.2 += 1;
                } 
                else {
                    break
                }
            }
            //measure bottom
            for bottom in ( i + 1 )..=max_row {
                let neighbour_height = forest.get(bottom).unwrap().get(j).unwrap();
                if height > neighbour_height && range_item.3 < (max_row - i)  {
                    range_item.3 += 1;
                } 
                else {
                    break
                }
            }
        }
    }
    let mut best_tree = 0;
    for i in 0..forest.len() {
        println!();
        for j in 0..forest.get(0).unwrap().len(){
            let vis = range_map.get(&(i,j)).unwrap();
            let vis_result = vis.0 * vis.1 * vis.2 * vis.3;
            if vis_result > best_tree {
                best_tree = vis_result;
            }
            print!("{} {:?}={} ",  forest.get(i).unwrap().get(j).unwrap(), vis, vis_result);
        }
    }
    println!();
    
    format!("Best tree = {}", best_tree)
}