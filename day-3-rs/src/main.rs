use std::fs;
use std::path::Path;
use regex::Regex;

fn main() {
    //Part 1
    // parse_for_mul_result("./src/input".as_ref());
    parse_instructions("./src/input".as_ref());
}

fn parse_for_mul_result(path: &Path){
    let mut mul_result: i32 = 0;
    let file_contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    for caps in re.captures_iter(&file_contents){
        let (num1, num2) = (&caps[1], &caps[2]);
        mul_result += num1.parse::<i32>().unwrap() * num2.parse::<i32>().unwrap();
    }
    println!("{}", mul_result);
}

fn parse_instructions(path: &Path){
    let mut can_mul: bool = true;
    let mut mul_result: i64 = 0;

    let file_contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();

    for cap in re.captures_iter(&file_contents){
        if cap.get(0).is_none() {
            continue;
        }

        let matched = cap.get(0).unwrap().as_str();

        if matched.starts_with("do(") {
            can_mul = true;
        }
        else if matched.starts_with("don't") {
            can_mul = false;
        }
        else if let Some(mul_x) = cap.get(1) {
            let mul_y = cap.get(2).unwrap();
            if can_mul {
                let x: i64 = mul_x.as_str().parse().unwrap();
                let y: i64 = mul_y.as_str().parse().unwrap();
                mul_result += x * y;
            }
        }
    }

    println!("Total mul result: {}", mul_result);
}

fn print_mul_state(state: &bool){
    println!("{}", state)
}

