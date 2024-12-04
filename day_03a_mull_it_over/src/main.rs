#![feature(let_chains)]

use std::{fs::read_to_string, iter::Peekable, str::Chars};

struct Lexer {
    input: String,
    result: Vec<[u32; 2]>,
}

#[derive(PartialEq)]
enum Multiply {
    Keyword,           // mul
    OpenParens,        // (
    CloseParens,       // )
    ArgumentSeparator, // ,
    FirstArg,          // number with up to 3 digits
    SecondArg,         // number with up to 3 digits
}

fn main() {
    let file = read_to_string("input.txt").expect("Expected input.txt file to exist");
    let mut lexer = Lexer {
        input: file,
        result: Vec::new(),
    };

    let content = lexer.input.chars();
    let mut peekable = content.peekable();
    let mut current_step: Option<Multiply> = None;
    let mut current_num_pair: [u32; 2] = [0; 2];
    while let Some(char) = peekable.next() {
        match char {
            // match for the Keyword
            'm' => {
                if peekable.next_if(|ch| ch == &'u').is_some()
                    && peekable.next_if(|ch| ch == &'l').is_some()
                {
                    current_step = Some(Multiply::Keyword);
                    continue;
                }
            }
            // match for the OpenParens
            '(' => {
                if current_step == Some(Multiply::Keyword) {
                    current_step = Some(Multiply::OpenParens);
                    continue;
                }
            }
            // match for the CloseParens
            ')' => {
                if current_step == Some(Multiply::SecondArg) {
                    lexer.result.push(current_num_pair);
                    current_step = None;
                    continue;
                }
            }
            // match for the ArgumentSeparator
            ',' => {
                if current_step == Some(Multiply::FirstArg) {
                    current_step = Some(Multiply::ArgumentSeparator);
                    continue;
                }
            }
            // match for numbers (up to 3 digits)
            first_digit @ '0'..='9' => {
                if current_step == Some(Multiply::OpenParens) {
                    current_step = Some(Multiply::FirstArg);
                    current_num_pair[0] = build_number(&mut peekable, first_digit);
                    continue;
                } else if current_step == Some(Multiply::ArgumentSeparator) {
                    current_step = Some(Multiply::SecondArg);
                    current_num_pair[1] = build_number(&mut peekable, first_digit);
                    continue;
                }
            }
            // a 'corrupt' value
            _ => (),
        }
        // resets step when 'corrupted' parts are found within valid parts
        current_step = None;
    }

    println!(
        "total of all multiplications: {}",
        lexer
            .result
            .iter()
            .map(|pair| pair[0] * pair[1])
            .sum::<u32>()
    )
}

fn build_number(peekable: &mut Peekable<Chars>, first_digit: char) -> u32 {
    let mut value = String::with_capacity(3);
    value.insert(0, first_digit);
    for idx in 1..=2 {
        if let Some(next_digit) = peekable.next_if(|ch| ('0'..='9').contains(ch)) {
            value.insert(idx, next_digit);
            continue;
        }
        break;
    }

    return value.parse().expect("U32 Value");
}
