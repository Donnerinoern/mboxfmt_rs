#![feature(string_remove_matches)]

use std::{env, fs};

mod parser;
mod formatter;
mod generator;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut input_filename  = &String::new();
    let mut output_filename = &String::new();

    match args.len() {
        1 | 2 => {
            help();
            return;
        }

        3 => {
            input_filename  = &args[args.len()-2];
            output_filename = &args[args.len()-1];
        }

        _ => {
            println!("Unknown args.");
            help();
            return;
        }
    }

    let source_file = fs::read_to_string(input_filename)
        .expect("File does not exist.");
    let source_vec: Vec<&str> = source_file.split_inclusive("\n").collect(); // Maybe parse_file
    let mut map = parser::parse_file(&source_vec);
    formatter::format(&mut map);
    generator::generate_file(output_filename, &map)
}

pub fn help() { // Potential arg: --overwrite
    println!("
Usage: mboxfmt <input file> <output filename>\n
Valid file extensions:
txt
html
md"
    );
}

pub fn print_error(msg: &str) {
    println!("Error: {}", msg);
    help();
}
