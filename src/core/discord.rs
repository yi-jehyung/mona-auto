use std::{io::Cursor, time::Duration};

use image::ImageFormat;
use reqwest::blocking::{multipart, Client};
use windows::Win32::Foundation::HWND;

use super::{capture, Setting};

pub fn send_discord(webhook_url: String, message: String) -> Result<(), String> {
    let name = "Monadium";

    let form = multipart::Form::new()
        .text("username", name)
        .text("content", message);

    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|e| format!("{:?}", e))?;

    let response = match client.post(webhook_url).multipart(form).send() {
        Ok(resp) => resp,
        Err(e) => return Err(format!("{:?}", e)),
    };

    if response.status().is_success() {
        Ok(())
    } else {
        let error_text = match response.text() {
            Ok(text) => text,
            Err(_) => "Unknown error".into(),
        };
        Err(format!("{:?}", error_text))
    }
}

pub fn send_discord_with_screenshot(webhook_url: String, message: String, hwnd_ptr: usize, setting: &Setting) -> Result<(), String> {
    let frame_image = match capture::capture_from_hwnd(HWND(hwnd_ptr as *mut std::ffi::c_void), setting.capture_type) {
        Ok(img) => Ok(img),
        Err(err) => {
            Err(err.to_string())
        }
    }?;
    
    let mut buf = Cursor::new(Vec::new());
    frame_image.write_to(&mut buf, ImageFormat::Png).unwrap();
    let png_bytes = buf.into_inner();
    
    let name = "Monadium";

    let form = multipart::Form::new()
        .text("username", name)
        .part("file", multipart::Part::bytes(png_bytes).file_name("screenshot.png"))
        .text("content", message);

    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|e| format!("{:?}", e))?;

    let response = match client.post(webhook_url).multipart(form).send() {
        Ok(resp) => resp,
        Err(e) => return Err(format!("{:?}", e)),
    };

    if response.status().is_success() {
        Ok(())
    } else {
        match response.text() {
            Ok(text) => Err(text),
            Err(err) => Err(err.to_string()),
        }
    }
}
