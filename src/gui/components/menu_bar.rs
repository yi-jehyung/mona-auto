use egui::Ui;
use i18n_embed::LanguageLoader;
use i18n_embed_fl::fl;
use unic_langid::langid;

use crate::{
    core::{Language, Project},
    gui::app::MyApp,
    Localizations,
};

pub fn menu(ui: &mut Ui, app: &mut MyApp) {
    egui::menu::bar(ui, |ui| {
        ui.add_enabled_ui(!app.is_automation_running, |ui| {
            ui.menu_button(fl!(app.i18n_loader, "menu-file"), |ui| {
                new_button(ui, app);
                open_button(ui, app);
                save_button(ui, app);
            });

            ui.menu_button(fl!(app.i18n_loader, "menu-menu"), |ui| {
                language_button(ui, app);
                update_button(ui, app);
                help_button(ui, app);
                quit_button(ui, app);
            });
        });
    });
}

fn new_button(ui: &mut Ui, app: &mut MyApp) {
    if ui.button(fl!(app.i18n_loader, "menu-file-new")).clicked() {
        ui.close_menu();
        match Project::make_new_project(&app.i18n_loader) {
            Ok(project) => app.project = project,
            Err(err) => {
                eprintln!("{err}");
                app.error_message = Some(err);
            }
        }
    }
}

fn open_button(ui: &mut Ui, app: &mut MyApp) {
    if ui.button(fl!(app.i18n_loader, "menu-file-open")).clicked() {
        match Project::load(&app.i18n_loader) {
            Ok(project) => {
                app.project = project.clone();
                app.setting.last_project_path = format!("{}/project.json", project.path.unwrap()).into();
                app.setting.save();
            }
            Err(error) => {
                println!("{error}");
                app.error_message = Some(error);
            },
        }
        ui.close_menu();
    }
}

fn save_button(ui: &mut Ui, app: &mut MyApp) {
    if ui.button(fl!(app.i18n_loader, "menu-file-save")).clicked() {
        app.project.save_file();
        ui.close_menu();
    }
}

fn language_button(ui: &mut Ui, app: &mut MyApp) {
    ui.menu_button(fl!(app.i18n_loader, "menu-lang"), |ui| {
        if ui.button("English").clicked() {
            app.i18n_loader
                .load_languages(&Localizations, &[langid!("en-US")])
                .expect("Failed to load language.");
            app.setting.language = Language::EnUS;
            app.setting.save();
            ui.close_menu();
        }
        if ui.button("한국어/Korean").clicked() {
            app.i18n_loader
                .load_languages(&Localizations, &[langid!("ko-KR")])
                .expect("Failed to load language.");
            app.setting.language = Language::KoKR;
            app.setting.save();
            ui.close_menu();
        }
        if ui.button("日本語/Japanes").clicked() {
            app.i18n_loader
                .load_languages(&Localizations, &[langid!("ja-JP")])
                .expect("Failed to load language.");
            app.setting.language = Language::JaJP;
            app.setting.save();
            ui.close_menu();
        }
        if ui.button("中文/Chinese").clicked() {
            app.i18n_loader
                .load_languages(&Localizations, &[langid!("zh-CN")])
                .expect("Failed to load language.");
            app.setting.language = Language::ZhCN;
            app.setting.save();
            ui.close_menu();
        }
    });
}

fn update_button(ui: &mut Ui, app: &mut MyApp) {
    ui.hyperlink_to(fl!(app.i18n_loader, "menu-check-updates"), "https://github.com/yi-jehyung/mona-auto/releases");
}
fn help_button(ui: &mut Ui, app: &mut MyApp) {
    ui.hyperlink_to(fl!(app.i18n_loader, "menu-help"), "https://github.com/yi-jehyung/mona-auto");
}
fn quit_button(ui: &mut Ui, app: &mut MyApp) {
    if ui.button(fl!(app.i18n_loader, "menu-quit")).clicked() {
        std::process::exit(0);
    }
}
