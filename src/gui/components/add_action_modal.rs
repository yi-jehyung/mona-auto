use egui::{Align, ComboBox, Context, Id, Modal, ScrollArea, Ui};

use crate::{
    core::action::{self, ActionType, KeyCode, KeyType},
    fl,
    gui::app::MyApp,
};

pub fn show_add_action_modal(ctx: &Context, app: &mut MyApp) {
    Modal::new(Id::new("action_modal"))
        .frame(egui::Frame::popup(&ctx.style()))
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                show_add_action_modal_header(ui, app);

                show_action_type_selector(ui, app);

                ui.separator();

                show_action_type_editor(ui, app);

                ui.add_space(8.0);
                ui.separator();

                show_confirm_cancel_ui(ctx, ui, app);
            });
        });
}

fn show_add_action_modal_header(ui: &mut Ui, app: &mut MyApp) {
    match app.edit_action {
        Some(_) => {
            ui.heading(fl!("add-action-modal-heading-edit"));
        }
        None => {
            ui.heading(fl!("add-action-modal-heading-add"));
        }
    }
}

fn show_action_type_selector(ui: &mut Ui, app: &mut MyApp) {
    ComboBox::from_label("")
        .selected_text(label_for_action_type(&app.action_type))
        .show_ui(ui, |ui| {
            ui.selectable_value(
                &mut app.action_type,
                ActionType::LeftClick {
                    x: 0,
                    y: 0,
                    use_matched_position: true,
                },
                fl!("action-panel-left-click"),
            );
            ui.selectable_value(
                &mut app.action_type,
                ActionType::RightClick {
                    x: 0,
                    y: 0,
                    use_matched_position: true,
                },
                fl!("action-panel-right-click"),
            );
            ui.selectable_value(
                &mut app.action_type,
                ActionType::Drag {
                    start: (0, 0),
                    end: (0, 0),
                    duration_ms: 500,
                },
                fl!("action-panel-drag"),
            );
            ui.selectable_value(
                &mut app.action_type,
                ActionType::Scroll {
                    x: 0,
                    y: 0,
                    delta: 120,
                    use_matched_position: true,
                },
                fl!("action-panel-scroll"),
            );
            ui.selectable_value(
                &mut app.action_type,
                ActionType::KeyInput {
                    keys: vec![KeyType::default()],
                },
                fl!("action-panel-key-input"),
            );
            ui.selectable_value(
                &mut app.action_type,
                ActionType::TextInput { text: "".to_string() },
                fl!("action-panel-text-input"),
            );
            ui.selectable_value(&mut app.action_type, ActionType::Delay { millis: 500 }, fl!("action-panel-delay"));
            ui.selectable_value(
                &mut app.action_type,
                ActionType::SendDiscord {
                    webhook_url: "".into(),
                    message: "".into(),
                    use_screenshot: false,
                },
                fl!("action-panel-send-discord"),
            );
            ui.selectable_value(
                &mut app.action_type,
                ActionType::ToggleEnable {
                    target: action::ToggleTarget::Image("".to_owned()),
                    enable: true,
                },
                fl!("action-panel-toggle-image-enable"),
            );
        });
}

fn show_action_type_editor(ui: &mut Ui, app: &mut MyApp) {
    match &mut app.action_type {
        ActionType::LeftClick {
            x,
            y,
            use_matched_position,
        } => {
            ui.horizontal(|ui| {
                ui.label("x: ");
                ui.add(egui::DragValue::new(x).range(0..=usize::MAX).speed(1));

                ui.label("y: ");
                ui.add(egui::DragValue::new(y).range(0..=usize::MAX).speed(1));
                ui.checkbox(use_matched_position, fl!("action-panel-checkbox-use-matched-position"));
            });
            ui.horizontal(|ui| {
                if ui.button(fl!("action-panel-button-open-coordinate-picker")).clicked() {
                    app.picked_position = None;
                    app.coordinate_picker_enable = true;
                }
            });
            if app.coordinate_picker_enable {
                if let Some((pick_x, pick_y)) = app.picked_position {
                    *x = pick_x;
                    *y = pick_y;
                }
            }
        }

        ActionType::RightClick {
            x,
            y,
            use_matched_position,
        } => {
            ui.horizontal(|ui| {
                ui.label("x: ");
                ui.add(egui::DragValue::new(x).range(0..=usize::MAX).speed(1));

                ui.label("y: ");
                ui.add(egui::DragValue::new(y).range(0..=usize::MAX).speed(1));
                ui.checkbox(use_matched_position, fl!("action-panel-checkbox-use-matched-position"));
            });
            ui.horizontal(|ui| {
                if ui.button(fl!("action-panel-button-open-coordinate-picker")).clicked() {
                    app.picked_position = None;
                    app.coordinate_picker_enable = true;
                }
            });
            if app.coordinate_picker_enable {
                if let Some((pick_x, pick_y)) = app.picked_position {
                    *x = pick_x;
                    *y = pick_y;
                }
            }
        }

        ActionType::Drag { start, end, duration_ms } => {
            ui.horizontal(|ui| {
                ui.label("start: ");
                ui.label("x: ");
                ui.add(egui::DragValue::new(&mut start.0).range(0..=usize::MAX).speed(1));
                ui.label("y: ");
                ui.add(egui::DragValue::new(&mut start.1).range(0..=usize::MAX).speed(1));
            });
            ui.horizontal(|ui| {
                if ui.button(fl!("action-panel-button-open-coordinate-picker")).clicked() {
                    app.picked_position = None;
                    app.coordinate_picker_enable = true;
                }
            });
            if app.coordinate_picker_enable {
                if let Some((pick_x, pick_y)) = app.picked_position {
                    start.0 = pick_x;
                    start.1 = pick_y;
                }
            }
            ui.horizontal(|ui| {
                ui.label("end: ");
                ui.label("x: ");
                ui.add(egui::DragValue::new(&mut end.0).range(0..=usize::MAX).speed(1));
                ui.label("y: ");
                ui.add(egui::DragValue::new(&mut end.1).range(0..=usize::MAX).speed(1));
            });
            ui.horizontal(|ui| {
                ui.label(fl!("action-panel-duration-ms"));
                ui.add(egui::DragValue::new(duration_ms).range(0..=usize::MAX).speed(1));
            });
        }

        ActionType::Scroll {
            x,
            y,
            delta,
            use_matched_position,
        } => {
            ui.horizontal(|ui| {
                let s = if *delta == 120 {
                    fl!("action-panel-scroll-direction-option-up").to_string()
                } else {
                    fl!("action-panel-scroll-direction-option-down").to_string()
                };
                ComboBox::from_label("").selected_text(s.to_string()).show_ui(ui, |ui| {
                    ui.selectable_value(delta, 120, fl!("action-panel-scroll-direction-option-up"));
                    ui.selectable_value(delta, -120, fl!("action-panel-scroll-direction-option-down"));
                });
            });
            ui.horizontal(|ui| {
                ui.label("x: ");
                ui.add(egui::DragValue::new(x).range(0..=usize::MAX).speed(1));

                ui.label("y: ");
                ui.add(egui::DragValue::new(y).range(0..=usize::MAX).speed(1));

                ui.checkbox(use_matched_position, fl!("action-panel-checkbox-use-matched-position"));
            });
            ui.horizontal(|ui| {
                if ui.button(fl!("action-panel-button-open-coordinate-picker")).clicked() {
                    app.picked_position = None;
                    app.coordinate_picker_enable = true;
                }
            });
            if app.coordinate_picker_enable {
                if let Some((pick_x, pick_y)) = app.picked_position {
                    *x = pick_x;
                    *y = pick_y;
                }
            }
        }

        ActionType::KeyInput { keys } => {
            ScrollArea::vertical().max_height(30.0).show(ui, |ui| {
                ui.horizontal(|ui| {
                    let columns = 8;
                    egui::Grid::new("key_grid").num_columns(columns).show(ui, |ui| {
                        for (i, key) in keys.iter().enumerate() {
                            let enable = Some(i) != app.key_index;
                            ui.add_enabled_ui(enable, |ui| {
                                let button = ui.button(format!("{key}"));
                                if button.clicked() {
                                    app.key_index = Some(i);
                                    ui.scroll_to_rect(button.rect, Some(Align::Center));
                                }
                                if i == keys.len() - 1 && app.scroll_to_bottom {
                                    ui.scroll_to_rect(button.rect, Some(Align::Center));
                                    app.scroll_to_bottom = false;
                                }
                            });
                            if (i + 1) % columns == 0 {
                                ui.end_row();
                            }
                        }
                    });
                });
            });
            ui.separator();
            if ui.button(fl!("action-panel-key-add")).clicked() {
                app.key_index = match app.key_index {
                    Some(index) => {
                        keys.push(keys[index]);
                        Some(keys.len() - 1)
                    }
                    None => {
                        keys.push(KeyType::DownAndUp(KeyCode::A));
                        Some(1)
                    }
                };
                app.scroll_to_bottom = true;
            }

            ui.horizontal(|ui| {
                if let Some(index) = app.key_index {
                    let key_code = match keys[index] {
                        KeyType::Down(key_code) => key_code,
                        KeyType::Up(key_code) => key_code,
                        KeyType::DownAndUp(key_code) => key_code,
                        KeyType::Delay(_) => KeyCode::A,
                    };
                    let key_type = match keys[index] {
                        KeyType::Down(_) => fl!("action-panel-key-type-down"),
                        KeyType::Up(_) => fl!("action-panel-key-type-up"),
                        KeyType::DownAndUp(_) => fl!("action-panel-key-type-down-and-up"),
                        KeyType::Delay(_) => fl!("action-panel-key-type-delay"),
                    };
                    ComboBox::new("key type", "").selected_text(key_type).show_ui(ui, |ui| {
                        ui.selectable_value(&mut keys[index], KeyType::Down(key_code), fl!("action-panel-key-type-down"));
                        ui.selectable_value(&mut keys[index], KeyType::Up(key_code), fl!("action-panel-key-type-up"));
                        ui.selectable_value(
                            &mut keys[index],
                            KeyType::DownAndUp(key_code),
                            fl!("action-panel-key-type-down-and-up"),
                        );
                        ui.selectable_value(&mut keys[index], KeyType::Delay(500), fl!("action-panel-key-type-delay"));
                    });

                    match &mut keys[index] {
                        KeyType::Down(ref mut current) | KeyType::Up(ref mut current) | KeyType::DownAndUp(ref mut current) => {
                            ComboBox::new("key code", "").selected_text(current.to_string()).show_ui(ui, |ui| {
                                for k in [
                                    KeyCode::A,
                                    KeyCode::B,
                                    KeyCode::C,
                                    KeyCode::D,
                                    KeyCode::E,
                                    KeyCode::F,
                                    KeyCode::G,
                                    KeyCode::H,
                                    KeyCode::I,
                                    KeyCode::J,
                                    KeyCode::K,
                                    KeyCode::L,
                                    KeyCode::M,
                                    KeyCode::N,
                                    KeyCode::O,
                                    KeyCode::P,
                                    KeyCode::Q,
                                    KeyCode::R,
                                    KeyCode::S,
                                    KeyCode::T,
                                    KeyCode::U,
                                    KeyCode::V,
                                    KeyCode::W,
                                    KeyCode::X,
                                    KeyCode::Y,
                                    KeyCode::Z,
                                    KeyCode::Num0,
                                    KeyCode::Num1,
                                    KeyCode::Num2,
                                    KeyCode::Num3,
                                    KeyCode::Num4,
                                    KeyCode::Num5,
                                    KeyCode::Num6,
                                    KeyCode::Num7,
                                    KeyCode::Num8,
                                    KeyCode::Num9,
                                    KeyCode::NumPad0,
                                    KeyCode::NumPad1,
                                    KeyCode::NumPad2,
                                    KeyCode::NumPad3,
                                    KeyCode::NumPad4,
                                    KeyCode::NumPad5,
                                    KeyCode::NumPad6,
                                    KeyCode::NumPad7,
                                    KeyCode::NumPad8,
                                    KeyCode::NumPad9,
                                    KeyCode::Enter,
                                    KeyCode::Escape,
                                    KeyCode::Backspace,
                                    KeyCode::Tab,
                                    KeyCode::Space,
                                    KeyCode::Shift,
                                    KeyCode::Ctrl,
                                    KeyCode::Alt,
                                    KeyCode::Left,
                                    KeyCode::Up,
                                    KeyCode::Right,
                                    KeyCode::Down,
                                    KeyCode::F1,
                                    KeyCode::F2,
                                    KeyCode::F3,
                                    KeyCode::F4,
                                    KeyCode::F5,
                                    KeyCode::F6,
                                    KeyCode::F7,
                                    KeyCode::F8,
                                    KeyCode::F9,
                                    KeyCode::F10,
                                    KeyCode::F11,
                                    KeyCode::F12,
                                    KeyCode::Delete,
                                    KeyCode::Insert,
                                    KeyCode::Home,
                                    KeyCode::End,
                                    KeyCode::PageUp,
                                    KeyCode::PageDown,
                                ] {
                                    if ui.selectable_label(*current == k, format!("{k}")).clicked() {
                                        *current = k;
                                    }
                                }
                                if ui.selectable_label(matches!(current, KeyCode::Custom(_)), "Custom...").clicked() {
                                    *current = KeyCode::Custom(0);
                                }
                            });
                            if let KeyCode::Custom(ref mut val) = current {
                                ui.horizontal(|ui| {
                                    ui.hyperlink_to(
                                        fl!("action-panel-key-custom-vk"),
                                        "https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes",
                                    );
                                    ui.add(egui::DragValue::new(val).range(0..=255).speed(1));
                                });
                            }
                        }
                        KeyType::Delay(delay) => {
                            ui.add(egui::DragValue::new(delay).range(0..=u32::MAX).speed(1));
                        }
                    }
                    if ui.button(fl!("action-panel-context-delete")).clicked() {
                        app.key_index = None;
                        keys.remove(index);
                    }
                }
            });
        }

        ActionType::TextInput { text } => {
            ui.text_edit_multiline(text);
        }

        ActionType::Delay { millis } => {
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(millis).range(0..=usize::MAX).speed(1));
                ui.label(fl!(
                    "action-panel-label-millis-with-seconds",
                    seconds = format!("{}", *millis as f64 / 1000.0)
                ));
            });
        }

        ActionType::SendDiscord {
            webhook_url,
            message,
            use_screenshot,
        } => {
            ui.label(fl!("action-panel-label-webhook-url"));

            ScrollArea::both().max_width(ui.available_width()).show(ui, |ui| {
                ui.text_edit_singleline(webhook_url);
            });
            ui.label(fl!("action-panel-label-message"));
            ui.text_edit_singleline(message);
            ui.checkbox(use_screenshot, fl!("action-panel-send-screenshot"));
        }

        ActionType::ToggleEnable { target, enable } => {
            ui.label(fl!("action-panel-label-target"));
            match target {
                crate::core::ToggleTarget::Image(target) => {
                    ui.text_edit_singleline(target);
                }
            }
            let text = match enable {
                true => fl!("action-panel-enable-enabled"),
                false => fl!("action-panel-enable-disabled"),
            };
            ComboBox::new("enable", "").selected_text(text).show_ui(ui, |ui| {
                ui.selectable_value(enable, true, fl!("action-panel-enable-enabled"));
                ui.selectable_value(enable, false, fl!("action-panel-enable-disabled"));
            });
        }
    }
}

fn show_confirm_cancel_ui(ctx: &Context, ui: &mut Ui, app: &mut MyApp) {
    ui.horizontal_centered(|ui| {
        if ui.button(fl!("add-action-modal-button-confirm")).clicked() {
            if let Some(item_index) = app.selected_item_index {
                match app.edit_action {
                    Some(action_index) => {
                        app.project.items[item_index].actions[action_index].action = app.action_type.clone();
                        app.edit_action = None;
                    }
                    None => {
                        app.project.add_action(item_index, app.action_type.clone());
                        app.project.save_file();
                    }
                }
            }
            app.show_action_modal = false;
        }
        if ui.button(fl!("add-action-modal-button-cancel")).clicked() {
            app.show_action_modal = false;
            app.edit_action = None;
        }
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            app.show_action_modal = false;
            app.edit_action = None;
        }
    });
}

fn label_for_action_type(action_type: &ActionType) -> String {
    use ActionType::*;
    match action_type {
        LeftClick { .. } => fl!("action-panel-left-click"),
        RightClick { .. } => fl!("action-panel-right-click"),
        Drag { .. } => fl!("action-panel-drag"),
        Scroll { .. } => fl!("action-panel-scroll"),
        KeyInput { .. } => fl!("action-panel-key-input"),
        TextInput { .. } => fl!("action-panel-text-input"),
        Delay { .. } => fl!("action-panel-delay"),
        SendDiscord { .. } => fl!("action-panel-send-discord"),
        ToggleEnable { .. } => fl!("action-panel-toggle-image-enable"),
    }
}
