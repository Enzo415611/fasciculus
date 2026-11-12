#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod alpm;
mod events;
mod term;

use crate::{events::events::events, term::init_term};

slint::include_modules!();

fn main() -> anyhow::Result<()> {
    let main_window = MainWindow::new()?;
    let weak = main_window.as_weak();
    let pair = init_term()?;
    events(weak)?;
    main_window.run()?;
    Ok(())
}
