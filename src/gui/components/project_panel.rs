use egui::Ui;
use i18n_embed_fl::fl;

use crate::gui::MyApp;

/// 프리셋 이름/설명 입력 폼 컴포넌트
pub fn project_panel(ui: &mut Ui, app: &mut MyApp) {
    let label = ui.label(fl!(app.i18n_loader, "project-panel-name"));
    ui.add_enabled_ui(!app.is_automation_running, |ui| {
        ui.text_edit_singleline(&mut app.project.name).labelled_by(label.id);
    });

    let label = ui.label(fl!(app.i18n_loader, "project-panel-description"));

    ui.add_enabled_ui(!app.is_automation_running, |ui| {
        ui.text_edit_multiline(&mut app.project.description).labelled_by(label.id);
    });
}
