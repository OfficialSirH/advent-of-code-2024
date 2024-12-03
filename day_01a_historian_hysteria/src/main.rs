use std::{
    fs::File,
    io::{BufRead, BufReader},
};

/// ## 01 Hystorian Hysteria Part 1
/// 1. Read the input file into 2 separate Vecs (Format: `{left_list} {right_list}`)
/// 2. Sort the Vecs into ascending order
/// 3. Accumulate the distances of each adjacent element
/// 4. print the total distance accumulated
fn main() {
    // step 1
    let lines =
        BufReader::new(File::open("input.txt").expect("Expected input.txt file to exist")).lines();

    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    for line in lines.flatten() {
        let splitted_line: Vec<i32> = line
            .split("   ")
            .map(|value| unsafe { value.parse::<i32>().unwrap_unchecked() })
            .collect();

        left_list.push(splitted_line[0]);
        right_list.push(splitted_line[1]);
    }

    // step 2
    left_list.sort();
    right_list.sort();

    // step 3
    let mut accumulated_distance = 0;
    for (index, location_id) in left_list.iter().enumerate() {
        accumulated_distance += (location_id - right_list[index]).abs();
    }

    // step 4
    println!("{}", accumulated_distance);
}
