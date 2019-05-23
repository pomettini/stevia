#![cfg_attr(feature = "clippy", allow(clippy_pedantic))]

extern crate clap;
extern crate stevia;

use clap::*;
use std::fs::File;
use std::io::prelude::*;
use stevia::*;

fn main() {
    let matches = App::new("stevia")
        .version("0.1")
        .author("Giorgio Pomettini <giorgio.pomettini@gmail.com>")
        .arg(Arg::with_name("file").index(1).required(true))
        .get_matches();

    let path = matches.value_of("file").unwrap();

    let mut file = File::open(path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Cannot read the file");

    let mut reader = Reader::from_text(&contents);
    reader.parse_all_lines();

    let mut writer = Writer::new();
    writer.process_lines(&reader);

    print!("{}", writer.output);
}

mod tests {
    #[allow(unused_imports)]
    use std::process::Command;

    #[test]
    fn test_load_no_argument() {
        let output = Command::new("./target/debug/stevia").output().unwrap();

        assert!(output.stderr.len() > 0);
    }

    #[test]
    fn test_load_file() {
        let output = Command::new("./target/debug/stevia")
            .arg("examples/example.ink")
            .output()
            .unwrap();

        assert!(output.stdout.len() > 0);
    }

    #[test]
    fn test_load_non_existent_file() {
        let output = Command::new("./target/debug/stevia")
            .arg("examples/examples.ink")
            .output()
            .unwrap();

        assert!(output.stderr.len() > 0);
    }

    #[test]
    fn test_functional_process_file_green() {
        let output = Command::new("./target/debug/stevia")
            .arg("examples/example.ink")
            .output()
            .unwrap();

        let text_output = "P;Hello there|P;I'm a VN written in the Ink format|P;Do you like it?|Q;Yes, I like it!;00120;No, I do not like it;00136|P;Thank you!|E;|P;Oh, I see|E;";

        assert_eq!(String::from_utf8_lossy(&output.stdout), text_output);
    }
}
