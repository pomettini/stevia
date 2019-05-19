extern crate clap;
extern crate stevia;

use clap::*;
use stevia::*;

fn main() {
    let matches = App::new("stevia")
        .version("0.1")
        .author("Giorgio Pomettini <giorgio.pomettini@gmail.com>")
        .arg(Arg::with_name("file").index(1).required(true))
        .get_matches();

    let file = matches.value_of("file").unwrap();

    let text = "ciao";

    let mut reader = Reader::from_text(text);
    reader.parse_all_lines();

    let mut writer = Writer::new();
    writer.process_lines(&reader);
}
