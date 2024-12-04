fn main() {
    let input: Vec<String> = std::fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    println!("{}", run_a(input))
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
                if check_next(&chars, i, j, (0, -1)) {count+=1;}
                if check_next(&chars, i, j, (0, 1)) {count+=1;}
                if check_next(&chars, i, j, (1, 0)) {count+=1;}
                if check_next(&chars, i, j, (-1, 0)) {count+=1;}
                if check_next(&chars, i, j, (0, -1)) {count+=1;}
                if check_next(&chars, i, j, (0, -1)) {count+=1;}
                if check_next(&chars, i, j, (0, -1)) {count+=1;}
                if check_next(&chars, i, j, (0, -1)) {count+=1;}
            }
        }
    }
    
    count
}

fn check_next(chars: &Vec<Vec<char>>, i: usize, j: usize, offset: (i32, i32)) -> bool {

    false
}
