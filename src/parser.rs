use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq)]
pub enum FieldType {
    None,
    To,
    From,
    Date,
    Subject,
    ContentPlain,
    ContentHtml
}

pub fn parse_file(vec: &Vec<&str>) -> HashMap<FieldType, String> {
    let mut map: HashMap<FieldType, String> = HashMap::new();
    let mut field_type: FieldType = FieldType::None;
    let mut extract = false;
    let mut index = 0;
    let mut subject_start = 0;
    let mut subject_end = 0;
    let mut subject = String::new();
    let mut content_plain_start = 0;
    let mut content_plain_end = 0;
    let mut content_plain = String::new();
    let mut content_html_start = 0;
    let mut content_html_end = 0;
    let mut content_html = String::new();

    for e in vec {
        index += 1;
        if !extract {
            if e.starts_with("From: ") {
                map.insert(FieldType::From, e.to_string());
            } else if e.starts_with("To: ") {
                map.insert(FieldType::To, e.to_string());
            } else if e.starts_with("Date: ") {
                map.insert(FieldType::Date, e.to_string());
            } else if e.starts_with("Subject:") {
                if e.len() <= 9 {
                    subject_start = index;
                    field_type = FieldType::Subject;
                    extract = true;
                    subject.push_str("Subject:")
                } else {
                    map.insert(FieldType::Subject, e.to_string());
                }
            } else if e.contains("Content-Type: text/plain;") {
                content_plain_start = index;
                field_type = FieldType::ContentPlain;
                extract = true;
            } else if e.contains("Content-Type: text/html;") {
                content_html_start = index;
                field_type = FieldType::ContentHtml;
                extract = true;
            }
        } else {
            match field_type {
                FieldType::Subject => {
                    if !e.starts_with(" ") {
                        subject_end = index;
                        extract = false;
                    }
                }

                FieldType::ContentPlain => {
                    if e.starts_with("--") {
                        content_plain_end = index;
                        extract = false;
                    } else if e.starts_with("Content-") {
                        content_plain_start = index;
                    }
                }

                FieldType::ContentHtml => {
                    if e.starts_with("--") && e.ends_with("--\n") {
                        content_html_end = index;
                        extract = false;
                    } else if e.starts_with("Content-") {
                        content_html_start = index;
                    }
                }

                _ => {
                }
            }
        }
    }
    if subject_start != 0 {
        for i in subject_start..subject_end-1 {
            subject.push_str(vec[i]);
        }
        map.insert(FieldType::Subject, subject);
    }
    if content_plain_start != 0 {
        for i in content_plain_start..content_plain_end-1 {
            content_plain.push_str(vec[i].trim_end_matches("=\n"));
        }
        map.insert(FieldType::ContentPlain, content_plain);
    }
    if content_html_start != 0 {
        for i in content_html_start..content_html_end-1 {
            content_html.push_str(vec[i].trim_end_matches("=\n"));
        }
        map.insert(FieldType::ContentHtml, content_html);
    }
    map
}
