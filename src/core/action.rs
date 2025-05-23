use std::fmt;

use i18n_embed::fluent::FluentLanguageLoader;
use i18n_embed_fl::fl;
use serde::{Deserialize, Serialize};

use crate::core::{
    window::{activate_window, bring_window_to_front},
    Input, Project, Setting,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    // pub id: u32,
    pub enabled: bool,
    pub action: ActionType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionType {
    LeftClick {
        x: u32,
        y: u32,
        use_matched_position: bool,
    },
    RightClick {
        x: u32,
        y: u32,
        use_matched_position: bool,
    },
    Drag {
        start: (u32, u32),
        end: (u32, u32),
        duration_ms: u32,
    },
    Scroll {
        x: u32,
        y: u32,
        delta: i16,
        use_matched_position: bool,
    },
    KeyInput {
        keys: Vec<KeyType>,
    },
    TextInput {
        text: String,
    },
    Delay {
        millis: u64,
    },
    SendDiscord {
        webhook_url: String,
        message: String,
        use_screenshot: bool,
    },
    ToggleEnable {
        target: ToggleTarget,
        enable: bool,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyType {
    Down(KeyCode),
    Up(KeyCode),
    DownAndUp(KeyCode),
    Delay(u64),
}

#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyCode {
    A, B, C, D, E, F, G, H, I, J, K, L, M,
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    Num0, Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9,
    NumPad0, NumPad1, NumPad2, NumPad3, NumPad4, NumPad5, NumPad6, NumPad7, NumPad8, NumPad9,
    Enter, Escape, Backspace, Tab, Space,
    Shift, Ctrl, Alt,
    Left, Up, Right, Down,
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    Delete, Insert, Home, End, PageUp, PageDown,
    Custom(u16),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToggleTarget {
    Image(String),
}

impl Action {
    pub fn execute(
        &self,
        top_hwnd: windows::Win32::Foundation::HWND,
        low_hwnd: windows::Win32::Foundation::HWND,
        project: &mut Project,
        setting: &Setting,
        matched_pos: Option<(u32, u32)>,
        loader: &FluentLanguageLoader,
    ) {
        let input = Input::new(low_hwnd);

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
        
        match &self.action {
            ActionType::LeftClick {
                x,
                y,
                use_matched_position,
            } => match use_matched_position {
                true => match matched_pos {
                    Some((matched_x, matched_y)) => {
                        println!("{}", fl!(loader, "message-action-left-click", x = matched_x, y = matched_y));
                        match setting.input_type {
                            crate::core::InputType::PostMessage => input.left_click(matched_x, matched_y),
                            crate::core::InputType::SendInput => input.left_click_send_input(matched_x, matched_y),
                            _ => {}
                        }
                    }
                    None => {
                        println!("{}", fl!(loader, "message-action-error-cant-find-matched-position", x = x, y = y));
                        println!("{}", fl!(loader, "message-action-left-click", x = x, y = y));
                        match setting.input_type {
                            crate::core::InputType::PostMessage => input.left_click(*x, *y),
                            crate::core::InputType::SendInput => input.left_click_send_input(*x, *y),
                            _ => {}
                        }
                    }
                },
                false => {
                    println!("{}", fl!(loader, "message-action-left-click", x = x, y = y));
                    match setting.input_type {
                        crate::core::InputType::PostMessage => input.left_click(*x, *y),
                        crate::core::InputType::SendInput => input.left_click_send_input(*x, *y),
                        _ => {}
                    }
                }
            },
            ActionType::RightClick {
                x,
                y,
                use_matched_position,
            } => match use_matched_position {
                true => match matched_pos {
                    Some((matched_x, matched_y)) => {
                        println!("{}", fl!(loader, "message-action-right-click", x = matched_x, y = matched_y));
                        match setting.input_type {
                            crate::core::InputType::PostMessage => input.right_click(matched_x, matched_y),
                            crate::core::InputType::SendInput => input.right_click_send_input(matched_x, matched_y),
                            _ => {}
                        }
                    }
                    None => {
                        println!("{}", fl!(loader, "message-action-error-cant-find-matched-position", x = x, y = y));
                        println!("{}", fl!(loader, "message-action-right-click", x = x, y = y));
                        match setting.input_type {
                            crate::core::InputType::PostMessage => input.right_click(*x, *y),
                            crate::core::InputType::SendInput => input.right_click_send_input(*x, *y),
                            _ => {}
                        }
                    }
                },
                false => {
                    println!("{}", fl!(loader, "message-action-right-click", x = x, y = y));
                    match setting.input_type {
                        crate::core::InputType::PostMessage => input.right_click(*x, *y),
                        crate::core::InputType::SendInput => input.right_click_send_input(*x, *y),
                        _ => {}
                    }
                }
            },
            ActionType::Drag { start, end, duration_ms } => {
                println!(
                    "{}",
                    fl!(loader, "message-action-drag", x1 = start.0, y1 = start.1, x2 = end.0, y2 = end.1)
                );

                match setting.input_type {
                    crate::core::InputType::PostMessage => input.drag(*start, *end, *duration_ms),
                    crate::core::InputType::SendInput => input.drag_send_input(*start, *end, *duration_ms),
                    _ => {}
                }
            }
            ActionType::Scroll {
                x,
                y,
                delta,
                use_matched_position,
            } => match use_matched_position {
                true => match matched_pos {
                    Some((matched_x, matched_y)) => {
                        println!("{}", fl!(loader, "message-action-scroll", x = x, y = y));
                        match setting.input_type {
                            crate::core::InputType::PostMessage => input.scroll(matched_x, matched_y, *delta),
                            crate::core::InputType::SendInput => input.scroll_send_input(matched_x, matched_y, *delta),
                            _ => {}
                        }
                    }
                    None => {
                        println!("{}", fl!(loader, "message-action-error-cant-find-matched-position", x = x, y = y));
                        println!("{}", fl!(loader, "message-action-scroll", x = x, y = y));
                        match setting.input_type {
                            crate::core::InputType::PostMessage => input.scroll(*x, *y, *delta),
                            crate::core::InputType::SendInput => input.scroll_send_input(*x, *y, *delta),
                            _ => {}
                        }
                    }
                },
                false => {
                    println!("{}", fl!(loader, "message-action-scroll", x = x, y = y));
                    match setting.input_type {
                        crate::core::InputType::PostMessage => input.scroll(*x, *y, *delta),
                        crate::core::InputType::SendInput => input.scroll_send_input(*x, *y, *delta),
                        _ => {}
                    }
                }
            },
            ActionType::KeyInput { keys } => {
                println!("{}", fl!(loader, "message-action-key-input"));
                for key in keys {
                    match key {
                        KeyType::Down(key_code) => {
                            match setting.input_type {
                                crate::core::InputType::PostMessage => input.key_down(key_code.to_vk()),
                                crate::core::InputType::SendInput => input.key_down_send_input(key_code.to_vk()),
                                _ => {}
                            };
                        }
                        KeyType::Up(key_code) => {
                            match setting.input_type {
                                crate::core::InputType::PostMessage => input.key_up(key_code.to_vk()),
                                crate::core::InputType::SendInput => input.key_up_send_input(key_code.to_vk()),
                                _ => {}
                            };
                        }
                        KeyType::DownAndUp(key_code) => {
                            match setting.input_type {
                                crate::core::InputType::PostMessage => {
                                    input.key_down(key_code.to_vk());
                                    input.wait(50);
                                    input.key_up(key_code.to_vk());
                                }
                                crate::core::InputType::SendInput => {
                                    input.key_down_send_input(key_code.to_vk());
                                    input.wait(50);
                                    input.key_up_send_input(key_code.to_vk());
                                }
                                _ => {}
                            };
                        }
                        KeyType::Delay(delay) => {
                            input.wait(*delay);
                        }
                    }
                }
            }
            ActionType::TextInput { text } => {
                println!("{}", fl!(loader, "message-action-text-input", text = text));
                input.input_text(text);
            }
            ActionType::Delay { millis } => {
                println!("{}", fl!(loader, "message-action-delay", millis = millis));
                input.wait(*millis)
            },
            ActionType::SendDiscord {
                webhook_url,
                message,
                use_screenshot,
            } => {                
                let webhook_url = webhook_url.clone();
                let message = message.clone();
                let hwnd_ptr = low_hwnd.0 as usize;
                let setting = setting.clone();
                if !use_screenshot {
                    println!("{}", fl!(loader, "message-action-send-discord", message = message.clone()));
                    std::thread::spawn(move || match crate::discord::send_discord(webhook_url, message) {
                        Ok(_) => {}
                        Err(err) => eprintln!("{err}"),
                    });
                } else {
                    println!("{}", fl!(loader, "message-action-send-discord-with-screenshot", message = message.clone()));
                    std::thread::spawn(move || {
                        match crate::discord::send_discord_with_screenshot(webhook_url, message, hwnd_ptr, &setting) {
                            Ok(_) => {}
                            Err(err) => eprintln!("{err}"),
                        }
                    });
                }
            }
            ActionType::ToggleEnable { target, enable } => match target {
                ToggleTarget::Image(name) => {
                    if *enable {
                        println!("{}", fl!(loader, "message-action-toogle-enable", name = name));
                    } else {
                        println!("{}", fl!(loader, "message-action-toogle-disable", name = name));
                    }
                    let len = project.items.len();
                    for i in 0..len {
                        if project.items[i].name == *name {
                            project.items[i].enabled = *enable;
                        }
                    }
                }
            },
        }
    }
}

impl ActionType {
    pub fn to_localized_string(&self, loader: &FluentLanguageLoader) -> String {
        match self {
            ActionType::LeftClick {
                x,
                y,
                use_matched_position,
            } => {
                if *use_matched_position {
                    fl!(loader, "action-display-leftclick-image")
                } else {
                    fl!(loader, "action-display-leftclick", x = x, y = y)
                }
            }

            ActionType::RightClick {
                x,
                y,
                use_matched_position,
            } => {
                if *use_matched_position {
                    fl!(loader, "action-display-rightclick-image")
                } else {
                    fl!(loader, "action-display-rightclick", x = x, y = y)
                }
            }

            ActionType::Drag { start, end, .. } => {
                fl!(
                    loader,
                    "action-display-drag",
                    start_x = start.0,
                    start_y = start.1,
                    end_x = end.0,
                    end_y = end.1
                )
            }

            ActionType::Scroll {
                x,
                y,
                delta,
                use_matched_position,
            } => {
                if *delta > 0 {
                    if *use_matched_position {
                        fl!(loader, "action-display-scroll-up-image")
                    } else {
                        fl!(loader, "action-display-scroll-up", x = x, y = y)
                    }
                } else if *use_matched_position {
                    fl!(loader, "action-display-scroll-down-image")
                } else {
                    fl!(loader, "action-display-scroll-down", x = x, y = y)
                }
            }

            ActionType::KeyInput { .. } => {
                fl!(loader, "action-display-keyinput")
            }

            ActionType::TextInput { .. } => {
                fl!(loader, "action-display-textinput")
            }

            ActionType::Delay { millis } => {
                fl!(loader, "action-display-delay", millis = millis)
            }

            ActionType::SendDiscord { .. } => {
                fl!(loader, "action-display-send-discord")
            }

            ActionType::ToggleEnable { target, enable } => {
                let ToggleTarget::Image(name) = target;

                if *enable {
                    fl!(loader, "action-display-enable", name = name)
                } else {
                    fl!(loader, "action-display-disable", name = name)
                }
            }
        }
    }
}

impl KeyType {
    pub fn default() -> Self {
        KeyType::DownAndUp(KeyCode::A)
    }
}

impl fmt::Display for KeyType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use KeyType::*;
        match self {
            Down(key) => write!(f, "↓ {}", key),
            Up(key) => write!(f, "↑ {}", key),
            DownAndUp(key) => write!(f, "⇵ {}", key),
            Delay(key) => write!(f, "{}ms", key),
        }
    }
}

#[rustfmt::skip]
impl KeyCode {
    pub fn to_vk(self) -> u16 {
        use KeyCode::*;
        match self {
            A => 0x41, B => 0x42, C => 0x43, D => 0x44, E => 0x45,
            F => 0x46, G => 0x47, H => 0x48, I => 0x49, J => 0x4A,
            K => 0x4B, L => 0x4C, M => 0x4D, N => 0x4E, O => 0x4F,
            P => 0x50, Q => 0x51, R => 0x52, S => 0x53, T => 0x54,
            U => 0x55, V => 0x56, W => 0x57, X => 0x58, Y => 0x59, Z => 0x5A,

            Num0 => 0x30, Num1 => 0x31, Num2 => 0x32, Num3 => 0x33, Num4 => 0x34,
            Num5 => 0x35, Num6 => 0x36, Num7 => 0x37, Num8 => 0x38, Num9 => 0x39,

            NumPad0 => 0x60, NumPad1 => 0x61, NumPad2 => 0x62, NumPad3 => 0x63, NumPad4 => 0x64,
            NumPad5 => 0x65, NumPad6 => 0x66, NumPad7 => 0x67, NumPad8 => 0x68, NumPad9 => 0x69,
            
            Enter => 0x0D, Escape => 0x1B, Backspace => 0x08, Tab => 0x09, Space => 0x20,
            Shift => 0x10, Ctrl => 0x11, Alt => 0x12,

            Left => 0x25, Up => 0x26, Right => 0x27, Down => 0x28,

            F1 => 0x70, F2 => 0x71, F3 => 0x72, F4 => 0x73, F5 => 0x74,
            F6 => 0x75, F7 => 0x76, F8 => 0x77, F9 => 0x78, F10 => 0x79,
            F11 => 0x7A, F12 => 0x7B,

            Delete => 0x2E, Insert => 0x2D, Home => 0x24, End => 0x23,
            PageUp => 0x21, PageDown => 0x22,

            Custom(key_code) => key_code,
        }
    }
}

impl fmt::Display for KeyCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use KeyCode::*;
        match self {
            Escape => write!(f, "Esc"),
            Delete => write!(f, "Del"),
            Insert => write!(f, "Ins"),
            PageUp => write!(f, "PgUp"),
            PageDown => write!(f, "PgDn"),
            Custom(code) => write!(f, "Custom({})", code),
            _ => write!(f, "{:?}", self),
        }
    }
}
