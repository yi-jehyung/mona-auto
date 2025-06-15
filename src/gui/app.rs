use std::{
    path::PathBuf,
    sync::{atomic::AtomicBool, mpsc, Arc},
    thread::JoinHandle,
};

use eframe::*;

use crate::core::{
    project::{self, Project},
    ActionType, Setting, WindowInfo,
};

pub struct MyApp {
    pub setting: Setting,
    pub project: Project,

    pub is_automation_running: bool,
    //action_panel.rs
    pub find_window_index: usize,
    pub target_window_rx: Option<mpsc::Receiver<crate::core::TargetWindow>>,

    pub engine_rx: Option<mpsc::Receiver<u32>>,
    pub stop_flag: Arc<AtomicBool>,
    pub handle: Vec<JoinHandle<()>>,

    //
    pub show_window_resize_modal: bool,
    pub window_size_operation: WindowSizeOperation,

    //image_list_panel.rs
    pub renaming: Option<usize>,
    pub rename_buffer: String,

    //action_panel.rs, image_list_panel.rs
    pub selected_item: Option<project::ActionItem>,
    pub selected_item_index: Option<usize>,

    //action_panel.rs
    pub show_action_modal: bool,
    pub edit_action: Option<usize>,
    pub action_type: ActionType,
    pub key_index: Option<usize>,
    pub scroll_to_bottom: bool,

    //coordinate_picker_viewport.rs
    pub coordinate_picker_enable: bool,
    pub picked_position: Option<(u32, u32)>,

    //image_preview.rs
    pub show_capture_modal: bool,
    pub show_image_edit_viewport: bool,

    //image_preview.rs
    pub image_cache: Option<(Arc<egui::ColorImage>, egui::TextureHandle)>,
    pub dynamic_image_cache: Option<image::DynamicImage>,
    pub needs_image_update: bool,

    //image_preview.rs
    pub capture_cache: Option<(Arc<egui::ColorImage>, egui::TextureHandle)>,

    //image_preview.rs
    pub drag_start: Option<egui::Pos2>,
    pub drag_end: Option<egui::Pos2>,
    pub crop_or_roi: CropOrRoi,
    pub local_a: Option<egui::Pos2>,
    pub local_b: Option<egui::Pos2>,
    pub roi_a: Option<egui::Pos2>,
    pub roi_b: Option<egui::Pos2>,
    pub use_crop_size: bool,
    pub is_roi_smaller_than_image: bool,

    pub error_message: Option<String>,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum WindowSizeOperation {
    RestorePrevious,
    UpdateCurrent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CropOrRoi {
    Crop,
    Roi,
}

impl MyApp {
    pub fn new(mut setting: Setting) -> Self {
        let mut project = Project::new();

        match setting.last_project_path {
            Some(ref path) => match Project::load_from_json(PathBuf::from(path)) {
                Ok(result) => project = result,
                Err(err) => {
                    eprintln!("{}", err);
                    let path = std::env::current_exe().unwrap().with_file_name("temp/project.json");
                    project.save_to_json(&path);
                    project.path = Some(
                        path.parent()
                            .unwrap()
                            .to_string_lossy()
                            .to_string()
                            .replace(std::path::MAIN_SEPARATOR, "/"),
                    );
                    setting.last_project_path = Some(path.to_string_lossy().to_string());
                }
            },
            None => {
                let path = std::env::current_exe().unwrap().with_file_name("temp").join("project.json");
                project.save_to_json(&path);
                project.path = Some(
                    path.parent()
                        .unwrap()
                        .to_string_lossy()
                        .to_string()
                        .replace(std::path::MAIN_SEPARATOR, "/"),
                );
                setting.last_project_path = Some(path.to_string_lossy().to_string());
                setting.save();
            }
        }

        Self {
            setting,
            project,

            is_automation_running: false,

            find_window_index: 0,
            target_window_rx: None,

            engine_rx: None,
            stop_flag: Arc::new(AtomicBool::new(false)),
            handle: vec![],

            show_window_resize_modal: false,
            window_size_operation: WindowSizeOperation::RestorePrevious,

            renaming: None,
            rename_buffer: String::new(),

            selected_item: None,
            selected_item_index: None,

            show_action_modal: false,
            edit_action: None,
            action_type: ActionType::LeftClick {
                x: 0,
                y: 0,
                use_matched_position: true,
            },
            key_index: None,
            scroll_to_bottom: false,

            coordinate_picker_enable: false,
            picked_position: None,

            show_capture_modal: false,
            show_image_edit_viewport: false,

            image_cache: None,
            dynamic_image_cache: None,
            needs_image_update: false,

            capture_cache: None,

            drag_start: None,
            drag_end: None,
            crop_or_roi: CropOrRoi::Crop,
            local_a: None,
            local_b: None,
            roi_a: None,
            roi_b: None,
            use_crop_size: true,
            is_roi_smaller_than_image: false,

            error_message: None,
        }
    }

    pub fn maybe_prompt_window_resize(&mut self) -> bool {
        match &self.project.window_info {
            Some(_) => {
                if self.project.has_window_changed() {
                    self.show_window_resize_modal = true;
                    return true;
                }
            }
            None => {
                if !self.project.target_windows.is_empty() {
                    let hwnd = self.project.target_windows.first().unwrap().get_first_hwnd().unwrap();
                    let info = WindowInfo::get_window_info(hwnd).unwrap();
                    self.project.window_info = Some(info);
                    return false;
                }
            }
        }
        false
    }
}
