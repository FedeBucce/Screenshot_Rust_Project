extern crate image;
extern crate screenshots;

use image::Rgba;
use screenshots::Screen;
use std::io::Error;
use std::path::Path;

fn main() -> Result<(), Error> {
    let screens = Screen::all().unwrap();

    let choice = 0; // Modifica questa variabile per selezionare lo schermo desiderato

    let screen = match screens.get(choice) {
        Some(selected_screen) => selected_screen,
        None => {
            eprintln!("Invalid screen choice");
            return Ok(());
        }
    };

    let image = &screen.capture().unwrap();
    save_image(image, "screenshot_1.png".to_string())?;

    let cropped_image = crop_image(&screen, 0, 0, 1000, 500);
    save_image(&cropped_image, "screenshot_2.png".to_string())?;

    Ok(())
}

fn save_image(image: &image::ImageBuffer<Rgba<u8>, Vec<u8>>, path: String) -> Result<(), Error> {
    image.save(Path::new(&path)).map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to save image: {}", e))
    })?;
    Ok(())
}

fn crop_image(screen: &Screen, x: i32, y: i32, width: u32, height: u32) -> image::ImageBuffer<Rgba<u8>, Vec<u8>> {
    screen.capture_area(x, y, width, height).unwrap()
}
