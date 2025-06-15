use egui::Ui;

use crate::{
    core::{Language, Project},
    fl,
    gui::app::MyApp,
    i18n::change_language,
};

pub fn menu(ui: &mut Ui, app: &mut MyApp) {
    egui::menu::bar(ui, |ui| {
        ui.add_enabled_ui(!app.is_automation_running, |ui| {
            ui.menu_button(fl!("menu-file"), |ui| {
                new_button(ui, app);
                open_button(ui, app);
                save_button(ui, app);
            });

            ui.menu_button(fl!("menu-menu"), |ui| {
                language_button(ui, app);
                update_button(ui);
                help_button(ui);
                quit_button(ui);
            });
        });
    });
}

fn new_button(ui: &mut Ui, app: &mut MyApp) {
    if ui.button(fl!("menu-file-new")).clicked() {
        ui.close_menu();
        match Project::make_new_project() {
            Ok(project) => {
                app.project = project.clone();
                app.setting.last_project_path = format!("{}/project.json", project.path.unwrap()).into();
                app.setting.save();
            }
            Err(err) => {
                eprintln!("{err}");
                app.error_message = Some(err);
            }
        }
    }
}

fn open_button(ui: &mut Ui, app: &mut MyApp) {
    if ui.button(fl!("menu-file-open")).clicked() {
        match Project::load() {
            Ok(project) => {
                app.project = project.clone();
                app.setting.last_project_path = format!("{}/project.json", project.path.unwrap()).into();
                app.setting.save();
            }
            Err(error) => {
                println!("{error}");
                app.error_message = Some(error);
            }
        }
        ui.close_menu();
    }
}

fn save_button(ui: &mut Ui, app: &mut MyApp) {
    if ui.button(fl!("menu-file-save")).clicked() {
        app.project.save_file();
        ui.close_menu();
    }
}

fn language_button(ui: &mut Ui, app: &mut MyApp) {
    let languages = [
        ("English", Language::EnUS),
        ("한국어/Korean", Language::KoKR),
        ("日本語/Japanese", Language::JaJP),
        ("中文/Chinese", Language::ZhCN),
    ];

    ui.menu_button(fl!("menu-lang"), |ui| {
        for (label, lang_enum) in languages {
            if ui.button(label).clicked() {
                if let Err(e) = change_language(&lang_enum.to_string()) {
                    eprintln!("Failed to change language: {e}");
                }
                app.setting.language = lang_enum;
                app.setting.save();
                ui.close_menu();
            }
        }
    });
}

fn update_button(ui: &mut Ui) {
    ui.hyperlink_to(fl!("menu-check-updates"), "https://github.com/yi-jehyung/mona-auto/releases");
}
fn help_button(ui: &mut Ui) {
    ui.hyperlink_to(fl!("menu-help"), "https://github.com/yi-jehyung/mona-auto");
}
fn quit_button(ui: &mut Ui) {
    if ui.button(fl!("menu-quit")).clicked() {
        std::process::exit(0);
    }
}
