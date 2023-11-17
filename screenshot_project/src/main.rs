extern crate image;
extern crate gif;
extern crate screenshots;

use image::Rgba;
use screenshots::Screen;
use std::fs::File;
use std::io::{Error, ErrorKind};
use std::path::Path;
use std::convert::TryInto;
use gif::{Encoder, Frame};

struct ImageParams {
    x_pos: i32,
    y_pos: i32,
    width: u32,
    height: u32,
}

fn main() -> Result<(), Error> {
    let screens = Screen::all().unwrap();

    let choice = 1;
    let format: &str = "gif";

    let screen = match screens.get(choice) {
        Some(selected_screen) => selected_screen,
        None => {
            eprintln!("Invalid screen choice");
            return Ok(());
        }
    };

    let mut image_params = ImageParams {
        x_pos: 0,
        y_pos: 0,
        width: screen.display_info.width,
        height: screen.display_info.height,
    };

    let image = &screen.capture().unwrap();
    if format == "gif" {
        save_gif(image, &format!("screenshot_1.{}", format))?;
    } else {
        save_image(image, &format!("screenshot_1.{}", format))?;
    }

    image_params.x_pos = 500;
    image_params.y_pos = 200;
    image_params.width = 1000;
    image_params.height = 500;

    let cropped_image = crop_image(&screen, &image_params);
    if format == "gif" {
        save_gif(&cropped_image, &format!("screenshot_2.{}", format))?;
    } else {
        save_image(&cropped_image, &format!("screenshot_2.{}", format))?;
    }

    Ok(())
}

fn save_image(image: &image::ImageBuffer<Rgba<u8>, Vec<u8>>, path: &str) -> Result<(), Error> {
    image.save(Path::new(path)).map_err(|e| {
        Error::new(ErrorKind::Other, format!("Failed to save image: {}", e))
    })?;
    Ok(())
}

fn crop_image(screen: &Screen, params: &ImageParams) -> image::ImageBuffer<Rgba<u8>, Vec<u8>> {
    screen
        .capture_area(params.x_pos, params.y_pos, params.width, params.height)
        .unwrap()
}

fn save_gif(image: &image::ImageBuffer<Rgba<u8>, Vec<u8>>, path: &str) -> Result<(), Error> {
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
