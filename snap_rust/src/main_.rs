use eframe::egui::{Shape, Rect, Visuals, SidePanel, Sense, Pos2, Vec2, Align, Button, DragValue, CentralPanel, Context, Layout, Direction, TopBottomPanel, ComboBox, ColorImage, ImageButton, Response, CursorIcon, Ui, Stroke};
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

use arboard::{Clipboard, ImageData};
use std::borrow::Cow;

use std::collections::HashSet;




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



fn linear_to_srgb(lin_rgb: [f32; 3]) -> [u8; 3] {
    let mut srgb: [u8; 3] = [0; 3];

    for i in 0..3 {
        let v = lin_rgb[i];
        if v <= 0.0 {
            srgb[i] = 0 as u8;
        } else if v >= 1.0 {
            srgb[i] = 255 as u8;
        } else {
            srgb[i] = (v.powf(1.0 / 2.2) * 255.0 + 0.5) as u8; // Corrected formula
        }
    }

    return srgb;
}

fn get_real_image_pos(pos: Pos2, image_rect_size: Vec2, real_image_size: [usize; 2]) -> Pos2{
    return Pos2::new(pos[0]*real_image_size[0] as f32/image_rect_size[0], pos[1]*real_image_size[1] as f32/image_rect_size[1]);
}

fn take_snapshot(disp: &DisplayInfo) -> Option<DynamicImage> {
    let screen : Screen = Screen::new(disp);
    let image_buffer = screen.capture().unwrap();
    let dynamic_image = DynamicImage::from(image_buffer);
    Some(dynamic_image)
 }

fn string_to_key(s: &str) -> Option<Code> {
    match s.to_uppercase().as_str() {
        "A" => Some(Code::KeyA),
        "B" => Some(Code::KeyB),
        "C" => Some(Code::KeyC),
        "D" => Some(Code::KeyD),
        "E" => Some(Code::KeyE),
        "F" => Some(Code::KeyF),
        "G" => Some(Code::KeyG),
        "H" => Some(Code::KeyH),
        "I" => Some(Code::KeyI),
        "J" => Some(Code::KeyJ),
        "K" => Some(Code::KeyK),
        "L" => Some(Code::KeyL),
        "M" => Some(Code::KeyM),
        "N" => Some(Code::KeyN),
        "O" => Some(Code::KeyO),
        "P" => Some(Code::KeyP),
        "Q" => Some(Code::KeyQ),
        "R" => Some(Code::KeyR),
        "S" => Some(Code::KeyS),
        "T" => Some(Code::KeyT),
        "U" => Some(Code::KeyU),
        "V" => Some(Code::KeyV),
        "W" => Some(Code::KeyW),
        "X" => Some(Code::KeyX),
        "Y" => Some(Code::KeyY),
        "Z" => Some(Code::KeyZ),
        _ => None,
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
    let mut native_options = eframe::NativeOptions::default();
    native_options.min_window_size = Some(Vec2::new(750., 500.));

    eframe::run_native(
        "SnapRust",
        native_options,
        Box::new(|cc| Box::new(SnapRustApp::new(cc))),
    )
    .unwrap();
}



 #[derive(PartialEq)]
 enum Tool {
     None,
     Pen,
     Highlighter,
     Crop
 }

struct Hotkey {
    label: String,
    modifier: String,
    tmp_modifier: String,
    code: String,
    tmp_code: String,
    registered_hotkey: HotKey
}

impl Hotkey {
    fn new(label: String, modifier: String, code: String) -> Self {
        let registered_hotkey = HotKey::new(string_to_modifiers(&modifier), string_to_key(&code).unwrap());
        Hotkey{
            label: label,
            modifier: modifier.clone(),
            tmp_modifier: modifier,
            code: code.clone(),
            tmp_code: code,
            registered_hotkey: registered_hotkey
        }
    }
}


struct SnapRustApp {
    snapshot: Option<DynamicImage>,
    snapshots_undo: VecDeque<DynamicImage>,
    snapshots_redo: VecDeque<DynamicImage>,
    display: Option<usize>,
    timer: Option<f64>,
    show_settings: bool,
    show_credits: bool,
    show_tools: bool,
    tool: Tool,
    tooling: bool,
    pen_color: [f32; 3],
    pen_size: usize,
    highlighter_color: [f32; 3],
    highlighter_size: usize,
    last_pos: Pos2,
    current_pos: Pos2,
    rx: Receiver<DynamicImage>,
    tx: Sender<DynamicImage>,
    hotkeys: Vec<Hotkey>,
    manager: GlobalHotKeyManager,
}

impl Default for SnapRustApp {
    fn default() -> Self {
        let (tx, rx) = channel();

        let mut hotkeys_vec: Vec<Hotkey> = Vec::new();
        hotkeys_vec.push(Hotkey::new("Copy".to_string(), "CTRL".to_string(), "C".to_string()));
        hotkeys_vec.push(Hotkey::new("Save".to_string(), "CTRL".to_string(), "S".to_string()));
        hotkeys_vec.push(Hotkey::new("Take".to_string(), "CTRL".to_string(), "T".to_string()));
        hotkeys_vec.push(Hotkey::new("None".to_string(), "CTRL".to_string(), "N".to_string()));
        hotkeys_vec.push(Hotkey::new("Pen".to_string(), "CTRL".to_string(), "P".to_string()));
        hotkeys_vec.push(Hotkey::new("Crop".to_string(), "CTRL".to_string(), "X".to_string()));
        hotkeys_vec.push(Hotkey::new("Undo".to_string(), "CTRL".to_string(), "Z".to_string()));
        hotkeys_vec.push(Hotkey::new("Redo".to_string(), "CTRL".to_string(), "Y".to_string()));

        SnapRustApp {
            snapshot: None,
            snapshots_undo: VecDeque::new(),
            snapshots_redo: VecDeque::new(),
            display: Some(0),
            timer: Some(0.),
            show_settings: false,
            show_credits: false,
            show_tools: false,
            tool: Tool::None,
            tooling: false,
            pen_color: [0.9, 0.3, 0.24],
            //pen_color: [230., 76., 60.],
            pen_size: 1,
            highlighter_color: [0.9, 0.3, 0.24],
            //highlighter_color: [230., 76., 60.],
            highlighter_size: 1,
            last_pos: Pos2::default(),
            current_pos: Pos2::default(),
            rx: rx,
            tx: tx,
            hotkeys: hotkeys_vec,
            manager: GlobalHotKeyManager::new().expect("Failed to initialize GlobalHotKeyManager"),
        }
    }
}

impl SnapRustApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(Visuals::dark());
        let mut app = SnapRustApp::default();
        app.register_hotkey();
        return app;
    }

    fn register_hotkey(&mut self) {
        for hotkey in self.hotkeys.iter_mut() {
            let modifier = hotkey.modifier.clone();
            let code = hotkey.code.clone();
            let registered_hotkey = HotKey::new(string_to_modifiers(&modifier), string_to_key(&code).unwrap());
            hotkey.registered_hotkey = registered_hotkey.clone();
            if let Err(err) = self.manager.register(registered_hotkey) {
                eprintln!("Failed to register hotkey: {}", err);
            }
        }
    }

    fn unregister_hotkeys(&mut self) {
        let mut hotkeys_to_unregister: Vec<HotKey> = Vec::new();

        for hotkey in self.hotkeys.iter() {
            hotkeys_to_unregister.push(hotkey.registered_hotkey);
        } 

        if let Err(err) = self.manager.unregister_all(&hotkeys_to_unregister) {
            eprintln!("Failed to unregister hotkeys: {}", err);
        }
    }

    fn get_snapshot(&mut self, ctx: &Context) {
   
        let display = self.display.unwrap().clone();
        let mut timer = self.timer.unwrap().clone();

        if timer < 1.{
            timer += 0.35;
        }

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
                let mut action: Option<String> = None;

                for hotkey in self.hotkeys.iter() {
                    if event.id == hotkey.registered_hotkey.id() {
                        action = Some(hotkey.label.clone());
                    }
                }

                match action {
                    Some(action_value) => {
                        if action_value =="Take".to_string() {
                            frame.set_visible(false);
                            self.get_snapshot(ctx);
                        }
                        else if action_value =="Save".to_string() {
                            if self.snapshot.is_some(){
                                self.save_snapshot();
                            }
                        }
                        else if action_value =="Copy".to_string() {
                            if self.snapshot.is_some(){
                                self.copy_snapshot();
                            }
                        }
                        else if action_value =="None".to_string() {
                            if self.snapshot.is_some(){
                                self.tool = Tool::None;
                            }
                        }
                        else if action_value =="Pen".to_string() {
                            if self.snapshot.is_some(){
                                self.tool = Tool::Pen;
                            }
                        }
                        else if action_value =="Crop".to_string() {
                            if self.snapshot.is_some(){
                                self.tool = Tool::Crop;
                            }
                        }
                        else if action_value =="Undo".to_string() {
                            if self.snapshot.is_some(){
                                self.undo();
                            }
                        }
                        else if action_value =="Redo".to_string() {
                            if self.snapshot.is_some(){
                                self.redo();
                            }
                        }
                    }
                    None => {
                        println!("No hotkey available");
                    }
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

    fn copy_snapshot(&mut self) {
        let mut clipboard = Clipboard::new().unwrap();
        let snapshot = self.snapshot.as_ref().unwrap();
        let image = ImageData{
            width: snapshot.width() as usize,
            height: snapshot.height() as usize,
            bytes: Cow::from(snapshot.as_bytes())
        };
        clipboard.set_image(image).ok();
    }

    fn undo(&mut self) {
        if self.snapshots_undo.len()>1{
            self.snapshots_redo.push_front(self.snapshots_undo.pop_back().unwrap());
            self.snapshot = Some(self.snapshots_undo.get(self.snapshots_undo.len()-1).unwrap().clone());
        }
    }

    fn redo(&mut self) {
        if self.snapshots_redo.len()>0{
            self.snapshots_undo.push_back(self.snapshots_redo.pop_front().unwrap());
            self.snapshot = Some(self.snapshots_undo.get(self.snapshots_undo.len()-1).unwrap().clone());
        }
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
            if self.tool == Tool::Pen || self.tool == Tool::Highlighter{
                let image_last_pos = get_real_image_pos(self.last_pos, image_response.rect.size(), real_image_size);
                let image_current_pos = get_real_image_pos(self.current_pos, image_response.rect.size(), real_image_size);
                
                if self.tool == Tool::Pen {
                    let rgb_color = linear_to_srgb(self.pen_color);
                    let color = [rgb_color[0], rgb_color[1], rgb_color[2], 255];
                    draw_thick_line(&mut self.snapshot.as_mut().unwrap(),
                        image_last_pos.into(),
                        image_current_pos.into(),
                        self.pen_size,
                        color.into()
                    );
                }
                else if self.tool == Tool::Highlighter{
                    let rgb_color = linear_to_srgb(self.highlighter_color);

                    let color = [rgb_color[0], rgb_color[1], rgb_color[2], 50];
                    draw_thick_line(&mut self.snapshot.as_mut().unwrap(),
                        image_last_pos.into(),
                        image_current_pos.into(),
                        self.highlighter_size,
                        color.into()
                    );
                }
                
                self.last_pos = self.current_pos;
            }
            else if self.tool == Tool::Crop {
                let rect = Rect::from_min_max(self.last_pos + image_response.rect.left_top().to_vec2(), self.current_pos + image_response.rect.left_top().to_vec2());
                let stroke = Stroke::new(1., Color32::from_rgba_premultiplied(255, 255, 255, 150));
                ui.painter().add(Shape::dashed_line(&[rect.left_top(), rect.right_top(), rect.right_bottom(), rect.left_bottom(), rect.left_top()], stroke, 6., 6.));
            }

            

        } else if image_response.drag_released() {  
            if self.tool == Tool::Pen || self.tool == Tool::Highlighter {
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
            _ => {
                image_response.on_hover_cursor(CursorIcon::Crosshair);
            }
        };
    }

    fn render_top_panel(&mut self, ctx: &Context, frame: &mut Frame) {
        if !self.show_settings && !self.show_credits {
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

                            if self.snapshot.is_some(){
                                let save_button = ui.add(Button::new("💾 Save as"));
                                if save_button.clicked() {
                                    self.save_snapshot();
                                }
                            }

                            if self.snapshot.is_some(){
                                let copy_button = ui.add(Button::new("📄 Copy"));
                                if copy_button.clicked() {
                                    self.copy_snapshot();
                                }
                            }

                            let settings_button = ui.add(Button::new("🔨 Settings"));
                            if settings_button.clicked() {

                                for hotkey in self.hotkeys.iter_mut(){
                                    hotkey.tmp_code = hotkey.code.clone();
                                    hotkey.tmp_modifier = hotkey.modifier.clone();
                                }

                                self.show_tools = false;
                                self.show_settings = true;
                            }

                            let credits_button = ui.add(Button::new("💻 Credits"));
                            if credits_button.clicked() {
                                self.show_tools = false;
                                self.show_credits = true;
                            }

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
            });
        }
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
                        let button_size = Vec2::from([40., 40.]);
                        
                        let none_button = ui.add(Button::new("🚫").rounding(5.).min_size(button_size));
                        if none_button.clicked() {
                            self.tool = Tool::None;
                        }
                        ui.separator();

                        let pen_button = ui.add(Button::new("✏").rounding(5.).min_size(button_size));
                        if pen_button.clicked() {
                            self.tool = Tool::Pen;
                        }
                        ui.separator();

                        let highlighter_button = ui.add(Button::new("🖍️").rounding(5.).min_size(button_size));
                        if highlighter_button.clicked() {
                            self.tool = Tool::Highlighter;
                        }
                        ui.separator();


                        let crop_button = ui.add(Button::new("✂").rounding(5.).min_size(button_size));
                        if crop_button.clicked() {
                            self.tool = Tool::Crop;
                        }
                        ui.separator();

                        let undo_button = ui.add(Button::new("↩").rounding(5.).min_size(button_size));
                        if undo_button.clicked() {
                            self.undo();
                        }
                        ui.separator();

                        let redo_button = ui.add(Button::new("↪").rounding(5.).min_size(button_size));
                        if redo_button.clicked() {
                            self.redo();
                        }
                        ui.separator();


                        match self.tool {
                            Tool::None => none_button.highlight(),
                            Tool::Pen => pen_button.highlight(),
                            Tool::Highlighter => highlighter_button.highlight(),
                            Tool::Crop => crop_button.highlight(),
                        };
                        
                    });

                    if self.tool == Tool::Pen {
                        ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
                            ui.add_space(10.);

                            ui.add(DragValue::new(&mut self.pen_size));
                            if self.pen_size<1{
                                self.pen_size = 1;
                            }
                            if self.pen_size>30{
                                self.pen_size = 30;
                            }
                            ui.add_space(5.);
                            ui.color_edit_button_rgb(&mut self.pen_color);
                            ui.add_space(5.);
                            ui.separator();
                        });
                    }
                    else if self.tool == Tool::Highlighter {
                        ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
                            ui.add_space(10.);

                            ui.add(DragValue::new(&mut self.highlighter_size));
                            if self.highlighter_size<1{
                                self.highlighter_size = 1;
                            }
                            if self.highlighter_size>30{
                                self.highlighter_size = 30;
                            }
                            ui.add_space(5.);
                            ui.color_edit_button_rgb(&mut self.highlighter_color);
                            ui.add_space(5.);
                            ui.separator();
                        });
                    }
                });
            });
        }
    }

    fn render_central_panel(&mut self, ctx: &Context, frame: &mut Frame) {

        CentralPanel::default().show(ctx, |ui| {
            if !self.show_settings && !self.show_credits{
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
            }
            else if self.show_settings{
                let mut shortcut_rect = ui.max_rect();
                let center = shortcut_rect.center();
                shortcut_rect.min.x = center[0] - 150.;
                shortcut_rect.max.x = center[0] + 150.;

                shortcut_rect.min.y = center[1] - 140.;
                shortcut_rect.max.y = center[1] + 140.;

                let shortcut_frame_rect = shortcut_rect.shrink(-40.);
                ui.painter().add(Shape::rect_filled(shortcut_frame_rect, 5., Color32::from_rgba_premultiplied(20, 20, 20, 100)));
                
                let mut shortcut_ui = ui.child_ui(shortcut_rect, *ui.layout());

                shortcut_ui.vertical(|ui| {                    
                    for (i, hotkey) in self.hotkeys.iter_mut().enumerate() {
                        ui.horizontal(|ui| {

                            ui.label(format!("{}:{}", hotkey.label.clone(), " ".repeat(10 - hotkey.label.len())));

                            
                            egui::ComboBox::from_id_source(i+2)
                            .width(80.)
                            .selected_text(format!("{:}", hotkey.tmp_modifier))
                            .show_ui(ui,|ui|  {                                   
                                ui.selectable_value(&mut hotkey.tmp_modifier, "CTRL".to_string(),  "CTRL");
                                ui.selectable_value(&mut hotkey.tmp_modifier, "SHIFT".to_string(),  "SHIFT");
                                ui.selectable_value(&mut hotkey.tmp_modifier, "ALT".to_string(),  "ALT");
                            });

                            ui.label(" + ");

                            ui.text_edit_singleline(&mut hotkey.tmp_code);


                            if hotkey.tmp_code.len() > 1{
                                let mut char_iterator = hotkey.tmp_code.chars();
                                let first_char = char_iterator.next().unwrap().to_string();
                                let char = char_iterator.next().unwrap().to_string();
                                let key = string_to_key(char.as_str());
                                if key.is_some(){
                                    hotkey.tmp_code = char;
                                }
                                else {
                                    hotkey.tmp_code = first_char;
                                }
                            }

                            hotkey.tmp_code = hotkey.tmp_code.to_uppercase();
                        });
                        ui.add_space(10.);
                    }
                });

                shortcut_ui.with_layout(Layout::right_to_left(Align::Max), |ui| {
                    
                    let apply_button = ui.add(Button::new("Apply"));
                    if apply_button.clicked() {
                        let mut valid = true;
                        let mut encountered_hotkeys = HashSet::new();

                        for hotkey in self.hotkeys.iter_mut() {
                            let key = (hotkey.tmp_modifier.clone(), hotkey.tmp_code.clone());
                            if !encountered_hotkeys.insert(key) {
                                valid = false;
                                break;
                            }
                        }

                        if valid {
                            for hotkey in self.hotkeys.iter_mut() {
                                hotkey.modifier = hotkey.tmp_modifier.clone();                            
                                hotkey.code = hotkey.tmp_code.clone();
                                
                            }
    
                            self.unregister_hotkeys();
                            self.register_hotkey();
    
                            self.show_settings = false;
                        }
                    }


                    let cancel_button = ui.add(Button::new("Cancel"));
                    if cancel_button.clicked() {
                        self.show_settings = false;
                    }

                });

            }
            else if self.show_credits{
                let mut credits_rect = ui.max_rect();
                let center = credits_rect.center();
                credits_rect.min.x = center[0] - 200.;
                credits_rect.max.x = center[0] + 200.;

                credits_rect.min.y = center[1] - 120.;
                credits_rect.max.y = center[1] + 120.;

                let credits_frame_rect = credits_rect.shrink(-40.);
                ui.painter().add(Shape::rect_filled(credits_frame_rect, 5., Color32::from_rgba_premultiplied(20, 20, 20, 100)));
                
                let mut credits_ui = ui.child_ui(credits_rect, *ui.layout());

                credits_ui.vertical_centered(|ui|{
                    ui.add_space(20.);
                    ui.heading("Credits");
                    ui.add_space(20.);
                    ui.monospace("SnapRust");
                    ui.monospace("Version 1.0");
                    ui.monospace("Developed by:");
                    ui.add_space(10.);
                    ui.monospace("Borella Simone");
                    ui.monospace("Buccellato Federico");
                    ui.monospace("Caretto Michelangelo");
                });
                
                credits_ui.with_layout(Layout::right_to_left(Align::Max), |ui| {
                    let back_button = ui.add(Button::new("Back"));
                    if back_button.clicked() {
                        self.show_credits = false;
                    }
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
