use eframe::egui::{Rect, Visuals, SidePanel, Sense, Pos2, Vec2, Align, Button, DragValue, CentralPanel, Context, Layout, Direction, TopBottomPanel, ComboBox, ColorImage, ImageButton, Response, CursorIcon, Ui, Stroke};
use eframe::Frame;

use egui::Color32;
use rfd::FileDialog;

use screenshots::display_info::DisplayInfo;
use screenshots::Screen;
use image::DynamicImage;
use imageproc::drawing::draw_filled_circle_mut;

use std::collections::VecDeque;
use std::thread;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Duration;

use chrono::Local;

use global_hotkey::{GlobalHotKeyManager, hotkey::{HotKey, Modifiers, Code}, HotKeyState};
use global_hotkey::GlobalHotKeyEvent;


// Functions
pub fn bresenham_line(x0: usize, y0: usize, x1: usize, y1: usize) -> Vec<(usize, usize)> {
    let mut points = Vec::new();

    let dx = (x1 as i32 - x0 as i32).abs();
    let dy = (y1 as i32 - y0 as i32).abs();

    let mut x = x0 as i32;
    let mut y = y0 as i32;

    let x_inc = if x1 > x0 { 1 } else { -1 };
    let y_inc = if y1 > y0 { 1 } else { -1 };

    let mut error = dx - dy;

    while x != x1 as i32 || y != y1 as i32 {
        points.push((x as usize, y as usize));

        let error2 = error * 2;

        if error2 > -dy {
            error -= dy;
            x += x_inc;
        }

        if error2 < dx {
            error += dx;
            y += y_inc;
        }
    }

    points.push((x1, y1));

    points
}

fn draw_thick_line(img: &mut DynamicImage, start:(f32, f32), end:(f32, f32), t: usize, color: [u8; 4]) {
    let segment = bresenham_line(start.0 as usize, start.1 as usize, end.0 as usize, end.1 as usize);
    for point in segment {
        draw_filled_circle_mut(img, (point.0 as i32, point.1 as i32), t as i32, color.into());
    }
}

fn get_real_image_pos(pos: Pos2, image_rect_size: Vec2, real_image_size: [usize; 2]) -> Pos2{
    return Pos2::new(pos[0]*real_image_size[0] as f32/image_rect_size[0], pos[1]*real_image_size[1] as f32/image_rect_size[1]);
}

fn take_snapshot(disp: &DisplayInfo) -> Option<DynamicImage> {
        
    let screen : Screen = Screen::new(disp);
    let image_buffer = screen.capture().unwrap();
    
    // let im = image_buffer.rgba();
    // let stride = im.len() / image_buffer.height() as usize;
    // let img: ImageBuffer<image::Rgba<u8>, Vec<u8>> =
    //     ImageBuffer::from_fn(image_buffer.width() as u32, image_buffer.height() as u32, |x, y| {
    //         let i = stride * y as usize + 4 * x as usize;
    //         image::Rgba([im[i], im[i + 1], im[i + 2], im[i + 3]])
    //     });

    let dynamic_image = DynamicImage::from(image_buffer);
    Some(dynamic_image)
 }

fn string_to_key(s: &str) -> Code {
    match s.to_uppercase().as_str() {
        "A" => Code::KeyA,
        "B" => Code::KeyB,
        "C" => Code::KeyC,
        "D" => Code::KeyD,
        "E" => Code::KeyE,
        "F" => Code::KeyF,
        "G" => Code::KeyG,
        "H" => Code::KeyH,
        "I" => Code::KeyI,
        "J" => Code::KeyJ,
        "K" => Code::KeyK,
        "L" => Code::KeyL,
        "M" => Code::KeyM,
        "N" => Code::KeyN,
        "O" => Code::KeyO,
        "P" => Code::KeyP,
        "Q" => Code::KeyQ,
        "R" => Code::KeyR,
        "S" => Code::KeyS,
        "T" => Code::KeyT,
        "U" => Code::KeyU,
        "V" => Code::KeyV,
        "W" => Code::KeyW,
        "X" => Code::KeyX,
        "Y" => Code::KeyY,
        "Z" => Code::KeyZ,
        _ => panic!("Chiave non valida"),
    }
}

fn string_to_modifiers(s: &String) -> Option<Modifiers> {
    match s.as_str() {
        "ALT" => Some(Modifiers::ALT),
        "CTRL" => Some(Modifiers::CONTROL),
        "SHIFT" => Some(Modifiers::SHIFT),
        _ => {
            todo!("Handle unknown modifier: {}", s);
        }
    }
}

fn main() {
    let mut my_app_instance = SnapRustApp::default();

    my_app_instance.register_hotkey();

    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "SnapRust",
        native_options,
        Box::new(|_cc| Box::new(my_app_instance)),
    )
    .unwrap();
}



 #[derive(PartialEq)]
 enum Tool {
     None,
     Pen,
     Eraser,
     Crop
 }

struct SnapRustApp {
    snapshot: Option<DynamicImage>,
    snapshots_undo: VecDeque<DynamicImage>,
    snapshots_redo: VecDeque<DynamicImage>,
    display: Option<usize>,
    timer: Option<f64>,
    show_tools: bool,
    tool: Tool,
    tooling: bool,
    color: [u8; 4],
    size: usize,
    last_pos: Pos2,
    current_pos: Pos2,
    rx: Receiver<DynamicImage>,
    tx: Sender<DynamicImage>,
    modifier_save: String,
    code_save: String,
    modifier_take: String,
    code_take: String,
    manager: GlobalHotKeyManager,
}

impl Default for SnapRustApp {
    fn default() -> Self {
        let (tx, rx) = channel();

        SnapRustApp {
            snapshot: None,
            snapshots_undo: VecDeque::new(),
            snapshots_redo: VecDeque::new(),
            display: Some(0),
            timer: Some(0.),
            show_tools: false,
            tool: Tool::None,
            tooling: false,
            color: [255, 0, 0, 255],
            size: 1,
            last_pos: Pos2::default(),
            current_pos: Pos2::default(),
            rx: rx,
            tx: tx,
            modifier_save:  "SHIFT".to_string(),
            code_save: "D".to_string(),
            modifier_take:  "CTRL".to_string(),
            code_take: "A".to_string(),
            manager: GlobalHotKeyManager::new().expect("Failed to initialize GlobalHotKeyManager"),
        }
    }
}

impl SnapRustApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(Visuals::dark());
        SnapRustApp::default()
    }

    fn register_hotkey(&mut self) {
        let modifier_take = self.modifier_take.clone();
        let code_take = self.code_take.clone();
        let hotkey_take = HotKey::new(string_to_modifiers(&modifier_take), string_to_key(&code_take));

        let modifier_save = self.modifier_save.clone();
        let code_save = self.code_save.clone();
        let hotkey_save = HotKey::new(string_to_modifiers(&modifier_save), string_to_key(&code_save));

        if let Err(err) = self.manager.register(hotkey_take) {
            eprintln!("Failed to register hotkey: {}", err);
        }
        if let Err(err) = self.manager.register(hotkey_save) {
            eprintln!("Failed to register hotkey: {}", err);
        }
    }

    fn unregister_hotkeys(&mut self) {
        let hotkeys_to_unregister: Vec<HotKey> = vec![
            HotKey::new(string_to_modifiers(&self.modifier_take), string_to_key(&self.code_take)),
            HotKey::new(string_to_modifiers(&self.modifier_save), string_to_key(&self.code_save)),
        ];

        if let Err(err) = self.manager.unregister_all(&hotkeys_to_unregister) {
            eprintln!("Failed to unregister hotkeys: {}", err);
        }
    }

    fn get_snapshot(&mut self, ctx: &Context) {
   
        let display = self.display.unwrap().clone();
        let timer = self.timer.unwrap().clone() + 0.25;
        let tx = self.tx.clone();
        let context = ctx.clone();

        thread::spawn(move || {
            let display_info = match DisplayInfo::all() {
                Ok(display_vec) => Some(display_vec[display]),
                Err(_) => panic!("Invalid screen choice")
            };

            thread::sleep(Duration::from_millis((timer * 1000.0) as u64));

            let snapshot = take_snapshot(&display_info.unwrap()).unwrap();
            tx.send(snapshot).ok();
            context.request_repaint();

        });
    }

    fn register_hotkey_listener(&mut self, ctx: &Context, frame: &mut Frame) {
        if let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
            if event.state == HotKeyState::Pressed {
                let modifier_take = self.modifier_take.clone();
                let code_take = self.code_take.clone();
                let hotkey_take = HotKey::new(string_to_modifiers(&modifier_take), string_to_key(&code_take));
    
                let modifier_save = self.modifier_save.clone();
                let code_save = self.code_save.clone();
                let hotkey_save = HotKey::new(string_to_modifiers(&modifier_save), string_to_key(&code_save));
    
                if event.id == hotkey_take.id() {
                    frame.set_visible(false);
                    self.get_snapshot(ctx);
                }
    
                if event.id == hotkey_save.id() {
                    self.save_snapshot();
                }
            }
        }
    }

    fn save_snapshot(&mut self) {

        let formatted_date = Local::now().format("%Y_%m_%d_%H_%M_%S").to_string();

        let path = FileDialog::new()
            .set_file_name("snapshot_".to_string() + &formatted_date)
            .add_filter("PNG", &["png"])
            .add_filter("JPG", &["jpg"])
            .add_filter("GIF", &["gif"])
            .add_filter("BMP", &["bmp"])
            .set_directory("~")
            .save_file();

        match path {
            Some(path) => {
                let snapshot = self.snapshot.as_ref().unwrap();

                match image::save_buffer(
                    path,
                    &snapshot.as_bytes(),
                    snapshot.width() as u32,
                    snapshot.height() as u32,
                    image::ColorType::Rgba8,
                ) {
                    Ok(_) => {},
                    Err(err) => println!("{}", err),
                }
            },
            None => println!("Invalid path"),
        };
    }

    fn update_editing(&mut self, ui: &mut Ui, image_response: Response, real_image_size: [usize; 2]) {
        if image_response.dragged(){

            // Update last and current position
            if !self.tooling {
                self.last_pos = match image_response.hover_pos() {
                    Some(pos) => (pos - image_response.rect.left_top()).to_pos2(),
                    None => self.last_pos,
                };
                self.tooling = true;
            }

            self.current_pos = match image_response.hover_pos() {
                Some(pos) => (pos - image_response.rect.left_top()).to_pos2(),
                None => self.current_pos,
            };


            // Apply tool
            if self.tool == Tool::Pen {
                let image_last_pos = get_real_image_pos(self.last_pos, image_response.rect.size(), real_image_size);
                let image_current_pos = get_real_image_pos(self.current_pos, image_response.rect.size(), real_image_size);
                
                draw_thick_line(&mut self.snapshot.as_mut().unwrap(),
                    image_last_pos.into(),
                    image_current_pos.into(),
                    self.size,
                    self.color.into());
                
                self.last_pos = self.current_pos;
            }
            else if self.tool == Tool::Crop {
                let rect = Rect::from_min_max(self.last_pos + image_response.rect.left_top().to_vec2(), self.current_pos + image_response.rect.left_top().to_vec2());
                let stroke = Stroke::new(1., Color32::from_rgba_premultiplied(255, 255, 255, 150));
                ui.painter().add(egui::Shape::dashed_line(&[rect.left_top(), rect.right_top(), rect.right_bottom(), rect.left_bottom(), rect.left_top()], stroke, 6., 6.));
            }

            

        } else if image_response.drag_released() {  
            if self.tool == Tool::Pen {
                self.snapshots_undo.push_back(self.snapshot.as_ref().unwrap().clone());
                self.snapshots_redo.clear();
            }
            else if self.tool == Tool::Crop {
                let image_last_pos = get_real_image_pos(self.last_pos, image_response.rect.size(), real_image_size);
                let image_current_pos = get_real_image_pos(self.current_pos, image_response.rect.size(), real_image_size);
                
                let width = (image_current_pos[0] - image_last_pos[0]).abs();
                let height = (image_current_pos[1] - image_last_pos[1]).abs();

                let mut crop_start_pos = image_last_pos;

                if image_current_pos[0] < crop_start_pos[0]{
                    crop_start_pos[0] -= width;
                }
                if image_current_pos[1] < crop_start_pos[1]{
                    crop_start_pos[1] -= height;
                }

                let cropped_image = self.snapshot.as_ref().unwrap().crop_imm(crop_start_pos[0] as u32, crop_start_pos[1] as u32, width as u32, height as u32);
                self.snapshot = Some(cropped_image.clone());
                self.snapshots_undo.push_back(cropped_image);
                self.snapshots_redo.clear();
            }
            
        }
        else{
            self.tooling = false;
        }
        
        match self.tool {
            Tool::None => {}
            // Tool::Text => {
            //     if !self.paint_info.text_info.writing{
            //         img.on_hover_cursor(CursorIcon::Text);
            //     }
            // }
            _ => {
                image_response.on_hover_cursor(CursorIcon::Crosshair);
            }
        };
    }

    fn render_top_panel(&mut self, ctx: &Context, frame: &mut Frame) {
        TopBottomPanel::top("top panel")
            .exact_height(36.)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                        let mut top_left_rect = ui.max_rect();
                        top_left_rect.max.y = top_left_rect.min.y + 36.;
                        top_left_rect.max.x = top_left_rect.max.x/2.;
                        top_left_rect = top_left_rect.shrink(4.0);
                        let mut top_left_panel_ui = ui.child_ui(top_left_rect, *ui.layout());

                        top_left_panel_ui.horizontal(|ui| {
                            let snapshot_button = ui.add(Button::new("📷 Snapshot"));
                            if snapshot_button.clicked() {
                                frame.set_visible(false);
                                self.get_snapshot(ctx);
                            }

                            if !self.snapshots_undo.is_empty(){
                                let save_button = ui.add(Button::new("💾 Save as"));
                                if save_button.clicked() {
                                    self.save_snapshot();
                                }
                            }
                            
                            ComboBox::from_id_source(0)
                            .selected_text(format!("🖵 Display {}", self.display.unwrap()))
                            .show_ui(ui, |ui| {
                                for (i, display) in DisplayInfo::all().unwrap().iter().enumerate(){
                                    ui.selectable_value(
                                        &mut self.display,
                                        Some(i),
                                        format!("🖵 Display {}  {}x{}", i, display.width as f32 * display.scale_factor, display.height as f32 * display.scale_factor)
                                    );
                                }
                                
                            });

                            ComboBox::from_id_source(1)
                            .selected_text(format!("🕓 {} sec", self.timer.unwrap()))
                            .show_ui(ui, |ui| {
                                let timer_values = [0., 1., 2., 3., 5., 10.]; 
                                for timer_val in timer_values{
                                    ui.selectable_value(
                                        &mut self.timer,
                                        Some(timer_val),
                                        format!("🕓 {} sec", timer_val)
                                    );
                                }
                            });

                        });
                    });

                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        let mut top_right_rect = ui.max_rect();
                        top_right_rect.max.y = top_right_rect.min.y + 36.;
                        top_right_rect.min.x = top_right_rect.max.x/2.;
                        top_right_rect = top_right_rect.shrink(4.0);
                        let mut top_right_panel_ui = ui.child_ui(top_right_rect, *ui.layout());

                        top_right_panel_ui.horizontal(|ui| {
                            if !self.snapshots_undo.is_empty(){
                                let tool_toggle_button = ui.add(Button::new("🔧 Show tools"));
                                if tool_toggle_button.clicked() {
                                    self.show_tools = !self.show_tools;
                                }
                            }
                        });
                    });
                });
            });
    }

    fn render_side_panel(&mut self, ctx: &Context, frame: &mut Frame) {
        if self.show_tools{
            SidePanel::right("right panel")
            .exact_width(80.)
            .show(ctx, |ui| {
                let mut side_rect = ui.max_rect();
                side_rect.max.x = side_rect.max.x - 12.;
                side_rect = side_rect.shrink(4.0);
                let mut side_panel_ui = ui.child_ui(side_rect, *ui.layout());


                side_panel_ui.vertical(|ui| {
                    ui.with_layout(Layout::top_down(Align::Center), |ui| {
                        let button_size = Vec2::from([45., 45.]);
                        
                        let none_button = ui.add(Button::new("None").rounding(5.).min_size(button_size));
                        if none_button.clicked() {
                            self.tool = Tool::None;
                        }
                        ui.separator();

                        let pen_button = ui.add(Button::new("Pen").rounding(5.).min_size(button_size));
                        if pen_button.clicked() {
                            self.tool = Tool::Pen;
                        }

                        ui.separator();

                        let eraser_button = ui.add(Button::new("Erase").rounding(5.).min_size(button_size));
                        if eraser_button.clicked() {
                            self.tool = Tool::Eraser;
                        }
                        ui.separator();


                        let crop_button = ui.add(Button::new("Crop").rounding(5.).min_size(button_size));
                        if crop_button.clicked() {
                            self.tool = Tool::Crop;
                        }
                        ui.separator();

                        let undo_button = ui.add(Button::new("Undo").rounding(5.).min_size(button_size));
                        if undo_button.clicked() {
                            if self.snapshots_undo.len()>1{
                                self.snapshots_redo.push_front(self.snapshots_undo.pop_back().unwrap());
                                self.snapshot = Some(self.snapshots_undo.get(self.snapshots_undo.len()-1).unwrap().clone());
                            }
                        }
                        ui.separator();

                        let redo_button = ui.add(Button::new("Redo").rounding(5.).min_size(button_size));
                        if redo_button.clicked() {
                            if self.snapshots_redo.len()>0{
                                self.snapshots_undo.push_back(self.snapshots_redo.pop_front().unwrap());
                                self.snapshot = Some(self.snapshots_undo.get(self.snapshots_undo.len()-1).unwrap().clone());
                            }
                        }
                        ui.separator();


                        match self.tool {
                            Tool::None => none_button.highlight(),
                            Tool::Pen => pen_button.highlight(),
                            Tool::Eraser => eraser_button.highlight(),
                            Tool::Crop => crop_button.highlight(),
                        };


                        if self.tool == Tool::Pen {
                            ui.color_edit_button_srgba_unmultiplied(&mut self.color);
                        }

                        if self.tool == Tool::Pen || self.tool == Tool::Eraser {
                            ui.add(DragValue::new(&mut self.size));

                            if self.size<1{
                                self.size = 1;
                            }
                            if self.size>30{
                                self.size = 30;
                            }

                        }


                    });
                    
                });
            });
        }
    }

    fn render_central_panel(&mut self, ctx: &Context, frame: &mut Frame) {

        CentralPanel::default().show(ctx, |ui| { 
            if self.snapshot.is_some(){
                
                ui.with_layout(Layout::top_down(Align::Center), |ui| {
                    let snapshot = self.snapshot.as_ref().unwrap();
                    let color_image = ColorImage::from_rgba_unmultiplied(
                        [snapshot.width() as usize, snapshot.height() as usize],
                        snapshot.as_bytes(),
                    );

                    let texture_handle = ui.ctx().load_texture(
                        "screenshot",
                        color_image,
                        Default::default(),
                    );

                    
                    let available_size = ui.available_size();
                    let aspect_ratio = texture_handle.aspect_ratio();
                    let mut size = available_size;
                    size.x = size.y * aspect_ratio;

                    if size.x > available_size.x || size.y > available_size.y {
                        size = available_size;
                        size.y = size.x / aspect_ratio;
                    }


                    let image_rect = Rect::from_center_size(ui.max_rect().center(), size);
                    let mut image_ui = ui.child_ui(image_rect, *ui.layout());



                    let image_response = image_ui.add(ImageButton::new(texture_handle.id(), size).frame(false).sense(Sense::click_and_drag()));
                    
                    if self.tool != Tool::None {
                        self.update_editing(&mut image_ui, image_response, texture_handle.size()); 
                    }
                });
            }
            else {
                ui.with_layout(Layout::centered_and_justified(Direction::TopDown), |ui| {
                    ui.add(egui::Spinner::new().size(40.0));
                });
            }
            
        });
    }

}

impl eframe::App for SnapRustApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        match self.rx.try_recv() {
            Ok(snapshot) => {
                self.snapshots_undo.clear();
                self.snapshots_redo.clear();

                self.snapshot = Some(snapshot.clone());
                self.snapshots_undo.push_back(snapshot);

                frame.set_visible(true);
            }
            Err(_) => {}
            
        }
        self.register_hotkey_listener(ctx, frame);
        
        self.render_top_panel(ctx, frame);
        self.render_central_panel(ctx, frame);
        self.render_side_panel(ctx, frame);
    }
}
