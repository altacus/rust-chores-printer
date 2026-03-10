use gtk4 as gtk;
use gtk::{prelude::*, Application, ApplicationWindow, Button, Box as GtkBox, Orientation, ComboBoxText};
use std::cell::RefCell;
use std::rc::Rc;
use crate::chores;

/// Sets up the GTK application window and UI
pub fn setup_ui(app: &Application) {
    app.connect_activate(|app| {
        // Create a window
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Printer GUI")
            .default_width(300)
            .default_height(150)
            .build();

        // Create a vertical box container
        let vbox = GtkBox::new(Orientation::Vertical, 10);
        vbox.set_margin_top(20);
        vbox.set_margin_bottom(20);
        vbox.set_margin_start(20);
        vbox.set_margin_end(20);

        // dropdown selector for date (today/tomorrow)
        let use_tomorrow = Rc::new(RefCell::new(false));

        // Create Olivia button
        let olivia_button = Button::with_label("Olivia");
        {
            let flag = use_tomorrow.clone();
            olivia_button.connect_clicked(move |_| {
                let use_tom = *flag.borrow();
                if let Err(e) = chores::print_olivia(use_tom) {
                    eprintln!("Error printing for Olivia: {}", e);
                }
            });
        }
        vbox.append(&olivia_button);

        // Create Madelyn button
        let madelyn_button = Button::with_label("Madelyn");
        {
            let flag = use_tomorrow.clone();
            madelyn_button.connect_clicked(move |_| {
                let use_tom = *flag.borrow();
                if let Err(e) = chores::print_madelyn(use_tom) {
                    eprintln!("Error printing for Madelyn: {}", e);
                }
            });
        }
        vbox.append(&madelyn_button);

        // Date selector combo box
        let date_combo = ComboBoxText::new();
        date_combo.append_text("Today");
        date_combo.append_text("Tomorrow");
        date_combo.set_active(Some(0));
        {
            let flag = use_tomorrow.clone();
            date_combo.connect_changed(move |combo| {
                if let Some(text) = combo.active_text() {
                    *flag.borrow_mut() = text.as_str() == "Tomorrow";
                }
            });
        }
        vbox.append(&date_combo);

        window.set_child(Some(&vbox));
        window.present();
    });
}
