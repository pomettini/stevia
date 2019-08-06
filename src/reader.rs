use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum LineType {
    Undefined,
    Text,
    Question,
    Bookmark,
    Constant,
    Comment,
    End,
}

pub struct Line {
    pub text: String,
    pub type_: LineType,
}

pub struct Reader {
    pub source: String,
    pub lines: Vec<Line>,
}

impl Line {
    pub const fn new(text: String) -> Self {
        Self {
            text,
            type_: LineType::Undefined,
        }
    }
}

impl Reader {
    pub fn from_text(source: &str) -> Self {
        Self {
            source: String::from(source),
            lines: Vec::new(),
        }
    }

    pub fn parse_all_lines(&mut self) {
        self.split_lines();
        self.check_lines_type();
    }

    fn split_lines(&mut self) {
        // Split each string by newline
        let lines: Vec<&str> = self.source.lines().collect();

        // Add lines to the list
        for line in lines {
            // Skips empty lines
            if !line.is_empty() {
                // Remove empty characters from the start of the string
                let string_without_spaces = line.trim_start();
                let string = Line::new(String::from(string_without_spaces));
                self.lines.push(string);
            }
        }
    }

    fn check_lines_type(&mut self) {
        for line in &mut self.lines {
            // If the line is empty it exits (but it shouldn't happen)
            if line.text.is_empty() {
                line.type_ = LineType::Undefined;
                return;
            }

            let char_ = if let Some(c) = line.text.as_bytes().get(0) {
                c
            } else {
                line.type_ = LineType::Undefined;
                return;
            };

            match char_ {
                b'a'..=b'z' | b'A'..=b'Z' | 0..=9 | b'"' | b'.' | b'\'' => {
                    if line.text.starts_with("CONST") {
                        line.type_ = LineType::Constant;
                    } else {
                        line.type_ = LineType::Text;
                    }
                }
                b'+' => line.type_ = LineType::Question,
                b'=' => line.type_ = LineType::Bookmark,
                b'/' => line.type_ = LineType::Comment,
                // TODO: Must check between END and JUMP
                b'-' => {
                    let re_text = Regex::new(r"\s?+->\s+?END").unwrap();
                    if re_text.is_match(&line.text) {
                        line.type_ = LineType::End;
                    } else {
                        line.type_ = LineType::Text;
                    }
                }
                _ => line.type_ = LineType::Undefined,
            }
        }
    }
}
