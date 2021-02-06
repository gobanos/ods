use std::collections::VecDeque;
use std::env::args;
use std::error::Error;
use std::fs::read_to_string;
use std::process::exit;

fn reverse(content: &str) {
    println!("Reverse the file content by storing it in a LIFO (VecDeque): ");
    let lines: VecDeque<&str> = content.lines().collect();
    for (i, line) in lines.into_iter().rev().enumerate() {
        println!("{}: {}", i, line);
    }
}

fn reverse_bulk_50(content: &str) {
    const BULK_SIZE: usize = 50;
    println!(
        "Reverse the file content {} lines at a time by storing it in a LIFO (VecDeque): ",
        BULK_SIZE
    );
    let mut buffer = VecDeque::with_capacity(50);
    let lines = &mut content.lines();

    for bulk_index in 0.. {
        buffer.clear();
        buffer.extend(lines.take(BULK_SIZE));
        if buffer.is_empty() {
            return;
        }
        for (i, line) in buffer.iter().rev().enumerate() {
            println!("{}: {}", i + BULK_SIZE * bulk_index, line);
        }
    }
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
    reverse_bulk_50(&file_content);

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
