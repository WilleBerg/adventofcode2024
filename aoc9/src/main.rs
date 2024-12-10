use std::collections::{HashMap, HashSet};

fn main() {
    let input: Vec<String> = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect();
    // println!("{}", run_a(input));
    println!("{}", run_b(input));
}

fn run_a(input: Vec<String>) -> i64 {
    let rinput = input[0].clone();
    let mut rstring: Vec<String> = vec![];
    let mut counter = 0;
    for (i,c) in rinput.chars().enumerate() {
        if i % 2 == 0 {
            for _ in 0..c.to_string().parse::<i32>().unwrap() {
                rstring.push(counter.to_string());
            }
            counter += 1;
        } else {
            for _ in 0..c.to_string().parse::<i32>().unwrap() {
                rstring.push('.'.to_string());
            }
        } 
    }
    println!("{:?}", rstring);

    let mut stack = rstring.clone();
    let mut map: HashMap<String, i32> = HashMap::new();
    stack = stack.into_iter().filter(|c| c != ".").collect();

    stack.iter().for_each(|s| {
        *map.entry(s.clone()).or_insert(0) += 1;
    });

    let mut finstring: Vec<String> = vec![];
    // let mut moved: HashSet<String> = HashSet::new();
    let mut tot_moved: usize = 0;
    
    for (_, c) in rstring.iter().enumerate() {
        if *c == '.'.to_string() {
            if let Some(val) = stack.pop() {
                let tmp: i32 = *map.get(&val).unwrap();
                if tmp > 0 {
                    finstring.push(val.clone());
                    *map.entry(val).or_insert(0) -= 1;
                    tot_moved += 1;
                } else {
                    break;
                }
            }
        } else {
            finstring.push(c.clone());
        }
    }
    for i in finstring.len() - tot_moved..finstring.len() {
        finstring[i] = ".".to_string();
    }
    let mut checksum: i64 = 0;
    for (i, c) in finstring.into_iter().enumerate() {
        print!("{}", c);
        if c == "." {
            break;
        }
        checksum += c.parse::<i64>().unwrap() * i as i64;
    }
    println!();
    // println!("{:?}", finstring);
    checksum 
}

fn run_b(input: Vec<String>) -> i64 {
    let rinput = input[0].clone();
    let mut rstring: Vec<String> = vec![];
    let mut counter = 0;
    let mut numindex: HashMap<String, Vec<usize>> = HashMap::new(); 
    let mut indexcounter = 0;
    for (i,c) in rinput.chars().enumerate() {
        if i % 2 == 0 {
            for _ in 0..c.to_string().parse::<i32>().unwrap() {
                rstring.push(counter.to_string());
                numindex.entry(counter.to_string()).or_insert(vec![]).push(indexcounter);
                indexcounter += 1;
            }
            counter += 1;
        } else {
            for _ in 0..c.to_string().parse::<i32>().unwrap() {
                rstring.push('.'.to_string());
                indexcounter += 1;
            }
        } 
    }
    println!("{:?}", rstring);
    println!("{:?}", numindex);

    let mut stack = rstring.clone();
    let mut map: HashMap<String, i32> = HashMap::new();
    let mut c: i32 = 0;
    let mut free_block: Vec<i32> = vec![];
    let mut s: HashSet<String> = HashSet::new();
    let mut unique_list: Vec<String> = vec![];
    let mut dot_to_num: Vec<usize> = vec![];
    for (i, c) in stack.iter().enumerate() {
        if *c == '.'.to_string() {
            dot_to_num.push(i);
        }
    }
    println!("{:?}", dot_to_num);
    stack.iter().for_each(|item| {
        if *item != '.'.to_string() {
            if !s.contains(item) {
                unique_list.push(item.clone());
            }
            s.insert(item.clone());
            if c != 0 {
                free_block.push(c);
                c = 0;
            }
        } else {
            c+=1;
        }
    });
    stack = stack.into_iter().filter(|c| c != ".").collect();
    println!("{:?}", free_block);
    stack.iter().for_each(|s| {
        *map.entry(s.clone()).or_insert(0) += 1;
    });
    let mut finstring: Vec<String>;
    let mut v: Vec<(i32, i32, i32)> = vec![];

    let mut block_to_indices: HashMap<i32, Vec<usize>> = HashMap::new();
    dot_to_num.reverse();
    for (i, block) in free_block.iter().enumerate() {
        for _ in 0..*block {
            block_to_indices.entry(i as i32).or_insert(vec![]).push(dot_to_num.pop().unwrap());
        } 
    }
    println!("block to indeciesasjdaks: {:?}", block_to_indices);
    unique_list.reverse();
    for val in unique_list.iter() {
        for (i, block) in free_block.iter().enumerate() {
            if *map.get(val).unwrap() <= *block && block_to_indices.get(&(i as i32)).unwrap().first().unwrap() < numindex.get(val).unwrap().first().unwrap(){
                // println!("less than {} {}", block_to_indices.get(&(i as i32)).unwrap().first().unwrap() , numindex.get(val).unwrap().first().unwrap());
                v.push((i as i32, val.parse().unwrap(), *map.get(val).unwrap()));
                free_block[i] -= *map.get(val).unwrap();
                // println!("Will put {} in block {}", val, i);
                break;
            }
        }
    }
    println!("Changes: {:?}", v);
    finstring = rstring.clone();
    let mut moved: Vec<String> = vec![];
    for change in v.into_iter() {
        moved.push(change.1.to_string());
        let indexs = block_to_indices.get_mut(&change.0).unwrap();
        for _ in 0..change.2 {
            let index = indexs.remove(0);
            finstring[index] = change.1.to_string();
        }
    }
    let mut moved_indexes: Vec<usize> = vec![];
    for m in moved.iter() {
        for (i, c) in rstring.iter().enumerate() {
            if c == m {
                moved_indexes.push(i);
            } 
        }
    }
    for i in moved_indexes.into_iter() {
        finstring[i] = ".".to_string();
    }
    let mut checksum: i64 = 0;
    for (i, c) in finstring.into_iter().enumerate() {
        print!("{}", c);
        if c != '.'.to_string() {
            checksum += c.parse::<i64>().unwrap() * i as i64;
        }
    }
    println!();
    checksum 
}
