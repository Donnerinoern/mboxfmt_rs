use std::collections::HashMap;
use crate::parser::FieldType;

pub fn format(map: &mut HashMap<FieldType, String>) {
    if map.get(&FieldType::Subject).unwrap().contains("=?iso-8859-1?Q?") {
        let subject = format_subject(&mut map.get(&FieldType::Subject).unwrap().to_string());
        map.remove(&FieldType::Subject);
        map.insert(FieldType::Subject, subject);
        let content = format_text(&mut map.get(&FieldType::ContentPlain).unwrap().to_string());
        map.remove(&FieldType::ContentPlain);
        map.insert(FieldType::ContentPlain, content);
    }

    // ret_map
}

// pub fn format_subject(subject_string: &mut String) -> String {
//     clean_subject(subject_string);
// }

pub fn format_text(text_string: &mut String) -> String {
    *text_string = text_string.replace("=E5", "å");
    *text_string = text_string.replace("=F8", "ø");
    // println!("{text_string}");
    text_string.to_string()
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
