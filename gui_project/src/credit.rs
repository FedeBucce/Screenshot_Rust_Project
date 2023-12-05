use egui::Grid;
use crate::MyApp;


use egui::{CentralPanel, Frame, Ui,Separator,TopBottomPanel};

pub fn show_credit_ui(app: &mut MyApp, ctx: &egui::Context, panel_frame:Frame) {
    // Define capture window
       // Define main window central panel
       egui::CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
    
        // // Retrieve screenshot if taken
        // if let Some(screenshot) = app.screenshot.as_ref() {
        //     app.texture = Some(ui.ctx().load_texture(
        //         "screenshot",
        //         screenshot.clone(),
        //         Default::default(),
        //     ));
        // }

        TopBottomPanel::bottom("footer").show(ctx, |ui|{
            ui.vertical_centered(|ui|{
                ui.add_space(10.);
                ui.monospace("PROJECT");
                ui.hyperlink("https://github.com/FedeBucce/Screenshot_Rust_Project/");
                ui.monospace("2022/2023");
                ui.add_space(10.);
            })
        });

    });}