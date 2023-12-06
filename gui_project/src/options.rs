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

        let mut content_ui = ui.child_ui(content_rect, *ui.layout());
        
        // Define New capture button and Save button aligned horizontally
       Grid::new("settings_grid")
                   .num_columns(4)
                    //.spacing([30.0, 30.0])
                     .striped(false)
                       .show(ui, |ui| { 
                        ui.end_row();
                        
                        
            let modificatore_screenshot: &str="FN";
            let tasto_screenshot="D";
            
           
            ui.label("       Take Screenshoot                Save Screenshoot");
            if ui.button("Change Hotkeys").clicked() {
                app.show_options = false;
                app.show_hotkeys=true;
               
            }
            
            let modificatore_save: &str="FN";
            let tasto_save="X";
            ui.end_row();
            ui.label(format!("                   {} + {}                             {}+{}",modificatore_screenshot, tasto_screenshot,modificatore_save, tasto_save));

          
            ui.end_row();
            ui.end_row();
           
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
            
            ui.end_row();
           
            ui.label(format!("{:#?}",app.path));
            ui.end_row();
            ui.end_row();
          
            ui.label("       Format");
        

               
                // if let Some(screenshot) = app.screenshot.as_ref() {
                //     let raw_data = screenshot.as_raw();
                //     let image_buffer = RgbaImage::from_raw(screenshot.width() as u32, screenshot.height() as u32, Vec::from(raw_data));
                //     let format: &str = "png";
                //     if let Some(img_buffer) = image_buffer.as_ref(){
                //         save_image_or_gif(img_buffer, format, "screen").ok();
                //     }
                //}
            
            egui::ComboBox::from_id_source("y")
            .selected_text(format!("{:}",app.format_tmp))
            .show_ui(ui,|ui|  {                                   
              ui.selectable_value(&mut app.format_tmp, "PNG".to_string(),  "PNG");
              ui.selectable_value( &mut app.format_tmp, "JPEG".to_string(),  "JPEG");
              ui.selectable_value( &mut app.format_tmp, "GIF".to_string(),  "GIF");
            });

           ui.end_row();
           
           if ui.button("Credit").clicked() {
            app.show_options = false;
            app.show_credit=true;
           
        }
        
        });


        // Define image_ui as content_ui child containing the image and with centered and justified layout
        // let mut image_rect = content_rect;
        // image_rect.min.y = content_rect.min.y + 10.0;
        // image_rect = image_rect.shrink(20.0);
        // let mut image_ui = content_ui.child_ui(image_rect, egui::Layout::centered_and_justified(egui::Direction::TopDown));


        // // Show image if taken holding real aspect_ratio or show a spinner
        // if let Some(texture) = app.texture.as_ref() {
        //     let available_size = image_ui.available_size();
        //     let aspect_ratio = texture.aspect_ratio();
        //     let mut size = available_size;
        //     size.x = size.y * aspect_ratio;

        //     if size.x > available_size.x || size.y > available_size.y {
        //         size = available_size;
        //         size.y = size.x / aspect_ratio;
        //     }

        //     image_ui.image((texture.id(), size));
        // } else {
        //     image_ui.add(Spinner::new().size(40.0));
        // }

    });}