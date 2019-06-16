use iui::prelude::*;
use iui::controls::*;
use stevia::gui::*;
use std::cell::RefCell;
use std::rc::Rc;

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

    let (title_entry, author_entry, mut cover_entry_button) =
        export_grid_init(&ui, &mut export_grid);
    cover_entry_button.on_clicked(&ui, {
        move |_| {
            unimplemented!();
        }
    });

    let mut program_vbox = VerticalBox::new(&ui);
    program_vbox.set_padded(&ui, true);

    let mut button = Button::new(&ui, "Load Ink File");
    button.on_clicked(&ui, {
        let ui = ui.clone();
        let win = win.clone();
        let state = state.clone();
        let title_entry = title_entry.clone();
        let author_entry = author_entry.clone();
        move |_| {
            if state.borrow().export_format == None {
                win.modal_err(&ui, "Warning", "Please select an export file format");
                return;
            }

            let file = match win.open_file(&ui) {
                Some(file_) => file_,
                None => return,
            };

            state
                .borrow_mut()
                .update(&ui, &title_entry, &author_entry, None);

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
