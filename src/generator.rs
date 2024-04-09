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
                    let mut buffer: String = String::new();
                    buffer.push_str(map.get(&FieldType::From).unwrap());
                    buffer.push_str(map.get(&FieldType::To).unwrap());
                    buffer.push_str(map.get(&FieldType::Date).unwrap());
                    buffer.push_str(map.get(&FieldType::Subject).unwrap());
                    buffer.push_str(map.get(&FieldType::ContentPlain).unwrap());
                    let _ = writer.write(buffer.as_bytes());
                }

                FileExtension::MD => {
                    let mut buffer: String = String::new();
                    buffer.push_str("### ");
                    buffer.push_str(map.get(&FieldType::From).unwrap());
                    buffer.push_str("\n");

                    buffer.push_str("### ");
                    buffer.push_str(map.get(&FieldType::To).unwrap());
                    buffer.push_str("\n");

                    buffer.push_str("### ");
                    buffer.push_str(map.get(&FieldType::Date).unwrap());
                    buffer.push_str("\n");

                    buffer.push_str("### ");
                    buffer.push_str(map.get(&FieldType::Subject).unwrap());
                    buffer.push_str("\n");

                    buffer.push_str("---");
                    buffer.push_str("\n");

                    buffer.push_str(map.get(&FieldType::ContentPlain).unwrap());
                    let _ = writer.write(buffer.as_bytes());
                }

                FileExtension::HTML => {
                    let mut buffer: String = String::new();
                    buffer.push_str("<h4>");
                    buffer.push_str(map.get(&FieldType::From).unwrap().trim_end());
                    buffer.push_str("</h4>\n");

                    buffer.push_str("<h4>");
                    buffer.push_str(map.get(&FieldType::To).unwrap().trim_end());
                    buffer.push_str("</h4>\n");

                    buffer.push_str("<h4>");
                    buffer.push_str(map.get(&FieldType::Date).unwrap().trim_end());
                    buffer.push_str("</h4>\n");

                    buffer.push_str("<h4>");
                    buffer.push_str(map.get(&FieldType::Subject).unwrap());
                    buffer.push_str("</h4>\n");

                    buffer.push_str(map.get(&FieldType::ContentHtml).unwrap());
                    let _ = writer.write(buffer.as_bytes());
                }
            }
        }

        Err(_s) => {
            print_error("File already exists.");
            return;
        }
    }
}
