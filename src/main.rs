use std::env;
use std::fs::*;
use std::io::*;
use std::path::*;

enum Token {
    Text,
    Question,
    Jump,
    End,
}

#[derive(Default)]
struct VNContent {
    lines: Vec<String>,
    bookmarks: Vec<u8>,
}

#[derive(Default)]
struct VNLexer {
    current_line_index: usize,
    current_line_buf: String,
}

impl VNLexer {
    fn next(&mut self, line: &str) {
        println!("{}", line);
    }
}

fn main() {
    let mut content: VNContent = Default::default();
    let mut lexer: VNLexer = Default::default();

    let input = File::open(Path::new("tests/input_example.txt")).unwrap();
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        let l = line.unwrap();
        lexer.next(&l);
    }
}
