use std::collections::HashMap;
use crate::{parser::FieldType, FileExtension};

pub fn format(map: &mut HashMap<FieldType, String>, file_extension: &FileExtension) {
    if map.get(&FieldType::Subject).unwrap().contains("=?iso-8859-1?Q?") {
        let subject = format_subject(&mut map.get(&FieldType::Subject).unwrap().to_string());
        map.remove(&FieldType::Subject);
        map.insert(FieldType::Subject, subject);
        let content = format_text(&mut map.get(&FieldType::ContentPlain).unwrap().to_string(), file_extension);
        map.remove(&FieldType::ContentPlain);
        map.insert(FieldType::ContentPlain, content);
    }
}

fn format_text(text_string: &mut String, file_extension: &FileExtension) -> String {
    *text_string = text_string.replace("=E5", "å");
    *text_string = text_string.replace("=F8", "ø");
    match file_extension {
        FileExtension::MD => {
            let mut new_string = String::new();
            for word in text_string.split(" ") {
                if word.contains("<http") && !word.contains("[") {
                    let mut s = String::from("[".to_owned() + word);
                    s = s.replace("<", "](");
                    s = s.replace(">", ")");

                    new_string.push_str(s.as_str());
                    new_string.push(' ');
                } else if word.contains("<http") && word.contains("[") {
                    let mut s = String::from(word);
                    s = s.replace("<", "(");
                    s = s.replace(">", ")\n");

                    new_string.push_str(s.as_str());
                    new_string.push(' ');
                } else {
                    new_string.push_str(word);
                    new_string.push(' ');
                }
            }
            new_string
        }

        _ => {
            text_string.to_string()
        }
    }
}

fn format_subject(string: &mut String) -> String {
    string.remove_matches("\n");
    string.remove_matches("?= =?");
    string.remove_matches("iso-8859-1?Q?");
    string.remove_matches("=?");
    string.remove_matches("?=");
    *string = string.replace("_", " ");
    *string = string.replace("=E5", "å");
    string.to_string()
}
