extern crate regex;

use regex::Regex;
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

#[derive(Default)]
pub struct Writer {
    pub index: usize,
    pub output: String,
    // TODO: Refactor to use better terms like symbols
    pub bookmarks: HashMap<String, usize>,
    pub jump_places: HashMap<String, Vec<usize>>,
}

impl Line {
    pub fn new(text: String) -> Line {
        Line {
            text,
            type_: LineType::Undefined,
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
            if !line.is_empty() {
                let l = Line::new(String::from(line));
                self.lines.push(l);
            }
        }
    }

    fn check_lines_type(&mut self) {
        for line in &mut self.lines {
            // If the line is empty exits (but it shouldn't happen)
            if line.text.is_empty() {
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
            jump_places: HashMap::new(),
        }
    }

    pub fn replace_jump_places(&mut self) {
        // TODO: Needs refactor
        for bookmark in &self.bookmarks {
            if !self.jump_places.contains_key::<str>(&bookmark.0) {
                return;
            }

            for jump_place in self.jump_places.get::<str>(&bookmark.0).unwrap() {
                let text_to_replace = &format!("{:05}", bookmark.1);
                let start = jump_place;
                let end = start + 5;

                self.output.replace_range(start..&end, text_to_replace);
            }
        }
    }

    pub fn process_lines(&mut self, input: &Reader) {
        let mut current_line: usize = 0;
        let mut last_line_type = &LineType::Undefined;

        for line in &input.lines {
            match line.type_ {
                LineType::Undefined => break,
                LineType::Text => {
                    self.output.push_str(&format!("P;{}", line.text));
                    self.index += line.text.len();
                    self.index += 2;
                }
                LineType::Question => {
                    // Check between brackets
                    let re_text = Regex::new(r"\[(.*?)\]")
                        .unwrap()
                        .captures(&line.text)
                        .unwrap();

                    // Check after arrow
                    let re_jump = Regex::new(r"\->\s+(.*)$")
                        .unwrap()
                        .captures(&line.text)
                        .unwrap();

                    let mut jump_pos_offset = 0;

                    // Q; prefix offset
                    if last_line_type != &LineType::Question {
                        self.output.push_str("Q;");
                        jump_pos_offset += 2;
                    }

                    // Add question text offset
                    // TODO: Not sure why I need that +1 to the offset
                    jump_pos_offset += &re_text[1].len() + 1;

                    // Add offset to current index
                    self.index += jump_pos_offset;

                    // TODO: Refactor this?
                    // If jump place key is empty, add an empty vector inside
                    self.jump_places
                        .entry(re_jump[1].to_string())
                        .or_insert_with(Vec::new);

                    // Add jump place to that vector
                    let mut indices = self.jump_places[&re_jump[1].to_string()].clone();
                    indices.push(self.index);

                    // Add to jump places
                    self.jump_places.insert(re_jump[1].to_string(), indices);

                    // Add to output (must have 5 numbers)
                    self.output.push_str(&format!("{};{:05}", &re_text[1], 0));

                    // Jump index offset
                    self.index += 5;
                }
                LineType::Bookmark => {
                    // Remove equal and white spaces
                    let chars_to_trim: &[char] = &['=', ' '];

                    // Add the new string to the bookmarks
                    let trimmed_string: &str = line.text.trim_matches(chars_to_trim);

                    self.bookmarks
                        .insert(trimmed_string.to_string(), self.index);
                }
                LineType::End => {
                    self.output.push_str(&String::from("E;"));
                    self.index += 2;
                }
            }

            last_line_type = &line.type_;

            current_line += 1;

            // Add separator until it's the last line
            // TODO: Needs refactor
            if current_line < input.lines.len() && line.type_ != LineType::Bookmark {
                if line.type_ == LineType::Question
                    && input.lines[current_line].type_ != LineType::Question
                {
                    self.output.push_str("|");
                    self.index += 1;
                } else if line.type_ == LineType::Question {
                    self.output.push_str(";");
                    self.index += 1;
                } else {
                    self.output.push_str("|");
                    self.index += 1;
                }
            }
        }
    }
}

// fn main() {
//     let source = "Hello";

//     let mut reader = Reader::from_text(source);
//     let mut writer = Writer::new();

//     print!("{:?}", context.source);
// }
