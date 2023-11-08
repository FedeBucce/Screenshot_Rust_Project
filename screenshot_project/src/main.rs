extern crate scrap;
extern crate image;

use scrap::{Capturer, Display};
use image::{DynamicImage, ImageBuffer};
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

    let screenshot = DynamicImage::ImageRgba8(ImageBuffer::from_raw(w, h, screenshot.to_vec()).unwrap());

    save_image(&screenshot)?;

    let x = 0;  
    let y = 0;
    let width = 200;  
    let height = 200;

    let cropped_image = crop_image(&screenshot, x, y, width, height);

    save_image(&cropped_image)?;

    Ok(())
}

fn crop_image(image: &DynamicImage, x: u32, y: u32, width: u32, height: u32) -> DynamicImage {
    image.crop_imm(x, y, width, height)
}

fn save_image(image: &DynamicImage) -> Result<(), Error> {
    image.save(Path::new("screenshot.png")).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    Ok(())
}
