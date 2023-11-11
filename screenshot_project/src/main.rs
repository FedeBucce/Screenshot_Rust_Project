extern crate scrap;
extern crate image;

use scrap::{Capturer, Display};
use image::{DynamicImage, Rgba, GenericImageView};
use std::io::Error;
use std::path::Path;

fn main() -> Result<(), Error> {
    // Ottieni il primo display disponibile
    let one_display = Display::primary()?;

    // Crea un nuovo oggetto Capturer per il display
    let mut capturer = Capturer::new(one_display)?;

    // Memorizza la larghezza e l'altezza del capturer
    let w = capturer.width() as u32;
    let h = capturer.height() as u32;

    println!("width: {}", w);
    println!("height: {}", h);

    // Cattura uno screenshot del display
    let screenshot = loop {
        match capturer.frame() {
            Ok(frame) => break frame,
            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                // Riprova
            }
            Err(error) => return Err(error),
        }
    };

    let screenshot = DynamicImage::ImageRgba8(image::ImageBuffer::from_raw(w, h, screenshot.to_vec()).unwrap());

    // Converte l'immagine RGBA in BGRA perche crap lavora in BGRA
    let screenshot = convert_rgba_to_bgra(&screenshot);

    let save_path = Path::new("screenshot.png");
    save_image(&screenshot, save_path)?;

    let x = 0;
    let y = 0;
    let width = 200;
    let height = 200;

    let cropped_image = crop_image(&screenshot, x, y, width, height);

    let cropped_save_path = Path::new("cropped_screenshot.png");
    save_image(&cropped_image, cropped_save_path)?;

     
    Ok(())
}

fn convert_rgba_to_bgra(image: &DynamicImage) -> DynamicImage {
    let mut bgra_image = image::ImageBuffer::new(image.width(), image.height());

    for (x, y, pixel) in bgra_image.enumerate_pixels_mut() {
        let rgba_pixel = image.get_pixel(x, y);
        *pixel = Rgba([rgba_pixel[2], rgba_pixel[1], rgba_pixel[0], rgba_pixel[3]]);
    }

    DynamicImage::ImageRgba8(bgra_image)
}

fn save_image(image: &DynamicImage, path: &Path) -> Result<(), Error> {
    image.save(path).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    Ok(())
}

fn crop_image(image: &DynamicImage, x: u32, y: u32, width: u32, height: u32) -> DynamicImage {
    image.crop_imm(x, y, width, height)
}