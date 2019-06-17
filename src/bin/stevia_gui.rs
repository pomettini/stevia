use iui::prelude::*;
use iui::controls::*;
use stevia::gui::*;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // Wrapped with Interior Mutability Pattern
    // Because I need to pass the state around between UI controls
    let state: Rc<RefCell<State>> = Rc::new(RefCell::new(State {
        input_file: None,
        output_file: None,
        export_format: None,
        title: String::new(),
        author: String::new(),
        cover: None,
    }));

    let ui = UI::init().expect("Couldn't initialize UI library");
    let mut window = Window::new(&ui, "Stevia GUI", 380, 480, WindowType::NoMenubar);

    let multiline_entry = MultilineEntry::new(&ui);
    let mut log_ctx = LogContext {
        ui: &ui,
        entry: multiline_entry.clone(),
    };

    let mut export_grid = LayoutGrid::new(&ui);
    export_grid.set_padded(&ui, true);
    export_grid.hide(&ui);

    let (title_entry, author_entry, mut cover_entry_button) =
        export_grid_init(&ui, &mut export_grid);
    cover_entry_button.on_clicked(&ui, {
        let ui = ui.clone();
        move |_| {
            unimplemented!();
        }
    });

    let mut program_vbox = VerticalBox::new(&ui);
    program_vbox.set_padded(&ui, true);

    let mut generate_button = Button::new(&ui, "Generate");
    generate_button.on_clicked(&ui, {
        let ui = ui.clone();
        let window = window.clone();
        let state = state.clone();
        move |_| {
            // You need to select an export format before exporting
            if state.borrow().export_format == None {
                window.modal_err(&ui, "Warning", "Please select an export file format");
                return;
            }

            // Ask the user for output file path
            let save_file_path = window.save_file(&ui);

            // Passes all the information to the global state
            state
                .borrow_mut()
                .update(&ui, &title_entry, &author_entry, None, save_file_path);

            // Generate the output file
            process(&mut log_ctx, &state.borrow());
        }
    });
    generate_button.hide(&ui);

    let mut load_file_button = Button::new(&ui, "Load Ink File");
    load_file_button.on_clicked(&ui, {
        let ui = ui.clone();
        let window = window.clone();
        let mut generate_button = generate_button.clone();
        let state = state.clone();
        move |button| {
            match window.open_file(&ui) {
                Some(file) => {
                    state.borrow_mut().input_file = Some(file.clone());
                    let file_name = file.file_name().unwrap().to_str().unwrap();
                    button.set_text(&ui, &format!("Loaded: {}", file_name));
                    // Show generate button only if file is loaded
                    generate_button.show(&ui);
                }
                None => {
                    return;
                }
            };
        }
    });
    program_vbox.append(&ui, load_file_button, LayoutStrategy::Compact);

    let mut file_format_cb = Combobox::new(&ui);
    file_format_cb.append(&ui, "Select export file format");
    file_format_cb.append(&ui, "Stevia");
    file_format_cb.append(&ui, "ePub");
    file_format_cb.set_selected(&ui, 0);
    file_format_cb.clone().on_selected(&ui, {
        let ui = ui.clone();
        let mut export_grid = export_grid.clone();
        move |index| {
            // FIXME: Must refactor
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
    program_vbox.append(&ui, generate_button, LayoutStrategy::Compact);

    program_vbox.append(&ui, HorizontalSeparator::new(&ui), LayoutStrategy::Compact);

    program_vbox.append(&ui, multiline_entry, LayoutStrategy::Stretchy);

    window.set_child(&ui, program_vbox);
    window.show(&ui);
    ui.main();
}
