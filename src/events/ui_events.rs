use slint::{ComponentHandle, ModelRc, VecModel, Weak};

use crate::{MainWindow, UiEvents, UiState, alpm::get_pkg_by_name};

pub fn ui_events(weak: Weak<MainWindow>) -> anyhow::Result<()> {
    let main_window = weak.unwrap();
    let ui_events = main_window.global::<UiEvents>();

    ui_events.on_open_link(|url| {
        if let Err(err) = open::that(url) {
            eprintln!("{}", err);
        }
    });

    ui_events.on_search_pkg({
        let main_window = weak.clone().unwrap();
        move |name| {
            let default_icon = main_window.global::<UiState>().get_default_icon();
            if let Some(pkg) = get_pkg_by_name(&name, default_icon) {
                let ui_state = main_window.global::<UiState>();
                ui_state.set_pkgs(ModelRc::new(VecModel::from(vec![pkg])));
            }
        }
    });

    ui_events.on_quit({
        let main_window = weak.clone().unwrap();
        move || {
            main_window
                .window()
                .dispatch_event(slint::platform::WindowEvent::CloseRequested);
        }
    });
    ui_events.on_minimize({
        let main_window = weak.clone().unwrap();
        move || {
            main_window.window().set_minimized(true);
        }
    });
    ui_events.on_maximize({
        let main_window = weak.clone().unwrap();
        move || {
            if main_window.window().is_maximized() {
                main_window.window().set_maximized(false);
            } else {
                main_window.window().set_maximized(true);
            }
        }
    });
    Ok(())
}
