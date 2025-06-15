use std::{
    collections::HashSet,
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

use crate::{
    action::{Action, ActionType},
    fl,
    window::{self, TargetWindow, WindowInfo},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub description: String,
    pub target_windows: Vec<TargetWindow>,
    pub window_info: Option<WindowInfo>,
    pub items: Vec<ActionItem>,
    #[serde(skip)]
    pub path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionItem {
    pub name: String,
    pub enabled: bool,
    pub image_path: Option<String>,
    pub roi: Roi,
    pub actions: Vec<Action>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Roi {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl Project {
    pub fn new() -> Self {
        Project {
            description: "".to_string(),
            target_windows: vec![window::TargetWindow { windows: Vec::new() }],
            window_info: None,
            items: Vec::new(),
            path: None,
        }
    }

    pub fn add_action_item(&mut self, name: &str, image_path: Option<&str>, roi_x: u32, roi_y: u32, roi_width: u32, roi_height: u32) {
        self.items.push(ActionItem {
            name: name.to_string(),
            image_path: image_path.map(|s| s.to_string()),
            enabled: true,
            actions: Vec::new(),
            roi: Roi {
                x: roi_x,
                y: roi_y,
                width: roi_width,
                height: roi_height,
            },
        });
    }

    pub fn add_action(&mut self, item_index: usize, action_type: ActionType) {
        if let Some(items) = self.items.get_mut(item_index) {
            items.add_action(action_type);
        } else {
            eprintln!("{}", fl!("message-project-invalid-index", index = item_index));
        }
    }

    pub fn save_to_json(&self, path: &PathBuf) {
        if let Some(parent_dir) = path.parent() {
            if let Err(err) = std::fs::create_dir_all(parent_dir) {
                eprintln!("{}", fl!("message-project-failed-create-dir", error = err.to_string()));
                return;
            }
        }

        if let Ok(json_data) = serde_json::to_string_pretty(&self) {
            if let Ok(mut file) = File::create(path) {
                if let Err(err) = file.write_all(json_data.as_bytes()) {
                    eprintln!("{}", fl!("message-project-failed-saved-json", error = err.to_string()));
                } else {
                    println!("{}", fl!("message-project-successfully-saved-json", path = path.to_string_lossy()));
                }
            } else {
                eprintln!("{}", fl!("message-project-failed-create-file", path = path.to_string_lossy()));
            }
        } else {
            eprintln!("Failed to convert to JSON!");
        }
    }

    pub fn load_from_json(path: PathBuf) -> Result<Self, String> {
        let mut file = File::open(&path).map_err(|e| fl!("error-project-failed-open-file", error = e.to_string()))?;

        let mut json_data = String::new();
        file.read_to_string(&mut json_data)
            .map_err(|e| fl!("error-project-failed-parse-json", error = e.to_string()))?;

        let mut project: Project =
            serde_json::from_str(&json_data).map_err(|e| fl!("error-project-failed-parse-json", error = e.to_string()))?;

        project.path = Some(
            path.parent()
                .unwrap()
                .to_string_lossy()
                .to_string()
                .replace(std::path::MAIN_SEPARATOR, "/"),
        );
        Ok(project)
    }

    pub fn load() -> Result<Self, String> {
        let path = rfd::FileDialog::new()
            .add_filter("json", &["json"])
            .pick_file()
            .ok_or_else(|| fl!("error-project-file-selection-canceled"))?;
        Project::load_from_json(path)
    }

    pub fn make_new_project() -> Result<Self, String> {
        let dir = rfd::FileDialog::new().pick_folder();
        if let Some(dir) = dir {
            let project = Project::new();
            let path = dir.join("project.json");
            project.save_to_json(&path);
            Project::load_from_json(path)
        } else {
            Err(fl!("error-project-file-create-project"))
        }
    }

    pub fn save_file(&self) {
        let dir = self.path.clone();
        if let Some(dir) = dir {
            let path = PathBuf::from(dir).join("project.json");
            self.save_to_json(&path);
        }
    }

    pub fn has_window_changed(&mut self) -> bool {
        match &self.window_info {
            Some(info) => {
                for window in self.target_windows.iter_mut() {
                    if let Err(err) = window.rebind_hwnds() {
                        eprintln!("{err}");
                        continue;
                    }
                    if !window.windows.is_empty() {
                        let current_info = WindowInfo::get_window_info(window.get_first_hwnd().unwrap()).unwrap();

                        if info.width != current_info.width {
                            return true;
                        }
                        if info.height != current_info.height {
                            return true;
                        }
                    }
                }
                false
            }
            None => false,
        }
    }

    pub fn next_name(&self) -> String {
        let base = "image";
        let mut used = HashSet::new();
        for item in &self.items {
            if let Some(rest) = item.name.strip_prefix(base) {
                if let Some(num_str) = rest.split_whitespace().next() {
                    if let Ok(n) = num_str.parse::<u32>() {
                        used.insert(n);
                    }
                }
            }
        }

        let mut idx = 1;
        while used.contains(&idx) {
            idx += 1;
        }
        format!("{} {}", base, idx)
    }
}

impl ActionItem {
    pub fn add_action(&mut self, action_type: ActionType) {
        self.actions.push(Action {
            enabled: true,
            action: action_type,
        });
    }
}
