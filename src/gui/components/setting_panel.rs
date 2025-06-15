use egui::{ComboBox, Ui};

use crate::{
    core::setting::{CaptureType, InputType},
    fl,
    gui::MyApp,
};

pub fn setting_panel(ui: &mut Ui, app: &mut MyApp) {
    ui.add_enabled_ui(!app.is_automation_running, |ui| {
        show_select_input_type(ui, app);
        show_select_capture_type(ui, app);
        show_select_loop_speed(ui, app);
        show_select_threshold(ui, app);
    });
}

fn show_select_input_type(ui: &mut Ui, app: &mut MyApp) {
    let text = match app.setting.input_type {
        InputType::PostMessage => "PostMessage",
        InputType::SendMessage => "SendMessage",
        InputType::SendInput => "SendInput",
    };
    ComboBox::new("input_type", fl!("setting-panel-label-input-type"))
        .selected_text(text)
        .show_ui(ui, |ui| {
            if ui
                .selectable_value(&mut app.setting.input_type, InputType::PostMessage, "Postinput")
                .clicked()
            {
                app.setting.input_type = InputType::PostMessage;
                app.setting.save();
            }
            if ui
                .selectable_value(&mut app.setting.input_type, InputType::SendInput, "SendInput")
                .clicked()
            {
                app.setting.input_type = InputType::SendInput;
                app.setting.save();
            }
        });
}
fn show_select_capture_type(ui: &mut Ui, app: &mut MyApp) {
    let text = match app.setting.capture_type {
        CaptureType::BitBlt => "BitBlt",
        CaptureType::PrintWindow => "PrintWindow",
    };
    ComboBox::new("capture_type", fl!("setting-panel-label-capture-type"))
        .selected_text(text)
        .show_ui(ui, |ui| {
            if ui
                .selectable_value(&mut app.setting.capture_type, CaptureType::BitBlt, "BitBlt")
                .clicked()
            {
                app.setting.capture_type = CaptureType::BitBlt;
                app.setting.save();
            }
            if ui
                .selectable_value(&mut app.setting.capture_type, CaptureType::PrintWindow, "PrintWindow")
                .clicked()
            {
                app.setting.capture_type = CaptureType::PrintWindow;
                app.setting.save();
            }
        });
}
fn show_select_loop_speed(ui: &mut Ui, app: &mut MyApp) {
    let text = match app.setting.loop_per_second {
        per if per <= 5 => fl!("setting-panel-loop-per-second-very-low"),
        per if per <= 10 => fl!("setting-panel-loop-per-second-low"),
        per if per <= 20 => fl!("setting-panel-loop-per-second-medium"),
        per if per <= 30 => fl!("setting-panel-loop-per-second-high"),
        _ => fl!("setting-panel-loop-per-second-very-high"),
    };
    ComboBox::new("loop_per_second", fl!("setting-panel-label-loop-per-second"))
        .selected_text(text)
        .show_ui(ui, |ui| {
            if ui
                .selectable_value(&mut app.setting.loop_per_second, 5, fl!("setting-panel-loop-per-second-very-low"))
                .clicked()
            {
                app.setting.loop_per_second = 5;
                app.setting.save();
            }
            if ui
                .selectable_value(&mut app.setting.loop_per_second, 10, fl!("setting-panel-loop-per-second-low"))
                .clicked()
            {
                app.setting.loop_per_second = 10;
                app.setting.save();
            }
            if ui
                .selectable_value(&mut app.setting.loop_per_second, 15, fl!("setting-panel-loop-per-second-medium"))
                .clicked()
            {
                app.setting.loop_per_second = 15;
                app.setting.save();
            }
            if ui
                .selectable_value(&mut app.setting.loop_per_second, 25, fl!("setting-panel-loop-per-second-high"))
                .clicked()
            {
                app.setting.loop_per_second = 25;
                app.setting.save();
            }
            if ui
                .selectable_value(&mut app.setting.loop_per_second, 60, fl!("setting-panel-loop-per-second-very-high"))
                .clicked()
            {
                app.setting.loop_per_second = 60;
                app.setting.save();
            }
        });
}

fn show_select_threshold(ui: &mut Ui, app: &mut MyApp) {
    let text = match app.setting.threshold {
        t if t >= 0.03 => fl!("setting-panel-threshold-very-sensitive"),
        t if t >= 0.02 => fl!("setting-panel-threshold-sensitive"),
        t if t >= 0.01 => fl!("setting-panel-threshold-medium"),
        _ => fl!("setting-panel-threshold-low"),
    };
    ComboBox::new("threshold", fl!("setting-panel-label-threshold"))
        .selected_text(text)
        .show_ui(ui, |ui| {
            if ui
                .selectable_value(&mut app.setting.threshold, 0.03, fl!("setting-panel-threshold-very-sensitive"))
                .clicked()
            {
                app.setting.threshold = 0.03;
                app.setting.save();
            }
            if ui
                .selectable_value(&mut app.setting.threshold, 0.02, fl!("setting-panel-threshold-sensitive"))
                .clicked()
            {
                app.setting.threshold = 0.02;
                app.setting.save();
            }
            if ui
                .selectable_value(&mut app.setting.threshold, 0.01, fl!("setting-panel-threshold-medium"))
                .clicked()
            {
                app.setting.threshold = 0.01;
                app.setting.save();
            }
            if ui
                .selectable_value(&mut app.setting.threshold, 0.005, fl!("setting-panel-threshold-low"))
                .clicked()
            {
                app.setting.threshold = 0.005;
                app.setting.save();
            }
        });
}
