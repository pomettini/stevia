extern crate stevia;

use stevia::gui::*;
use iui::prelude::*;
use iui::controls::*;
use std::process::Command;
use std::path::{Path, PathBuf};
use std::fs::*;

// Run with RUST_TEST_THREADS=1 cargo test

#[allow(unused_macros)]
macro_rules! SETUP_UI_MULTILINE {
    ($ui:ident, $log_ctx:ident, $multiline:ident) => {
        let $ui = UI::init().unwrap();
        let $multiline = MultilineEntry::new(&$ui);

        let mut $log_ctx = LogContext {
            ui: &$ui,
            entry: $multiline.clone(),
        };
    };
}

#[allow(unused_macros)]
macro_rules! FREE {
    ($control:expr) => {
        unsafe {
            // Memory needs to be released, otherwise it will panic
            Into::<Control>::into($control).destroy();
        }
    };
}

#[test]
fn test_log_green() {
    SETUP_UI_MULTILINE!(ui, log_ctx, multiline_entry);

    log(&mut log_ctx, "Hello");

    let value = multiline_entry.value(&ui);

    FREE!(multiline_entry);

    assert_eq!(value, "Hello\n");
}

#[test]
fn test_log_red() {
    SETUP_UI_MULTILINE!(ui, log_ctx, multiline_entry);

    log(&mut log_ctx, "Hello");

    let value = multiline_entry.value(&ui);

    FREE!(multiline_entry);

    assert_ne!(value, "");
}

#[test]
fn test_clear_log_green() {
    SETUP_UI_MULTILINE!(ui, log_ctx, multiline_entry);

    log(&mut log_ctx, "Hello");

    clear_log(&mut log_ctx);

    let value = multiline_entry.value(&ui);

    FREE!(multiline_entry);

    assert_eq!(value, "");
}

#[test]
fn test_clear_log_red() {
    SETUP_UI_MULTILINE!(ui, log_ctx, multiline_entry);

    log(&mut log_ctx, "Hello");

    clear_log(&mut log_ctx);

    let value = multiline_entry.value(&ui);

    FREE!(multiline_entry);

    assert_ne!(value, "Hello");
}

#[test]
fn test_functional_no_export_format() {
    SETUP_UI_MULTILINE!(ui, log_ctx, multiline_entry);

    let state = State {
        input_file: Some(PathBuf::from(r"examples/example.ink")),
        output_file: Some(PathBuf::from(r"examples/example.stevia")),
        export_format: None,
        title: String::from("Hello world"),
        author: String::from("Pomettini"),
        cover: None,
    };

    process(&mut log_ctx, &state);

    FREE!(multiline_entry);

    assert_eq!(Path::new("examples/example.stevia").exists(), false);

    clean();
}

#[test]
fn test_stevia_functional_correct() {
    SETUP_UI_MULTILINE!(ui, log_ctx, multiline_entry);

    let state = State {
        input_file: Some(PathBuf::from(r"examples/example.ink")),
        output_file: Some(PathBuf::from(r"examples/example.stevia")),
        export_format: Some(ExportFormat::Stevia),
        title: String::from("Hello world"),
        author: String::from("Pomettini"),
        cover: None,
    };

    process(&mut log_ctx, &state);

    FREE!(multiline_entry);

    let expected = "P;Hello there|P;I'm a VN written in the Ink format|P;Do you like it?|Q;Yes, I like it!;00120;No, I do not like it;00136|P;Thank you!|E;|P;Oh, I see|E;";
    let contents = read_to_string("examples/example.stevia").expect("Cannot find .stevia file");

    assert_eq!(contents, expected);

    clean();
}

#[test]
fn test_stevia_functional_no_input_file() {
    SETUP_UI_MULTILINE!(ui, log_ctx, multiline_entry);

    let state = State {
        input_file: None,
        output_file: Some(PathBuf::from(r"examples/example.stevia")),
        export_format: Some(ExportFormat::Stevia),
        title: String::from("Hello world"),
        author: String::from("Pomettini"),
        cover: None,
    };

    process(&mut log_ctx, &state);

    FREE!(multiline_entry);

    assert_eq!(Path::new("examples/example.stevia").exists(), false);

    clean();
}

#[test]
fn test_functional_epub_correct() {
    SETUP_UI_MULTILINE!(ui, log_ctx, multiline_entry);

    let state = State {
        input_file: Some(PathBuf::from(r"examples/example.ink")),
        output_file: Some(PathBuf::from(r"examples/example.epub")),
        export_format: Some(ExportFormat::Epub),
        title: String::from("Hello world"),
        author: String::from("Pomettini"),
        cover: None,
    };

    process(&mut log_ctx, &state);

    FREE!(multiline_entry);

    assert_eq!(Path::new("examples/example.epub").exists(), true);

    clean();
}

#[test]
fn test_functional_epub_no_input_file() {
    SETUP_UI_MULTILINE!(ui, log_ctx, multiline_entry);

    let state = State {
        input_file: None,
        output_file: Some(PathBuf::from(r"examples/example.epub")),
        export_format: Some(ExportFormat::Epub),
        title: String::from("Hello world"),
        author: String::from("Pomettini"),
        cover: None,
    };

    process(&mut log_ctx, &state);

    FREE!(multiline_entry);

    assert_eq!(Path::new("examples/example.epub").exists(), false);

    clean();
}

#[test]
fn test_functional_epub_no_title() {
    SETUP_UI_MULTILINE!(ui, log_ctx, multiline_entry);

    let state = State {
        input_file: Some(PathBuf::from(r"examples/example.ink")),
        output_file: Some(PathBuf::from(r"examples/example.epub")),
        export_format: Some(ExportFormat::Epub),
        title: String::from(""),
        author: String::from("Pomettini"),
        cover: None,
    };

    process(&mut log_ctx, &state);

    FREE!(multiline_entry);

    assert_eq!(Path::new("examples/example.epub").exists(), false);

    clean();
}

#[test]
fn test_functional_epub_no_author() {
    SETUP_UI_MULTILINE!(ui, log_ctx, multiline_entry);

    let state = State {
        input_file: Some(PathBuf::from(r"examples/example.ink")),
        output_file: Some(PathBuf::from(r"examples/example.epub")),
        export_format: Some(ExportFormat::Epub),
        title: String::from("Hello world"),
        author: String::from(""),
        cover: None,
    };

    process(&mut log_ctx, &state);

    FREE!(multiline_entry);

    assert_eq!(Path::new("examples/example.epub").exists(), false);

    clean();
}

#[allow(dead_code)]
fn clean() {
    // TODO: Merge clean commands
    Command::new("find")
        .arg(".")
        .arg("-name")
        .arg("*.stevia")
        .arg("-delete")
        .output()
        .expect("Clean command failed");

    Command::new("find")
        .arg(".")
        .arg("-name")
        .arg("*.epub")
        .arg("-delete")
        .output()
        .expect("Clean command failed");
}
