mod chores;
mod ui;

use gtk4::Application;
use gtk4::prelude::ApplicationExtManual;

fn main() {
    let app = Application::builder()
        .application_id("com.example.printer-gui")
        .build();

    ui::setup_ui(&app);
    app.run();
}
