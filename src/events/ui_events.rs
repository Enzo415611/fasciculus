use slint::{ComponentHandle, Weak};

use crate::{MainWindow, UiEvents};

pub fn ui_events(weak: Weak<MainWindow>) {
    let main_window = weak.unwrap();
    let ui_events = main_window.global::<UiEvents>();
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
}
