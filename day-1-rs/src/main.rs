use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::io;

fn main() {
    let (mut list_a,mut list_b) = seperate_lists();

    list_a.sort();
    list_b.sort();

    let distance = find_distance(&list_a, &list_b);
    println!("Distance: {}", distance);
    let similarity = find_similarity(&list_a, &list_b);
    println!("Similarity: {}", similarity);
}

fn find_similarity(list_a: &Vec<i32>, list_b: &Vec<i32>) -> i32 {
    let mut similarity = 0;
    for i in list_a.iter() {
        let occurance = list_b.iter().filter(|&&b| b == *i).count() as i32;
        similarity += i * occurance;
    }
    similarity
}

fn find_distance(list_a: &Vec<i32>, list_b: &Vec<i32>) -> u32 {
    let mut distance: u32 = 0;
    for i in 0..list_a.len(){
        distance = distance + list_a[i].abs_diff(list_b[i]);
    }
    distance
}

fn seperate_lists()-> (Vec<i32>, Vec<i32>){
    let mut list_a: Vec<i32> = Vec::new();
    let mut list_b: Vec<i32> = Vec::new();

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines.flatten(){
            let split_line = line.split_whitespace().collect::<Vec<&str>>();
            list_a.push(split_line[0].parse::<i32>().expect("Could not parse string to int"));
            list_b.push(split_line[1].parse::<i32>().expect("Could not parse string to int"));
        }
    }

    (list_a, list_b)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


// Requirements
// Location ID = historically significant locations

// Process
// 1. Find the smallest number in left list
// 2. Find the smallest number in right list
// 3. Find the value between the two values (higher number - lower number  = distance)
// 4. Add all distances up together

// Important Notes
// - Duplicate values must all be used as values before moving on the next highest value.
//      i.e if there are 5 3's in the left list you must compare every 3 to a number in the right list
// - The lists are the same length
//      - They can be indexed at the same time.

// Brute Force Solution
// Ingest lists into two sorted vectors.
// Go from index 0 to last index and find distance of object from corresponding index
// Print the distance


