use std::{fs, path::Path};

use image::{DynamicImage, RgbaImage};
use win_screenshot::prelude::{capture_window_ex, Area, Using};
use windows::Win32::Foundation::HWND;

use crate::setting::CaptureType;

pub fn capture_from_hwnd(hwnd: HWND, capture_type: CaptureType) -> Result<DynamicImage, String> {
    let buf = match capture_type {
        CaptureType::BitBlt => {
            capture_window_ex(hwnd.0 as isize, Using::BitBlt, Area::Full, None, None)
                .map_err(|e| format!("{e}"))?
        }
        CaptureType::PrintWindow => {
            capture_window_ex(hwnd.0 as isize, Using::PrintWindow, Area::Full, None, None)
                .map_err(|e| format!("{e}"))?
        }
    };
    let img = DynamicImage::ImageRgba8(
        // RgbaImage::from_raw(buf.width, buf.height, buf.pixels).ok_or("이미지 생성 실패")?,
        RgbaImage::from_raw(buf.width, buf.height, buf.pixels).ok_or("Failed to create image")?,
    );
    Ok(img)
}

pub fn save_image(
    dynamic_image: DynamicImage,
    area: Option<((u32, u32), (u32, u32))>,
    path: &Path,
) {
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            if let Err(err) = fs::create_dir_all(parent) {
                eprintln!("{}", err);
                return;
            }
        }
    }

    let mut image = dynamic_image.clone();
    if let Some(((x, y), (width, height))) = area {
        image = image.crop(x, y, width, height);
    }
    match image.save(path) {
        Ok(_) => {}
        Err(err) => eprintln!("{err}"),
    };
}
