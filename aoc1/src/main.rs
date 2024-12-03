use std::collections::HashMap;

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
    let mut result: i32 = 0;
    
    let mut split_input: (Vec<i32>, Vec<i32>) = input.into_iter()
        .map(|item| {
            let splits: Vec<String> = item.split("   ")
                .map(|s| s.to_string())
                .collect();
            let left = splits[0].clone().parse::<i32>().unwrap();
            let right = splits[1].clone().parse::<i32>().unwrap();

            (left, right)
        }).collect();
    split_input.0.sort();
    split_input.1.sort();
    for i in 0..split_input.0.len() {
        result += (split_input.0[i] - split_input.1[i]).abs()
    }
    return result
}

fn run_b(input: Vec<String>) -> i32 {
    let mut result: i32 = 0;
    
    let mut split_input: (Vec<i32>, Vec<i32>) = input.into_iter()
        .map(|item| {
            let splits: Vec<String> = item.split("   ")
                .map(|s| s.to_string())
                .collect();
            let left = splits[0].clone().parse::<i32>().unwrap();
            let right = splits[1].clone().parse::<i32>().unwrap();

            (left, right)
        }).collect();
    let mut number_of_times: HashMap<i32, i32> = HashMap::new();
    for num in split_input.1 {
        *number_of_times.entry(num).or_insert(0) += 1;
    }
    for num in split_input.0 {
        if let Some(value) = number_of_times.get(&num) {
            result += value * num
        }
    }
    return result
}
