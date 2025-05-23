use std::{
    sync::{self, atomic, mpsc},
    thread, time,
};

use i18n_embed::{
    fluent::{fluent_language_loader, FluentLanguageLoader},
    LanguageLoader,
};
use i18n_embed_fl::fl;
use image::GrayImage;
use unic_langid::LanguageIdentifier;

use crate::{
    core::{
        capture,
        matcher::match_template,
        window::{self, activate_window, bring_window_to_front},
    },
    Localizations, Project, Setting,
};

pub fn start_automation_loop(
    project: Project,
    setting: Setting,
    tx: mpsc::Sender<u32>,
    stop_flag: sync::Arc<atomic::AtomicBool>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let loader: FluentLanguageLoader = fluent_language_loader!();
        loader
            .load_languages(&Localizations, &[loader.fallback_language().clone()])
            .unwrap();
        let langid: LanguageIdentifier = setting
            .language
            .to_string()
            .parse()
            .expect("Invalid language identifier");
        loader
            .load_languages(&Localizations, &[langid])
            .expect("Failed to load language.");

        let mut project = project.clone();

        match project.windows.rebind_hwnds() {
            Ok(_) => {}
            Err(err) => {
                eprintln!("{}", err);
            }
        }

        let top_hwnd = match project.get_first_hwnd(&loader) {
            Ok(hwnd) => hwnd,
            Err(_err) => {
                eprintln!("{}",fl!(loader, "message-automation-loop-error-cant-find-window"));
                return; // 스레드 종료
            }
        };
        let low_hwnd = match project.get_last_hwnd(&loader) {
            Ok(hwnd) => hwnd,
            Err(_err) => {
                eprintln!("{}",fl!(loader, "message-automation-loop-error-cant-find-window"));
                return; // 스레드 종료
            }
        };
        println!("[TARGET HWND] {:?}", low_hwnd);

        let target_delta_time = time::Duration::from_secs_f32(1.0 / setting.loop_per_second as f32);

        while !stop_flag.load(atomic::Ordering::SeqCst) {
            if window::is_f8_pressed() && tx.send(1).is_err() {
                break;
            }

            let time_0 = time::Instant::now();

            match setting.input_type {
                crate::core::InputType::PostMessage => {
                    activate_window(top_hwnd);
                    activate_window(low_hwnd);
                }
                crate::core::InputType::SendInput => {
                    bring_window_to_front(top_hwnd);
                    bring_window_to_front(low_hwnd);
                }
                _ => {}
            }

            let frame_image = match capture::capture_from_hwnd(low_hwnd, setting.capture_type) {
                Ok(img) => img,
                Err(err) => {
                    eprintln!("{}",fl!(loader, "message-automation-loop-error-fail-capture", error = err));
                    continue;
                }
            };

            // 이미지 처리: DynamicImage -> GrayImage
            let gray_frame: GrayImage = frame_image.to_luma8();

            let len = project.items.len();
            for i in 0..len {
                let item = project.items[i].clone();
                if !item.enabled {
                    continue;
                }

                let Some(path) = &project.path else { return };

                match item.image_path {
                    Some(image_path) => {
                        let image_path = format!("{}/image/{}", path, image_path);
                        let gray_template = match image::open(image_path) {
                            Ok(img) => img.to_luma8(),
                            Err(err) => {
                                eprintln!("{}", fl!(loader, "message-automation-loop-error-fail-load-template", error = err.to_string()));
                                continue;
                            }
                        };
                        let threshold = setting.threshold;
                        let (x, y, score) = match_template(&gray_frame, &gray_template, &item.roi);

                        if score <= threshold {
                            for action in item.actions.iter() {
                                if action.enabled {
                                    println!("{}", fl!(loader, "message-automation-loop-template-found", name = item.name.clone(), x = x, y = y, similarity = score));
                                    action.execute(top_hwnd, low_hwnd, &mut project, &setting, Some((x, y)), &loader);
                                }
                            }
                        } else {
                            // eprintln!("{}", fl!(loader, "message-automation-loop-error-match-failed", similarity = score));
                        }
                    }
                    None => {
                        for action in item.actions.iter() {
                            if action.enabled {
                                println!("{}", fl!(loader, "message-automation-loop-found", name = item.name.clone()));
                                action.execute(top_hwnd, low_hwnd, &mut project, &setting, Some((0, 0)), &loader);
                            }
                        }
                    }
                }
            }

            let elapsed = time_0.elapsed();
            if elapsed < target_delta_time {
                thread::sleep(target_delta_time - elapsed);
            }

            // let frame_time = if elapsed < target_delta_time { target_delta_time } else { elapsed };
            // let fps = 1.0 / frame_time.as_secs_f32();
            // println!("FPS: {:.2}", fps);
        }
    })
}
