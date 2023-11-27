#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use::egui::{TextureHandle, ColorImage};
use std::sync::Arc;
use screenshots::Screen;
use image::{ImageBuffer, RgbaImage};

mod screenshot_module;
use screenshot_module::*;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([450.0, 300.0]),
        persist_window: false,
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
    // screenshot: ImageBuffer<Rgba<u8>, Vec<u8>>
    texture: Option<TextureHandle>,
    screenshot: Option<Arc<ColorImage>>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            screens: Screen::all().unwrap(),
            texture: None,
            screenshot: None
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            if let Some(screenshot) = self.screenshot.take() {
                self.texture = Some(ui.ctx().load_texture(
                    "screenshot",
                    screenshot,
                    Default::default(),
                ));
            }

            // ui.heading("My egui Application");
            ui.horizontal(|ui| {
                if ui.button("+ New capture").clicked() {
                    let choice = 0;
                    let screen = choose_screen(&self.screens, choice);
                    let shot = capture_full_screen(&screen).unwrap();
                    let raw_data: &[u8] = &shot.as_raw();
                    let color_image = ColorImage::from_rgba_unmultiplied([shot.width() as usize, shot.height() as usize], raw_data);
                    self.screenshot = Option::from(Arc::new(color_image));
                }

                if ui.button("Save").clicked() {
                    if let Some(screenshot) = self.screenshot.take() {
                        println!("HERE");
                        let raw_data = screenshot.as_raw();
                        let image_buffer = RgbaImage::from_raw(screenshot.width() as u32, screenshot.height() as u32, Vec::from(raw_data));
                        let format: &str = "png";
                        if let Some(img_buffer) = image_buffer.as_ref(){
                            let res = save_image_or_gif(img_buffer, format, "screen");
                        }
                    }
                }
            });

            if let Some(texture) = self.texture.as_ref() {
                ui.image((texture.id(), ui.available_size()));
            } else {
                ui.spinner();
            }


               
        });
    }
}