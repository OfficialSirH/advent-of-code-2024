use std::{
    fs::File,
    io::{BufRead, BufReader},
};

/// ## 01 Hystorian Hysteria Part 2
/// 1. Read the input file into 2 separate Vecs (Format: `{left_list} {right_list}`)
/// 2. Iterate over the left_list, accumulating how much each element from it is repeated from the right_list
/// 3. multiply each element by its repetition
/// 4. accumulate all of the left_list after previous modifications
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

    let similarity_score = left_list
        .iter()
        .map(|location_id| {
            // step 2
            right_list
                .iter()
                .filter(|ln_id| location_id == *ln_id)
                // step 3
                .sum::<i32>()
        })
        // step 4
        .sum::<i32>();

    println!("{}", similarity_score);
}
