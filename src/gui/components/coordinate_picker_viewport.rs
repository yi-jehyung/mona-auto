use egui::{Color32, Context, Image, Pos2, Rect, Ui, Vec2};

use crate::{
    fl,
    gui::{
        defines::*,
        util::{capture_image, to_local_pos, to_screen_pos2},
        MyApp,
    },
};

pub fn show_coordinate_picker_viewport(ctx: &Context, app: &mut MyApp) {
    ctx.show_viewport_immediate(
        egui::ViewportId::from_hash_of("coordinate_picker_viewport"),
        egui::ViewportBuilder::default()
            .with_title("Coordinate Picker")
            .with_inner_size([400.0, 200.0])
            .with_resizable(false),
        |ctx, class| {
            assert!(
                class == egui::ViewportClass::Immediate,
                "This egui backend doesn't support multiple viewports"
            );

            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading(fl!("coordinate-picker-heading"));

                ui.horizontal(|ui| draw_action_buttons(ctx, ui, app));

                ui.add_space(12.0);

                show_image(ctx, ui, app);
            });

            if ctx.input(|i| i.viewport().close_requested()) {
                app.coordinate_picker_enable = false;
            }
            if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
                app.coordinate_picker_enable = false;
            }
        },
    );
}

fn show_image(ctx: &Context, ui: &mut Ui, app: &mut MyApp) {
    ui.horizontal(|ui| {
        let mut image_size = Vec2::ZERO;
        ui.group(|ui| {
            if let Some(_item) = &app.selected_item {
                if let Some((_, texture)) = &app.capture_cache {
                    image_size = texture.size_vec2();
                    draw_image_canvas(ctx, ui, app, texture.clone(), image_size);
                };
            } else {
                ui.centered_and_justified(|ui| {
                    ui.label(fl!("image-preview-panel-no-selection"));
                });
            }
        });
    });
}

fn draw_action_buttons(ctx: &Context, ui: &mut Ui, app: &mut MyApp) {
    if ui.button(fl!("image-edit-viewport-button-retake")).clicked() {
        capture_image(ctx, app);
    }
    if ui.button(fl!("coordinate-picker-button-close")).clicked() {
        app.coordinate_picker_enable = false;
    }
}

fn draw_image_canvas(ctx: &Context, ui: &mut Ui, app: &mut MyApp, texture: egui::TextureHandle, image_size: Vec2) {
    let aspect = image_size.y / image_size.x;
    let mut display_width = image_size.x;
    let mut display_height = image_size.y;

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

        handle_click_selection(&response, app, image_rect, image_size, Vec2::new(display_width, display_height));

        if let Some((x, y)) = app.picked_position {
            let scale = Vec2::new(display_width / image_size.x, display_height / image_size.y);
            let pos = Pos2::new(x as f32, y as f32);
            let screen_pos = to_screen_pos2(pos, image_rect, scale);
            painter.circle_filled(screen_pos, 4.0, Color32::YELLOW);
            painter.text(
                screen_pos + Vec2::new(6.0, 0.0),
                egui::Align2::LEFT_TOP,
                format!("x={x}, y={y}"),
                egui::TextStyle::Body.resolve(ui.style()),
                Color32::WHITE,
            );
        }

        let inner_size = [display_width + 30.0, display_height + 120.0];
        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(inner_size.into()));
    });
}

fn handle_click_selection(response: &egui::Response, app: &mut MyApp, image_rect: Rect, image_size: Vec2, display_size: Vec2) {
    if response.clicked() || response.dragged() {
        if let Some(pos) = response.interact_pointer_pos() {
            let clamped = pos.clamp(image_rect.min, image_rect.max);
            let local = to_local_pos(clamped, image_rect);

            let scale_x = image_size.x / display_size.x;
            let scale_y = image_size.y / display_size.y;

            let scaled = Pos2::new((local.x * scale_x).round(), (local.y * scale_y).round());

            app.picked_position = Some((scaled.x.round() as u32, scaled.y.round() as u32));
        }
    }
}
