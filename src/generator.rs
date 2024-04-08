use std::{collections::HashMap, fs, io::{BufWriter, Write}};
use crate::parser::FieldType;
use crate::help;
use crate::print_error;

enum FileExt {
    TXT,
    MD,
    HTML
}

pub fn generate_file(output_filename: &String, map: &HashMap<FieldType, String>) {
    let file_extension = output_filename.split_once(".");
    let mut file_ext_enum: Option<FileExt> = None;
    match file_extension {
        Some(s) => {
            match s.1 {
                "txt" => {
                    file_ext_enum = Some(FileExt::TXT);
                }

                "md" => {
                    file_ext_enum = Some(FileExt::MD);
                }

                "html" => {
                    file_ext_enum = Some(FileExt::HTML);
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

    let file = fs::File::create_new(output_filename);
    match file {
        Ok(f) => {
            let mut writer = BufWriter::new(f);
            match file_ext_enum.expect("???") {
                FileExt::TXT => {
                    writer.write(map.get(&FieldType::From).unwrap().as_bytes());
                    writer.write(map.get(&FieldType::To).unwrap().as_bytes());
                    writer.write(map.get(&FieldType::Date).unwrap().as_bytes());
                    writer.write(map.get(&FieldType::Subject).unwrap().as_bytes());
                    writer.write(map.get(&FieldType::ContentPlain).unwrap().as_bytes());
                }

                FileExt::MD => {
                }

                FileExt::HTML => {
                }
            }
        }

        Err(_s) => {
            print_error("File already exists.");
            return;
        }
    }
}
