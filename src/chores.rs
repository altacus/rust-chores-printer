use recibo::{Alignment, NetworkDriver, Printer};
use chrono::{Datelike, Local, Duration};
use std::error::Error;

/// Generates a formatted date string for the chore list
pub fn formatted_date(use_tomorrow: bool) -> String {
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

/// Prints a chore list for the specified child
pub fn print_chores(name: &str, use_tomorrow: bool) -> Result<(), Box<dyn Error>> {
    let driver = NetworkDriver::open("192.168.1.87", 9100)?;
    let mut printer = Printer::open(driver)?;

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

/// Prints Olivia's chore list
pub fn print_olivia(use_tomorrow: bool) -> Result<(), Box<dyn Error>> {
    print_chores("Olivia", use_tomorrow)
}

/// Prints Madelyn's chore list
pub fn print_madelyn(use_tomorrow: bool) -> Result<(), Box<dyn Error>> {
    print_chores("Madelyn", use_tomorrow)
}
