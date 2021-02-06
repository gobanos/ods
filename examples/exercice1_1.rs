use std::collections::VecDeque;
use std::env::args;
use std::error::Error;
use std::fs::read_to_string;
use std::process::exit;

fn reverse(content: &str) {
    println!("Reverse the file content by storing it in a LIFO (VecDeque): ");
    let lines: VecDeque<&str> = content.lines().collect();
    for (i, line) in lines.into_iter().rev().enumerate() {
        println!("{}: '{}'", i + 1, line);
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
            println!("{}: '{}'", i + BULK_SIZE * bulk_index + 1, line);
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
        println!("{}: '{}'", i + 1, line);
    }
    // Run with blank detection
    for (i, line) in lines.enumerate() {
        let default = buffer.pop_front().expect("buffer is empty");
        let line = if line.is_empty() { default } else { line };
        buffer.push_back(line);
        println!("{}: '{}'", i + BUFFER_SIZE + 1, line);
    }
    assert_eq!(buffer.capacity(), original_capacity);
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
