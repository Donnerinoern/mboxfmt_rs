use std::{collections::HashMap, fs, io::{BufWriter, Write}};
use crate::parser::FieldType;
use crate::print_error;
use crate::FileExtension;

pub fn generate_file(output_filename: &String, map: &HashMap<FieldType, String>, file_extension: FileExtension) {
    let file = fs::File::create_new(output_filename);
    match file {
        Ok(f) => {
            let mut writer = BufWriter::new(f);
            match file_extension {
                FileExtension::TXT => {
                    writer.write(map.get(&FieldType::From).unwrap().as_bytes());
                    writer.write(map.get(&FieldType::To).unwrap().as_bytes());
                    writer.write(map.get(&FieldType::Date).unwrap().as_bytes());
                    writer.write(map.get(&FieldType::Subject).unwrap().as_bytes());
                    writer.write(map.get(&FieldType::ContentPlain).unwrap().as_bytes());
                }

                FileExtension::MD => {
                    let mut buffer: String = String::new();
                    buffer.push_str("### ");
                    buffer.push_str(map.get(&FieldType::From).unwrap());
                    buffer.push_str("### ");
                    buffer.push_str(map.get(&FieldType::To).unwrap());
                    buffer.push_str("### ");
                    buffer.push_str(map.get(&FieldType::Date).unwrap());
                    buffer.push_str("### ");
                    buffer.push_str(map.get(&FieldType::Subject).unwrap());
                    buffer.push_str("---");
                    buffer.push_str(map.get(&FieldType::ContentPlain).unwrap());
                    writer.write(buffer.as_bytes());
                }

                FileExtension::HTML => {
                }
            }
        }

        Err(_s) => {
            print_error("File already exists.");
            return;
        }
    }
}
