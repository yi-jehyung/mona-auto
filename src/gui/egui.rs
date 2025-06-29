use eframe::*;
use egui::{
    epaint::text::{FontInsert, InsertFontFamily},
    Color32,
};
use egui_extras::{Size, StripBuilder};

use crate::gui::{app::*, components, defines::*, util::load_icon};

pub fn run_gui() -> eframe::Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_icon(load_icon())
            .with_inner_size(egui::vec2(WINDOW_WIDTH, WINDOW_HEIGHT))
            .with_min_inner_size(egui::vec2(WINDOW_WIDTH, 400.0))
            .with_max_inner_size(egui::vec2(WINDOW_WIDTH, 9999.9))
            .with_resizable(true)
            .with_maximize_button(false),
        ..Default::default()
    };

    run_native(
        &format!("{APP_NAME} v{APP_VERSION}"),
        options,
        Box::new(|cc| {
            let ctx = &cc.egui_ctx;

            #[cfg(debug_assertions)]
            ctx.set_debug_on_hover(true);

            egui_extras::install_image_loaders(ctx);
            ctx.add_font(FontInsert::new(
                "NotoSansCJKkr-Regular",
                egui::FontData::from_static(APP_FONT),
                vec![
                    InsertFontFamily {
                        family: egui::FontFamily::Proportional,
                        priority: egui::epaint::text::FontPriority::Highest,
                    },
                    InsertFontFamily {
                        family: egui::FontFamily::Monospace,
                        priority: egui::epaint::text::FontPriority::Lowest,
                    },
                ],
            ));

            let setting = crate::core::Setting::load();
            let code = setting.language.to_string();
            if let Err(e) = crate::i18n::change_language(&code) {
                eprintln!("Failed to load language: {e}");
            }
            Ok(Box::new(MyApp::new(setting)))
        }),
    )
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.selected_item_index {
            Some(index) => self.selected_item = self.project.items.get(index).cloned(),
            None => self.selected_item = None,
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            // menu
            components::menu_bar::menu(ui, self);

            let point = 50.0 + (self.project.target_windows.len() * 25) as f32;
            // Layout
            // ┌────────────┐
            // │            │
            // │            │
            // └────────────┘
            // ┌────────────┐
            // │            │
            // └────────────┘
            // ┌────────────┐
            // │            │
            // │            │
            // │            │
            // │            │
            // └────────────┘
            StripBuilder::new(ui)
                .size(Size::exact(100.0))
                .size(Size::exact(point))
                .size(Size::remainder().at_least(200.0))
                .vertical(|mut strip| {
                    // 1st row: project_panel, setting_panel
                    // ┌────────────┐->┌─────┐┌─────┐
                    // │████████████│->│     ││     │
                    // │████████████│->│     ││     │
                    // └────────────┘->└─────┘└─────┘
                    // ┌────────────┐
                    // │            │
                    // └────────────┘
                    // ┌────────────┐
                    // │            │
                    // │            │
                    // │            │
                    // │            │
                    // └────────────┘
                    strip.strip(|builder| {
                        builder.sizes(Size::remainder(), 2).horizontal(|mut strip| {
                            // project_panel
                            // ┌─────┐┌─────┐
                            // │█████││     │
                            // │█████││     │
                            // └─────┘└─────┘
                            strip.cell(|ui| {
                                ui.painter()
                                    .rect_filled(ui.available_rect_before_wrap(), 8.0, Color32::from_black_alpha(80));
                                components::project_panel::project_panel(ui, self);
                            });
                            // setting_panel
                            // ┌─────┐┌─────┐
                            // │     ││█████│
                            // │     ││█████│
                            // └─────┘└─────┘
                            strip.cell(|ui| {
                                ui.painter()
                                    .rect_filled(ui.available_rect_before_wrap(), 8.0, Color32::from_black_alpha(80));
                                components::setting_panel::setting_panel(ui, self);
                            });
                        });
                    });
                    // 2nd row: control_panel
                    // ┌────────────┐
                    // │            │
                    // │            │
                    // └────────────┘
                    // ┌────────────┐
                    // │████████████│
                    // └────────────┘
                    // ┌────────────┐
                    // │            │
                    // │            │
                    // │            │
                    // │            │
                    // └────────────┘
                    strip.cell(|ui| {
                        ui.painter()
                            .rect_filled(ui.available_rect_before_wrap(), 8.0, Color32::from_black_alpha(80));
                        components::control_panel::control_panel(ui, self);
                    });
                    // 3rd row: image_list_panel, image_preview, action_panel
                    // ┌────────────┐
                    // │            │
                    // │            │
                    // └────────────┘
                    // ┌────────────┐
                    // │            │
                    // └────────────┘
                    // ┌────────────┐->┌─────┐┌─────┐
                    // │████████████│->│     ││     │
                    // │████████████│->│     ││     │
                    // │████████████│->│     ││     │
                    // │████████████│->│     ││     │
                    // └────────────┘->└─────┘└─────┘
                    strip.strip(|builder| {
                        builder.sizes(Size::remainder(), 2).horizontal(|mut strip| {
                            // image_list_panel
                            // ┌─────┐┌─────┐
                            // │█████││     │
                            // │█████││     │
                            // │█████││     │
                            // │█████││     │
                            // └─────┘└─────┘
                            strip.cell(|ui| {
                                ui.group(|ui| {
                                    ui.set_min_size(ui.available_size());
                                    components::image_list_panel::image_list_panel(ui, self);
                                });
                            });
                            // image_preview, action_panel
                            // ┌─────┐┌─────┐
                            // │     ││█████│
                            // │     ││█████│
                            // │     ││█████│
                            // │     ││█████│
                            // └─────┘└─────┘
                            //        ↓↓↓↓↓↓↓
                            //        ┌─────┐
                            //        │     │
                            //        └─────┘
                            //        ┌─────┐
                            //        │     │
                            //        └─────┘
                            strip.strip(|builder| {
                                builder.size(Size::exact(130.0)).size(Size::remainder()).vertical(|mut strip| {
                                    //image_preview
                                    // ┌─────┐
                                    // │█████│
                                    // └─────┘
                                    // ┌─────┐
                                    // │     │
                                    // └─────┘
                                    strip.cell(|ui| {
                                        components::image_preview_panel::image_preview(ctx, ui, self);
                                    });
                                    //action_panel
                                    // ┌─────┐
                                    // │     │
                                    // └─────┘
                                    // ┌─────┐
                                    // │█████│
                                    // └─────┘
                                    strip.cell(|ui| {
                                        ui.group(|ui| {
                                            ui.set_min_size(ui.available_size());
                                            components::action_panel::action_panel(ui, self);
                                        });
                                    });
                                });
                            });
                        });
                    });
                });
        });

        if self.show_image_edit_viewport {
            components::image_edit_viewport::show_image_edit_viewport(ctx, self);
        }

        if self.coordinate_picker_enable {
            components::coordinate_picker_viewport::show_coordinate_picker_viewport(ctx, self);
        }

        if self.show_window_resize_modal {
            components::window_resize_modal::show_window_resize_modal(ctx, self);
        }

        if self.show_action_modal {
            components::add_action_modal::show_add_action_modal(ctx, self);
        }

        if self.error_message.is_some() {
            components::error_modal::show_error_modal(ctx, self);
        }

        if let Some(rx) = &self.target_window_rx {
            if let Ok(found) = rx.try_recv() {
                self.project.target_windows[self.find_window_index] = found;
                self.target_window_rx = None;

                self.maybe_prompt_window_resize();
                self.project.save_file();
            }
        }

        if let Some(rx) = &self.engine_rx {
            while let Ok(v) = rx.try_recv() {
                if v == 1 {
                    self.stop_flag.store(true, std::sync::atomic::Ordering::SeqCst);

                    for handle in self.handle.drain(..) {
                        let _ = handle.join();
                    }

                    self.is_automation_running = false;
                }
            }
            self.engine_rx = None;
        }
    }
}
