use std::{ffi::OsString, os::windows::ffi::OsStringExt, thread, time::Duration};

use serde::{Deserialize, Serialize};
use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VK_LBUTTON};
use windows::Win32::UI::WindowsAndMessaging::{
    FindWindowExW, FindWindowW, GetClassNameW, GetCursorPos, GetParent, GetWindowRect, GetWindowTextW, SetWindowPos, WindowFromPoint,
    SWP_NOACTIVATE, SWP_NOZORDER,
};
use windows::{
    core::*,
    Win32::{
        Graphics::Gdi::ClientToScreen,
        UI::{
            Input::KeyboardAndMouse::VK_F8,
            WindowsAndMessaging::{BringWindowToTop, IsIconic, SetForegroundWindow, ShowWindow, SW_RESTORE},
        },
    },
};

use crate::fl;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetWindow {
    pub windows: Vec<Window>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Window {
    #[serde(skip)]
    pub hwnd: HWND,
    pub title: String,
    pub class: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowInfo {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}
impl TargetWindow {
    pub fn new() -> Self {
        Self { windows: vec![] }
    }
    pub fn rebind_hwnds(&mut self) -> std::result::Result<(), String> {
        if self.windows.is_empty() {
            return Err("The list of saved windows is empty.".to_string());
        }

        if let Some(hwnd) = find_window_handle(&self.windows[0].title, &self.windows[0].class) {
            self.windows[0].hwnd = hwnd;
        } else {
            return Err(format!(
                "Failed to find the HWND of the top-level window '{}' (class: '{}').",
                self.windows[0].title, self.windows[0].class
            ));
        }

        for i in 1..self.windows.len() {
            let parent_hwnd = self.windows[i - 1].hwnd;
            if parent_hwnd.0.is_null() {
                return Err(format!(
                    "Could not find the parent HWND of the '{}' window, stopping traversal.",
                    self.windows[i].title
                ));
            }

            if let Some(hwnd) = find_child_window_handle(parent_hwnd, &self.windows[i].title, &self.windows[i].class) {
                self.windows[i].hwnd = hwnd;
            } else {
                return Err(format!(
                    "Could not find the '{}' (class: '{}') window under parent '{}'.",
                    self.windows[i].title,
                    self.windows[i].class,
                    self.windows[i - 1].title
                ));
            }
        }
        Ok(())
    }

    pub fn get_first_hwnd(&self) -> std::result::Result<HWND, String> {
        self.windows.first().map(|w| w.hwnd).ok_or(fl!("error-project-no-first-window"))
    }

    pub fn get_last_hwnd(&self) -> std::result::Result<HWND, String> {
        self.windows.last().map(|w| w.hwnd).ok_or(fl!("error-project-last-first-window"))
    }

    pub fn get_first_title(&self) -> String {
        match self.windows.first() {
            Some(win) => win.title.clone(),
            None => " ".to_string(),
        }
    }
}

impl WindowInfo {
    pub fn get_window_info(hwnd: HWND) -> Result<WindowInfo> {
        unsafe {
            let mut rect = RECT::default();
            if GetWindowRect(hwnd, &mut rect).is_err() {
                return Err(Error::from_win32());
            }

            let x = rect.left;
            let y = rect.top;
            let width = rect.right - rect.left;
            let height = rect.bottom - rect.top;

            Ok(WindowInfo { x, y, width, height })
        }
    }
}

fn find_window_handle(title: &str, class: &str) -> Option<HWND> {
    let class_wide: Vec<u16> = class.encode_utf16().chain(Some(0)).collect();
    let title_wide: Vec<u16> = title.encode_utf16().chain(Some(0)).collect();
    unsafe { FindWindowW(PCWSTR(class_wide.as_ptr()), PCWSTR(title_wide.as_ptr())).ok() }
}

fn find_child_window_handle(parent_hwnd: HWND, title: &str, class: &str) -> Option<HWND> {
    let class_wide: Vec<u16> = class.encode_utf16().chain(Some(0)).collect();
    let title_wide: Vec<u16> = title.encode_utf16().chain(Some(0)).collect();
    unsafe { FindWindowExW(Some(parent_hwnd), None, PCWSTR(class_wide.as_ptr()), PCWSTR(title_wide.as_ptr())).ok() }
}

fn get_window_title(hwnd: HWND) -> String {
    let mut buffer = [0u16; 256];
    unsafe {
        let length = GetWindowTextW(hwnd, &mut buffer);
        if length > 0 {
            return OsString::from_wide(&buffer[..length as usize]).to_string_lossy().into_owned();
        }
    }
    String::new()
}

fn get_window_class(hwnd: HWND) -> String {
    let mut buffer = [0u16; 256];
    unsafe {
        let length = GetClassNameW(hwnd, &mut buffer);
        if length > 0 {
            return OsString::from_wide(&buffer[..length as usize]).to_string_lossy().into_owned();
        }
    }
    String::new()
}

pub fn run_capture_loop() -> TargetWindow {
    println!("Left-click the window to save its information...");
    loop {
        unsafe {
            let mut point = windows::Win32::Foundation::POINT { x: 0, y: 0 };
            if GetCursorPos(&mut point).is_ok() {
                let hwnd = WindowFromPoint(point);
                println!("{point:?} {hwnd:?}");

                if is_left_click_pressed() {
                    let windows = get_window_hierarchy(hwnd);
                    return windows;
                }
            }
        }
        thread::sleep(Duration::from_millis(33));
    }
}

fn is_left_click_pressed() -> bool {
    unsafe { GetAsyncKeyState(VK_LBUTTON.0 as i32) < 0 }
}

pub fn is_f8_pressed() -> bool {
    unsafe { GetAsyncKeyState(VK_F8.0 as i32) < 0 }
}

pub fn client_to_screen(hwnd: HWND, x: i32, y: i32) -> Option<(i32, i32)> {
    unsafe {
        let mut point = windows::Win32::Foundation::POINT { x, y };
        if ClientToScreen(hwnd, &mut point).as_bool() {
            Some((point.x, point.y))
        } else {
            None
        }
    }
}

fn get_window_hierarchy(hwnd: HWND) -> TargetWindow {
    let mut hierarchy = Vec::new();
    let mut current_hwnd = Some(hwnd);

    while let Some(hwnd) = current_hwnd {
        hierarchy.push(Window {
            hwnd,
            title: get_window_title(hwnd),
            class: get_window_class(hwnd),
        });
        current_hwnd = unsafe { GetParent(hwnd).ok() };
    }

    hierarchy.reverse();
    TargetWindow { windows: hierarchy }
}

pub fn restore_window(hwnd: HWND, info: WindowInfo) {
    let current_info = WindowInfo::get_window_info(hwnd).unwrap();
    unsafe {
        let _ = SetWindowPos(
            hwnd,
            Some(HWND(std::ptr::null_mut())),
            current_info.x,
            current_info.y,
            info.width,
            info.height,
            SWP_NOZORDER | SWP_NOACTIVATE,
        );
    }
}

pub fn activate_window(hwnd: HWND) {
    unsafe {
        if IsIconic(hwnd).as_bool() {
            let _ = ShowWindow(hwnd, SW_RESTORE);
            std::thread::sleep(std::time::Duration::from_millis(200));

            let _ = SetForegroundWindow(hwnd);
        }
    }
}

pub fn bring_window_to_front(hwnd: HWND) {
    unsafe {
        if IsIconic(hwnd).as_bool() {
            let _ = ShowWindow(hwnd, SW_RESTORE);
            std::thread::sleep(std::time::Duration::from_millis(200));
        }

        let _ = BringWindowToTop(hwnd);

        let _ = SetForegroundWindow(hwnd);
    }
}

unsafe impl Send for Window {}
unsafe impl Send for TargetWindow {}
