fn main() {
    let input: Vec<String> = std::fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    // println!("{}", run_a(input))
    println!("{}", run_b(input))
}

fn run_a(input: Vec<String>) -> i32 {
    let fixed_input: Vec<Vec<i32>> = input.iter().map(|item| {
        let items: Vec<i32> = item.split(" ").map(|num| {
            num.parse::<i32>().unwrap()
        }).collect();
        items
    }).collect();
    let mut counter = 0;
    for item in fixed_input {
        let is_inc;
        let mut un_safe = false;
        if item[0] > item[1] {
            is_inc = false;
        } else if item[0] == item[1] {
            continue;
        } else {
            is_inc = true;
        }
        for (i, num) in item.iter().enumerate() {
            print!("{} ", num);
            if i == item.len() - 1 {
                break;
            }
            let diff = num - &item[i + 1];
            if diff.abs() > 3 {
                un_safe = true;
                break;
            }
            if is_inc {
                if num < &item[i + 1] {
                    continue;
                } else {
                    un_safe = true;
                    break;
                }
            } else {
                if num > &item[i + 1] {
                    continue;
                } else {
                    un_safe = true;
                    break;
                }
            }
        }
        if !un_safe {
            println!("Was safe");
            counter += 1;
        } else {
            println!("Was unsafe");
        }
    }
    counter
}

fn run_b(input: Vec<String>) -> i32 {
    let fixed_input: Vec<Vec<i32>> = input.iter().map(|item| {
        let items: Vec<i32> = item.split(" ").map(|num| {
            num.parse::<i32>().unwrap()
        }).collect();
        items
    }).collect();
    let mut counter = 0;
    for item in fixed_input {
        if test_vec(item.clone()) {
            for i in 0..item.len() {
                if !test_vec(create_vec(item.clone(), i as i32)) {
                    counter += 1;
                    break;
                }
            }
        } else {
            counter += 1
        }
    }
    return counter;
}

fn create_vec(item: Vec<i32>, skip_index: i32) -> Vec<i32> {
    let mut res = vec![];
    for (i, num) in item.iter().enumerate() {
        if i as i32 != skip_index {
            res.push(num.clone());
        }
    } 
    return res
}

fn test_vec(item: Vec<i32>) -> bool {
    let is_inc;
    let mut un_safe = false;
    if item[0] > item[1] {
        is_inc = false;
    } else if item[0] == item[1] {
        return true;
    } else {
        is_inc = true;
    }
    for (i, num) in item.iter().enumerate() {
        print!("{} ", num);
        if i == item.len() - 1 {
            break;
        }
        let diff = num - &item[i + 1];
        if diff.abs() > 3 {
            un_safe = true;
            break;
        }
        if is_inc {
            if num < &item[i + 1] {
                continue;
            } else {
                un_safe = true;
                break;
            }
        } else {
            if num > &item[i + 1] {
                continue;
            } else {
                un_safe = true;
                break;
            }
        }
    }
    if !un_safe {
        println!("Was safe");
        return false
    }
    println!("Was unsafe");
    return true 
}
