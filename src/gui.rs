use iui::prelude::*;
use iui::controls::*;
use std::path::{Path, PathBuf};
use crate::reader::*;
use crate::writer::*;
use crate::epub_writer::*;
use std::fs::File;
use std::io::prelude::*;

macro_rules! evaluate_or_return {
    ($condition:ident, $ctx:ident, $success:expr, $fail:expr) => {
        match $condition {
            Ok(_) => {
                log($ctx, $success);
            }
            Err(fail) => {
                log($ctx, &format!("{}: {}", $fail, fail));
                return Err(());
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
                return Err(());
            }
        };
    };
}

#[derive(Clone)]
pub struct LogContext<'a> {
    pub ui: &'a UI,
    pub entry: MultilineEntry,
}

#[derive(PartialEq)]
pub enum ExportFormat {
    Stevia,
    Epub,
}

pub struct State<'a> {
    pub input_file: Option<PathBuf>,
    pub output_file: Option<PathBuf>,
    pub export_format: Option<ExportFormat>,
    pub title: String,
    pub author: String,
    pub cover: Option<&'a Path>,
}

impl<'a> State<'a> {
    pub fn update(
        &mut self,
        ui: &UI,
        title: &Entry,
        author: &Entry,
        cover: Option<&'a Path>,
        output_file: Option<PathBuf>,
    ) {
        self.title = title.value(ui);
        self.author = author.value(ui);
        self.cover = cover;
        self.output_file = output_file;
    }
}

pub fn process(ctx: &mut LogContext, state: &State) -> Result<(), ()> {
    clear_log(ctx);

    let input_file = match &state.input_file {
        Some(path) => path,
        None => return Err(()),
    };

    let output_file = match &state.output_file {
        Some(path) => path,
        None => return Err(()),
    };

    let file = File::open(input_file);
    let mut file = unwrap_or_return!(file, ctx, "File loaded", "Cannot load the file");

    let mut contents = String::new();
    let file_contents = file.read_to_string(&mut contents);
    evaluate_or_return!(file_contents, ctx, "File read", "Cannot read the file");

    log(ctx, "Started parsing");

    let mut reader = Reader::from_text(&contents);
    reader.parse_all_lines();

    log(ctx, "Completed parsing");

    match state.export_format {
        None => return Err(()),
        Some(ExportFormat::Stevia) => {
            log(ctx, "Started exporting to Stevia");

            let mut writer = Writer::new();
            writer.process_lines(&reader);

            // The output file path needs to have .stevia as file extension
            let file_create_result = File::create(output_file.with_extension("stevia"));
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

            let cover_path;

            if state.title.is_empty() {
                log(ctx, "Please enter the title");
                return Err(());
            }

            if state.author.is_empty() {
                log(ctx, "Please enter the author");
                return Err(());
            }

            if state.cover.is_none() {
                log(ctx, "No cover set, using the default one");
                cover_path = Path::new("examples/cover.jpg");
            } else {
                cover_path = state.cover.unwrap();
            }

            log(ctx, "Started parsing");

            let mut epub_writer = EpubWriter::new(&state.title, &state.author, cover_path);
            epub_writer.process_lines(&reader);

            let epub_writer_result = epub_writer.generate();
            let epub = if let Some(contents) = epub_writer_result {
                log(ctx, "Completed parsing");
                contents
            } else {
                log(ctx, "Cannot parse the Ink file");
                return Err(());
            };

            // The output file path needs to have .epub as file extension
            let file_create_result = File::create(output_file.with_extension("epub"));
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

    Ok(())
}

pub fn log(ctx: &mut LogContext, message: &str) {
    let mut content = ctx.entry.value(ctx.ui);
    content.push_str(message);
    content.push_str("\n");
    ctx.entry.set_value(ctx.ui, &content);
}

pub fn clear_log(ctx: &mut LogContext) {
    ctx.entry.set_value(ctx.ui, "");
}

pub fn export_grid_init(ui: &UI, grid: &mut LayoutGrid) -> (Entry, Entry, Button) {
    // Entries
    let title = Entry::new(ui);
    let author = Entry::new(ui);
    let cover = Button::new(ui, "Please select the cover file");

    // Labels
    grid.append(
        ui,
        Label::new(ui, "Ebook title:"),
        0,
        0,
        1,
        1,
        GridExpand::Both,
        GridAlignment::Fill,
        GridAlignment::Fill,
    );

    grid.append(
        ui,
        Label::new(ui, "Ebook author:"),
        0,
        1,
        1,
        1,
        GridExpand::Both,
        GridAlignment::Fill,
        GridAlignment::Fill,
    );

    grid.append(
        ui,
        Label::new(ui, "Ebook cover:"),
        0,
        2,
        1,
        1,
        GridExpand::Both,
        GridAlignment::Fill,
        GridAlignment::Fill,
    );

    // Entries
    grid.append(
        ui,
        title.clone(),
        1,
        0,
        1,
        1,
        GridExpand::Both,
        GridAlignment::Fill,
        GridAlignment::Fill,
    );

    grid.append(
        ui,
        author.clone(),
        1,
        1,
        1,
        1,
        GridExpand::Both,
        GridAlignment::Fill,
        GridAlignment::Fill,
    );

    grid.append(
        ui,
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
}
