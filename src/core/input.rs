use std::{thread::sleep, time::Duration};

use windows::Win32::{
    Foundation::{HWND, LPARAM, WPARAM},
    UI::{
        Input::KeyboardAndMouse::{
            SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, INPUT_MOUSE, KEYBDINPUT, KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP, MOUSEEVENTF_HWHEEL,
            MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP, MOUSEEVENTF_WHEEL, MOUSEINPUT,
            VIRTUAL_KEY,
        },
        WindowsAndMessaging::{
            PostMessageW, SetCursorPos, WM_CHAR, WM_KEYDOWN, WM_KEYUP, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MOUSEHWHEEL, WM_MOUSEMOVE,
            WM_MOUSEWHEEL, WM_RBUTTONDOWN, WM_RBUTTONUP,
        },
    },
};

use crate::core::window;

pub struct Input {
    pub hwnd: HWND,
}

impl Input {
    pub fn new(hwnd: HWND) -> Self {
        Self { hwnd }
    }

    pub fn left_click(&self, x: u32, y: u32) {
        let lparam = LPARAM((y * 0x10000 + x) as isize);
        unsafe {
            let _ = PostMessageW(Some(self.hwnd), WM_LBUTTONDOWN, WPARAM(1), lparam);
            let _ = PostMessageW(Some(self.hwnd), WM_LBUTTONUP, WPARAM(0), lparam);
        }
    }

    pub fn left_click_send_input(&self, x: u32, y: u32) {
        let x = x as i32;
        let y = y as i32;

        if let Some((screen_x, screen_y)) = window::client_to_screen(self.hwnd, x, y) {
            unsafe {
                let _ = SetCursorPos(screen_x, screen_y);
            }

            let inputs = [
                INPUT {
                    r#type: INPUT_MOUSE,
                    Anonymous: INPUT_0 {
                        mi: MOUSEINPUT {
                            dx: 0,
                            dy: 0,
                            mouseData: 0,
                            dwFlags: MOUSEEVENTF_LEFTDOWN,
                            time: 0,
                            dwExtraInfo: 0,
                        },
                    },
                },
                INPUT {
                    r#type: INPUT_MOUSE,
                    Anonymous: INPUT_0 {
                        mi: MOUSEINPUT {
                            dx: 0,
                            dy: 0,
                            mouseData: 0,
                            dwFlags: MOUSEEVENTF_LEFTUP,
                            time: 0,
                            dwExtraInfo: 0,
                        },
                    },
                },
            ];

            unsafe {
                SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
            }
        }
    }

    pub fn right_click(&self, x: u32, y: u32) {
        let lparam = LPARAM((y * 0x10000 + x) as isize);
        unsafe {
            let _ = PostMessageW(Some(self.hwnd), WM_RBUTTONDOWN, WPARAM(1), lparam);
            let _ = PostMessageW(Some(self.hwnd), WM_RBUTTONUP, WPARAM(0), lparam);
        }
    }

    pub fn right_click_send_input(&self, x: u32, y: u32) {
        let x = x as i32;
        let y = y as i32;

        if let Some((screen_x, screen_y)) = window::client_to_screen(self.hwnd, x, y) {
            unsafe {
                let _ = SetCursorPos(screen_x, screen_y);
            }

            let inputs = [
                INPUT {
                    r#type: INPUT_MOUSE,
                    Anonymous: INPUT_0 {
                        mi: MOUSEINPUT {
                            dx: 0,
                            dy: 0,
                            mouseData: 0,
                            dwFlags: MOUSEEVENTF_RIGHTDOWN,
                            time: 0,
                            dwExtraInfo: 0,
                        },
                    },
                },
                INPUT {
                    r#type: INPUT_MOUSE,
                    Anonymous: INPUT_0 {
                        mi: MOUSEINPUT {
                            dx: 0,
                            dy: 0,
                            mouseData: 0,
                            dwFlags: MOUSEEVENTF_RIGHTUP,
                            time: 0,
                            dwExtraInfo: 0,
                        },
                    },
                },
            ];

            unsafe {
                SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
            }
        }
    }

    // delta 120 위로 스크롤, -120 아래로 스크롤
    pub fn scroll(&self, x: u32, y: u32, delta: i16) {
        let lparam = makelparam(x as i32, y as i32);
        let wparam = WPARAM(((delta as i32) as usize) << 16);
        unsafe {
            let _ = PostMessageW(Some(self.hwnd), WM_MOUSEWHEEL, wparam, lparam);
        }
    }

    pub fn scroll_send_input(&self, x: u32, y: u32, delta: i16) {
        let x = x as i32;
        let y = y as i32;

        if let Some((screen_x, screen_y)) = window::client_to_screen(self.hwnd, x, y) {
            unsafe {
                let _ = SetCursorPos(screen_x, screen_y);
            }

            let input = INPUT {
                r#type: INPUT_MOUSE,
                Anonymous: INPUT_0 {
                    mi: MOUSEINPUT {
                        dx: 0,
                        dy: 0,
                        mouseData: delta as u32,
                        dwFlags: MOUSEEVENTF_WHEEL,
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            };

            unsafe {
                SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
            }
        }
    }

    /// delta 120 오른쪽으로 스크롤, -120 왼쪽으로 스크롤
    #[allow(dead_code)]
    pub fn scroll_horizontal(&self, x: u32, y: u32, delta: i16) {
        let lparam = makelparam(x as i32, y as i32);
        let wparam = WPARAM(((delta as i32) as usize) << 16);
        unsafe {
            let _ = PostMessageW(Some(self.hwnd), WM_MOUSEHWHEEL, wparam, lparam);
        }
    }

    #[allow(dead_code)]
    pub fn scroll_horizontal_send_input(&self, x: u32, y: u32, delta: i16) {
        let x = x as i32;
        let y = y as i32;

        if let Some((screen_x, screen_y)) = window::client_to_screen(self.hwnd, x, y) {
            unsafe {
                let _ = SetCursorPos(screen_x, screen_y);
            }

            let input = INPUT {
                r#type: INPUT_MOUSE,
                Anonymous: INPUT_0 {
                    mi: MOUSEINPUT {
                        dx: 0,
                        dy: 0,
                        mouseData: delta as u32,
                        dwFlags: MOUSEEVENTF_HWHEEL,
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            };

            unsafe {
                SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
            }
        }
    }

    pub fn drag(&self, start: (u32, u32), end: (u32, u32), duration_ms: u32) {
        let (sx, sy) = (start.0 as i32, start.1 as i32);
        let (ex, ey) = (end.0 as i32, end.1 as i32);

        let steps = 100;
        let sleep_ms = duration_ms / steps as u32;

        unsafe {
            let lparam_down = makelparam(sx, sy);
            let _ = PostMessageW(Some(self.hwnd), WM_LBUTTONDOWN, WPARAM(1), lparam_down);

            for i in 1..=steps {
                let ix = sx + (ex - sx) * i / steps;
                let iy = sy + (ey - sy) * i / steps;
                // println!("{ix} {iy}");
                let lparam_move = makelparam(ix, iy);
                // PostMessageW(Some(self.hwnd), WM_LBUTTONDOWN, WPARAM(1), lparam_move);
                let _ = PostMessageW(Some(self.hwnd), WM_MOUSEMOVE, WPARAM(1), lparam_move);
                sleep(Duration::from_millis(sleep_ms as u64));
            }

            let lparam_up = makelparam(ex, ey);
            let _ = PostMessageW(Some(self.hwnd), WM_LBUTTONUP, WPARAM(0), lparam_up);
        }
    }

    pub fn drag_send_input(&self, start: (u32, u32), end: (u32, u32), duration_ms: u32) {
        let (sx, sy) = (start.0 as i32, start.1 as i32);
        let (ex, ey) = (end.0 as i32, end.1 as i32);

        if let (Some((start_x, start_y)), Some((end_x, end_y))) = (
            window::client_to_screen(self.hwnd, sx, sy),
            window::client_to_screen(self.hwnd, ex, ey),
        ) {
            let steps = 100;
            let sleep_ms = duration_ms / steps;

            unsafe {
                let _ = SetCursorPos(start_x, start_y);
            }

            let down = INPUT {
                r#type: INPUT_MOUSE,
                Anonymous: INPUT_0 {
                    mi: MOUSEINPUT {
                        dx: 0,
                        dy: 0,
                        mouseData: 0,
                        dwFlags: MOUSEEVENTF_LEFTDOWN,
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            };

            unsafe {
                SendInput(&[down], std::mem::size_of::<INPUT>() as i32);
            }

            for i in 1..=steps {
                let ix = start_x + (end_x - start_x) * i as i32 / steps as i32;
                let iy = start_y + (end_y - start_y) * i as i32 / steps as i32;
                unsafe {
                    let _ = SetCursorPos(ix, iy);
                }
                sleep(Duration::from_millis(sleep_ms as u64));
            }

            let up = INPUT {
                r#type: INPUT_MOUSE,
                Anonymous: INPUT_0 {
                    mi: MOUSEINPUT {
                        dx: 0,
                        dy: 0,
                        mouseData: 0,
                        dwFlags: MOUSEEVENTF_LEFTUP,
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            };

            unsafe {
                SendInput(&[up], std::mem::size_of::<INPUT>() as i32);
            }
        }
    }

    //key

    pub fn key_down(&self, key: u16) {
        unsafe {
            let _ = PostMessageW(Some(self.hwnd), WM_KEYDOWN, WPARAM(key as usize), LPARAM(0));
        }
    }

    pub fn key_down_send_input(&self, key: u16) {
        let input = INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(key),
                    wScan: 0,
                    dwFlags: KEYBD_EVENT_FLAGS(0),
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        };

        unsafe {
            SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
        }
    }

    pub fn key_up(&self, key: u16) {
        unsafe {
            let _ = PostMessageW(Some(self.hwnd), WM_KEYUP, WPARAM(key as usize), LPARAM(0));
        }
    }

    pub fn key_up_send_input(&self, key: u16) {
        let input = INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(key),
                    wScan: 0,
                    dwFlags: KEYEVENTF_KEYUP,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        };

        unsafe {
            SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
        }
    }

    pub fn input_text(&self, text: &str) {
        for c in text.chars() {
            unsafe {
                let _ = PostMessageW(Some(self.hwnd), WM_CHAR, WPARAM(c as usize), LPARAM(0));
            }
        }
    }

    pub fn wait(&self, millis: u64) {
        let duration = Duration::from_millis(millis);
        sleep(duration);
    }
}

pub fn makelparam(x: i32, y: i32) -> LPARAM {
    // LOWORD = x, HIWORD = y
    LPARAM(((y as isize) << 16) | (x as isize & 0xFFFF))
}
