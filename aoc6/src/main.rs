fn main() {
    let input: Vec<String> = std::fs::read_to_string("./input2")
        .unwrap().lines().map(String::from).collect();
    // println!("{}", run_a(input));
    println!("{}", run_b(input));

}

fn run_b(input: Vec<String>) -> i32 {
    let finput: Vec<Vec<char>> = input.into_iter()
        .map(|s| {
            s.chars().collect::<Vec<char>>()
        }).collect();
    
    let mut start_pos: (usize, usize) = (0, 0);
    for i in 0..finput.len() {
        for j in 0..finput[i].len() {
            if finput[i][j] == '^' {
                start_pos = (i, j);
            }
        }
    }
    let mut counter = 0;
    for i in 0..finput.len() {
        for j in 0..finput[i].len() {
            if finput[i][j] != '#' {
                let mut tmp = finput.clone();
                tmp[i][j] = '#';
                if run_map(tmp, &start_pos) {
                    counter += 1;
                }
            }
        }
    }
    counter
}

fn run_map(finput: Vec<Vec<char>>, start_pos: &(usize, usize)) -> bool {
    let mut pos: (i32, i32) = (start_pos.0 as i32, start_pos.1 as i32);
    let mut offset: (i32, i32) = (-1, 0);
    let map: std::collections::HashMap<(i32, i32), (i32, i32)> = std::collections::HashMap::from([
        ((-1, 0), (0, 1)),
        ((0, 1), (1, 0)),
        ((1,0), (0, -1)),
        ((0, -1), (-1, 0)),
    ]);
    let mut printer: std::collections::HashSet<((i32, i32), (i32, i32))> = std::collections::HashSet::new();
    loop {
        if printer.contains(&(pos, offset)) {
            return true; 
        }
        printer.insert((pos.clone(), offset)); 
        let tmppos = (pos.0 + offset.0, pos.1 + offset.1);
        if tmppos.0 < 0 || tmppos.0 >= finput.len() as i32 {
            return false;
        }
        if tmppos.1 < 0 || tmppos.1 >= finput[0].len() as i32 {
            return false;
        }
        if finput[tmppos.0 as usize][tmppos.1 as usize] == '#' {
            offset = *map.get(&offset).unwrap();
        } else {
            pos = tmppos;
        }
    }

}

fn run_a(input: Vec<String>) -> i32 {
    let finput: Vec<Vec<char>> = input.into_iter()
        .map(|s| {
            s.chars().collect::<Vec<char>>()
        }).collect();
    
    let mut start_pos: (usize, usize) = (0, 0);
    for i in 0..finput.len() {
        for j in 0..finput[i].len() {
            if finput[i][j] == '^' {
                start_pos = (i, j);
            }
        }
    }
    let mut counter = 0;
    let mut pos: (i32, i32) = (start_pos.0 as i32, start_pos.1 as i32);
    let mut offset: (i32, i32) = (-1, 0);
    let map: std::collections::HashMap<(i32, i32), (i32, i32)> = std::collections::HashMap::from([
        ((-1, 0), (0, 1)),
        ((0, 1), (1, 0)),
        ((1,0), (0, -1)),
        ((0, -1), (-1, 0)),
    ]);
    let mut printer: std::collections::HashSet<(i32, i32)> = std::collections::HashSet::new();
    loop {
        println!("{} : {}", pos.0, pos.1);
        printer.insert(pos.clone()); 
        let tmppos = (pos.0 + offset.0, pos.1 + offset.1);
        if tmppos.0 < 0 || tmppos.0 >= finput.len() as i32 {
            break;
        }
        if tmppos.1 < 0 || tmppos.1 >= finput[0].len() as i32 {
            break;
        }
        if finput[tmppos.0 as usize][tmppos.1 as usize] == '#' {
            offset = *map.get(&offset).unwrap();
        } else {
            pos = tmppos;
            // counter += 1;
        }
    }
    
    for i in 0..finput.len() {
        for j in 0..finput[i].len() {
            if printer.contains(&(i as i32, j as i32))  {
                print!("X");
                counter += 1;
            } else {
                print!("{}", finput[i][j]);
            }
        }
        println!();
    }
    counter
}


