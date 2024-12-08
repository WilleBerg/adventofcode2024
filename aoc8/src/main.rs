use std::collections::HashMap;

fn main() {
    let input: Vec<String> = std::fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect();
    // println!("{}", run_a(input));
    println!("{}", run_b(input));
}

fn calc_pos_b(first: (i32, i32), second: (i32, i32)) -> Vec<(i32, i32)> {
    let mut res: Vec<(i32, i32)> = vec![];
    let diff1 = (first.0 - second.0, first.1 - second.1);
    let diff2 = (second.0 - first.0, second.1 - first.1);
    let mut f = first.clone();
    let mut s = second.clone();
    let mut counter = 0;
    loop {
        let new1: (i32, i32) = (f.0 + diff1.0, f.1 + diff1.1); 
        let new2: (i32, i32) = (s.0 + diff2.0, s.1 + diff2.1);
        res.push(new1);
        res.push(new2);
        f = new1;
        s = new2;
        counter += 1;
        if counter > 1000 {
            break;
        }
    }

    res
}


fn run_b(input: Vec<String>) -> i32 {
    let mut map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut keys: Vec<char> = vec![];
    let mut extra = 0;
    let mut set: std::collections::HashSet<(i32, i32)> = std::collections::HashSet::new();
    let mut fin: Vec<(i32, i32)> = vec![];
    let mut fixed: Vec<Vec<char>> = input.into_iter()
        .map(|s| {
            s.chars().collect::<Vec<char>>()
        }).collect();
    for (i,row) in fixed.iter().enumerate() {
        for (j,col) in row.iter().enumerate() {
            if *col != '.' {
                if set.insert((i as i32, j as i32)) {
                    fin.push((i as i32, j as i32));
                }
                map.entry(*col).or_insert(vec![]).push((i as i32, j as i32));
                if !keys.contains(col) {
                    keys.push(*col);
                }
            }
        }
    }
    
    let mut counter = 0;
    println!("{:?}", keys);
    for key in keys {
        let ent = map.get(&key).unwrap();
        for (i, pos) in ent.iter().enumerate() {
            for j in i + 1..ent.len() {
                let tmp = calc_pos_b(ent[i], ent[j]);
                for val in tmp {
                    if val.0 >= 0 && val.0 < fixed.len() as i32 {
                        if val.1 >= 0 && val.1 < fixed[0].len() as i32{
                            if !set.contains(&val) {
                                println!("{} {} {}", val.0, val.1, key);
                                println!("{:?}", set);
                                fin.push(val);
                                set.insert(val);
                            }
                        }
                    }
                }  
            }
        }
    } 
    for good_var_name in fin.iter() {
        println!("{:?}", good_var_name);
        if fixed[good_var_name.0 as usize][good_var_name.1 as usize] == '.' || fixed[good_var_name.0 as usize][good_var_name.1 as usize] == '#' {
            fixed[good_var_name.0 as usize][good_var_name.1 as usize] = '#';
        }
        counter += 1;
    }
    for i in 0..fixed.len() {
        for j in 0..fixed[0].len() {
            print!("{}", fixed[i][j]);
        }
        println!();
    }
    counter + extra
}

fn run_a(input: Vec<String>) -> i32 {
    let mut map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut keys: Vec<char> = vec![];
    let mut fixed: Vec<Vec<char>> = input.into_iter()
        .map(|s| {
            s.chars().collect::<Vec<char>>()
        }).collect();
    for (i,row) in fixed.iter().enumerate() {
        for (j,col) in row.iter().enumerate() {
            if *col != '.' {
                map.entry(*col).or_insert(vec![]).push((i as i32, j as i32));
                if !keys.contains(col) {
                    keys.push(*col);
                }
            }
        }
    }
    
    let mut counter = 0;
    let mut fin: Vec<(i32, i32)> = vec![];
    println!("{:?}", keys);
    let mut set: std::collections::HashSet<(i32, i32)> = std::collections::HashSet::new();
    for key in keys {
        let ent = map.get(&key).unwrap();
        for (i, pos) in ent.iter().enumerate() {
            for j in i + 1..ent.len() {
                let tmp = calc_pos(ent[i], ent[j]);
                for val in tmp {
                    if val.0 >= 0 && val.0 < fixed.len() as i32 {
                        if val.1 >= 0 && val.1 < fixed[0].len() as i32{
                            if !set.contains(&val) {
                                println!("{} {} {}", val.0, val.1, key);
                                println!("{:?}", set);
                                fin.push(val);
                                set.insert(val);
                            }
                        }
                    }
                }  
            }
        }
    } 
    for good_var_name in fin.iter() {
        println!("{:?}", good_var_name);
        // if fixed[good_var_name.0 as usize][good_var_name.1 as usize] == '.' || fixed[good_var_name.0 as usize][good_var_name.1 as usize] == '#' {
        fixed[good_var_name.0 as usize][good_var_name.1 as usize] = '#';
        // }
        counter += 1;
    }
    for i in 0..fixed.len() {
        for j in 0..fixed[0].len() {
            print!("{}", fixed[i][j]);
        }
        println!();
    }
    counter 
}

fn calc_pos(first: (i32, i32), second: (i32, i32)) -> Vec<(i32, i32)> {
    let mut res: Vec<(i32, i32)> = vec![];
    let diff1 = (first.0 - second.0, first.1 - second.1);
    let diff2 = (second.0 - first.0, second.1 - first.1);
    let new1: (i32, i32) = (first.0 + diff1.0, first.1 + diff1.1); 
    let new2: (i32, i32) = (second.0 + diff2.0, second.1 + diff2.1);

    res.push(new1);
    res.push(new2);
    res
}









