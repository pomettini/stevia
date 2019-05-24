#![cfg_attr(feature = "clippy", allow(clippy_pedantic))]

extern crate clap;
extern crate stevia;

use clap::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use stevia::*;

fn main() {
    let matches = App::new("stevia")
        .version("0.1")
        .author("Giorgio Pomettini <giorgio.pomettini@gmail.com>")
        .arg(Arg::with_name("file").index(1).required(true))
        .get_matches();

    let path = Path::new(matches.value_of("file").unwrap());

    let mut file = File::open(path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Cannot read the file");

    let mut reader = Reader::from_text(&contents);
    reader.parse_all_lines();

    let mut writer = Writer::new();
    writer.process_lines(&reader);

    // TODO: Needs refactor urgently
    let mut output_file = File::create(format!(
        "{}.stevia",
        path.file_stem().unwrap().to_str().unwrap()
    ))
    .expect("Cannot create output file");

    output_file
        .write_all(&writer.output.as_bytes())
        .expect("Cannot write file content");
}

mod tests {
    #[allow(unused_imports)]
    use std::process::Command;

    #[test]
    fn test_load_no_argument() {
        clean();

        let output = Command::new("./target/debug/stevia").output().unwrap();

        assert!(output.stderr.len() > 0);

        clean();
    }

    #[test]
    fn test_load_file() {
        clean();

        let output = Command::new("./target/debug/stevia")
            .arg("examples/example.ink")
            .output()
            .unwrap();

        assert!(output.stderr.len() == 0);

        clean();
    }

    #[test]
    fn test_load_non_existent_file() {
        clean();

        let output = Command::new("./target/debug/stevia")
            .arg("examples/examples.ink")
            .output()
            .unwrap();

        assert!(output.stderr.len() > 0);

        clean();
    }

    #[test]
    fn test_functional_process_file_green() {
        clean();

        let _output = Command::new("./target/debug/stevia")
            .arg("examples/example.ink")
            .output()
            .unwrap();

        let expected_output = "P;Hello there|P;I'm a VN written in the Ink format|P;Do you like it?|Q;Yes, I like it!;00120;No, I do not like it;00136|P;Thank you!|E;|P;Oh, I see|E;";

        let output = Command::new("cat")
            .arg("example.stevia")
            .output()
            .expect("Cannot find .stevia file");

        assert_eq!(String::from_utf8_lossy(&output.stdout), expected_output);

        clean();
    }

    #[allow(dead_code)]
    fn clean() {
        Command::new("find")
            .arg(".")
            .arg("-name")
            .arg("*.stevia")
            .arg("-delete")
            .output()
            .unwrap();

        // println!("status: {}", output.status);
        // println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        // println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }
}
