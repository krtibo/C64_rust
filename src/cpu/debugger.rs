#![allow(non_snake_case)]
#![allow(unused)]

extern crate sfml;
use sfml::graphics::*;
use sfml::system::Vector2f;
use std::borrow::Borrow;
use sfml::SfBox;
use sfml::window::*;
use sfml::window::mouse::*;
use super::cpu::MOS6510;
use std::collections::VecDeque;
use sfml::system::Vector2i;
use sfml::window::Event::*;

pub struct EventCodes {
    pub OK     : u8,
    pub EXIT   : u8,
    pub PAUSE  : u8,
    pub RESET  : u8,
}

pub struct Snapshot {
    instruction : String,
    AX          : String,
    SY          : String,
    PC          : String,
    Pb          : String,
    cycle       : String,
}

impl Snapshot {
    pub fn new() -> Snapshot {
        let empty = "".to_string();
        Snapshot {
            instruction : empty.clone(),
            AX          : empty.clone(),
            SY          : empty.clone(),
            PC          : empty.clone(),
            Pb          : empty.clone(),
            cycle       : empty.clone(),
        }
    }
}

pub struct Debugger {
    dbg              : RenderWindow,
    text             : Vec<String>,
    font             : SfBox<Font>,
    active_state     : u8,
    selected_state   : u8,
    line_count       : u8,
    LIGHT_BLUE       : Color,
    DARK_BLUE        : Color,
    PURPLE           : Color,
    LIGHT_RED        : Color,
    BLACK            : Color,
    snapshots        : VecDeque<Snapshot>,
    pub events       : EventCodes,
    is_paused        : bool
}

impl Debugger {
    pub fn new() -> Debugger {
        Debugger {
            dbg             : RenderWindow::new (
                            (1850, 1600),
                            "C64 DBG",
                            Style::TITLEBAR | Style::CLOSE,
                            &Default::default(),),
            text            : Vec::new(),
            font            : Font::from_file("res/C64_pro.ttf").unwrap(),
            active_state    : 0,
            selected_state  : 0,
            line_count      : 0,
            LIGHT_BLUE      : Color::rgb(134, 122, 221),
            DARK_BLUE       : Color::rgb(72, 59, 170),
            PURPLE          : Color::rgb(221, 136, 85),
            LIGHT_RED       : Color::rgb(255, 119, 119),
            BLACK           : Color::rgb(51, 51, 51),
            snapshots       : VecDeque::new(),
            events          : EventCodes { OK: 0, EXIT: 1, PAUSE: 2, RESET: 3 },
            is_paused       : false,
        }
    }

    pub fn clear(&mut self) {
        self.dbg.clear(self.DARK_BLUE);
    }

    pub fn render(&mut self, ram: &[u8]) {
        self.render_instructions();
        self.render_registers();
        self.memory_map(ram);
        self.dbg.display();
    }

    fn poll_keyboard(&mut self, event: Event) -> u8 {
        match event {
            KeyPressed { code, .. } => match code {
                Key::Escape => { 
                    self.dbg.close(); 
                    return self.events.EXIT; },
                Key::Space => { 
                    self.is_paused = !self.is_paused; 
                    return self.events.PAUSE; },
                Key::R => { 
                    if (!self.is_paused) { self.clear_snapshots(); }
                    return self.events.RESET; },
                Key::Down => { 
                    if (self.is_paused) {
                        if (self.line_count - 1 > self.active_state) {
                            self.active_state += 1;
                            if (self.active_state > 48) {
                                self.selected_state = 48;
                            } else {
                                self.selected_state = self.active_state;
                            }
                        }
                    } 
                    return self.events.OK;
                },
                Key::Up => {
                    if (self.is_paused) {
                        if (self.active_state > 0) {
                            self.active_state -= 1;
                            if (self.active_state > 48) {
                                self.selected_state = 48;
                            } else {
                                self.selected_state = self.active_state;
                            }
                        }
                    }
                    self.events.OK
                },
                _ => self.events.OK
            },
            _ => self.events.OK
        }
    }

    fn poll_mouse(&mut self, event: Event) -> u8 {
        match event {
            MouseWheelScrolled { wheel, delta, .. } => match wheel {
                Wheel::VerticalWheel => {
                    if (delta > 0.0 && self.is_paused) {
                        if (self.line_count - 1 > self.active_state) {
                            self.active_state += 1;
                            if (self.active_state > 48) {
                                self.selected_state = 48;
                            } else {
                                self.selected_state = self.active_state;
                            }
                        }
                    }
                    if (delta < 0.0 && self.is_paused) {
                        if (self.active_state > 0) {
                            self.active_state -= 1;
                            if (self.active_state > 48) {
                                self.selected_state = 48;
                            } else {
                                self.selected_state = self.active_state;
                            }
                        }
                    }
                    self.events.OK
                },
                _ => self.events.OK
            }
            _ => self.events.OK
        }
    }

    pub fn poll(&mut self) -> u8 {
        while let Some(event) = self.dbg.poll_event() {
            let keyboard_event = self.poll_keyboard(event);
            if keyboard_event != 0 { return keyboard_event; }

            let mouse_event = self.poll_mouse(event);
            if mouse_event != 0 { return mouse_event; }

            match event {
                Closed => { self.dbg.close(); return self.events.EXIT; }
                _ => {}
            }
        }
        self.events.OK
    }

    pub fn create_snapshot(&mut self, text: String, cpu: &MOS6510) {
        if self.snapshots.len() == 255 {
            self.snapshots.pop_back();
        }

        let mut snapshot: Snapshot = Snapshot::new();
        snapshot.instruction = text;
        snapshot.AX = format!("A:  {}       X:  {}", format!("0x{:02X}", cpu.A), format!("0x{:02X}", cpu.X));
        snapshot.SY = format!("S:  {}       Y:  {}", format!("0x{:02X}", cpu.S), format!("0x{:02X}", cpu.Y));
        snapshot.PC = format!("PC: {}",format!("0x{:04X}", cpu.PC));
        snapshot.Pb = format!("{:08b}", cpu.P);
        snapshot.cycle = format!("CYCLE:   {}", cpu.cycle);
        self.snapshots.push_front(snapshot);
    }

    pub fn clear_snapshots(&mut self) {
        self.active_state = 0;
        self.selected_state = 0;
        self.snapshots.clear();
    }

    pub fn render_instructions(&mut self) {
        // takes the instructions from the snapshots and renders them
        self.line_count = self.snapshots.len() as u8;
        const BG_WIDTH  : u32 = 1015;
        const BG_HEIGHT : u32 = 30;

        // main texture pixel array and object
        let mut texture_pixels: [u8; 4] = [134, 122, 221, 255];
        let mut texture = Texture::new().unwrap();
        Texture::create(&mut texture, 1, 1);
        unsafe {
            texture.update_from_pixels(&texture_pixels, 1, 1, 0, 0);
        }

        // header background sprite based on the generic texture with scaling and rendering
        self.render_sprite(&texture, Vector2f::new(0.0, 0.0), Vector2f::new(1850.0, BG_HEIGHT as f32));

        // header text initialization and rendering
        self.render_text("    PC    |           CPU INSTRUCTION           |      REGISTERS & FLAGS".to_string(), 15.0, 3.0, self.DARK_BLUE);

        let mut start_index = 0;
        let mut end_index = 0;
        if (self.active_state <= 48) { 
            start_index = 0; 
            if (self.snapshots.len() > 48) {
                end_index = 49;
            } else {
                end_index = self.snapshots.len();
            }
        } else {
            start_index = self.active_state - 48;
            end_index = self.active_state as usize + 1;
        }

        let mut line_number = 0;
        for i in start_index..end_index as u8 {
            if (line_number == self.selected_state && self.is_paused) {
                let mut bg_sprite = self.selected_state as usize * 32 + 34;
                if (bg_sprite > 1570) {
                    bg_sprite = 1570;
                }
                // text background sprite based on the generic texture with scaling
                self.render_sprite(&texture, Vector2f::new(0.0, bg_sprite as f32), Vector2f::new(BG_WIDTH as f32, BG_HEIGHT as f32));
                self.render_text(self.snapshots[i as usize].instruction.clone(), 15.0, line_number as f32 * 32.0 + 37.0, self.DARK_BLUE);
            } else {
                self.render_text(self.snapshots[i as usize].instruction.clone(), 15.0, line_number as f32 * 32.0 + 37.0, self.LIGHT_BLUE);
            }
            if (self.snapshots[i as usize].instruction.contains("PPU RENDER")) {
                self.render_text(self.snapshots[i as usize].instruction.clone(), 15.0, line_number as f32 * 32.0 + 37.0, self.PURPLE);
            }
            line_number += 1;
        }
        // separator sprite based on the generic texture with scaling
        self.render_sprite(&texture, Vector2f::new(1011.0, 0.0), Vector2f::new(4.0, 1600.0));
    }

    fn render_text(&mut self, value: String, horizontal_position: f32, vertical_position: f32, color: Color) {
        let font_size = 22;
        let mut render_register = Text::new(&value, &self.font, font_size);
        render_register.set_position((horizontal_position, vertical_position));
        render_register.set_fill_color(color);
        self.dbg.draw(&render_register);
    }
    
    fn render_sprite(&mut self, texture: &Texture, position: Vector2f, scale: Vector2f) {
        let mut sprite = Sprite::with_texture(texture);
        sprite.set_position(position);
        sprite.set_scale(scale);
        self.dbg.draw(&sprite);
    }

    pub fn render_registers(&mut self) {
        // takes the register states from the snapshots and renders them
        self.render_text(self.snapshots[self.active_state as usize].AX.clone(), 1032.0, 38.0, self.LIGHT_RED);
        self.render_text(self.snapshots[self.active_state as usize].SY.clone(), 1032.0, 70.0, self.LIGHT_RED);
        self.render_text(self.snapshots[self.active_state as usize].PC.clone(), 1032.0, 102.0, self.LIGHT_RED);
        self.render_text("FLAGS:".to_string(), 1032.0, 166.0, self.LIGHT_RED);

        // flags text and coloring based on turned on or off
        let flags_label: String = "NV1BDIZC".to_string();
        for i in 0..self.snapshots[self.active_state as usize].Pb.len() {
            if self.snapshots[self.active_state as usize].Pb.chars().nth(i).unwrap() == '1' {
                self.render_text(flags_label[i..i+1].to_string().clone(), 1200.0 + i as f32 * 40.0, 166.0, self.LIGHT_RED);
            } else {
                self.render_text(flags_label[i..i+1].to_string().clone(), 1200.0 + i as f32 * 40.0, 166.0, self.LIGHT_BLUE);
            }
        }

        self.render_text(self.snapshots[self.active_state as usize].cycle.clone(), 1032.0, 198.0, self.LIGHT_RED);
    }

    pub fn memory_map(&mut self, ram: &[u8]) {
        let mut pixels: [u8; 256 * 256 * 4] = [255; 256 * 256 * 4];
        let current_pc_str = &self.snapshots[self.active_state as usize].PC[6..];
        let current_pc: usize = usize::from_str_radix(current_pc_str, 16).unwrap();

        for i in 0..ram.len() {
            if (ram[i] != 0) {
                pixels[i*4] = 134;
                pixels[i*4+1] = 122;
                pixels[i*4+2] = 221;
                pixels[i*4+3] = 255;
            } else {
                pixels[i*4] = 62;
                pixels[i*4+1] = 49;
                pixels[i*4+2] = 160;
                pixels[i*4+3] = 255;
            }
            if (i == current_pc) {
                pixels[i*4] = 255;
                pixels[i*4+1] = 255;
                pixels[i*4+2] = 255;
                pixels[i*4+3] = 255;
            }
        }

        let mut texture = Texture::new().unwrap();
        Texture::create(&mut texture, 256, 256);
        unsafe {
            texture.update_from_pixels(&pixels, 256, 256, 0, 0);
        }
        let mut sprite = Sprite::with_texture(&texture);
        sprite.set_position((1050.0, 800.0));
        sprite.set_scale((3.0, 3.0));
        self.dbg.draw(&sprite);
    }
}
