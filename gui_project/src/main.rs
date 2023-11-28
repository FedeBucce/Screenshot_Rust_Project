#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use::egui::{TextureHandle, ColorImage};
use eframe::egui::ViewportCommand;
use std::sync::Arc;
use screenshots::Screen;
use image::RgbaImage;

mod screenshot_module;
use screenshot_module::*;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
        .with_transparent(true)
        .with_decorations(false)
        .with_inner_size([450.0, 300.0])
        .with_min_inner_size([450.0, 300.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Screen capture",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::<MyApp>::default()
        }),
    )


}

fn choose_screen(screens: &Vec<Screen>, choice: usize) -> &Screen {
    match screens.get(choice) {
        Some(selected_screen) => selected_screen,
        None => {
            eprintln!("Invalid screen choice");
            panic!("Invalid screen choice");
        }
    }
}


struct MyApp {
    screens: Vec<Screen>,
    texture: Option<TextureHandle>,
    screenshot: Option<Arc<ColorImage>>,
    show_main_screen: bool,
    show_capture_screen: bool
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            screens: Screen::all().unwrap(),
            texture: None,
            screenshot: None,
            show_main_screen: true,
            show_capture_screen: false
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
    
        egui::CentralPanel::default().frame(panel_frame).show(ctx, |ui| {

            if let Some(screenshot) = self.screenshot.as_ref() {
                self.texture = Some(ui.ctx().load_texture(
                    "screenshot",
                    screenshot.clone(),
                    Default::default(),
                ));
            }





            let app_rect = ui.max_rect();

            let title_bar_height = 32.0;
            let title_bar_rect = {
                let mut rect = app_rect;
                rect.max.y = rect.min.y + title_bar_height;
                rect
            };

            let title = "Screen capture";
            title_bar_ui(ui, title_bar_rect, title);

            // Add the contents:
            let content_rect = {
                let mut rect = app_rect;
                rect.min.y = title_bar_rect.max.y;
                rect
            }
            .shrink(8.0);
            let mut content_ui = ui.child_ui(content_rect, *ui.layout());


            

            // ui.heading("My egui Application");
            content_ui.horizontal(|ui| {
                if ui.button("+ New capture").clicked() {
                    self.show_capture_screen = true;
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

            if let Some(texture) = self.texture.as_ref() {
                content_ui.image((texture.id(), ui.available_size()));
            } else {
                content_ui.spinner();
            }

        });



        if self.show_capture_screen {
            self.show_main_screen = false;
            ctx.show_viewport_immediate(
                egui::ViewportId::from_hash_of("capture"),
                egui::ViewportBuilder::default()
                    .with_transparent(true)
                    .with_decorations(false)
                    .with_inner_size([200.0, 100.0])
                    .with_min_inner_size([200.0, 100.0]),
                |ctx, class| {
                    assert!(
                        class == egui::ViewportClass::Immediate,
                        "This egui backend doesn't support multiple viewports"
                    );


                    
                    let panel_frame = egui::Frame {
                        fill: ctx.style().visuals.window_fill(),
                        rounding: 10.0.into(),
                        stroke: ctx.style().visuals.widgets.noninteractive.fg_stroke,
                        outer_margin: 0.5.into(),
                        ..Default::default()
                    };


                    egui::CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
                        let app_rect = ui.max_rect();

                        let title_bar_height = 32.0;
                        let title_bar_rect = {
                            let mut rect = app_rect;
                            rect.max.y = rect.min.y + title_bar_height;
                            rect
                        };

                        let title = "Screen capture";
                        title_bar_ui(ui, title_bar_rect, title);

                        // Add the contents:
                        let content_rect = {
                            let mut rect = app_rect;
                            rect.min.y = title_bar_rect.max.y;
                            rect
                        }
                        .shrink(8.0);
                        let mut content_ui = ui.child_ui(content_rect, *ui.layout());
                        
                        if content_ui.button("Capture").clicked() {
                            let choice = 0;
                            let screen = choose_screen(&self.screens, choice);
                            let shot = capture_full_screen(&screen).unwrap();
                            let raw_data: &[u8] = &shot.as_raw();
                            let color_image = ColorImage::from_rgba_unmultiplied([shot.width() as usize, shot.height() as usize], raw_data);
                            self.screenshot = Option::from(Arc::new(color_image));
                            self.show_capture_screen = false;
                            self.show_main_screen = true;
                        }
                    });

                    if ctx.input(|i| i.viewport().close_requested()) {
                        // Tell parent viewport that we should not show next frame:
                        self.show_capture_screen = false;
                        self.show_main_screen = true;
                    }
                },
            );
        }
    }
}




fn title_bar_ui(ui: &mut egui::Ui, title_bar_rect: eframe::epaint::Rect, title: &str) {
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

    // Paint the line under the title:
    // painter.line_segment(
    //     [
    //         title_bar_rect.left_bottom() + vec2(1.0, 0.0),
    //         title_bar_rect.right_bottom() + vec2(-1.0, 0.0),
    //     ],
    //     ui.visuals().widgets.noninteractive.bg_stroke,
    // );

    // Interact with the title bar (drag to move window):
    if title_bar_response.is_pointer_button_down_on() {
        ui.ctx().send_viewport_cmd(ViewportCommand::StartDrag);
    }

    ui.allocate_ui_at_rect(title_bar_rect, |ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.visuals_mut().button_frame = false;
            ui.add_space(8.0);
            close_maximize_minimize(ui);
        });
    });
}

/// Show some close/maximize/minimize buttons for the native window.
fn close_maximize_minimize(ui: &mut egui::Ui) {
    use egui::{Button, RichText};

    let button_height = 12.0;

    let close_response = ui
        .add(Button::new(RichText::new("❌").size(button_height)))
        .on_hover_text("Close the window");
    if close_response.clicked() {
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
    }

    // let is_maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
    // if is_maximized {
    //     let maximized_response = ui
    //         .add(Button::new(RichText::new("🗗").size(button_height)))
    //         .on_hover_text("Restore window");
    //     if maximized_response.clicked() {
    //         ui.ctx()
    //             .send_viewport_cmd(ViewportCommand::Maximized(false));
    //     }
    // } else {
    //     let maximized_response = ui
    //         .add(Button::new(RichText::new("🗗").size(button_height)))
    //         .on_hover_text("Maximize window");
    //     if maximized_response.clicked() {
    //         ui.ctx().send_viewport_cmd(ViewportCommand::Maximized(true));
    //     }
    // }

    let minimized_response = ui
        .add(Button::new(RichText::new("🗕").size(button_height)))
        .on_hover_text("Minimize the window");
    if minimized_response.clicked() {
        ui.ctx().send_viewport_cmd(ViewportCommand::Minimized(true));
    }
}