use std::collections::{HashSet, VecDeque, BTreeSet, BTreeMap};
use std::env::args;
use std::error::Error;
use std::fs::read_to_string;
use std::process::exit;
use std::iter::repeat;

fn reverse(content: &str) {
    println!("Reverse the file content by storing it in a LIFO (VecDeque): ");
    let lines: VecDeque<&str> = content.lines().collect();
    for (i, line) in lines.into_iter().rev().enumerate() {
        display(i, line);
    }
}

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

fn sort_by_length_uniques(content: &str) {
    println!("Sort lines by length then alphabetical order, remove duplicated using a BinaryTree (BTreeSet)");
    let lines : BTreeSet<(usize, &str)> = content.lines().map(|line| (line.len(), line)).collect();
    for (i, (_, line)) in lines.into_iter().enumerate() {
        display(i, line);
    }
}

fn sort_by_length_all(content: &str) {
    println!("Sort lines by length then alphabetical order using a BinaryTree (BTreeMap)");
    let mut lines: BTreeMap<(usize, &str), usize> = BTreeMap::new();
    for line in content.lines() {
        *lines.entry((line.len(), line)).or_default() += 1;
    }
    for (i, line) in lines.into_iter().flat_map(|((_, line), count)| repeat(line).take(count)).enumerate() {
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

    Ok(())
}

fn main() {
    if run().is_err() {
        exit(1);
    }
}

fn print_help() {
    println!("USAGE: exercice1_1 [FILENAME]");
}
