use std::{
    sync::{self, atomic, mpsc, Arc},
    thread, time,
};

use crate::{
    core::{
        capture,
        matcher::match_template,
        window::{self, activate_window, bring_window_to_front},
    },
    fl, Project, Setting,
};

pub fn start_automation_loop(
    project: Project,
    setting: Setting,
    tx: mpsc::Sender<u32>,
    stop_flag: sync::Arc<atomic::AtomicBool>,
) -> Vec<thread::JoinHandle<()>> {
    let mut handles = Vec::new();
    let setting = std::sync::Arc::new(setting);
    let tx = std::sync::Arc::new(tx);

    for window in &project.target_windows {
        let mut window = window.clone();
        let mut project = project.clone();
        if let Err(err) = window.rebind_hwnds() {
            eprintln!("{err}");
            continue;
        }

        let setting = Arc::clone(&setting);
        let tx = Arc::clone(&tx);
        let stop_flag = Arc::clone(&stop_flag);
        let items = project.items.clone();
        let path = project.path.clone();

        let handle = thread::spawn(move || {
            let top_hwnd = match window.get_first_hwnd() {
                Ok(hwnd) => hwnd,
                Err(err) => {
                    eprintln!("{}", fl!("message-automation-loop-error-cant-find-window"));
                    eprintln!("{err}");
                    return;
                }
            };
            let low_hwnd = match window.get_last_hwnd() {
                Ok(hwnd) => hwnd,
                Err(err) => {
                    eprintln!("{}", fl!("message-automation-loop-error-cant-find-window"));
                    eprintln!("{err}");
                    return;
                }
            };

            println!("[TARGET HWND] {:?}", low_hwnd);

            let delta = time::Duration::from_secs_f32(1.0 / setting.loop_per_second as f32);

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
                        eprintln!("{}", fl!("message-automation-loop-error-fail-capture", error = err));
                        continue;
                    }
                };

                let gray_frame = frame_image.to_luma8();

                for item in &items {
                    if !item.enabled {
                        continue;
                    }

                    match &item.image_path {
                        Some(image_path) => {
                            let Some(base_path) = &path else { continue };
                            let full_path = format!("{}/image/{}", base_path, image_path);

                            let template = match image::open(full_path) {
                                Ok(img) => img.to_luma8(),
                                Err(err) => {
                                    eprintln!(
                                        "{}",
                                        fl!("message-automation-loop-error-fail-load-template", error = err.to_string())
                                    );
                                    continue;
                                }
                            };

                            let (x, y, score) = match_template(&gray_frame, &template, &item.roi);
                            if score <= setting.threshold {
                                for action in &item.actions {
                                    if action.enabled {
                                        println!(
                                            "{}",
                                            fl!(
                                                "message-automation-loop-template-found",
                                                name = item.name.clone(),
                                                x = x,
                                                y = y,
                                                similarity = score
                                            )
                                        );
                                        action.execute(top_hwnd, low_hwnd, &mut project, &setting, Some((x, y)));
                                    }
                                }
                            }
                        }
                        None => {
                            for action in &item.actions {
                                if action.enabled {
                                    println!("{}", fl!("message-automation-loop-found", name = item.name.clone()));
                                    action.execute(top_hwnd, low_hwnd, &mut project, &setting, Some((0, 0)));
                                }
                            }
                        }
                    }
                }

                let elapsed = time_0.elapsed();
                if elapsed < delta {
                    thread::sleep(delta - elapsed);
                }

                // let frame_time = if elapsed < target_delta_time { target_delta_time } else { elapsed };
                // let fps = 1.0 / frame_time.as_secs_f32();
                // println!("FPS: {:.2}", fps);
            }
        });

        handles.push(handle);
    }

    handles
}
