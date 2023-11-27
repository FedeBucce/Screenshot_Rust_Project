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
use std::time::Duration;
use gif::{Encoder, Frame};
use hotkey::modifiers;

struct ImageParams {
    x_pos: i32,
    y_pos: i32,
    width: u32,
    height: u32,
}

fn main() -> Result<(), Error> {
    // Tutti gli schermi totali, se si deve prendere il monitor principale ricorda che screen.display_info.is_primary = true
    let screens = Screen::all().unwrap();

    // Scelte da prendere dal front-end
    let choice = 0;
    let format: &str = "png";
    let time = Duration::from_secs(5);

    // Creazione Hotkey
    let mut hotkey = hotkey::Listener::new();
    hotkey.register_hotkey(modifiers::CONTROL | modifiers::SHIFT, 'A' as u32, || {
        println!("Ctrl-Shift-A pressed!"); //Insert code for hotkeys
    })
    .unwrap();

    // Rimane in ascolto della hot keys, probabilmente lo dovrò mettere in un thread separato
    // hotkey.listen();

    // Fa partire il timer
    //sleep(time);

    // Sceglie il monitor
    let screen = choose_screen(&screens, choice); // qua se mi serve lo schermo principale passo l'indice legato ad esso

    // Screen di tutta la schermata
    capture_full_screen_and_save(&screen, format, "screenshot_full")?;

    // Screen ritagliato con parametri inventanti che dovranno essere passati dal front-end
    let cropped_image_params = ImageParams {
        x_pos: 0,
        y_pos: 200,
        width: 1000,
        height: 500,
    };
    capture_cropped_screen_and_save(&screen, &cropped_image_params, format, "screenshot_cropped")?;

    Ok(())
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

fn capture_full_screen_and_save(screen: &Screen, format: &str, file_name: &str) -> Result<(), Error> {
    let image = &screen.capture().unwrap();
    save_image_or_gif(image, format, file_name)
}

fn capture_cropped_screen_and_save(screen: &Screen, params: &ImageParams, format: &str, file_name: &str) -> Result<(), Error> {
    let cropped_image = crop_image(screen, params);
    save_image_or_gif(&cropped_image, format, file_name)
}

fn save_image_or_gif(image: &image::ImageBuffer<Rgba<u8>, Vec<u8>>, format: &str, file_name: &str) -> Result<(), Error> {
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
