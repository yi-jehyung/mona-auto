use egui::{Context, Id, Modal};
use i18n_embed_fl::fl;

use crate::gui::MyApp;

pub fn show_error_modal(ctx: &Context, app: &mut MyApp) {
    Modal::new(Id::new("error_modal"))
        .frame(egui::Frame::popup(&ctx.style()))
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading(fl!(app.i18n_loader, "error-modal-heading"));

                if let Some(err) = &app.error_message {
                    ui.label(err);
                }

                if ui.button(fl!(app.i18n_loader, "error-modal-button-ok")).clicked() {
                    app.error_message = None;
                }
            });
        });

    if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
        app.error_message = None;
    }
}