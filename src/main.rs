#![feature(string_remove_matches)]

use std::{env, fs, io::Write};

use crate::parser::FieldType;
mod parser;
mod formatter;

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
            println!("Unknown args!");
            help();
            return;
        }
    }

    let source_file = fs::read_to_string(input_filename)
        .expect("File does not exist.");

    let source_vec: Vec<&str> = source_file.split_inclusive("\n").collect(); // Maybe parse_file
                                                                             // should take a
                                                                             // string and split
    let mut map = parser::parse_file(&source_vec);

    formatter::format(&mut map);

    let mut file = fs::File::create_new(output_filename).unwrap();
    file.write_all(map.get(&FieldType::From).unwrap().as_bytes());
    file.write_all(b"\n");
    file.write_all(map.get(&FieldType::To).unwrap().as_bytes());
    file.write_all(b"\n");
    file.write_all(map.get(&FieldType::Date).unwrap().as_bytes());
    file.write_all(b"\n");
    file.write_all(map.get(&FieldType::Subject).unwrap().as_bytes());
    file.write_all(b"\n\n");
    file.write_all(map.get(&FieldType::ContentPlain).unwrap().as_bytes());
}

fn help() {
    println!("
Usage: mboxto <input file> <output filename>\n
Valid file extensions:
txt
html
md"
    );
}
