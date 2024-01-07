#![allow(non_snake_case)]
use super::debugger::Debugger;
extern crate sfml;
use sfml::graphics::*;
use sfml::system::Vector2f;
use crate::cpu::MMU;

fn render_text(debugger: &mut Debugger, value: String, horizontal_position: f32, vertical_position: f32, color: Color) {
    let font_size = 22;
    let mut render_register = Text::new(&value, &debugger.font, font_size);
    render_register.set_position(Vector2f::new(horizontal_position * debugger.scale, vertical_position * debugger.scale));
    render_register.set_scale(Vector2f::new(debugger.scale, debugger.scale));
    render_register.set_fill_color(color);
    debugger.dbg.draw(&render_register);
}

fn render_sprite(debugger: &mut Debugger, texture: &Texture, position: Vector2f, scale: Vector2f) {
    let mut sprite = Sprite::with_texture(texture);
    sprite.set_position(Vector2f::new(position.x * debugger.scale, position.y * debugger.scale));
    sprite.set_scale(Vector2f::new(scale.x * debugger.scale, scale.y * debugger.scale));
    debugger.dbg.draw(&sprite);
}

pub fn render_registers(debugger: &mut Debugger) {
    // takes the register states from the snapshots and renders them
    render_text(debugger, debugger.snapshots[debugger.active_state as usize].AX.clone(), 1032.0, 38.0, debugger.LIGHT_RED);
    render_text(debugger, debugger.snapshots[debugger.active_state as usize].SY.clone(), 1032.0, 70.0, debugger.LIGHT_RED);
    render_text(debugger, debugger.snapshots[debugger.active_state as usize].PC.clone(), 1032.0, 102.0, debugger.LIGHT_RED);
    render_text(debugger, "FLAGS:".to_string(), 1032.0, 166.0, debugger.LIGHT_RED);

    // flags text and coloring based on turned on or off
    let flags_label: String = "NV1BDIZC".to_string();
    for i in 0..debugger.snapshots[debugger.active_state as usize].Pb.len() {
        if debugger.snapshots[debugger.active_state as usize].Pb.chars().nth(i).unwrap() == '1' {
            render_text(debugger, flags_label[i..i+1].to_string().clone(), 1200.0 + i as f32 * 40.0, 166.0, debugger.LIGHT_RED);
        } else {
            render_text(debugger, flags_label[i..i+1].to_string().clone(), 1200.0 + i as f32 * 40.0, 166.0, debugger.LIGHT_BLUE);
        }
    }

    render_text(debugger, debugger.snapshots[debugger.active_state as usize].cycle.clone(), 1032.0, 198.0, debugger.LIGHT_RED);
    render_text(debugger, debugger.snapshots[debugger.active_state as usize].memory_map.clone(), 1032.0, 230.0, debugger.LIGHT_RED);
}

pub fn render_instructions(debugger: &mut Debugger) {
    // takes the instructions from the snapshots and renders them
    debugger.line_count = debugger.snapshots.len() as u8;
    const BG_WIDTH	: u32 = 1015;
    const BG_HEIGHT : u32 = 30;

    // main texture pixel array and object
    let mut texture_pixels: [u8; 4] = [134, 122, 221, 255];
    let mut texture = Texture::new().unwrap();
    Texture::create(&mut texture, 1, 1);
    unsafe {
        texture.update_from_pixels(&texture_pixels, 1, 1, 0, 0);
    }

    // header background sprite based on the generic texture with scaling and rendering
    render_sprite(debugger, &texture, Vector2f::new(0.0, 0.0), Vector2f::new(1850.0, BG_HEIGHT as f32));

    // header text initialization and rendering
    render_text(debugger, "	  PC	|			CPU INSTRUCTION			  |		 REGISTERS & FLAGS".to_string(), 15.0, 3.0, debugger.DARK_BLUE);

    let mut start_index = 0;
    let mut end_index = 0;
    if (debugger.active_state <= 48) { 
        start_index = 0; 
        if (debugger.snapshots.len() > 48) {
            end_index = 49;
        } else {
            end_index = debugger.snapshots.len();
        }
    } else {
        start_index = debugger.active_state - 48;
        end_index = debugger.active_state as usize + 1;
    }

    let mut line_number = 0;
    for i in start_index..end_index as u8 {
        if (line_number == debugger.selected_state && debugger.is_paused) {
            let mut bg_sprite = debugger.selected_state as usize * 32 + 34;
            if (bg_sprite > 1570) {
                bg_sprite = 1570;
            }
            // text background sprite based on the generic texture with scaling
            render_sprite(debugger, &texture, Vector2f::new(0.0, bg_sprite as f32), Vector2f::new(BG_WIDTH as f32, BG_HEIGHT as f32));
            render_text(debugger, debugger.snapshots[i as usize].instruction.clone(), 15.0, line_number as f32 * 32.0 + 37.0, debugger.DARK_BLUE);
        } else {
            render_text(debugger, debugger.snapshots[i as usize].instruction.clone(), 15.0, line_number as f32 * 32.0 + 37.0, debugger.LIGHT_BLUE);
        }
        if (debugger.snapshots[i as usize].instruction.contains("PPU RENDER")) {
            render_text(debugger, debugger.snapshots[i as usize].instruction.clone(), 15.0, line_number as f32 * 32.0 + 37.0, debugger.PURPLE);
        }
        line_number += 1;
    }
    // separator sprite based on the generic texture with scaling
    render_sprite(debugger, &texture, Vector2f::new(1011.0, 0.0), Vector2f::new(4.0, 1600.0));
}

pub fn render_memory_banks(debugger: &mut Debugger, mmu: &mut MMU) {
    let mut basic_color: Color = if mmu.loram() && mmu.hiram() { debugger.LIGHT_RED } else { debugger.LIGHT_BLUE };
    let mut kernal_color: Color = if mmu.hiram() { debugger.LIGHT_RED } else { debugger.LIGHT_BLUE };
    let mut io_color: Color = debugger.LIGHT_BLUE;
    let mut char_color: Color = debugger.LIGHT_BLUE;
    if mmu.loram() || mmu.hiram() {
        if mmu.charen() {
            io_color = debugger.LIGHT_RED;
        } else {
            char_color = debugger.LIGHT_RED;
        }
    }
    render_text(debugger, "BASIC".to_string(), 1032.0, 262.0, basic_color);
    render_text(debugger, "I/O".to_string(), 1160.0, 262.0, io_color);
    render_text(debugger, "CHAR".to_string(), 1250.0, 262.0, char_color);
    render_text(debugger, "KERNAL".to_string(), 1360.0, 262.0, kernal_color);
}

pub fn memory_map(debugger: &mut Debugger, mmu: &mut MMU) {
    let mut pixels: [u8; 256 * 256 * 4] = [255; 256 * 256 * 4];
    let current_pc_str = &debugger.snapshots[debugger.active_state as usize].PC[6..];
    let current_pc: usize = usize::from_str_radix(current_pc_str, 16).unwrap();

    for i in 0..mmu.RAM.len() {
        if (mmu.read(i as u16) != 0) {
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
    render_sprite(debugger, &texture, Vector2f::new(1050.0, 800.0), Vector2f::new(3.0, 3.0));
}
