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
        .text(&format!("{}'s Chores", name))?
        .text(&date_str)?
        .text_size(1, 1)? // Normal text size
        .text(" Morning")?
        .text("  __ Stayed in bed?")?
        .text("  __ Make Bed")?
        .text("  __ Brush Teeth")?
        .text("  __ Clean up Breakfast")?
        .feed(2)?
        .text(" After School")?
        .text("  __ Wash Hands")?
        .text("  __ Unpack backpacks")?
        .text("  __ Hang up coats & put away shoes")?
        .text("  __ Put clothes in hamper")?
        .text("  __ Take Shower/Bath")?
        .text("  __ Put away shoes/coat")?
        .text("  __ Do homework")?
        .text("  __ Play Piano?")?
        .feed(2)?
        .text(" Night time")?
        .text("  __ Set table")?
        .text("  __ Clean up dinner")?
        .text("  __ Vacuum/Mop")?
        .text("  __ Floss Teeth")?
        .text("  __ Brush Teeth")?
        .feed(8)?
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
