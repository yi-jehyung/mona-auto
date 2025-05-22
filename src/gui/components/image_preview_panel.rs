use egui::{Context, Id, Image, Modal, Ui, Widget};
use i18n_embed_fl::fl;

use crate::gui::{
    app::{CropOrRoi, MyApp},
    util::{capture_image, load_image_cache},
};

pub fn image_preview(ctx: &Context, ui: &mut Ui, app: &mut MyApp) {
    ui.vertical(|ui| {
        load_image_cache(app, ctx);

        show_image_preview(ui, app);

        let enabled = !app.is_automation_running && app.selected_item_index.is_some();
        ui.add_enabled_ui(enabled, |ui| {
            show_retake_button(ctx, ui, app);
        });

        if app.show_capture_modal {
            show_add_capture_modal(ctx, app);
        }

        show_roi(ui, app);
    });
}

fn show_image_preview(ui: &mut Ui, app: &MyApp) {
    if let Some((_, texture)) = &app.image_cache {
        ui.group(|ui| {
            ui.set_width(ui.available_width());
            ui.set_height(70.0);
            ui.centered_and_justified(|ui| {
                Image::new(texture).shrink_to_fit().ui(ui);
            });
        });
    } else {
        ui.group(|ui| {
            ui.set_width(ui.available_width());
            ui.set_height(70.0);
            ui.centered_and_justified(|ui| {
                if app.selected_item.is_none() {
                    ui.label(fl!(app.i18n_loader, "image-preview-panel-no-selection"));
                    return;
                }

                if app.project.path.is_none() {
                    ui.label(fl!(app.i18n_loader, "image-preview-panel-no-path"));
                    return;
                }

                if app.selected_item.clone().unwrap().image_path.is_none() {
                    ui.label(fl!(app.i18n_loader, "image-preview-panel-always-run"));
                    return;
                }

                ui.label(fl!(app.i18n_loader, "image-preview-panel-load-failed"));
            });
        });
    }
}

fn show_retake_button(ctx: &Context, ui: &mut Ui, app: &mut MyApp) {
    if ui.button(fl!(app.i18n_loader, "image-preview-panel-button-retake")).clicked() {
        reset_viewport_state(app);
        capture_image(ctx, app);
        app.show_capture_modal = true;
        app.show_image_edit_viewport = true;
    }
}

fn show_roi(ui: &mut egui::Ui, app: &MyApp) {
    ui.horizontal(|ui| {
        ui.label("ROI");
        match &app.selected_item {
            Some(selected_item) => {
                let roi = &selected_item.roi;
                ui.code(format!("{} {} {} {}", roi.x, roi.y, roi.width, roi.height))
            }
            None => ui.code("0 0 0 0"),
        };
        // if ui.button("확인").clicked() {
        //     todo!();
        // }
        // if ui.button("재설정").clicked() {
        //     todo!();
        // }
    });
}

fn show_add_capture_modal(ctx: &Context, app: &mut MyApp) {
    Modal::new(Id::new("capture_modal"))
        .frame(egui::Frame::popup(&ctx.style()))
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading(fl!(app.i18n_loader, "image-preview-panel-modal-heading"));
                ui.label(fl!(app.i18n_loader, "image-preview-panel-note"));
            });
        });
}

fn reset_viewport_state(app: &mut MyApp) {
    app.capture_cache = None;
    app.drag_start = None;
    app.drag_end = None;
    app.crop_or_roi = CropOrRoi::Crop;
    app.roi_a = None;
    app.roi_b = None;
    app.local_a = None;
    app.local_b = None;
    app.use_crop_size = true;
}
