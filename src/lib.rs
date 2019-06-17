extern crate regex;
extern crate image;

// Priority:
// TODO: Document the format
// TODO: The GUI should stay in another crate
// Secondary:
// TODO: Add a way to load/manage/change backgrounds
// TODO: Add a way to test all the branches automatically
// TODO: Add a test executable (GGEZ?)
// TODO: Export the .h file for GBA
// TODO: Implement jumps
// TODO: Implement multi line comments

pub mod epub_writer;
pub mod gui;
pub mod reader;
pub mod writer;
