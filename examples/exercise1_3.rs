//! # Exercise 1.3.
//! A matched string is a sequence of {, }, (, ), [, and ] characters
//! that are properly matched. For example, “{{()[]}}” is a matched string, but
//! this “{{()]}” is not, since the second { is matched with a ]. Show how to
//! use a stack so that, given a string of length n, you can determine if it is a
//! matched string in O(n) time.
//!
//! # Solution
//! We can use push_back each opening char in a stack or pop_back and check that the last
//! open char matches the current closing one.
//! At the end of the function, the sequence is a matched string is the stack is empty.

use std::collections::VecDeque;
use std::env::args;
use std::error::Error;
use std::process::exit;

fn is_matched_string(sequence: &str) -> Result<bool, Box<dyn Error>> {
    let mut stack = VecDeque::new();
    for char in sequence.chars() {
        match char {
            '{' | '(' | '[' => stack.push_back(char),
            '}' => {
                if stack.pop_back() != Some('{') {
                    return Ok(false);
                }
            }
            ')' => {
                if stack.pop_back() != Some('(') {
                    return Ok(false);
                }
            }
            ']' => {
                if stack.pop_back() != Some('[') {
                    return Ok(false);
                }
            }
            _ => return Err(format!("Invalid char: '{}'", char).into()),
        }
    }
    Ok(stack.is_empty())
}

fn run() -> Result<(), Box<dyn Error>> {
    if args().len() != 2 {
        return Err("Invalid number of arguments".into());
    }

    let sequence = args().nth(1).expect("sequence");
    if is_matched_string(&sequence)? {
        println!("'{}' is a matched string", sequence);
    } else {
        println!("'{}' is not a matched string", sequence);
    }
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err);
        print_help();
        exit(1);
    }
}

fn print_help() {
    println!("USAGE: exercise1_3 [SEQUENCE]\n\n[SEQUENCE] is a sequence of {{, }}, (, ), [, and ] characters.");
}
