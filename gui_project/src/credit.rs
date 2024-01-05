use crate::MyApp;
use egui::{CentralPanel, Frame, TopBottomPanel};

pub fn show_credit_ui(app: &mut MyApp, ctx: &egui::Context, panel_frame: Frame) {
    // Define capture window
    // Define main window central panel
    egui::CentralPanel::default().frame(panel_frame).show(ctx, |_ui| {
        CentralPanel::default().show(ctx, |ui| {
            // other TITLE
            // ui.add_space(5.0);
            // ui.vertical_centered(|ui| {
            //     ui.heading("CREDIT");
            // });
            // ui.add_space(5.0);
            // let sep = Separator::default().spacing(20.);
            // ui.add(sep);
            let app_rect = ui.max_rect();

            // Define title bar
            let mut title_bar_rect = app_rect;
            title_bar_rect.max.y = title_bar_rect.min.y + 32.0;
            super::title_bar_ui(app, ui, title_bar_rect, "Credit");
            ui.vertical_centered(|ui| {
                ui.add_space(50.);
                ui.monospace("Screen Capture");
                ui.monospace("Version 1.0");
                ui.monospace("Developed by");
                // ui.indent("tab", |ui| {
                ui.monospace("Caretto Michelangelo");
                ui.monospace("Buccellato Federico");
                ui.monospace("Borella Simone");
                // });
            });
        });

        TopBottomPanel::bottom("footer").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.);
                ui.monospace("PROJECT");
                ui.hyperlink("https://github.com/FedeBucce/Screenshot_Rust_Project/");
                ui.monospace("2022/2023");
                ui.add_space(10.);
            })
        });
    });
}
