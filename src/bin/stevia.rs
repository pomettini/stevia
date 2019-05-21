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
}

pub mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_load_file() {
        unimplemented!();
    }

    #[test]
    #[should_panic]
    fn test_load_non_existent_file() {
        unimplemented!();
    }

    #[test]
    #[should_panic]
    fn test_load_broken_file() {
        unimplemented!();
    }

    #[test]
    fn test_functional_process_file() {
        unimplemented!();
    }

    #[test]
    #[should_panic]
    fn test_functional_process_broken_file() {
        unimplemented!();
    }
}
