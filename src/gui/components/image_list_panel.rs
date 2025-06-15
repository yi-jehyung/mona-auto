use egui::{
    text_edit::{TextEdit, TextEditOutput},
    Color32, Frame, Label, Margin, RichText, ScrollArea, Sense, Ui,
};

use crate::{fl, gui::app::MyApp};

/// Pending user actions triggered from the context menu
///
/// 사용자가 컨텍스트 메뉴에서 클릭한 항목에 대한 대기 작업 상태
struct Pending {
    rename: Option<(usize, String)>,
    delete: Option<usize>,
    move_up: Option<usize>,
    move_down: Option<usize>,
}

pub fn image_list_panel(ui: &mut Ui, app: &mut MyApp) {
    let mut pending = Pending {
        rename: None,
        delete: None,
        move_up: None,
        move_down: None,
    };

    show_image_list_header(ui, app);

    ui.add_space(2.0);

    ui.add_enabled_ui(!app.is_automation_running, |ui| {
        show_image_list(ui, app, &mut pending);
    });

    // Handle user-triggered actions such as edit, delete, and reordering of actions
    // 사용자 입력에 따라 액션 수정/삭제/순서 변경을 처리
    handle_action_events(app, pending);
}

fn show_image_list_header(ui: &mut Ui, app: &mut MyApp) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(fl!("image-list-panel-label")).strong());
        ui.add_enabled_ui(!app.is_automation_running, |ui| {
            if ui.button(fl!("image-list-panel-button-add-image")).clicked() {
                let name = app.project.next_name();
                app.project.add_action_item(&name, None, 0, 0, 0, 0);
                app.project.save_file();
            }
        });
    });
}

fn show_image_list(ui: &mut Ui, app: &mut MyApp, pending: &mut Pending) {
    ScrollArea::vertical().max_height(ui.available_height()).show(ui, |ui| {
        ui.with_layout(egui::Layout::top_down(egui::Align::TOP), |ui| {
            ui.set_width(ui.available_width());

            for (index, item) in &mut app.project.items.iter_mut().enumerate() {
                let bg = if app.selected_item_index == Some(index) {
                    ui.visuals().selection.bg_fill
                } else {
                    Color32::TRANSPARENT
                };

                Frame::new().fill(bg).inner_margin(Margin::same(4)).show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.checkbox(&mut item.enabled, "").context_menu(|ui| {
                            if ui.button(fl!("image-list-panel-context-rename")).clicked() {
                                app.renaming = Some(index);
                                app.rename_buffer = item.name.clone();
                                ui.close_menu();
                            }
                            if ui.button(fl!("image-list-panel-context-delete")).clicked() {
                                pending.delete = Some(index);
                                ui.close_menu();
                            }
                            if ui.button(fl!("image-list-panel-context-move-up")).clicked() {
                                pending.move_up = Some(index);
                                ui.close_menu();
                            }
                            if ui.button(fl!("image-list-panel-context-move-down")).clicked() {
                                pending.move_down = Some(index);
                                ui.close_menu();
                            }
                        });

                        if Some(index) == app.renaming {
                            let text_output: TextEditOutput = TextEdit::singleline(&mut app.rename_buffer).show(ui);
                            // 엔터+포커스 아웃 감지
                            if text_output.response.lost_focus() {
                                if let Some(_i) = app.renaming {
                                    pending.rename = Some((index, app.rename_buffer.clone()));
                                    app.renaming = None;
                                }
                            }
                        } else {
                            let label = ui
                                .add(Label::new(&item.name).sense(Sense::click()))
                                .on_hover_text(format!("{:#?}", item));
                            label.context_menu(|ui| {
                                if ui.button(fl!("image-list-panel-context-rename")).clicked() {
                                    app.renaming = Some(index);
                                    app.rename_buffer = item.name.clone();
                                    ui.close_menu();
                                }
                                if ui.button(fl!("image-list-panel-context-delete")).clicked() {
                                    pending.delete = Some(index);
                                    ui.close_menu();
                                }
                                if ui.button(fl!("image-list-panel-context-move-up")).clicked() {
                                    pending.move_up = Some(index);
                                    ui.close_menu();
                                }
                                if ui.button(fl!("image-list-panel-context-move-down")).clicked() {
                                    pending.move_down = Some(index);
                                    ui.close_menu();
                                }
                            });

                            if label.clicked() {
                                app.selected_item_index = Some(index);
                                app.selected_item = Some(item.clone());
                                app.needs_image_update = true;
                            }
                        }
                    });
                });
            }
        });
    });
}

fn handle_action_events(app: &mut MyApp, pending: Pending) {
    if let Some((i, new_name)) = pending.rename {
        app.project.items[i].name = new_name;
    }
    if let Some(index) = pending.delete {
        app.project.items.remove(index);
        if app.selected_item_index == Some(index) {
            app.selected_item_index = None;
        }
        app.project.save_file();
        app.needs_image_update = true;
    }
    if let Some(index) = pending.move_up {
        if index > 0 && index < app.project.items.len() {
            app.project.items.swap(index, index - 1);
            if app.selected_item_index == Some(index) {
                app.selected_item_index = None;
            }
            app.project.save_file();
        }
    }
    if let Some(index) = pending.move_down {
        if index + 1 < app.project.items.len() {
            app.project.items.swap(index, index + 1);
            if app.selected_item_index == Some(index) {
                app.selected_item_index = None;
            }
            app.project.save_file();
        }
    }
}
