#![feature(string_remove_matches)]

use std::{env, fs};

mod parser;
mod formatter;
mod generator;

enum FileExtension {
    TXT,
    MD,
    HTML
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut input_filename  = &String::new(); // Maybe turn into Option<&String>
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

    let file_extension = output_filename.split_once(".");
    let mut file_ext_enum: Option<FileExtension> = None;
    match file_extension {
        Some(s) => {
            match s.1 {
                "txt" => {
                    file_ext_enum = Some(FileExtension::TXT);
                }

                "md" => {
                    file_ext_enum = Some(FileExtension::MD);
                }

                "html" => {
                    file_ext_enum = Some(FileExtension::HTML);
                }

                _ => {
                    print_error("Unknown file extension.");
                    return;
                }
            }
        }

        None => {
            print_error("Missing file extension.");
            return;
        }
    }

    match file_ext_enum {
        Some(e) => {
            formatter::format(&mut map, &e);
            generator::generate_file(output_filename, &map, e);
        }

        None => {
            print_error("Missing file extension.");
            return;
        }
    }
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
