fn main() {
    let input: Vec<String> = std::fs::read_to_string("./input")
        .unwrap().lines().map(|s| s.to_string()).collect();
    // println!("{}", run_a(input));
    println!("{}", run_b(input));
}


fn run_b(input: Vec<String>) -> i64 {
    let values: Vec<(i64, Vec<i64>)> = input.into_iter()
        .map(|l| {
            let splitted: Vec<String> = l.split(":")
                .map(|s| s.to_string())
                .collect();
            let nums: Vec<i64> = splitted[1].split(" ")
                .filter(|&s| s != "")
                .map(|s| s.to_string())
                .map(|n| n.parse::<i64>().unwrap())
                .collect();
            let num: i64 = splitted[0].parse::<i64>().unwrap();
            (num, nums)
        }).collect();
    println!("{:?}", values);
    let mut counter = 0;
    for val in values {
        if is_possible_b(val.clone()) {
            counter += val.0;
        }
    }
    counter
}

fn is_possible_b(nums: (i64, Vec<i64>)) -> bool {
    let mut all: Vec<i64> = vec![*nums.1.first().unwrap()];
    for i in 1..nums.1.len() {
        let mut tmp: Vec<i64> = vec![];
        for num in all.iter() {
            tmp.push(num * nums.1[i]);
            tmp.push(num + nums.1[i]);
            tmp.push(third(*num, nums.1[i]));
        }
        all = tmp;
    }
    for num in all {
        if num == nums.0 {
            return true;
        }
    }
    false
}

fn third(num1: i64, num2: i64) -> i64 {
    let mut s = num1.to_string();
    num2.to_string().chars().for_each(|ss| s.push(ss));
    s.parse::<i64>().unwrap()
}

fn run_a(input: Vec<String>) -> i64 {
    let values: Vec<(i64, Vec<i64>)> = input.into_iter()
        .map(|l| {
            let splitted: Vec<String> = l.split(":")
                .map(|s| s.to_string())
                .collect();
            let nums: Vec<i64> = splitted[1].split(" ")
                .filter(|&s| s != "")
                .map(|s| s.to_string())
                .map(|n| n.parse::<i64>().unwrap())
                .collect();
            let num: i64 = splitted[0].parse::<i64>().unwrap();
            (num, nums)
        }).collect();
    println!("{:?}", values);
    let mut counter = 0;
    for val in values {
        if is_possible(val.clone()) {
            counter += val.0;
        }
    }
    counter
}

fn is_possible(nums: (i64, Vec<i64>)) -> bool {
    let mut all: Vec<i64> = vec![*nums.1.first().unwrap()];
    for i in 1..nums.1.len() {
        let mut tmp: Vec<i64> = vec![];
        for num in all.iter() {
            tmp.push(num * nums.1[i]);
            tmp.push(num + nums.1[i]);
        }
        all = tmp;
    }
    for num in all {
        if num == nums.0 {
            return true;
        }
    }
    false
}

