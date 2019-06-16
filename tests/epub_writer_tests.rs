extern crate stevia;
#[macro_use]
extern crate maplit;

use stevia::epub_writer::*;
use stevia::reader::*;

#[allow(unused_macros)]
macro_rules! SETUP_WRITER {
    ($input:expr, $reader:ident, $writer:ident) => {
        let input = $input;
        let mut $reader = Reader::from_text(input);
        $reader.parse_all_lines();

        let mut $writer = EpubWriter::new("I love Rust", "Pomettini", "examples/cover.jpg");
        $writer.process_lines(&$reader);
    };
}

// --- TEXT ---

#[test]
fn test_writer_print_one() {
    SETUP_WRITER!("Hello world", reader, writer);

    assert_eq!(writer.page_content, vec!["<p>Hello world</p>"]);

    assert_eq!(reader.lines[0].type_, LineType::Text);
}

#[test]
fn test_writer_print_two() {
    SETUP_WRITER!(
        r#"Hello world
Ciao mondo"#,
        reader,
        writer
    );

    assert_eq!(
        writer.page_content,
        vec!["<p>Hello world</p><p>Ciao mondo</p>"]
    );

    assert_eq!(reader.lines[0].type_, LineType::Text);
    assert_eq!(reader.lines[1].type_, LineType::Text);
}

#[test]
fn test_writer_print_three() {
    SETUP_WRITER!(
        r#"Hello world
Ciao mondo
Bonjour monde"#,
        reader,
        writer
    );

    assert_eq!(
        writer.page_content,
        vec!["<p>Hello world</p><p>Ciao mondo</p><p>Bonjour monde</p>"]
    );

    assert_eq!(reader.lines[0].type_, LineType::Text);
    assert_eq!(reader.lines[1].type_, LineType::Text);
    assert_eq!(reader.lines[2].type_, LineType::Text);
}

// --- QUESTIONS ---

#[test]
fn test_writer_question_one() {
    SETUP_WRITER!(
        "+ [Hello world] -> example
+ [Ciao mondo] -> sample
=== example
Hello world
=== sample
Ciao mondo
",
        reader,
        writer
    );

    assert_eq!(writer.page_content, vec!["<p><a href=\"chapter_1.xhtml\">Hello world</a></p><p><a href=\"chapter_2.xhtml\">Ciao mondo</a></p>", "<p>Hello world</p>", "<p>Ciao mondo</p>"]);

    assert_eq!(writer.bookmark_table["example"], 1);
    assert_eq!(writer.bookmark_table["sample"], 2);

    assert_eq!(reader.lines[0].type_, LineType::Question);
    assert_eq!(reader.lines[1].type_, LineType::Question);
    assert_eq!(reader.lines[2].type_, LineType::Bookmark);
    assert_eq!(reader.lines[3].type_, LineType::Text);
    assert_eq!(reader.lines[4].type_, LineType::Bookmark);
    assert_eq!(reader.lines[5].type_, LineType::Text);
}

// --- END ---

#[test]
fn test_writer_end_one() {
    SETUP_WRITER!("-> END", reader, writer);

    assert_eq!(writer.page_content, vec![""]);

    assert_eq!(reader.lines[0].type_, LineType::End);
}

#[test]
fn test_writer_end_two() {
    SETUP_WRITER!(
        "Hello world
-> END",
        reader,
        writer
    );

    assert_eq!(writer.page_content, vec!["<p>Hello world</p>"]);

    assert_eq!(reader.lines[0].type_, LineType::Text);
    assert_eq!(reader.lines[1].type_, LineType::End);
}

// --- BOOKMARKS ---

// --- CONSTANTS ---

// --- COMMENTS ---

#[test]
fn test_writer_comment_one() {
    SETUP_WRITER!("// Hello world", reader, writer);

    assert_eq!(writer.page_content, vec![""]);

    assert_eq!(reader.lines[0].type_, LineType::Comment);
}

#[test]
fn test_writer_comment_two() {
    SETUP_WRITER!(
        "// Hello world
// Ciao mondo",
        reader,
        writer
    );

    assert_eq!(writer.page_content, vec![""]);

    assert_eq!(reader.lines[0].type_, LineType::Comment);
    assert_eq!(reader.lines[1].type_, LineType::Comment);
}

#[test]
fn test_writer_comment_and_text() {
    SETUP_WRITER!(
        "// Hello world
// Ciao mondo
Bonjour monde",
        reader,
        writer
    );

    assert_eq!(writer.page_content, vec!["<p>Bonjour monde</p>"]);

    assert_eq!(reader.lines[0].type_, LineType::Comment);
    assert_eq!(reader.lines[1].type_, LineType::Comment);
    assert_eq!(reader.lines[2].type_, LineType::Text);
}

// --- FUNCTIONAL TESTS ---

#[test]
fn functional_test_one() {
    SETUP_WRITER!(
        "Hello there

I'm a VN written in the Ink format

Do you like it?

-> END",
        reader,
        writer
    );

    assert_eq!(
        writer.page_content,
        vec!["<p>Hello there</p><p>I\'m a VN written in the Ink format</p><p>Do you like it?</p>"]
    );

    assert_eq!(reader.lines[0].type_, LineType::Text);
    assert_eq!(reader.lines[1].type_, LineType::Text);
    assert_eq!(reader.lines[2].type_, LineType::Text);
    assert_eq!(reader.lines[3].type_, LineType::End);
}

#[test]
fn functional_test_two() {
    SETUP_WRITER!(
        "Hello there

I'm a VN written in the Ink format

Do you like it?

+ [Yes, I like it!] -> like
+ [No, I do not like it] -> hate

=== like

Thank you!

-> END

=== hate

Oh, I see

-> END",
        reader,
        writer
    );

    assert_eq!(
        writer.page_content,
        vec!["<p>Hello there</p><p>I\'m a VN written in the Ink format</p><p>Do you like it?</p><p><a href=\"chapter_1.xhtml\">Yes, I like it!</a></p><p><a href=\"chapter_2.xhtml\">No, I do not like it</a></p>", "<p>Thank you!</p>", "<p>Oh, I see</p>"]
    );

    assert_eq!(
        writer.bookmark_table,
        hashmap! {
            String::from("like") => 1 as usize,
            String::from("hate") => 2 as usize,
        }
    );

    // TODO: Refactor this
    assert_eq!(reader.lines[0].type_, LineType::Text);
    assert_eq!(reader.lines[1].type_, LineType::Text);
    assert_eq!(reader.lines[2].type_, LineType::Text);
    assert_eq!(reader.lines[3].type_, LineType::Question);
    assert_eq!(reader.lines[4].type_, LineType::Question);
    assert_eq!(reader.lines[5].type_, LineType::Bookmark);
    assert_eq!(reader.lines[6].type_, LineType::Text);
    assert_eq!(reader.lines[7].type_, LineType::End);
    assert_eq!(reader.lines[8].type_, LineType::Bookmark);
    assert_eq!(reader.lines[9].type_, LineType::Text);
    assert_eq!(reader.lines[10].type_, LineType::End);
}
