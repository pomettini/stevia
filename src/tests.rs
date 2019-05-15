use super::*;

#[test]
fn test_parse_text_line_one() {
    let input = r#"Hello world"#;

    let mut reader = Reader::from_text(input);
    reader.parse_all_lines();

    assert_eq!(reader.lines.len(), 1);

    assert_eq!(reader.lines[0].text, "Hello world");
    assert_eq!(reader.lines[0].type_, LineType::Text);
}

#[test]
fn test_parse_text_line_two() {
    let input = r#"Hello world
Ciao mondo"#;

    let mut reader = Reader::from_text(input);
    reader.parse_all_lines();

    assert_eq!(reader.lines.len(), 2);

    assert_eq!(reader.lines[0].text, "Hello world");
    assert_eq!(reader.lines[1].text, "Ciao mondo");

    assert_eq!(reader.lines[0].type_, LineType::Text);
    assert_eq!(reader.lines[1].type_, LineType::Text);
}

#[test]
fn test_parse_text_line_three() {
    let input = r#"Hello world
Ciao mondo
Bonjour monde"#;

    let mut reader = Reader::from_text(input);
    reader.parse_all_lines();

    assert_eq!(reader.lines.len(), 3);

    assert_eq!(reader.lines[0].text, "Hello world");
    assert_eq!(reader.lines[1].text, "Ciao mondo");
    assert_eq!(reader.lines[2].text, "Bonjour monde");

    assert_eq!(reader.lines[0].type_, LineType::Text);
    assert_eq!(reader.lines[1].type_, LineType::Text);
    assert_eq!(reader.lines[2].type_, LineType::Text);
}

#[test]
fn test_parse_empty_line() {
    let input = r#"
    "#;

    let mut reader = Reader::from_text(input);
    reader.parse_all_lines();

    assert_eq!(reader.lines.len(), 2);

    assert_eq!(reader.lines[0].type_, LineType::Empty);
}

// #[test]
// fn test_undefined() {
//     let output = "A;";
//     assert_eq!("", output);
// }

// #[test]
// fn test_print() {
//     let output = "P;Hello There";
//     assert_eq!("", output);
// }

// #[test]
// fn test_question_one() {
//     let output = "Q;Yes, I like it!;00120";
//     assert_eq!("", output);
// }

// #[test]
// fn test_question_two() {
//     let output = "Q;Yes, I like it!;00120;No, I do not like it;00136";
//     assert_eq!("", output);
// }

// #[test]
// fn test_question_missing_jump() {
//     let output = "Q;Yes, I like it!";
//     assert_eq!("", output);
// }

// #[test]
// fn test_jump() {
//     let output = "J;00001";
//     assert_eq!("", output);
// }

// #[test]
// fn test_end() {
//     let output = "E;";
//     assert_eq!("", output);
// }
