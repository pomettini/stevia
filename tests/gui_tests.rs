extern crate stevia;

use stevia::gui::*;
use iui::prelude::*;
use iui::controls::*;
use std::process::Command;

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
fn test_stevia_functional() {
    SETUP_UI_MULTILINE!(ui, log_ctx, multiline_entry);

    let state = State {
        input_file: None,
        output_file: None,
        export_format: None,
        title: String::new(),
        author: String::new(),
        cover: None,
    };

    process(&mut log_ctx, &state);

    FREE!(multiline_entry);

    // TEST STUFF

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
        .expect("Clean command failed");
}
