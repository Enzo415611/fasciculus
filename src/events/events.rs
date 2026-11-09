use slint::{ComponentHandle, Weak};

use crate::{MainWindow, UiState, events::ui_events::ui_events};

pub fn events(weak: Weak<MainWindow>) -> anyhow::Result<()> {
    let main_window = weak.unwrap();
    let ui_state = main_window.global::<UiState>();
    ui_events(weak.clone())?;
    Ok(())
}
