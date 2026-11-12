use slint::Weak;

use crate::{MainWindow, events::ui_events::ui_events};

pub fn events(weak: Weak<MainWindow>) -> anyhow::Result<()> {
    ui_events(weak.clone())?;
    Ok(())
}
