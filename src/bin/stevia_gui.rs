use iui::prelude::*;
use iui::controls::*;
use std::path::{Path, PathBuf};
use stevia::reader::*;
use stevia::writer::*;
use stevia::epub_writer::*;
use std::fs::File;
use std::io::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
struct LogContext<'a> {
    ui: &'a UI,
    entry: MultilineEntry,
}

#[derive(PartialEq)]
enum ExportFormat {
    Stevia,
    Epub,
}

struct State<'a> {
    export_format: Option<ExportFormat>,
    title: String,
    author: String,
    cover: Option<&'a Path>,
}

impl<'a> State<'a> {
    fn update(&mut self) {}
}

macro_rules! evaluate_or_return {
    ($condition:ident, $ctx:ident, $success:expr, $fail:expr) => {
        match $condition {
            Ok(_) => {
                log($ctx, $success);
            }
            Err(fail) => {
                log($ctx, &format!("{}: {}", $fail, fail));
                return;
            }
        };
    };
}

macro_rules! unwrap_or_return {
    ($condition:ident, $ctx:ident, $success:expr, $fail:expr) => {
        match $condition {
            Ok(result) => {
                log($ctx, $success);
                result
            }
            Err(fail) => {
                log($ctx, &format!("{}: {}", $fail, fail));
                return;
            }
        };
    };
}

fn main() {
    // Wrapped with Interior Mutability Pattern
    // Because I need to pass the state around between UI controls
    let state: Rc<RefCell<State>> = Rc::new(RefCell::new(State {
        export_format: None,
        title: String::new(),
        author: String::new(),
        cover: None,
    }));

    let ui = UI::init().expect("Couldn't initialize UI library");
    let mut win = Window::new(&ui, "Stevia GUI", 380, 480, WindowType::NoMenubar);

    let multiline_entry = MultilineEntry::new(&ui);
    let mut log_ctx = LogContext {
        ui: &ui,
        entry: multiline_entry.clone(),
    };

    let mut export_grid = LayoutGrid::new(&ui);
    export_grid.set_padded(&ui, true);
    export_grid.hide(&ui);

    let (mut title_entry, mut author_entry, mut cover_entry_button) = {
        // Entries
        let title = Entry::new(&ui);
        let author = Entry::new(&ui);
        let cover = Button::new(&ui, "Please select the cover file");

        // Labels
        export_grid.append(
            &ui,
            Label::new(&ui, "Ebook title:"),
            0,
            0,
            1,
            1,
            GridExpand::Both,
            GridAlignment::Fill,
            GridAlignment::Fill,
        );

        export_grid.append(
            &ui,
            Label::new(&ui, "Ebook author:"),
            0,
            1,
            1,
            1,
            GridExpand::Both,
            GridAlignment::Fill,
            GridAlignment::Fill,
        );

        export_grid.append(
            &ui,
            Label::new(&ui, "Ebook cover:"),
            0,
            2,
            1,
            1,
            GridExpand::Both,
            GridAlignment::Fill,
            GridAlignment::Fill,
        );

        // Entries
        export_grid.append(
            &ui,
            title.clone(),
            1,
            0,
            1,
            1,
            GridExpand::Both,
            GridAlignment::Fill,
            GridAlignment::Fill,
        );

        export_grid.append(
            &ui,
            author.clone(),
            1,
            1,
            1,
            1,
            GridExpand::Both,
            GridAlignment::Fill,
            GridAlignment::Fill,
        );

        export_grid.append(
            &ui,
            cover.clone(),
            1,
            2,
            1,
            1,
            GridExpand::Both,
            GridAlignment::Fill,
            GridAlignment::Fill,
        );

        (title, author, cover)
    };

    let mut program_vbox = VerticalBox::new(&ui);
    program_vbox.set_padded(&ui, true);

    let mut button = Button::new(&ui, "Load Ink File");
    button.on_clicked(&ui, {
        let ui = ui.clone();
        let win = win.clone();
        let state = state.clone();
        move |_| {
            if state.borrow().export_format == None {
                win.modal_err(&ui, "Warning", "Please select an export file format");
                return;
            }

            let file = match win.open_file(&ui) {
                Some(file_) => file_,
                None => return,
            };


            process(&mut log_ctx, &state.borrow(), file);
        }
    });
    program_vbox.append(&ui, button, LayoutStrategy::Compact);

    let mut file_format_cb = Combobox::new(&ui);
    file_format_cb.append(&ui, "Select export file format");
    file_format_cb.append(&ui, "Stevia");
    file_format_cb.append(&ui, "ePub");
    file_format_cb.set_selected(&ui, 0);
    file_format_cb.clone().on_selected(&ui, {
        let ui = ui.clone();
        let mut export_grid = export_grid.clone();
        move |index| {
            // TODO: Must refactor
            match index {
                0 => state.borrow_mut().export_format = None,
                1 => state.borrow_mut().export_format = Some(ExportFormat::Stevia),
                2 => state.borrow_mut().export_format = Some(ExportFormat::Epub),
                _ => state.borrow_mut().export_format = None,
            }

            // If Epub is selected show export controls
            if index == 2 {
                export_grid.show(&ui);
            } else {
                export_grid.hide(&ui);
            }
        }
    });

    program_vbox.append(&ui, file_format_cb, LayoutStrategy::Compact);

    program_vbox.append(&ui, export_grid, LayoutStrategy::Compact);

    program_vbox.append(&ui, HorizontalSeparator::new(&ui), LayoutStrategy::Compact);
    program_vbox.append(&ui, multiline_entry, LayoutStrategy::Stretchy);

    win.set_child(&ui, program_vbox);
    win.show(&ui);
    ui.main();
}

fn process(ctx: &mut LogContext, state: &State, path: PathBuf) {
    clear_log(ctx);

    let file = File::open(path.clone());
    let mut file = unwrap_or_return!(file, ctx, "File loaded", "Cannot load the file");

    let mut contents = String::new();
    let file_contents = file.read_to_string(&mut contents);
    evaluate_or_return!(file_contents, ctx, "File read", "Cannot read the file");

    log(ctx, "Started parsing");

    let mut reader = Reader::from_text(&contents);
    reader.parse_all_lines();

    log(ctx, "Completed parsing");

    match state.export_format {
        None => (),
        Some(ExportFormat::Stevia) => {
            log(ctx, "Started exporting to Stevia");

            let mut writer = Writer::new();
            writer.process_lines(&reader);

            // TODO: Needs refactor urgently
            let file_name = path.file_stem().unwrap().to_str().unwrap();

            let file_create_result = File::create(format!("{}.stevia", &file_name));
            let mut file_output = unwrap_or_return!(
                file_create_result,
                ctx,
                "Created output file",
                "Cannot create the output file"
            );

            let file_write_result = file_output.write_all(writer.output.as_bytes());
            evaluate_or_return!(
                file_write_result,
                ctx,
                "Written to Stevia file",
                "Cannot write to Stevia file"
            );

            log(ctx, "Stevia exporting completed");
        }
        Some(ExportFormat::Epub) => {
            log(ctx, "Started exporting to ePub");

            // TODO: Needs refactor urgently
            let file_name = path.file_stem().unwrap().to_str().unwrap();

            log(ctx, "Started parsing");

            // TODO: Remove hardcoded values
            let mut epub_writer = EpubWriter::new("I love Rust", "Pomettini", "examples/cover.jpg");
            epub_writer.process_lines(&reader);

            let epub_writer_result = epub_writer.generate();
            let epub = match epub_writer_result {
                Some(contents) => {
                    log(ctx, "Completed parsing");
                    contents
                }
                None => {
                    log(ctx, "Cannot parse the Ink file");
                    return;
                }
            };

            let file_create_result = File::create(format!("{}.epub", file_name));
            let mut file_output = unwrap_or_return!(
                file_create_result,
                ctx,
                "Created output file",
                "Cannot create the output file"
            );

            let file_write_result = file_output.write_all(&epub);
            evaluate_or_return!(
                file_write_result,
                ctx,
                "Written to ePub file",
                "Cannot write to ePub file"
            );

            log(ctx, "ePub exporting completed");
        }
    }
}

fn log(ctx: &mut LogContext, message: &str) {
    let mut content = ctx.entry.value(&ctx.ui);
    content.push_str(&message);
    content.push_str("\n");
    ctx.entry.set_value(&ctx.ui, &content);
}

fn clear_log(ctx: &mut LogContext) {
    ctx.entry.set_value(&ctx.ui, "");
}
