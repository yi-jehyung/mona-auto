use egui::{ComboBox, Context, Id, Modal, Ui};
use i18n_embed_fl::fl;

use crate::{
    core::window::{restore_window, WindowInfo},
    gui::app::{MyApp, WindowSizeOperation},
};

pub fn show_window_resize_modal(ctx: &Context, app: &mut MyApp) {
    Modal::new(Id::new("window_resize_modal"))
        .frame(egui::Frame::popup(&ctx.style()))
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(fl!(app.i18n_loader, "window-resize-modal-heading"));
                show_select_option(ui, app);
                ui.spacing();

                ui.horizontal_centered(|ui| {
                    confirm_button(ui, app);
                    cancel_button(ui, app);
                });
            });
        });
}

fn show_select_option(ui: &mut Ui, app: &mut MyApp) {
    ComboBox::new("window_resize", "")
        .selected_text(match app.window_size_operation {
            WindowSizeOperation::RestorePrevious => {
                fl!(app.i18n_loader, "window-resize-modal-option-restore")
            }
            WindowSizeOperation::UpdateCurrent => {
                fl!(app.i18n_loader, "window-resize-modal-option-update")
            }
        })
        .show_ui(ui, |ui| {
            ui.selectable_value(
                &mut app.window_size_operation,
                WindowSizeOperation::RestorePrevious,
                fl!(app.i18n_loader, "window-resize-modal-option-restore"),
            );
            ui.selectable_value(
                &mut app.window_size_operation,
                WindowSizeOperation::UpdateCurrent,
                fl!(app.i18n_loader, "window-resize-modal-option-update"),
            );
        });
}

fn confirm_button(ui: &mut Ui, app: &mut MyApp) {
    if ui.button(fl!(app.i18n_loader, "window-resize-modal-button-confirm")).clicked() {
        match app.window_size_operation {
            WindowSizeOperation::RestorePrevious => {
                restore_window(app.project.get_first_hwnd(&app.i18n_loader).unwrap(), app.project.window_info.clone().unwrap());
                let info = WindowInfo::get_window_info(app.project.get_first_hwnd(&app.i18n_loader).unwrap()).unwrap();
                app.project.window_info = Some(info);
                app.project.save_file();
            }
            WindowSizeOperation::UpdateCurrent => {
                let info = WindowInfo::get_window_info(app.project.get_first_hwnd(&app.i18n_loader).unwrap()).unwrap();
                app.project.window_info = Some(info);
                app.project.save_file();
            }
        }
        app.show_window_resize_modal = false;
    }
}

fn cancel_button(ui: &mut Ui, app: &mut MyApp) {
    if ui.button(fl!(app.i18n_loader, "window-resize-modal-button-cancel")).clicked() {
        app.show_window_resize_modal = false;
    }
}
