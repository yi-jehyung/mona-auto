use egui::{ColorImage, Context, Pos2, Rect, TextureHandle, Vec2};
use image::DynamicImage;

use crate::gui::defines::APP_ICON;
use crate::gui::MyApp;

pub fn load_icon() -> eframe::egui::IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::load_from_memory(APP_ICON).expect("Failed to open icon path").into_rgba8();

        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    eframe::egui::IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}

pub fn load_image_cache(app: &mut MyApp, ctx: &Context) {
    if app.selected_item.is_none() {
        app.image_cache = None;
    }
    if !app.needs_image_update {
        return;
    }

    let Some(selected_item) = &app.selected_item else {
        return;
    };
    let Some(path) = &app.project.path else {
        return;
    };

    match &selected_item.image_path {
        Some(image_path) => {
            let image_path = format!("{}/image/{}", path, image_path);

            match image::open(&image_path) {
                Ok(image) => {
                    app.dynamic_image_cache = Some(image.clone());
                    let (color_image, texture) = image_to_texture(ctx, image);
                    app.image_cache = Some((color_image.into(), texture));
                    app.needs_image_update = false;
                }
                Err(err) => {
                    eprintln!("이미지 로딩 실패: {}", err);
                    app.error_message = Some(err.to_string());
                    app.image_cache = None;
                    app.needs_image_update = false;
                }
            }
        }
        None => {
            app.image_cache = None;
            app.needs_image_update = false;
        }
    }
}

pub fn image_to_texture(ctx: &Context, image: DynamicImage) -> (ColorImage, TextureHandle) {
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    let color_image = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
    let texture = ctx.load_texture("preview", color_image.clone(), Default::default());
    (color_image, texture)
}

pub fn capture_image(ctx: &Context, app: &mut MyApp) {
    if let Some(_window) = app.project.target_windows.first() {
        if let Err(err) = app.project.target_windows[0].rebind_hwnds() {
            eprintln!("rebind_hwnds 실패: {err}");
            app.error_message = Some(err);
            return;
        }
    } else {
        eprintln!("창이 없습니다.");
        return;
    }

    let hwnd = match app.project.target_windows.first().unwrap().get_last_hwnd() {
        Ok(hwnd) => hwnd,
        Err(err) => {
            eprintln!("get_last_hwnd 실패 {err}");
            app.error_message = Some(err);
            return;
        }
    };

    match crate::core::capture::capture_from_hwnd(hwnd, app.setting.capture_type) {
        Ok(image) => {
            app.dynamic_image_cache = Some(image.clone());
            let (color_image, texture) = image_to_texture(ctx, image);
            app.capture_cache = Some((color_image.into(), texture));
        }
        Err(err) => {
            println!("capture_from_hwnd 실패 {}", err);
            app.error_message = Some(err);
        }
    }
}

pub fn to_local_pos(pos: Pos2, image_rect: Rect) -> Pos2 {
    (pos - image_rect.min).to_pos2()
}

pub fn to_screen_pos2(pos: Pos2, image_rect: Rect, scale: Vec2) -> Pos2 {
    image_rect.min + (pos.to_vec2() * scale)
}
