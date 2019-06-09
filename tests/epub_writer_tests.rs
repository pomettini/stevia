extern crate stevia;

use stevia::epub_writer::*;
use stevia::reader::*;

#[allow(unused_macros)]
macro_rules! SETUP_WRITER {
    ($input:expr, $reader:ident, $writer:ident) => {
        let input = $input;
        let mut $reader = Reader::from_text(input);
        $reader.parse_all_lines();

        // let mut $writer = Writer::new();
        // $writer.process_lines(&$reader);
    };
}
