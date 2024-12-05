use std::collections::{HashMap, HashSet};

fn main() {
    let input: Vec<String> = std::fs::read_to_string("./input")
        .unwrap().lines().map(String::from).collect();
    println!("{}", run_b(input));
}

fn run_a(input: Vec<String>) -> i32 {
    let mut inp1: Vec<String> = vec![];
    let mut inp2: Vec<String> = vec![];
    let mut switch = false; 
    for line in input {
        if line == "" {
            switch = true; 
            continue;
        }
        if switch {
            inp2.push(line);
        } else {
            inp1.push(line);
        }
    }
    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
    inp1.into_iter().for_each(|item| {
        let splits: Vec<String> = item.split("|").map(|s| s.to_string()).collect();
        map.entry(splits[0].parse().unwrap()).or_default().push(splits[1].parse().unwrap());
    });
    println!("{:?}", map);

    let mut counter: i32 = 0;
    let mut good_ones: Vec<String> = vec![];
    for ele in inp2.into_iter() {
        let nums: Vec<String> = ele.split(",").map(|s| s.to_string()).collect();
        let mut set: HashSet<i32> = HashSet::new();
        let mut good = true;
        println!("{:?}", nums);
        for ele_num in nums {
            let parsed: i32 = ele_num.parse().unwrap();
            if let Some(val) = map.get(&parsed) {
                for num in val {
                    if set.contains(num) {
                        good = false;
                        println!("bAD: {:?}", set);
                        break;
                    }
                }
            }
            if !good {break;}
            set.insert(parsed);
        }
        if good {
            good_ones.push(ele.clone());
        }
        // if set.contains(&parsed) {
        //     
        // }
    } 
    println!("{:?}", good_ones);
    for num in good_ones {
        counter += get_middle(num.split(",").map(|s| s.to_string()).collect());
    }
    counter 
}

fn get_middle(middle: Vec<String>) -> i32 {
    middle[middle.len() / 2].parse::<i32>().unwrap()

}

fn run_b(input: Vec<String>) -> i32 {
    let mut inp1: Vec<String> = vec![];
    let mut inp2: Vec<String> = vec![];
    let mut switch = false; 
    for line in input {
        if line == "" {
            switch = true; 
            continue;
        }
        if switch {
            inp2.push(line);
        } else {
            inp1.push(line);
        }
    }
    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
    inp1.into_iter().for_each(|item| {
        let splits: Vec<String> = item.split("|").map(|s| s.to_string()).collect();
        map.entry(splits[0].parse().unwrap()).or_default().push(splits[1].parse().unwrap());
    });
    println!("{:?}", map);

    let mut counter: i32 = 0;
    let mut bad_ones: Vec<String> = vec![];
    for ele in inp2.into_iter() {
        if !is_good(&ele, &map).0 {
            bad_ones.push(ele.clone());
        }
    } 
    for bad in bad_ones.iter_mut() {
        loop {
            let info = is_good(&bad, &map);
            if info.0 {break;}
            let mut nums_strings: Vec<String> = bad.split(",").map(|s| s.to_string()).collect();
            let mut fi = 0;
            let mut si = 0;
            for (i, num) in nums_strings.iter().enumerate() {
                if num.parse::<i32>().unwrap() == info.1 {
                    fi = i;
                } else if num.parse::<i32>().unwrap() == info.2 {
                    si = i;
                }
            }
            nums_strings.swap(fi, si);
            *bad = nums_strings.join(",");
        }
    }

    for num in bad_ones{
        counter += get_middle(num.split(",").map(|s| s.to_string()).collect());
    }
    counter 
}


fn is_good(ele: &String, map: &HashMap<i32, Vec<i32>>) -> (bool, i32, i32) {
    let nums: Vec<String> = ele.split(",").map(|s| s.to_string()).collect();
    let mut set: HashSet<i32> = HashSet::new();
    println!("{:?}", nums);
    for ele_num in nums {
        let parsed: i32 = ele_num.parse().unwrap();
        if let Some(val) = map.get(&parsed) {
            for num in val {
                if set.contains(num) {
                    return (false, parsed, *num)
                }
            }
        }
        set.insert(parsed);
    }
    (true, 0, 0)
}

