fn main() {
    let input: Vec<String> = std::fs::read_to_string("./2input")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    // println!("{}", run_a(input))
    println!("{}", run_b(input))
}

fn run_b(input: Vec<String>) -> i32 {
    let chars: Vec<Vec<char>> = input.into_iter()
        .map(|s| {
            let r: Vec<char> = s.chars().collect();
            r
        }).collect();
    let mut count: i32 = 0;
    for (i, c_vec) in chars.iter().enumerate() {
        for (j, c) in c_vec.iter().enumerate() {
            if *c == 'A' {
                let m: Vec<char> = vec!['A', 'M'];
                let s: Vec<char> = vec!['A', 'S'];
                // S S
                //  A
                // M M
                if check_next(&chars, i, j, (1, 1), &m) {
                    if check_next(&chars, i, j, (-1, 1), &m) {
                        if check_next(&chars, i, j, (-1, -1), &s) {
                            if check_next(&chars, i, j, (1, -1), &s) {
                                count += 1;        
                            }
                        }
                    }
                }
                // S M
                //  A
                // S M
                if check_next(&chars, i, j, (1, 1), &m) {
                    if check_next(&chars, i, j, (-1, 1), &s) {
                        if check_next(&chars, i, j, (-1, -1), &s) {
                            if check_next(&chars, i, j, (1, -1), &m) {
                                count += 1;        
                            }
                        }
                    }
                }
                // M M
                //  A
                // S S
                if check_next(&chars, i, j, (1, 1), &s) {
                    if check_next(&chars, i, j, (-1, 1), &s) {
                        if check_next(&chars, i, j, (-1, -1), &m) {
                            if check_next(&chars, i, j, (1, -1), &m) {
                                count += 1;        
                            }
                        }
                    }
                }
                // M S
                //  A
                // M S
                if check_next(&chars, i, j, (1, 1), &s) {
                    if check_next(&chars, i, j, (-1, 1), &m) {
                        if check_next(&chars, i, j, (-1, -1), &m) {
                            if check_next(&chars, i, j, (1, -1), &s) {
                                count += 1;        
                            }
                        }
                    }
                }
            }
        }
    }
    
    count
}

fn run_a(input: Vec<String>) -> i32 {
    let chars: Vec<Vec<char>> = input.into_iter()
        .map(|s| {
            let r: Vec<char> = s.chars().collect();
            r
        }).collect();
    let mut count: i32 = 0;
    for (i, c_vec) in chars.iter().enumerate() {
        for (j, c) in c_vec.iter().enumerate() {
            if *c == 'X' {
                let xmas: Vec<char> = vec!['X', 'M', 'A', 'S'];
                if check_next(&chars, i, j, (0, -1), &xmas) {count+=1;}
                if check_next(&chars, i, j, (0, 1), &xmas) {count+=1;}
                if check_next(&chars, i, j, (1, 0), &xmas) {count+=1;}
                if check_next(&chars, i, j, (-1, 0), &xmas) {count+=1;}
                if check_next(&chars, i, j, (1, 1), &xmas) {count+=1;}
                if check_next(&chars, i, j, (1, -1), &xmas) {count+=1;}
                if check_next(&chars, i, j, (-1, 1), &xmas) {count+=1;}
                if check_next(&chars, i, j, (-1, -1), &xmas) {count+=1;}
            }
        }
    }
    
    count
}

fn check_next(chars: &Vec<Vec<char>>, i: usize, j: usize, offset: (i32, i32), xmas: &Vec<char>) -> bool {
    // let xmas: Vec<char> = vec!['X', 'M', 'A', 'S'];
    let mut counter = 0;
    while counter < xmas.len() as i32 {
        println!("i: {}", (i as i32 + offset.0 * counter) as usize);
        println!("j: {}", (j as i32 + offset.1 * counter) as usize);
        if (i as i32) + offset.0 * counter >= chars.len() as i32 || (i as i32) + offset.0 * counter < 0 {
            println!("HELLO1");
            return false;
        }
        if (j as i32) + offset.1 * counter >= chars[i].len() as i32 || (j as i32) + offset.1 * counter < 0 {
            println!("HELLO2");
            return false;
        }
        println!("THIS IS IT: {}", chars[(i as i32 + offset.0 * counter) as usize][(j as i32 + offset.1 * counter) as usize]);
        if chars[(i as i32 + offset.0 * counter) as usize][(j as i32 + offset.1 * counter) as usize] == xmas[counter as usize] {
            println!("HELLO");
            counter += 1;  
        } else {
            println!("HELLO3");
            return false;
        }
    }
    true 
}
