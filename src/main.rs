#![feature(string_remove_matches)]

use std::env;

mod parser;
mod formatter;
mod generator;

enum FileExtension {
    TXT,
    MD,
    HTML,
    Unknown
}

fn main() {
    // let args: Vec<String> = env::args().collect();

    // let filenames = match args.len() { // TODO: Change this
    //     1 | 2 => {
    //         help();
    //         return;
    //     }
    //
    //     3 => {
    //         (&args[args.len()-2], &args[args.len()-1])
    //     }
    //
    //     _ => {
    //         println!("Unknown args.");
    //         help();
    //         return;
    //     }
    // };

    let args: Vec<String> = if env::args().len() > 3 {
        env::args().collect()
    } else {
        help();
        return;
    };

    let filenames = (&args[args.len()-2], &args[args.len()-1]);
    let mut overwrite = false;
    for arg in &args {
        if arg == "-o" || arg == "--overwrite" {
            overwrite = true;
        } else if arg.contains("--") {
            println!("mboxfmt: unrecognized argument '{}'", arg);
        }
    }

    let mut content_map = parser::parse_file(filenames.0);
    let file_extension = filenames.1.split_once(".").expect("mboxfmt: missing file extension.").1;
    let file_ext_enum = match file_extension {
        "txt" => {
            FileExtension::TXT
        }

        "md" => {
            FileExtension::MD
        }

        "html" => {
            FileExtension::HTML
        }

        _ => {
            FileExtension::Unknown
        }
    };
    formatter::format(&mut content_map, &file_ext_enum);
    generator::generate_file(filenames.1, &content_map, file_ext_enum);
}

fn parse_args() {
}

pub fn help() { // Potential arg: --overwrite
    println!("
Usage: mboxfmt <input file> <output filename>\n
Arguments:
-o            --overwrite\n
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
