use std::path::Path;

use egui::{Color32, Context, Image, Pos2, Rect, Stroke, StrokeKind, Ui, Vec2};

use crate::{
    core::capture,
    fl,
    gui::{
        app::{CropOrRoi, MyApp},
        defines::*,
        util::{capture_image, to_local_pos, to_screen_pos2},
    },
};

pub fn show_image_edit_viewport(ctx: &Context, app: &mut MyApp) {
    ctx.show_viewport_immediate(
        egui::ViewportId::from_hash_of("image_edit_viewport"),
        egui::ViewportBuilder::default()
            .with_title("Image Editor")
            .with_inner_size([400.0, 200.0])
            .with_resizable(false),
        |ctx, class| {
            assert!(
                class == egui::ViewportClass::Immediate,
                "This egui backend doesn't support multiple viewports"
            );

            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading(fl!("image-edit-viewport-heading"));

                draw_action_buttons(ctx, ui, app);

                ui.add_space(12.0);

                ui.horizontal(|ui| {
                    let mut image_size = Vec2::ZERO;

                    draw_image_section(ctx, ui, app, &mut image_size);
                    ui.vertical(|ui| draw_position_controls(ui, app, image_size));
                });
            });

            if ctx.input(|i| i.viewport().close_requested()) {
                // Tell parent viewport that we should not show next frame:
                app.show_capture_modal = false;
                app.show_image_edit_viewport = false;
            }
            if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
                app.show_capture_modal = false;
                app.show_image_edit_viewport = false;
            }
        },
    );
}

fn draw_action_buttons(ctx: &Context, ui: &mut Ui, app: &mut MyApp) {
    ui.horizontal(|ui| {
        if ui.button(fl!("image-edit-viewport-button-retake")).clicked() {
            capture_image(ctx, app);
        }

        if ui.button(fl!("image-edit-viewport-button-crop")).clicked() {
            app.crop_or_roi = CropOrRoi::Crop;
        }

        if ui.button(fl!("image-edit-viewport-button-roi")).clicked() {
            app.crop_or_roi = CropOrRoi::Roi;
            app.use_crop_size = false;
        }

        ui.add_enabled_ui(!app.is_roi_smaller_than_image, |ui| {
            if ui.button(fl!("image-edit-viewport-button-ok")).clicked() {
                match save_cropped_image_and_update_roi(app) {
                    Ok(()) => {
                        println!("성공적으로 저장됨");
                        app.project.save_file();
                    }
                    Err(err) => {
                        eprintln!("{err}");
                        app.error_message = Some(err);
                    }
                }
                app.needs_image_update = true;
                app.show_capture_modal = false;
                app.show_image_edit_viewport = false;
            }
        });

        if ui.button(fl!("image-edit-viewport-button-cancel")).clicked() {
            app.show_capture_modal = false;
            app.show_image_edit_viewport = false;
        }
    });
}

fn draw_image_section(ctx: &Context, ui: &mut Ui, app: &mut MyApp, image_size: &mut Vec2) {
    ui.group(|ui| {
        if let Some(_item) = &app.selected_item {
            if let Some((_, texture)) = &app.capture_cache {
                *image_size = texture.size_vec2();
                draw_image_canvas(ctx, ui, app, texture.clone(), *image_size);
            };
        } else {
            ui.centered_and_justified(|ui| {
                ui.label(fl!("image-preview-panel-no-selection"));
            });
        }
    });
}

fn draw_image_canvas(ctx: &Context, ui: &mut Ui, app: &mut MyApp, texture: egui::TextureHandle, image_size: Vec2) {
    let aspect = image_size.y / image_size.x;
    let mut display_width = image_size.x;
    let mut display_height = image_size.y;

    // 먼저 너비 기준으로 줄이되, 높이가 초과되면 다시 높이 기준으로 계산
    if display_width > MAX_IMAGE_WIDTH {
        display_width = MAX_IMAGE_WIDTH;
        display_height = display_width * aspect;
    }
    if display_height > MAX_IMAGE_HEIGHT {
        display_height = MAX_IMAGE_HEIGHT;
        display_width = display_height / aspect;
    }

    ui.set_width(display_width);
    ui.set_height(display_height);

    ui.centered_and_justified(|ui| {
        if app.local_a.is_none() {
            app.local_a = Some(Pos2::default());
        }
        if app.local_b.is_none() {
            app.local_b = Some(image_size.to_pos2());
        }

        let (response, painter) = ui.allocate_painter(egui::Vec2::new(display_width, display_height), egui::Sense::click_and_drag());

        Image::new(&texture)
            .max_size(egui::Vec2::new(display_width, display_height))
            .paint_at(ui, response.rect);

        let image_rect = Rect::from_min_size(
            (response.rect.center()
                - (Pos2 {
                    x: display_width,
                    y: display_height,
                } / 2.0))
                .to_pos2(),
            Vec2::new(display_width, display_height),
        );

        handle_drag_selection(&response, app, image_rect, image_size, Vec2::new(display_width, display_height));

        if response.drag_stopped() {
            println!("크롭 좌표: {:?} -> {:?}", app.drag_start, app.drag_end);
        }

        // Draw selection rectangle
        let scale = Vec2::new(display_width / image_size.x, display_height / image_size.y);
        let screen_crop_a = app.local_a.map(|pos| to_screen_pos2(pos, image_rect, scale));
        let screen_crop_b = app.local_b.map(|pos| to_screen_pos2(pos, image_rect, scale));
        let screen_roi_a = app.roi_a.map(|pos| to_screen_pos2(pos, image_rect, scale));
        let screen_roi_b = app.roi_b.map(|pos| to_screen_pos2(pos, image_rect, scale));

        if let (Some(a), Some(b)) = (screen_crop_a, screen_crop_b) {
            painter.rect_stroke(Rect::from_two_pos(a, b), 0.0, Stroke::new(2.0, Color32::RED), StrokeKind::Middle);
        }

        if !app.use_crop_size {
            if let (Some(a), Some(b)) = (screen_roi_a, screen_roi_b) {
                painter.rect_stroke(Rect::from_two_pos(a, b), 0.0, Stroke::new(2.0, Color32::BLUE), StrokeKind::Middle);
            }
        }

        let mut inner_size = Vec2::new(display_width + 200.0, display_height + 120.0);
        if inner_size.x < 700.0 {
            inner_size.x = 700.0;
        }
        if inner_size.y < 300.0 {
            inner_size.y = 300.0;
        }

        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(inner_size));
    });
}

fn draw_position_controls(ui: &mut Ui, app: &mut MyApp, image_size: Vec2) {
    ui.group(|ui| {
        if let (Some(start), Some(end)) = (app.local_a.as_mut(), app.local_b.as_mut()) {
            ui.label(fl!("image-edit-viewport-label-image-range"));
            ui.label("pos a: ");
            ui.horizontal(|ui| {
                ui.label("x: ");
                ui.add(egui::DragValue::new(&mut start.x).range(0.0..=image_size.x).speed(1.0));
                ui.label("y: ");
                ui.add(egui::DragValue::new(&mut start.y).range(0.0..=image_size.y).speed(1.0));
            });
            ui.label("pos b: ");
            ui.horizontal(|ui| {
                ui.label("x: ");
                ui.add(egui::DragValue::new(&mut end.x).range(0.0..=image_size.x).speed(1.0));
                ui.label("y: ");
                ui.add(egui::DragValue::new(&mut end.y).range(0.0..=image_size.y).speed(1.0));
            });
        }
    });
    ui.group(|ui| {
        ui.label(fl!("image-edit-viewport-label-roi-range"));
        ui.checkbox(&mut app.use_crop_size, fl!("image-edit-viewport-checkbox-use-crop"));
        if !app.use_crop_size {
            if let (Some(crop_start), Some(crop_end), Some(roi_start), Some(roi_end)) =
                (app.local_a, app.local_b, app.roi_a.as_mut(), app.roi_b.as_mut())
            {
                ui.label("pos a: ");
                ui.horizontal(|ui| {
                    ui.label("x: ");
                    ui.add(egui::DragValue::new(&mut roi_start.x).range(0.0..=image_size.x).speed(1.0));
                    ui.label("y: ");
                    ui.add(egui::DragValue::new(&mut roi_start.y).range(0.0..=image_size.y).speed(1.0));
                });
                ui.label("pos b: ");
                ui.horizontal(|ui| {
                    ui.label("x: ");
                    ui.add(egui::DragValue::new(&mut roi_end.x).range(0.0..=image_size.x).speed(1.0));
                    ui.label("y: ");
                    ui.add(egui::DragValue::new(&mut roi_end.y).range(0.0..=image_size.y).speed(1.0));
                });

                let crop_rect = Rect::from_two_pos(crop_start, crop_end);
                let roi_rect = Rect::from_two_pos(*roi_start, *roi_end);

                if crop_rect.width() > roi_rect.width() || crop_rect.height() > roi_rect.height() {
                    ui.colored_label(Color32::YELLOW, fl!("image-edit-viewport-warning-roi-size"));
                    app.is_roi_smaller_than_image = true;
                } else {
                    app.is_roi_smaller_than_image = false;
                }
            }
        }
    });
    ui.group(|ui| {
        ui.label(format!("image_size {:?}", image_size));
    });
}

fn normalize_and_scale(a: Pos2, b: Pos2, image_size: Vec2, display_size: Vec2) -> (Pos2, Pos2) {
    let rect = Rect::from_two_pos(a, b);

    let scale_x = image_size.x / display_size.x;
    let scale_y = image_size.y / display_size.y;

    let start_scaled = Pos2::new((rect.min.x * scale_x).round(), (rect.min.y * scale_y).round());
    let end_scaled = Pos2::new((rect.max.x * scale_x).round(), (rect.max.y * scale_y).round());

    (start_scaled, end_scaled)
}

fn save_cropped_image_and_update_roi(app: &mut MyApp) -> Result<(), String> {
    let index = app.selected_item_index.ok_or("선택된 항목이 없습니다")?;
    let image = app.dynamic_image_cache.clone().ok_or("이미지가 없습니다")?;
    let project_path = app.project.path.as_ref().ok_or("프로젝트 경로가 없습니다")?;

    let uuid = uuid::Uuid::new_v4();
    let file_name = format!("{uuid}.png");
    let path = Path::new(project_path).join("image").join(&file_name);

    let crop_rect = Rect::from_two_pos(
        app.local_a.ok_or("크롭 시작 좌표가 없습니다")?,
        app.local_b.ok_or("크롭 끝 좌표가 없습니다")?,
    );
    let roi_rect = if app.use_crop_size || app.roi_a.is_none() || app.roi_b.is_none() {
        crop_rect
    } else {
        Rect::from_two_pos(
            app.roi_a.ok_or("ROI 시작 좌표가 없습니다")?,
            app.roi_b.ok_or("ROI 끝 좌표가 없습니다")?,
        )
    };

    let x = crop_rect.min.x as u32;
    let y = crop_rect.min.y as u32;
    let width = crop_rect.width() as u32;
    let height = crop_rect.height() as u32;

    let roi = &mut app.project.items[index].roi;
    roi.x = roi_rect.min.x as u32;
    roi.y = roi_rect.min.y as u32;
    roi.width = roi_rect.width() as u32;
    roi.height = roi_rect.height() as u32;

    app.project.items[index].image_path = Some(file_name);

    capture::save_image(image, Some(((x, y), (width, height))), &path);
    println!("이미지 저장 완료 {:?}", path);
    Ok(())
}

fn handle_drag_selection(response: &egui::Response, app: &mut MyApp, image_rect: Rect, image_size: Vec2, display_size: Vec2) {
    if response.drag_started() {
        if let Some(pos) = response.interact_pointer_pos() {
            let clamped = pos.clamp(image_rect.min, image_rect.max);
            app.drag_start = Some(clamped);
        }
    }
    if response.dragged() {
        if let Some(pos) = response.interact_pointer_pos() {
            let clamped = pos.clamp(image_rect.min, image_rect.max);
            app.drag_end = Some(clamped);
        }

        if let (Some(drag_start), Some(drag_end)) = (app.drag_start, app.drag_end) {
            let start = to_local_pos(drag_start, image_rect);
            let end = to_local_pos(drag_end, image_rect);
            let (local_start, local_end) = normalize_and_scale(start, end, image_size, display_size);

            match app.crop_or_roi {
                CropOrRoi::Crop => {
                    app.local_a = Some(local_start);
                    app.local_b = Some(local_end);
                }
                CropOrRoi::Roi => {
                    app.roi_a = Some(local_start);
                    app.roi_b = Some(local_end);
                }
            }
        }
    }
}
