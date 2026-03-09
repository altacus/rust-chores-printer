use recibo::{Alignment, NetworkDriver, Printer};
use std::error::Error;
use gtk4 as gtk;
use gtk::{prelude::*, Application, ApplicationWindow, Button, Box as GtkBox, Orientation, ComboBoxText};
use chrono::{Datelike, Local, Duration};
use std::cell::RefCell;
use std::rc::Rc;

// helper to compute formatted date string, either today or tomorrow
fn formatted_date(use_tomorrow: bool) -> String {
    let base = if use_tomorrow {
        Local::now() + Duration::days(1)
    } else {
        Local::now()
    };
    format!(" {} {} {:02} {} \n",
            base.format("%A"),
            base.format("%B"),
            base.day(),
            base.year())
}


// generic printer function for a named child
fn print_chores(name: &str, use_tomorrow: bool) -> Result<(), Box<dyn Error>> {
    let driver = NetworkDriver::open("192.168.1.87", 9100)?;
    let mut printer = Printer::open(driver)?;

    // formatted date according to selection
    let date_str = formatted_date(use_tomorrow);

    printer
        .init()? // Initializes the printer (clears settings)
        .align(Alignment::Left)? // Left align for checklist format
        .text_size(2, 2)? // Normal text size
        .text(&format!("{}'s Chores\n", name))?
        .text(&date_str)?
        .text_size(1, 1)? // Normal text size
        .text("\n Checklist\n")?
        .text("  __ Stayed in bed?\n")?
        .text("  __ Play Piano?\n")?
        .text("\n Morning\n")?
        .text("  __ Make Bed\n")?
        .text("  __ Brush Teeth\n")?
        .text("  __ Clean up Breakfast\n")?
        .text("\n After School\n")?
        .text("  __ Wash Hands\n")?
        .text("  __ Unpack backpacks\n")?
        .text("  __ Take Shower/Bath\n")?
        .text("  __ Put away shoes/coat\n")?
        .text("  __ Do homework\n")?
        .text("\n Night time\n")?
        .text("  __ Clean up dinner\n")?
        .text("  __ Floss Teeth\n")?
        .text("  __ Brush Teeth\n")?
        .text("\n@}~~^~~~~~\n\n")?
        .feed(6)?
        .cut()?; // Cuts the paper

    Ok(())
}

// convenience wrappers kept for backward compatibility if needed
fn print_olivia(use_tomorrow: bool) -> Result<(), Box<dyn Error>> {
    print_chores("Olivia", use_tomorrow)
}

fn print_madelyn(use_tomorrow: bool) -> Result<(), Box<dyn Error>> {
    print_chores("Madelyn", use_tomorrow)
}

fn main() {
    let app = Application::builder()
        .application_id("com.example.printer-gui")
        .build();

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
            if let Err(e) = print_olivia(use_tom) {
                eprintln!("Error printing for Olivia: {}", e);
                // You could show an error dialog here
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
            if let Err(e) = print_madelyn(use_tom) {
                eprintln!("Error printing for Madelyn: {}", e);
                // You could show an error dialog here
            }
        });
    }
    vbox.append(&madelyn_button);

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

app.run();
}
