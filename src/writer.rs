use regex::Regex;
use std::collections::*;
use crate::reader::*;

#[derive(Default)]
pub struct Writer {
    pub index: usize,
    pub output: String,
    pub symbols: HashMap<String, usize>,
    pub branch_table: HashMap<String, Vec<usize>>,
    pub constants: HashMap<String, String>,
}

impl Writer {
    pub fn new() -> Self {
        Self {
            index: 0,
            output: String::new(),
            symbols: HashMap::new(),
            branch_table: HashMap::new(),
            constants: HashMap::new(),
        }
    }

    pub fn replace_branch_table(&mut self) {
        // TODO: Needs refactor
        for symbol in &self.symbols {
            if self.branch_table.contains_key::<str>(&symbol.0) {
                for jump_place in self.branch_table.get::<str>(&symbol.0).unwrap() {
                    // The jump should have leading zeros and have a length of five
                    // Example: 123 -> 00123
                    let text_to_replace = &format!("{:05}", symbol.1);
                    let start = jump_place;
                    let end = start + 5;

                    self.output.replace_range(start..&end, text_to_replace);
                }
            }
        }
    }

    pub fn process_lines(&mut self, input: &Reader) {
        let mut current_line: usize = 0;
        let mut last_line_type = &LineType::Undefined;

        for line in &input.lines {
            match line.type_ {
                LineType::Undefined => panic!(format!("Line {} cannot be parsed", &current_line)),
                LineType::Text => {
                    let re_key = Regex::new(r"\{(?P<key>.*?)\}").unwrap();

                    // If text has variables inside
                    if re_key.is_match(&line.text) {
                        // Copy the buffer
                        let mut output = line.text.clone();

                        for caps in re_key.captures_iter(&line.text) {
                            // I replace the content of the variable
                            // With the value on the constant table
                            let key = caps.get(1).unwrap().as_str();
                            output = output.replace(&format!("{{{}}}", key), &self.constants[key]);
                        }

                        // Push the buffer to the output
                        self.push_to_output(&format!("P;{}", &output));
                    } else {
                        // If has no variables inside
                        self.push_to_output(&format!("P;{}", line.text));
                    }
                }
                LineType::Question => {
                    // Check between brackets
                    let re_text = Regex::new(r"\[(.*?)\]")
                        .unwrap()
                        .captures(&line.text)
                        .unwrap_or_else(|| {
                            panic!(
                                "Cannot get key of question while parsing at line {}",
                                &current_line
                            )
                        });

                    // Check after arrow
                    let re_jump = Regex::new(r"\->\s+(.*)$")
                        .unwrap()
                        .captures(&line.text)
                        .unwrap_or_else(|| {
                            panic!(
                                "Cannot get value of question while parsing at line {}",
                                &current_line
                            )
                        });

                    let mut jump_pos_offset = 0;

                    // Q; prefix offset
                    if last_line_type != &LineType::Question {
                        self.push_to_output("Q;");
                    }

                    // Add question text offset
                    jump_pos_offset += &re_text[1].len() + 1;

                    // Add offset to current index
                    self.index += jump_pos_offset;

                    // If jump place key is empty, add an empty vector inside
                    self.branch_table
                        .entry(re_jump[1].to_string())
                        .or_insert_with(Vec::new);

                    // Add jump place to that vector
                    let mut indices = self.branch_table[&re_jump[1].to_string()].clone();
                    indices.push(self.index);

                    // Add to jump places
                    self.branch_table.insert(re_jump[1].to_string(), indices);

                    // Add to output (must have 5 numbers)
                    self.output.push_str(&format!("{};{:05}", &re_text[1], 0));

                    // Jump index offset
                    self.index += 5;
                }
                LineType::Bookmark => {
                    // Remove equal characters and white spaces
                    let chars_to_trim: &[char] = &['=', ' '];

                    // Add the new string to the symbols
                    let trimmed_string: &str = line.text.trim_matches(chars_to_trim);

                    self.symbols.insert(trimmed_string.to_string(), self.index);
                }
                LineType::Constant => {
                    let re_key = Regex::new(r#" ((?:\\.|[^"\\])*) ="#)
                        .unwrap()
                        .captures(&line.text)
                        .unwrap_or_else(|| {
                            panic!(
                                "Cannot get key while parsing constant at line {}",
                                &current_line
                            )
                        });

                    let re_value = Regex::new(r#""((?:\\.|[^"\\])*)""#)
                        .unwrap()
                        .captures(&line.text)
                        .unwrap_or_else(|| {
                            panic!(
                                "Cannot get value while parsing constant at line {}",
                                &current_line
                            )
                        });

                    // Remove leading and trailing spaces
                    let const_name = re_key[1].trim().to_string();
                    let const_value = re_value[1].trim().to_string();

                    // Insert constant to table
                    self.constants.insert(const_name, const_value);
                }
                LineType::Comment => {}
                LineType::End => {
                    self.push_to_output("E;");
                }
            }

            last_line_type = &line.type_;

            current_line += 1;

            // If it's the last line, it exits the function
            if current_line >= input.lines.len() {
                return;
            }

            match line.type_ {
                LineType::Undefined => {
                    panic!(format!("Line {} cannot be parsed", &current_line - 1))
                }
                LineType::Text => {
                    self.push_to_output("|");
                }
                LineType::Question => {
                    if input.lines[current_line].type_ == LineType::Question {
                        self.push_to_output(";");
                    } else {
                        self.push_to_output("|");
                    }
                }
                LineType::End => {
                    self.push_to_output("|");
                }
                _ => (),
            }

            self.replace_branch_table();
        }
    }

    fn push_to_output(&mut self, text: &str) {
        // Add processed line to the output
        self.output.push_str(&text);
        // Increase current index
        self.index += &text.len();
    }
}
