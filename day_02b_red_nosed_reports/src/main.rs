use std::{
    fs::File,
    io::{BufRead, BufReader},
};

/// ## 02 Red-Nosed Reports Part 2
/// 1. Read the input file into Vecs of numbers (Format: `{number} *`)
/// 2. iterate through the reports and filter for those that fit the following rules:
///     - The levels are either all increasing or all decreasing.
///     - Any two adjacent levels differ by at least one and at most three.
///     - There is an exception for one bad level
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
            let mut failure_point: Option<usize> = None;

            let is_safe = levels.iter().enumerate().all(|enum_params| {
                safety_check(&levels, &mut increasing, &mut failure_point, enum_params)
            });

            if !is_safe {
                for (index, _) in levels.iter().enumerate() {
                    let mut adjusted_levels = levels.clone();
                    adjusted_levels.remove(index);
                    let is_adjustment_safe =
                        adjusted_levels.iter().enumerate().all(|enum_params| {
                            safety_check(
                                &adjusted_levels,
                                &mut increasing,
                                &mut failure_point,
                                enum_params,
                            )
                        });

                    if is_adjustment_safe {
                        return true;
                    }
                }
                return false;
            }

            return true;
        })
        .filter(|is_safe| *is_safe == true)
        .count();

    println!("total safe reports: {safe_count}")
}

fn safety_check(
    levels: &Vec<i32>,
    increasing: &mut bool,
    failure_point: &mut Option<usize>,
    (index, level): (usize, &i32),
) -> bool {
    if index == levels.len() - 1 {
        return true;
    }

    let comparison_index = (index + 1).min(levels.len() - 1);
    let comparison_level = levels[comparison_index];

    if index == 0 {
        *increasing = comparison_level > *level;
    }

    let rule_one = if *increasing {
        comparison_level > *level
    } else {
        *level > comparison_level
    };

    let diff = (*level - comparison_level).abs();
    if rule_one {
        if diff == diff.clamp(1, 3) {
            return true;
        }
    }

    if failure_point.is_none() {
        if diff > 3 {
            *failure_point = Some(comparison_index);
        } else {
            *failure_point = Some(index);
        }
    }

    return false;
}
