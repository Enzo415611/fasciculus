#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod events;

use crate::events::events::events;

slint::include_modules!();

fn main() -> anyhow::Result<()> {
    let main_window = MainWindow::new()?;
    let weak = main_window.as_weak();
    events(weak)?;
    main_window.run()?;

    Ok(())
}
