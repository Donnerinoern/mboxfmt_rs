#![feature(string_remove_matches)]

use std::{env, fs, io::Write};

use crate::parser::FieldType;
mod parser;
mod formatter;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        help();
        return;
    }
    let source_file = fs::read_to_string(&args[1])
        .expect("Could not read file.");
    let output = &args[2];

    let source_vec: Vec<&str> = source_file.split_inclusive("\n").collect();
    let mut map = parser::parse_file(&source_vec);
    // let mut subject = map.get(&FieldType::Subject).unwrap().to_string();
    // let mut text = map.get(&FieldType::ContentPlain).unwrap().to_string();

    formatter::format(&mut map);
    // formatter::format_subject(&mut map.get(&FieldType::Subject).unwrap().to_string());
    // formatter::format_text(&mut map.get(&FieldType::ContentPlain).unwrap().to_string());

    // println!("{text}");
    let mut file = fs::File::create_new(output).unwrap();
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
Usage: mboxto [arg [...]] <input> <output>\n
Arguments:
--help                   -h
--force FILE_EXT         -f FILE_EXT\n
Valid file extensions:
txt
html
md"
    );
}
