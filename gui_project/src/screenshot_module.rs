// screenshot_module.rs

extern crate image;
extern crate gif;
extern crate screenshots;
extern crate hotkey;

use image::Rgba;
use screenshots::Screen;
use std::fs::File;
use std::io::{Error, ErrorKind};
use std::path::Path;
use std::convert::TryInto;
use gif::{Encoder, Frame};


pub struct ImageParams {
    pub x_pos: i32,
    pub y_pos: i32,
    pub width: u32,
    pub height: u32,
}

pub fn choose_screen(screens: &Vec<Screen>, choice: usize) -> &Screen {
    match screens.get(choice) {
        Some(selected_screen) => selected_screen,
        None => {
            eprintln!("Invalid screen choice");
            panic!("Invalid screen choice");
        }
    }
}

pub fn capture_full_screen(screen: &Screen) -> Result<image::ImageBuffer<Rgba<u8>, Vec<u8>>, Error> {
    let image = screen.capture().unwrap();
    Ok(image)
}

pub fn capture_full_screen_and_save(screen: &Screen, format: &str, file_name: &str) -> Result<(), Error> {
    let image = &screen.capture().unwrap();
    save_image_or_gif(image, format, file_name)
}

pub fn capture_cropped_screen_and_save(screen: &Screen, params: &ImageParams, format: &str, file_name: &str) -> Result<(), Error> {
    let cropped_image = crop_image(screen, params);
    save_image_or_gif(&cropped_image, format, file_name)
}

pub fn save_image_or_gif(image: &image::ImageBuffer<Rgba<u8>, Vec<u8>>, format: &str, file_name: &str) -> Result<(), Error> {
    match format.to_lowercase().as_str() {
        "jpg" | "jpeg" | "png" | "gif" => {
            if format == "gif" {
                save_gif(image, &format!("{}.{}", file_name, format))
            } else {
                save_image(image, &format!("{}.{}", file_name, format))
            }
        }
        _ => Err(Error::new(ErrorKind::Other, "Unsupported format")),
    }
}

pub fn save_image(image: &image::ImageBuffer<Rgba<u8>, Vec<u8>>, path: &str) -> Result<(), Error> {
    image.save(Path::new(path)).map_err(|e| {
        Error::new(ErrorKind::Other, format!("Failed to save image: {}", e))
    })?;
    Ok(())
}

pub fn crop_image(screen: &Screen, params: &ImageParams) -> image::ImageBuffer<Rgba<u8>, Vec<u8>> {
    screen
        .capture_area(params.x_pos, params.y_pos, params.width, params.height)
        .unwrap()
}

pub fn save_gif(image: &image::ImageBuffer<Rgba<u8>, Vec<u8>>, path: &str) -> Result<(), Error> {
    let mut file = File::create(path)?;
    let mut encoder = Encoder::new(
        &mut file,
        image.width().try_into().unwrap(),
        image.height().try_into().unwrap(),
        &[],
    )
    .unwrap_or_else(|e| {
        eprintln!("Failed to create GIF encoder: {}", e);
        std::process::exit(1);
    });

    let frame = Frame::from_rgba_speed(
        image.width().try_into().unwrap(),
        image.height().try_into().unwrap(),
        &mut image.clone().into_raw(),
        10,
    );
    encoder.write_frame(&frame).map_err(|e| {
        Error::new(ErrorKind::Other, format!("Failed to write GIF frame: {}", e))
    })?;

    Ok(())
}
