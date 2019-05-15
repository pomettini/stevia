use std::collections::*;

pub mod tests;

#[derive(Debug, PartialEq)]
pub enum LineType {
    Undefined,
    Text,
    Question,
    Bookmark,
    End,
}

pub struct Line {
    pub text: String,
    pub type_: LineType,
}

pub struct Question {
    pub text: String,
    pub jump_pos: usize,
}

pub struct Reader {
    pub source: String,
    pub lines: Vec<Line>,
}

pub struct Writer {
    pub index: usize,
    pub output: String,
    pub bookmarks: HashMap<String, usize>,
}

impl Line {
    pub fn new(text: String) -> Line {
        Line {
            text: text,
            type_: LineType::Undefined,
        }
    }
}

impl Question {
    pub fn new(text: String) -> Question {
        Question {
            text: String::new(),
            jump_pos: 0,
        }
    }
}

impl Reader {
    pub fn from_text(source: &str) -> Reader {
        Reader {
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
            if line.len() > 0 {
                let l = Line::new(String::from(line));
                self.lines.push(l);
            }
        }
    }

    fn check_lines_type(&mut self) {
        for line in &mut self.lines {
            // If the line is empty exits (but it shouldn't happen)
            if line.text.len() == 0 {
                line.type_ = LineType::Undefined;
                return;
            }

            let first_char = line.text.as_bytes().get(0).unwrap();

            match first_char {
                b'a'...b'z' | b'A'...b'Z' | 0...9 => line.type_ = LineType::Text,
                b'+' => line.type_ = LineType::Question,
                // TODO: Must check that there are three equals
                b'=' => line.type_ = LineType::Bookmark,
                // TODO: Must check between END and JUMP
                b'-' => line.type_ = LineType::End,
                _ => line.type_ = LineType::Undefined,
            }
        }
    }
}

impl Writer {
    pub fn new() -> Writer {
        Writer {
            index: 0,
            output: String::new(),
            bookmarks: HashMap::new(),
        }
    }

    pub fn process_lines(&mut self, input: &Reader) {
        let mut current_line: usize = 0;

        for line in &input.lines {
            match line.type_ {
                LineType::Undefined => break,
                LineType::Text => self.output.push_str(&format!("P;{}", line.text)),
                LineType::Bookmark => {
                    // Remove equal and white spaces
                    let chars_to_trim: &[char] = &['=', ' '];
                    // Add the new string to the bookmarks
                    let trimmed_string: &str = line.text.trim_matches(chars_to_trim);
                    self.bookmarks
                        .insert(trimmed_string.to_string(), self.index);
                }
                LineType::End => self.output.push_str(&format!("E;")),
                _ => break,
            }

            // Index should add 3 at the end because
            // The first two characters are for the prefix
            // And the last one for the suffix
            if line.type_ != LineType::Bookmark {
                self.index += line.text.len() + 3;
            }

            current_line += 1;

            // Add separator until it's the last line
            if current_line < input.lines.len() && line.type_ != LineType::Bookmark {
                self.output.push_str("|");
            }
        }
    }
}

fn main() {
    let source = "Hello";

    let mut reader = Reader::from_text(source);
    let mut writer = Writer::new();

    // print!("{:?}", context.source);
}
