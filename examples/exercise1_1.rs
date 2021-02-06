//! # Exercise 1.1.
//! This exercise is designed to help familiarize the reader with
//! choosing the right data structure for the right problem. If implemented,
//! the parts of this exercise should be done by making use of an implementation
//! of the relevant interface (Stack, Queue, Deque, USet, or SSet) provided by the language.
//! Solve the following problems by reading a text file one line at a time
//! and performing operations on each line in the appropriate data structure(s).
//! Your implementations should be fast enough that even files containing
//! a million lines can be processed in a few seconds.

use rand::prelude::SliceRandom;
use rand::thread_rng;
use std::collections::{BTreeMap, BTreeSet, HashSet, VecDeque};
use std::env::args;
use std::error::Error;
use std::fs::read_to_string;
use std::iter::repeat;
use std::process::exit;

/// # Question 1.1.1
/// Read the input one line at a time and then write the lines out in
/// reverse order, so that the last input line is printed first, then the
/// second last input line, and so on.
///
/// # Solution
/// Use a LIFO data structure (`VecDeque` in that case).
fn reverse(content: &str) {
    println!("Reverse the file content by storing it in a LIFO (VecDeque): ");
    let lines: VecDeque<&str> = content.lines().collect();
    for (i, line) in lines.into_iter().rev().enumerate() {
        display(i, line);
    }
}

/// # Question 1.1.2
/// Read the first 50 lines of input and then write them out in reverse
/// order. Read the next 50 lines and then write them out in reverse
/// order. Do this until there are no more lines left to read, at which
/// point any remaining lines should be output in reverse order.
/// In other words, your output will start with the 50th line, then the
/// 49th, then the 48th, and so on down to the first line. This will be
/// followed by the 100th line, followed by the 99th, and so on down to
/// the 51st line. And so on.
/// Your code should never have to store more than 50 lines at any given
/// time.
///
/// # Solution
/// Reuse a LIFO data structure (`VecDeque`) with given capacity and empty
/// it each time 50th iteration.
fn reverse_bulk(content: &str) {
    const BULK_SIZE: usize = 50;
    println!(
        "Reverse the file content {} lines at a time by storing it in a LIFO (VecDeque): ",
        BULK_SIZE
    );
    let mut buffer = VecDeque::with_capacity(BULK_SIZE);
    let original_capacity = buffer.capacity();
    let lines = &mut content.lines();

    for bulk_index in 0.. {
        buffer.clear();
        buffer.extend(lines.take(BULK_SIZE));
        if buffer.is_empty() {
            break;
        }
        for (i, line) in buffer.iter().rev().enumerate() {
            display(i + BULK_SIZE * bulk_index, line);
        }
    }
    assert_eq!(buffer.capacity(), original_capacity);
}

/// # Question 1.1.3
/// Read the input one line at a time. At any point after reading the
/// first 42 lines, if some line is blank (i.e., a string of length 0), then
/// output the line that occured 42 lines prior to that one. For example,
/// if Line 242 is blank, then your program should output line 200.
/// This program should be implemented so that it never stores more
/// than 43 lines of the input at any given time.
///
/// # Solution
/// Fill a FIFO data structure (here a `VecDeque`) with the 42 first lines
/// then pop_front the default entry and select it if current line is empty.
/// Finally push_back the displayed line.
fn fill_blanks(content: &str) {
    const BUFFER_SIZE: usize = 42;
    println!(
        "Replace blanks lines (after line {size}) with the one {size} lines before using a Deque (VecDeque)",
        size = BUFFER_SIZE
    );
    let mut buffer = VecDeque::with_capacity(BUFFER_SIZE);
    let original_capacity = buffer.capacity();
    let lines = &mut content.lines();

    // Fill the buffer
    for (i, line) in lines.take(BUFFER_SIZE).enumerate() {
        buffer.push_back(line);
        display(i, line);
    }
    // Run with blank detection
    for (i, line) in lines.enumerate() {
        let default = buffer.pop_front().expect("buffer is empty");
        let line = if line.is_empty() { default } else { line };
        buffer.push_back(line);
        display(i + BUFFER_SIZE, line);
    }
    assert_eq!(buffer.capacity(), original_capacity);
}

/// # Question 1.1.4
/// Read the input one line at a time and write each line to the output
/// if it is not a duplicate of some previous input line. Take special care
/// so that a file with a lot of duplicate lines does not use more memory
/// than what is required for the number of unique lines.
///
/// # Solution
/// Filter values by inserting them in a Set (here a `HashSet`) and
/// checking the value wasn't previously in the set.
fn uniques(content: &str) {
    println!("Remove duplicates lines using a Set (HashSet)");
    let mut unique_lines = HashSet::new();
    for (i, line) in content
        .lines()
        .filter(|&line| unique_lines.insert(line))
        .enumerate()
    {
        display(i, line);
    }
}

/// # Question 1.1.5
/// Read the input one line at a time and write each line to the output
/// only if you have already read this line before. (The end result is that
/// you remove the first occurrence of each line.) Take special care so
/// that a file with a lot of duplicate lines does not use more memory
/// than what is required for the number of unique lines.
///
/// # Solution
/// Filter values by inserting them in a Set (here a `HashSet`) and
/// checking the value was previously in the set.
fn duplicates(content: &str) {
    println!("Remove first occurrence of each line using a Set (HashSet)");
    let mut unique_lines = HashSet::new();
    for (i, line) in content
        .lines()
        .filter(|&line| !unique_lines.insert(line))
        .enumerate()
    {
        display(i, line);
    }
}

/// # Question 1.1.6
/// Read the entire input one line at a time. Then output all lines sorted
/// by length, with the shortest lines first. In the case where two lines
/// have the same length, resolve their order using the usual “sorted
/// order.” Duplicate lines should be printed only once.
///
/// # Solution
/// Insert the tuple (line_length, line) in a BinaryTree (`BTreeSet`)
/// then read it in order.
fn sort_by_length_uniques(content: &str) {
    println!("Sort lines by length then alphabetical order, remove duplicated using a BinaryTree (BTreeSet)");
    let lines: BTreeSet<(usize, &str)> = content.lines().map(|line| (line.len(), line)).collect();
    for (i, (_, line)) in lines.into_iter().enumerate() {
        display(i, line);
    }
}

/// # Question 1.1.7
/// Do the same as the previous question except that duplicate lines
/// should be printed the same number of times that they appear in the
/// input.
///
/// # Solution
/// Insert the tuple (line_length, line) in a BinaryTree (`BTreeMap`),
/// increasing the count each time then read it in order.
fn sort_by_length_all(content: &str) {
    println!("Sort lines by length then alphabetical order using a BinaryTree (BTreeMap)");
    let mut lines: BTreeMap<(usize, &str), usize> = BTreeMap::new();
    for line in content.lines() {
        *lines.entry((line.len(), line)).or_default() += 1;
    }
    for (i, line) in lines
        .into_iter()
        .flat_map(|((_, line), count)| repeat(line).take(count))
        .enumerate()
    {
        display(i, line);
    }
}

/// # Question 1.1.8
/// Read the entire input one line at a time and then output the even
/// numbered lines (starting with the first line, line 0) followed by the
/// odd-numbered lines.
///
/// # Solution
/// Use a `Vec` to store the lines then read the even indices then the odd ones.
fn even_then_odd(content: &str) {
    println!("Display the even numbered lines then the odd numbered ones using a Vec");
    let lines: Vec<&str> = content.lines().collect();
    for (i, line) in lines.iter().step_by(2).enumerate() {
        display(i, line);
    }
    for (i, line) in lines.iter().skip(1).step_by(2).enumerate() {
        display(i + (lines.len() + 1) / 2, line);
    }
}

/// # Question 1.1.9
/// Read the entire input one line at a time and randomly permute the
/// lines before outputting them. To be clear: You should not modify
/// the contents of any line. Instead, the same collection of lines should
/// be printed, but in a random order.
///
/// # Solution
/// Use a `Vec` to store the lines and shuffle it !
/// An alternative would be to use the randomness from a `HashSet` (but is it really random ?).
fn random_sort(content: &str) {
    println!("Display the lines in a random order using a Vec");
    let mut lines: Vec<&str> = content.lines().collect();
    let mut rng = thread_rng();
    lines.shuffle(&mut rng);
    for (i, line) in lines.into_iter().enumerate() {
        display(i, line);
    }
}

fn display(index: usize, line: &str) {
    println!("{}: '{}'", index + 1, line);
}

fn run() -> Result<(), Box<dyn Error>> {
    if args().len() != 2 {
        eprintln!("Invalid number of arguments");
        print_help();
        return Err("".into());
    }

    let file = args().nth(1).expect("filename");
    let file_content = read_to_string(&file)?;

    reverse(&file_content);
    reverse_bulk(&file_content);
    fill_blanks(&file_content);
    uniques(&file_content);
    duplicates(&file_content);
    sort_by_length_uniques(&file_content);
    sort_by_length_all(&file_content);
    even_then_odd(&file_content);
    random_sort(&file_content);

    Ok(())
}

fn main() {
    if run().is_err() {
        exit(1);
    }
}

fn print_help() {
    println!("USAGE: exercise1_1 [FILENAME]");
}
