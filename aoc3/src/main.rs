fn main() {
    let input: Vec<String> = std::fs::read_to_string("./input2.txt").unwrap()
        .lines()
        .map(String::from)
        .collect();
    println!("{}", run_a(input));
}

fn helper(char_vec: Vec<char>, en_muls: bool) -> (i32, bool) {
    let mut result = 0;
    let mut enable_muls = en_muls;
    for (i, c) in char_vec.iter().enumerate() {
        if *c == 'd' {
            let mut tmp: String = String::new();
            let mut counter = 0;
            while counter < 7 {
                tmp.push(char_vec[i + counter]);
                counter += 1;
            }
            if &tmp[0..4] == "do()" {
                println!("1: {}", &tmp[0..4]);
                enable_muls = true;
            } else if &tmp[0..7] == "don't()" {
                println!("2: {}", &tmp[0..7]);
                enable_muls = false;
            }

        }
        if *c == 'm' && enable_muls {
            let mut tmp: String = String::new();
            let mut counter = 0;
            let first_num;
            let second_num;
            while counter < 4 {
                tmp.push(char_vec[i + counter]);
                counter += 1;
            }
            // println!("{}", tmp[0..4].to_string());
            // println!("{}, {}", char_vec[i + 4], char_vec[i + 5]);
            if !(&tmp[0..4] == "mul(") {
                continue;
            }
            if !char_vec[i + 4].is_numeric() {
                continue;
            }
            let first_nums = find_number(&char_vec, i + 4);
            // println!("HEREI TI {}", char_vec[ first_nums.1]);
            if char_vec[first_nums.1] != ',' {
                continue;
            }
            if !char_vec[first_nums.1 + 1].is_numeric() {
                continue;
            }
            let second_nums = find_number(&char_vec, first_nums.1 + 1);
            if char_vec[second_nums.1] != ')' {
                continue;
            }
            first_num  = first_nums.0;
            second_num = second_nums.0;
            result += first_num * second_num;
            // println!("found oone: mul({},{})", first_num, second_num);
        } 
    }

    (result, enable_muls)
}
fn run_a(input: Vec<String>) -> i32 {
    let mut sum = 0;
    let mut en_mul = true;
    for inp in input {
        let tmp = helper(inp.chars().collect(), en_mul);
        sum += tmp.0;
        en_mul = tmp.1;
    }
    return sum;
}

fn find_number(vec: &Vec<char>, start_index: usize) -> (i32, usize) {
    let mut counter = 0;
    let mut whole_number = "".to_string();
    loop {
        if vec[start_index + counter].is_numeric() {
            whole_number.push(vec[start_index + counter]); 
            counter += 1;
        } else {
            break;
        }
    }
    // println!("THE NUMBEr {}", whole_number);
    (whole_number.parse().unwrap(), counter + start_index)
}
