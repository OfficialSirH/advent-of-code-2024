use std::{
    fs::File,
    io::{BufRead, BufReader},
};

/// ## 02 Red-Nosed Reports Part 1
/// 1. Read the input file into Vecs of numbers (Format: `{number} *`)
/// 2. iterate through the reports and filter for those that fit the following rules:
///     - The levels are either all increasing or all decreasing.
///     - Any two adjacent levels differ by at least one and at most three.
fn main() {
    // step 1
    let reports =
        BufReader::new(File::open("input.txt").expect("Expected input.txt file to exist"))
            .lines()
            .flatten()
            .map(|levels| {
                levels
                    .split(' ')
                    .map(|level| unsafe { level.parse::<i32>().unwrap_unchecked() })
                    .collect::<Vec<i32>>()
            });

    // step 2
    let safe_count = reports
        .map(|levels| {
            let mut increasing = false;

            levels.iter().enumerate().all(|(index, level)| {
                if index == levels.len() - 1 {
                    return true;
                }

                let comparison_level = levels[(index + 1).min(levels.len() - 1)];

                if index == 0 {
                    increasing = comparison_level > *level;
                }

                let rule_one = if increasing {
                    comparison_level > *level
                } else {
                    *level > comparison_level
                };

                if rule_one {
                    let diff = (*level - comparison_level).abs();
                    if diff == diff.clamp(1, 3) {
                        return true;
                    }
                }
                return false;
            })
        })
        .filter(|is_safe| *is_safe == true)
        .count();

    println!("total safe reports {safe_count}")
}
