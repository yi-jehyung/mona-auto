use std::{
    sync::{atomic::Ordering, mpsc},
    thread,
};

use egui::{RichText, Ui};
use i18n_embed_fl::fl;

use crate::gui::app::MyApp;

pub fn control_panel(ui: &mut Ui, app: &mut MyApp) {
    let target_window_label = ui.label(RichText::new(fl!(app.i18n_loader, "control-panel-label-target-window")).strong());
    ui.columns(2, |columns| {
        columns[0].horizontal(|ui| {
            ui.text_edit_singleline(&mut app.target_window)
                .labelled_by(target_window_label.id)
                .on_hover_text(format!("{:#?}", app.project.windows));
            app.target_window = app.project.get_first_title();
        });
        columns[1].horizontal(|ui| {
            app.target_window = app.project.get_first_title();
            show_find_button(ui, app);

            show_start_button(ui, app);

            show_stop_button(ui, app);
        });
    });
}

fn show_find_button(ui: &mut Ui, app: &mut MyApp) {
    let find_button = egui::Button::new(fl!(app.i18n_loader, "control-panel-button-find"));
    if ui.add_enabled(!app.is_automation_running, find_button).clicked() {
        let (tx, rx) = mpsc::channel();
        app.target_window_rx = Some(rx);

        thread::spawn(move || {
            let windows = crate::core::run_capture_loop();
            println!("{:?}", windows);
            tx.send(windows).unwrap();
        });
    }
}

fn show_start_button(ui: &mut Ui, app: &mut MyApp) {
    let start_button = egui::Button::new(fl!(app.i18n_loader, "control-panel-button-start"));
    if ui.add_enabled(!app.is_automation_running, start_button).clicked() {
        match app.project.windows.rebind_hwnds() {
            Ok(_) => {
                let hwnd1 = app.project.get_first_hwnd(&app.i18n_loader).unwrap();
                let hwnd2 = app.project.get_last_hwnd(&app.i18n_loader).unwrap();
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
                if app.maybe_prompt_window_resize() {
                    return;
                }
                // 이미 실행 중이 아니면 스레드 띄우기
                if app.handle.is_none() {
                    let (tx, rx) = mpsc::channel();
                    app.engine_rx = Some(rx);
                    app.stop_flag.store(false, Ordering::SeqCst);
                    let flag = app.stop_flag.clone();
                    let handle = crate::core::automation_loop::start_automation_loop(app.project.clone(), app.setting.clone(), tx, flag);
                    app.handle = Some(handle);
                    app.is_automation_running = true;
                }
            }
            Err(err) => {
                eprintln!("{err}");
                app.error_message = Some(err);
            }
        }
    }
}

fn show_stop_button(ui: &mut Ui, app: &mut MyApp) {
    let stop_button = egui::Button::new(fl!(app.i18n_loader, "control-panel-button-stop"));
    if ui.add_enabled(app.is_automation_running, stop_button).clicked() {
        // 실행 중이면 종료 플래그 세우고 스레드 합류
        if let Some(handle) = app.handle.take() {
            app.stop_flag.store(true, Ordering::SeqCst);
            let _ = handle.join();
            app.is_automation_running = false;
        }
    }
}