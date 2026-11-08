#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod alpm;
mod events;
use std::sync::{Arc, Mutex};

use ::alpm::Alpm;

use crate::{alpm::sync_alpm_dbs, events::events::events};

slint::include_modules!();

fn main() -> anyhow::Result<()> {
    let main_window = MainWindow::new()?;
    let weak = main_window.as_weak();
    let alpm = sync_alpm_dbs()?;
    events(weak, &alpm)?;
    main_window.run()?;
    Ok(())
}
