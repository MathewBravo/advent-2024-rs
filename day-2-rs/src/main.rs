use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn main() {
    let mut safety_score: i32 = 0;
    if let Ok(lines) = read_lines("./src/input") {
        for line in lines.flatten() {
            // Collect parsed integers or print an error if parsing fails
            let l_list: Result<Vec<i32>, _> = line
                .split_whitespace()
                .map(|s| s.parse::<i32>())
                .collect();

            match l_list {
                Ok(numbers) => {
                    if is_safe(&numbers){
                        safety_score += 1;
                    }
                },
                Err(e) => eprintln!("Failed to parse line: '{}', Error: {}", line, e),
            }
        }
    }

    println!("Safety score: {}", safety_score);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn is_safe(numbers: &Vec<i32>) -> bool {
    if check_sorted(numbers) && numbers.windows(2).all(|w| w[0].abs_diff(w[1]) >= 1 && w[0].abs_diff(w[1]) <= 3) {
        return true;
    }

    for i in 0..numbers.len() {
        let mut modified = numbers.clone();
        modified.remove(i);
        if check_sorted(&modified) && modified.windows(2).all(|w| w[0].abs_diff(w[1]) >= 1 && w[0].abs_diff(w[1]) <= 3) {
            return true;
        }
    }
    false
}



fn check_sorted(numbers: &Vec<i32>) -> bool {
    is_sorted_ascending(&numbers) || is_sorted_descending(&numbers)
}

fn is_sorted_ascending<T: Ord>(list: &[T]) -> bool {
    list.windows(2).all(|w| w[0] <= w[1])
}
fn is_sorted_descending<T: Ord>(list: &[T]) -> bool {
    list.windows(2).all(|w| w[0] >= w[1])
}
