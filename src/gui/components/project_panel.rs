use egui::{ScrollArea, Ui};

use crate::{fl, gui::MyApp};

/// 프리셋 이름/설명 입력 폼 컴포넌트
pub fn project_panel(ui: &mut Ui, app: &mut MyApp) {
    let label = ui.label(fl!("project-panel-description"));

    ui.add_enabled_ui(!app.is_automation_running, |ui| {
        ScrollArea::vertical().max_height(80.0).show(ui, |ui| {
            ui.text_edit_multiline(&mut app.project.description).labelled_by(label.id);
        });
    });
}
