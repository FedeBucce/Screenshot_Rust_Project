use egui::Grid;



use egui::{CentralPanel, Frame, Ui};

pub fn show_hotkeys_ui( ctx: &egui::Context, panel_frame:Frame) {
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
    super::title_bar_ui(ui, title_bar_rect, "Change Hotkeys");



    // Define content_ui as ui child containing buttons and image
    let mut content_rect = app_rect;
    content_rect.min.y = title_bar_rect.max.y;
    content_rect = content_rect.shrink(8.0);

    let mut content_ui = ui.child_ui(content_rect, *ui.layout());
    
    // Define New capture button and Save button aligned horizontally
    ui.group(|ui|{
   Grid::new("blabla")
               .num_columns(2)
                .spacing([40.0, 4.0])
                 .striped(false)
                
                   .show(ui, |ui| {
                    ui.group(|ui: &mut egui::Ui|{
                        Grid::new("ass")
                    .num_columns(2)
                     .spacing([40.0, 4.0])
                      .striped(false)
                        .show(ui, |ui| {
                        ui.label("ScreenShot");
                         ui.end_row();
                        ui.group(|ui|{//FORM SCREENSHOT
                            Grid::new("ass")
                            .num_columns(2)
                             .spacing([40.0, 4.0])
                              .striped(false)
                                .show(ui, |ui| {

                                        ui.label("Change Modifier");
                                         
                                        ui.end_row();

                                        ui.label("Change Code");
                                        ui.end_row();


                        });});
                    });});});

                    ui.end_row();
                    Grid::new("ass")
                    .num_columns(2)
                     .spacing([40.0, 4.0])
                      .striped(false)
                        .show(ui, |ui| {
                         ui.group(|ui|{
                             ui.group(|ui|{
                                ui.label("due");
                                ui.end_row();
                                ui.label("due");
                                ui.end_row();
                                ui.label("due");
                                ui.end_row();
                             });
                         });
                           
                   });
                
                
                });});
            }
 