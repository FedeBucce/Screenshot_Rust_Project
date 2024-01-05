use crate::MyApp;
use egui::{Grid, Frame};

pub fn show_hotkeys_ui(app: &mut MyApp, ctx: &egui::Context, panel_frame: Frame) {
    app.unregister_hotkeys();

    egui::CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
        // Retrieve screenshot if taken
        // if let Some(screenshot) = self.screenshot.as_ref() {
        //     self.texture = Some(ui.ctx().load_texture(
        //         "screenshot",
        //         screenshot.clone(),
        //         Default::default(),
        //     ));
        // }

        let app_rect = ui.max_rect();

        // Define title bar
        let mut title_bar_rect = app_rect;
        title_bar_rect.max.y = title_bar_rect.min.y + 32.0;
        super::title_bar_ui(app, ui, title_bar_rect, "Change Hotkeys");

        // Define content_ui as ui child containing buttons and image
        let mut content_rect = app_rect;
        content_rect.min.y = title_bar_rect.max.y;
        //content_rect = content_rect.shrink(8.0);

        // Define New capture button and Save button aligned horizontally
        //ui.group(|ui|{
        ui.vertical_centered(|ui| {
            ui.group(|ui: &mut egui::Ui| {
                ui.label("ScreenShot");

                ui.group(|ui| {
                    // FORM SCREENSHOT
                    Grid::new("ass")
                        .num_columns(2)
                        .spacing([40.0, 4.0])
                        .striped(false)
                        .show(ui, |ui| {
                            ui.label("Change Modifier");

                            egui::ComboBox::from_id_source("y")
                                .selected_text(format!("{:}", app.modifier_sh_tmp))
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut app.modifier_sh_tmp, "FN".to_string(), "FN");
                                    ui.selectable_value(&mut app.modifier_sh_tmp, "ALT".to_string(), "ALT");
                                    ui.selectable_value(&mut app.modifier_sh_tmp, "CTRL".to_string(), "CTRL");
                                    ui.selectable_value(&mut app.modifier_sh_tmp, "SHIFT".to_string(), "SHIFT");
                                });

                            ui.end_row();

                            ui.label("Change Code");
                            egui::ComboBox::from_id_source("x")
                                .selected_text(format!("{:}", app.code_sh_tmp))
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut app.code_sh_tmp, "A".to_string(), "A");
                                    ui.selectable_value(&mut app.code_sh_tmp, "B".to_string(), "B");
                                    ui.selectable_value(&mut app.code_sh_tmp, "E".to_string(), "E");
                                    ui.selectable_value(&mut app.code_sh_tmp, "F".to_string(), "F");
                                    ui.selectable_value(&mut app.code_sh_tmp, "G".to_string(), "G");
                                    ui.selectable_value(&mut app.code_sh_tmp, "H".to_string(), "H");
                                    ui.selectable_value(&mut app.code_sh_tmp, "I".to_string(), "I");
                                    ui.selectable_value(&mut app.code_sh_tmp, "L".to_string(), "L");
                                    ui.selectable_value(&mut app.code_sh_tmp, "M".to_string(), "M");
                                    ui.selectable_value(&mut app.code_sh_tmp, "N".to_string(), "N");
                                    ui.selectable_value(&mut app.code_sh_tmp, "O".to_string(), "O");
                                    ui.selectable_value(&mut app.code_sh_tmp, "P".to_string(), "P");
                                    ui.selectable_value(&mut app.code_sh_tmp, "Q".to_string(), "Q");
                                    ui.selectable_value(&mut app.code_sh_tmp, "R".to_string(), "R");
                                    ui.selectable_value(&mut app.code_sh_tmp, "S".to_string(), "S");
                                    ui.selectable_value(&mut app.code_sh_tmp, "U".to_string(), "U");
                                    ui.selectable_value(&mut app.code_sh_tmp, "V".to_string(), "V");
                                    ui.selectable_value(&mut app.code_sh_tmp, "Z".to_string(), "Z");
                                    ui.selectable_value(&mut app.code_sh_tmp, "W".to_string(), "W");
                                    ui.selectable_value(&mut app.code_sh_tmp, "X".to_string(), "X");
                                });
                            ui.end_row();
                        });
                });
            });

            ui.group(|ui: &mut egui::Ui| {
                ui.label("Save");

                ui.group(|ui| {
                    // FORM SCREENSHOT
                    Grid::new("asas")
                        .num_columns(2)
                        .spacing([40.0, 4.0])
                        .striped(false)
                        .show(ui, |ui| {
                            ui.label("Change Modifier");

                            egui::ComboBox::from_id_source("xy")
                                .selected_text(format!("{:}", app.modifier_save_tmp))
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut app.modifier_save_tmp, "FN".to_string(), "FN");
                                    ui.selectable_value(&mut app.modifier_save_tmp, "ALT".to_string(), "ALT");
                                    ui.selectable_value(&mut app.modifier_save_tmp, "CTRL".to_string(), "CTRL");
                                    ui.selectable_value(&mut app.modifier_save_tmp, "SHIFT".to_string(), "SHIFT");
                                });

                            ui.end_row();

                            ui.label("Change Code");
                            egui::ComboBox::from_id_source("xe")
                                .selected_text(format!("{:}", app.code_save_tmp))
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut app.code_save_tmp, "A".to_string(), "A");
                                    ui.selectable_value(&mut app.code_save_tmp, "B".to_string(), "B");
                                    ui.selectable_value(&mut app.code_save_tmp, "E".to_string(), "E");
                                    ui.selectable_value(&mut app.code_save_tmp, "F".to_string(), "F");
                                    ui.selectable_value(&mut app.code_save_tmp, "G".to_string(), "G");
                                    ui.selectable_value(&mut app.code_save_tmp, "H".to_string(), "H");
                                    ui.selectable_value(&mut app.code_save_tmp, "I".to_string(), "I");
                                    ui.selectable_value(&mut app.code_save_tmp, "L".to_string(), "L");
                                    ui.selectable_value(&mut app.code_save_tmp, "M".to_string(), "M");
                                    ui.selectable_value(&mut app.code_save_tmp, "N".to_string(), "N");
                                    ui.selectable_value(&mut app.code_save_tmp, "O".to_string(), "O");
                                    ui.selectable_value(&mut app.code_save_tmp, "P".to_string(), "P");
                                    ui.selectable_value(&mut app.code_save_tmp, "Q".to_string(), "Q");
                                    ui.selectable_value(&mut app.code_save_tmp, "R".to_string(), "R");
                                    ui.selectable_value(&mut app.code_save_tmp, "S".to_string(), "S");
                                    ui.selectable_value(&mut app.code_save_tmp, "U".to_string(), "U");
                                    ui.selectable_value(&mut app.code_save_tmp, "V".to_string(), "V");
                                    ui.selectable_value(&mut app.code_save_tmp, "Z".to_string(), "Z");
                                    ui.selectable_value(&mut app.code_save_tmp, "W".to_string(), "W");
                                    ui.selectable_value(&mut app.code_save_tmp, "X".to_string(), "X");
                                });
                            ui.end_row();
                        });
                });
            });

            ui.group(|ui| {
                Grid::new("OtherHK")
                    .num_columns(2)
                    .max_col_width(150.0)
                    .min_col_width(150.0)
                    .spacing([40.0, 7.0])
                    .striped(false)
                    .show(ui, |ui| {
                        ui.label("Other hot keys: ");
                        ui.end_row();
                        ui.label("- Copy :");
                        ui.label(format!("{}", "CTRL+C".to_string()));
                        ui.end_row();
                        ui.label("- Undo :");
                        ui.label(format!("{} ", "CTRL+Z".to_string()));
                        ui.end_row();
                        ui.label("- Redo :");
                        ui.label(format!("{}", "CTRL+Y".to_string()));
                        ui.end_row();
                        ui.label("- Clear :");
                        ui.label(format!("{}", "CTRL+D".to_string()));
                        ui.end_row();
                        ui.label("- Cut :");
                        ui.label(format!("{}", "CTRL+T".to_string()));
                    });
            });
        });

        app.register_hotkey();
    });
}
