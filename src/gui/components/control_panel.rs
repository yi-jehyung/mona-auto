use std::{
    sync::{atomic::Ordering, mpsc},
    thread,
};

use egui::{RichText, Ui};

use crate::{core::TargetWindow, fl, gui::app::MyApp};

pub fn control_panel(ui: &mut Ui, app: &mut MyApp) {
    let target_window_label = ui.label(RichText::new(fl!("control-panel-label-target-window")).strong());

    let mut delete_indices = vec![];

    ui.columns(2, |columns| {
        columns[0].horizontal(|ui| {
            let mut target_window = "".to_string();
            if let Some(first) = app.project.target_windows.first() {
                target_window = first.get_first_title();
            }

            ui.text_edit_singleline(&mut target_window)
                .labelled_by(target_window_label.id)
                .on_hover_text(format!("{:#?}", app.project.target_windows[0]));
        });
        columns[1].horizontal(|ui| {
            show_find_button(ui, app, 0);

            show_start_button(ui, app);

            show_stop_button(ui, app);
        });
    });

    for i in 1..app.project.target_windows.len() {
        ui.columns(2, |columns| {
            columns[0].horizontal(|ui| {
                let mut target_window = app.project.target_windows[i].get_first_title();

                ui.text_edit_singleline(&mut target_window)
                    .on_hover_text(format!("{:#?}", app.project.target_windows[i]));
            });
            columns[1].horizontal(|ui| {
                show_find_button(ui, app, i);
                if show_delete_button(ui, app) {
                    delete_indices.push(i);
                }
            });
        });
    }

    show_add_button(ui, app);

    for i in delete_indices.into_iter().rev() {
        app.project.target_windows.remove(i);
    }
}

fn show_find_button(ui: &mut Ui, app: &mut MyApp, index: usize) {
    let find_button = egui::Button::new(fl!("control-panel-button-find"));
    if ui.add_enabled(!app.is_automation_running, find_button).clicked() {
        app.find_window_index = index;
        let (tx, rx) = mpsc::channel();
        app.target_window_rx = Some(rx);

        thread::spawn(move || {
            let windows = crate::core::run_capture_loop();
            println!("{windows:?}");
            tx.send(windows).unwrap();
        });
    }
}

fn show_start_button(ui: &mut Ui, app: &mut MyApp) {
    let start_button = egui::Button::new(fl!("control-panel-button-start"));
    if ui.add_enabled(!app.is_automation_running, start_button).clicked() {
        for w in app.project.target_windows.iter_mut() {
            match w.rebind_hwnds() {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("{err}");
                    app.error_message = Some(err);
                    return;
                }
            }

            let hwnd1 = w.get_first_hwnd().unwrap();
            let hwnd2 = w.get_last_hwnd().unwrap();
            match app.setting.input_type {
                crate::core::InputType::PostMessage => {
                    crate::core::activate_window(hwnd1);
                    crate::core::activate_window(hwnd2);
                }
                crate::core::InputType::SendInput => {
                    crate::core::bring_window_to_front(hwnd1);
                    crate::core::bring_window_to_front(hwnd2);
                }
                _ => {}
            }
        }
        if app.maybe_prompt_window_resize() {
            return;
        }

        if app.handle.is_empty() {
            let (tx, rx) = mpsc::channel();
            app.engine_rx = Some(rx);
            app.stop_flag.store(false, Ordering::SeqCst);

            let flag = app.stop_flag.clone();
            let handle = crate::core::automation_loop::start_automation_loop(app.project.clone(), app.setting.clone(), tx, flag);

            app.handle = handle;
            app.is_automation_running = true;
        }
    }
}

fn show_stop_button(ui: &mut Ui, app: &mut MyApp) {
    let stop_button = egui::Button::new(fl!("control-panel-button-stop"));

    if ui.add_enabled(app.is_automation_running, stop_button).clicked() {
        app.stop_flag.store(true, Ordering::SeqCst);

        for handle in app.handle.drain(..) {
            let _ = handle.join();
        }

        app.is_automation_running = false;
    }
}

fn show_add_button(ui: &mut Ui, app: &mut MyApp) {
    let add_button = egui::Button::new(fl!("control-panel-button-add"));
    if ui.add_enabled(!app.is_automation_running, add_button).clicked() {
        app.project.target_windows.push(TargetWindow::new());
    }
}

fn show_delete_button(ui: &mut Ui, app: &mut MyApp) -> bool {
    let delete_button = egui::Button::new(fl!("control-panel-button-delete"));
    ui.add_enabled(!app.is_automation_running, delete_button).clicked()
}
