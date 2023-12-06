#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use egui::{Spinner, Shape, Pos2, Color32, Stroke,Grid};
use::egui::{TextureHandle, ColorImage};
use native_dialog::{FileDialog, MessageDialog, MessageType};
use eframe::egui::ViewportCommand;
use std::{sync::Arc, path::PathBuf};
use screenshots::Screen;
use image::RgbaImage;

mod screenshot_module;
use screenshot_module::*;

mod hotkeys;
use hotkeys::show_hotkeys_ui;
mod options;
use options::show_options_ui;
mod credit;
use credit::show_credit_ui;
fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    
    let options = eframe::NativeOptions {
        
        viewport: egui::ViewportBuilder::default()
        .with_transparent(true)
        .with_decorations(false)
        .with_inner_size([450.0, 400.0])
        .with_min_inner_size([450.0, 300.0]),
        
        
        ..Default::default()
    };

  

    eframe::run_native(
        "Screen capture",
        options,
        Box::new(|cc| {
            // egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::<MyApp>::default()
        }),
    )
}
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
enum Enum {
    First,
    Second,
    Third,
}


pub struct MyApp {
    screens: Vec<Screen>,
    texture: Option<TextureHandle>,
    screenshot: Option<Arc<ColorImage>>,
    show_main_screen: bool,
    show_capture_screen: bool,
    show_options: bool,
    show_hotkeys: bool,
    show_credit: bool,
    modifier_save_tmp: String,
    code_save_tmp: String,
    code_sh_tmp: String,
    modifier_sh_tmp: String,
    format_tmp: String,
    fullscreen: bool,
    path: PathBuf
    

   
}


impl MyApp {
    fn take_screenshot(&mut self) {
        let choice = 0;
        let screen = choose_screen(&self.screens, choice);
        let shot = capture_full_screen(&screen).unwrap();
        let raw_data: &[u8] = &shot.as_raw();
        let color_image = ColorImage::from_rgba_unmultiplied([shot.width() as usize, shot.height() as usize], raw_data);
        self.screenshot = Option::from(Arc::new(color_image));
        self.show_main_screen = true;
        

    }
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            screens: Screen::all().unwrap(),
            texture: None,
            screenshot: None,
            show_main_screen: true,
            show_capture_screen: false,
            show_options: false,
            show_hotkeys: false,
            show_credit: false,
            modifier_save_tmp:  "FN".to_string(),
            code_save_tmp: "I".to_string(),
            code_sh_tmp:  "X".to_string(),
            modifier_sh_tmp: "FN".to_string(),
            format_tmp: "JPG".to_string(),
            fullscreen: false,
            path: PathBuf::from(r"C:\Users\night\Desktop\PDS\Screenshot_Rust_Project\gui_project")


            

        }
    }
}

impl eframe::App for MyApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }


    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        let panel_frame = egui::Frame {
            fill: ctx.style().visuals.window_fill(),
            rounding: 10.0.into(),
            stroke: ctx.style().visuals.widgets.noninteractive.fg_stroke,
            outer_margin: 0.5.into(),
            ..Default::default()
        };
        
        if self.show_main_screen {
            // Define main window central panel
            egui::CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
    
                // Retrieve screenshot if taken
                if let Some(screenshot) = self.screenshot.as_ref() {
                    self.texture = Some(ui.ctx().load_texture(
                        "screenshot",
                        screenshot.clone(),
                        Default::default(),
                    ));
                }
    
                let app_rect = ui.max_rect();

                // Define title bar                
                let mut title_bar_rect = app_rect;
                title_bar_rect.max.y = title_bar_rect.min.y + 32.0;
                title_bar_ui(self,ui, title_bar_rect, "Screen capture");


    
                // Define content_ui as ui child containing buttons and image
                let mut content_rect = app_rect;
                content_rect.min.y = title_bar_rect.max.y;
                content_rect = content_rect.shrink(8.0);

                let mut content_ui = ui.child_ui(content_rect, *ui.layout());
                
                // Define New capture button and Save button aligned horizontally
                content_ui.horizontal(|ui| {
                    if ui.button("+ New capture").clicked() {
                        self.show_capture_screen = true;
                        self.show_main_screen = false;
                       
                    }

                    if ui.button("⚙").clicked() {
                        self.show_options=true;
                        self.show_main_screen = false;
                        
                       

                    }
                    if ui.button("Save").clicked() {
                        if let Some(screenshot) = self.screenshot.as_ref() {
                            let raw_data = screenshot.as_raw();
                            let image_buffer = RgbaImage::from_raw(screenshot.width() as u32, screenshot.height() as u32, Vec::from(raw_data));
                            let format: &str = "png";
                            if let Some(img_buffer) = image_buffer.as_ref(){
                                save_image_or_gif(img_buffer, format, "screen").ok();
                            }
                        }
                    }
                });
    

                // Define image_ui as content_ui child containing the image and with centered and justified layout
                let mut image_rect = content_rect;
                image_rect.min.y = content_rect.min.y + 10.0;
                image_rect = image_rect.shrink(20.0);
                let mut image_ui = content_ui.child_ui(image_rect, egui::Layout::centered_and_justified(egui::Direction::TopDown));
    

                // Show image if taken holding real aspect_ratio or show a spinner
                if let Some(texture) = self.texture.as_ref() {
                    let available_size = image_ui.available_size();
                    let aspect_ratio = texture.aspect_ratio();
                    let mut size = available_size;
                    size.x = size.y * aspect_ratio;
    
                    if size.x > available_size.x || size.y > available_size.y {
                        size = available_size;
                        size.y = size.x / aspect_ratio;
                    }

                    image_ui.image((texture.id(), size));
                } else {
                    image_ui.add(Spinner::new().size(40.0));
                }
    
            });
        }

        if self.show_capture_screen {
            // Define capture window
            ctx.show_viewport_immediate(
                egui::ViewportId::from_hash_of("capture"),
                egui::ViewportBuilder::default()
                    .with_transparent(true)
                    .with_decorations(false)
                    .with_inner_size([200.0, 100.0])
                    .with_min_inner_size([200.0, 100.0]),
                |ctx, class| {

                    // Define capture window central panel
                    egui::CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
                        let app_rect = ui.max_rect();

                        // Define title bar
                        let mut title_bar_rect = app_rect;
                        title_bar_rect.max.y = app_rect.min.y + 32.0;
                        title_bar_ui(self,ui, title_bar_rect, "Screen capture");


                        // Define content_ui as ui child capture button with centered layout
                        let mut content_rect = app_rect;
                        content_rect.min.y = title_bar_rect.max.y;
                        content_rect = content_rect.shrink(8.0);
                        let content_ui = ui.child_ui(content_rect, egui::Layout::centered_and_justified(egui::Direction::TopDown));
                        
                        // Define capture button
                        let circle_shape = egui::epaint::CircleShape {
                            center: content_ui.max_rect().center(),
                            radius: 36.0,
                            fill: Color32::WHITE,
                            stroke: Stroke::default()
                        };
                        content_ui.painter().add(Shape::Circle(circle_shape));

                        let circle_shape = egui::epaint::CircleShape {
                            center: content_ui.max_rect().center(),
                            radius: 32.0,
                            fill: Color32::WHITE,
                            stroke: Stroke::new(4.0, Color32::BLACK)
                        };
                        content_ui.painter().add(Shape::Circle(circle_shape));

                        // Define button click interaction
                        let interaction_rect = circle_shape.visual_bounding_rect();
                        let button_response = ui.interact(interaction_rect, egui::Id::new("button"), egui::Sense::click());
                        
                        if button_response.clicked() {
                            self.show_capture_screen = false;
                            self.show_main_screen = true;
                            // ui.set_visible(false); // Not running
                            self.take_screenshot();
                        }

                    });

                    if ctx.input(|i| i.viewport().close_requested()) {
                        // Close if x pressed
                        self.show_capture_screen = false;
                        self.show_main_screen = true;
                    }
                },
            );
        }

    //     if self.show_options {
    //         // Define capture window
    //         ctx.show_viewport_immediate(
    //             egui::ViewportId::from_hash_of("capture"),
    //             egui::ViewportBuilder::default()
    //                 .with_transparent(true)
    //                 .with_decorations(false)
    //                 .with_inner_size([200.0, 100.0])
    //                 .with_min_inner_size([200.0, 100.0]),
    //             |ctx, class| {

    //                 // Define capture window central panel
    //                 egui::CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
    //                     let app_rect = ui.max_rect();

    //                     // Define title bar
    //                     let mut title_bar_rect = app_rect;
    //                     title_bar_rect.max.y = app_rect.min.y + 32.0;
    //                     title_bar_ui(ui, title_bar_rect, "Screen capture");
    //                     Grid::new("settings_grid")
    //                     .num_columns(2)
    //                     .spacing([40.0, 4.0])
    //                     .striped(false)
    //                     .show(ui, |ui| {
    //                     ui.label("Change Hot Key");
    //                     ui.end_row();
    //                     ui.label("Path");
    //                     ui.end_row();
    //                     ui.label("Save");
    //                     ui.end_row();
    //                     ui.label("Default Hot Key");
                        



    //                     });


    //                 if ctx.input(|i| i.viewport().close_requested()) {
    //                     // Close if x pressed
    //                     self.show_options = false;
    //                     self.show_main_screen = true;
    //                 }
    //             },
    //         );
    //     })
    //  }
    
     if self.show_options {
        show_options_ui(self,ctx,panel_frame)    
                           }
     if self.show_hotkeys{ 
     show_hotkeys_ui(self,ctx,panel_frame);    
                         } 
    if self.show_credit{
        show_credit_ui(self,ctx,panel_frame);
    }
}
    }






pub fn title_bar_ui(app: &mut MyApp,ui: &mut egui::Ui, title_bar_rect: eframe::epaint::Rect, title: &str) {
    use egui::*;

    let painter = ui.painter();

    let title_bar_response = ui.interact(title_bar_rect, Id::new("title_bar"), Sense::click());
      
    // Paint the title:
    painter.text(
        title_bar_rect.center(),
        Align2::CENTER_CENTER,
        title,
        FontId::proportional(20.0),
        ui.style().visuals.text_color(),
    );
   

    // Interact with the title bar (drag to move window):
    if title_bar_response.is_pointer_button_down_on() {
        ui.ctx().send_viewport_cmd(ViewportCommand::StartDrag);
    }
          //MAXIMINIZE_MINIMIXE-UI
    ui.allocate_ui_at_rect(title_bar_rect, |ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
             
            
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.visuals_mut().button_frame = false;
            ui.add_space(8.0);
            close_maximize_minimize(app,ui);
            
         
         
           
        });
    });

        //HOMEPAGE AND BACK BUTTON
    ui.allocate_ui_at_rect(title_bar_rect, |ui| {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.visuals_mut().button_frame = false;
            ui.add_space(8.0);
          
            if ui.button("🏠").on_hover_text("Homepage").clicked() {
                app.show_main_screen=true;
                app.show_options=false;
                app.show_hotkeys=false;
                app.show_credit=false;
                
            }
               //undo_button from hotkeys
               if(app.show_hotkeys) 
               {
                        if ui.button("↩").on_hover_text("Back").clicked() {
                   app.show_hotkeys=false;
                   app.show_options=true;
                        }
               
               }    
    
               else if(app.show_credit)
               {
                if ui.button("↩").on_hover_text("Back").clicked() {
                   app.show_credit=false;
                   app.show_options=true;
                        }
               
               } 
        });
    });
}


fn close_maximize_minimize(app: &mut MyApp,ui: &mut egui::Ui) {
    use egui::{Button, RichText};

    let button_height = 12.0;
 
    let close_response = ui
        .add(Button::new(RichText::new("❌").size(button_height)))
        .on_hover_text("Close the window");
    if close_response.clicked() {
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
    }

    let minimized_response = ui
        .add(Button::new(RichText::new("🗕").size(button_height)))
        .on_hover_text("Minimize the window");
    if minimized_response.clicked() {
        ui.ctx().send_viewport_cmd(ViewportCommand::Minimized(true));
    }
    if(app.fullscreen==false){
        
    let maximized_response = ui
    .add(Button::new(RichText::new("□").size(button_height)))
    .on_hover_text("Full Screen");
if maximized_response.clicked() {
    ui.ctx().send_viewport_cmd(ViewportCommand::Maximized(true));
    app.fullscreen=true;
}
}
else{
    if(app.fullscreen==true){
       
    let maximized_response = ui
    .add(Button::new(RichText::new("□").size(button_height)))
    .on_hover_text("Exit Full Screen");
if maximized_response.clicked() {
    let vec_2=egui::Vec2::new(450.0,400.0);
    ui.ctx().send_viewport_cmd(ViewportCommand::InnerSize(vec_2));
    app.fullscreen=false;
}

}

}}