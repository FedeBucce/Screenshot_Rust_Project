use egui::Grid;
use crate::MyApp;

use native_dialog::FileDialog;
use egui::{CentralPanel, Frame, Ui,Separator};


pub fn show_options_ui(app: &mut MyApp, ctx: &egui::Context, panel_frame:Frame) {
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

        let app_rect = ui.max_rect();

        // Define title bar                
        let mut title_bar_rect = app_rect;
        title_bar_rect.max.y = title_bar_rect.min.y + 32.0;
        
        super::title_bar_ui(app,ui, title_bar_rect, "Options");
      


        // Define content_ui as ui child containing buttons and image
        let mut content_rect = app_rect;
        content_rect.min.y = title_bar_rect.max.y;
        content_rect = content_rect.shrink(8.0);

       
        
        // Define New capture button and Save button aligned horizontally
    
                        
            let modificatore_screenshot: &str="FN";
            let tasto_screenshot="D";
           ui.vertical(|ui| {
          
           ui.horizontal(  |ui| {
            ui.label("       Take Screenshoot                Save Screenshoot");
            if ui.button("Change Hotkeys").clicked() {
                app.show_options = false;
                app.show_hotkeys=true;
               
            }
        });
            
            let modificatore_save: &str="FN";
            let tasto_save="X";
           
            ui.label(format!("                  {} + {}                             {}+{}",modificatore_screenshot, tasto_screenshot,modificatore_save, tasto_save));
            ui.separator();
          
           
            ui.horizontal(  |ui| {
            ui.label("       Path");
            if ui.button("Change Path").clicked() {
               let new_path = FileDialog::new()
               .set_location(&app.path)
               .show_open_single_dir()
               .unwrap();
              //Prevengo errori per path nulli
              if(new_path.is_some()){
                  app.path=new_path.unwrap();
              }
              
            }
        });
            
        
           
            ui.label(format!("{:#?}",app.path));
            ui.separator();
           

                   ui.horizontal(  |ui| {
            ui.label("       Format");
            egui::ComboBox::from_id_source("y")
            .selected_text(format!("{:}",app.format_tmp))
            .show_ui(ui,|ui|  {                                   
              ui.selectable_value(&mut app.format_tmp, "PNG".to_string(),  "PNG");
              ui.selectable_value( &mut app.format_tmp, "JPEG".to_string(),  "JPEG");
              ui.selectable_value( &mut app.format_tmp, "GIF".to_string(),  "GIF");
            });
        });
        ui.separator();
           ui.vertical_centered(|ui|{
           if ui.button("Credit").clicked() {
            app.show_options = false;
            app.show_credit=true;
           
        }
    });
        });
});
   }