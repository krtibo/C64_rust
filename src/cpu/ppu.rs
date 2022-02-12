#![allow(non_snake_case)]
#![allow(unused)]

extern crate sfml;
use sfml::graphics::*;
use sfml::window::*;
use crate::Event::*;
use super::cpu::MOS6510;

pub struct PPU {
    scr             : RenderWindow, // 320×200
    LIGHT_BLUE      : Color,
    DARK_BLUE       : Color,
    PURPLE          : Color,
    LIGHT_RED       : Color,
    BLACK           : Color,
}

impl PPU {

    pub fn new() -> PPU {
        PPU {
            scr             : RenderWindow::new(
                            (320*4, 200*4),
                            "C64 Screen",
                            Style::TITLEBAR | Style::CLOSE,
                            &Default::default(),),
            LIGHT_BLUE      : Color::rgb(134, 122, 221),
            DARK_BLUE       : Color::rgb(72, 59, 170),
            PURPLE          : Color::rgb(147, 81, 182),
            LIGHT_RED       : Color::rgb(255, 119, 119),
            BLACK           : Color::rgb(51, 51, 51),
        }
    }

    pub fn clear(&mut self) {
        self.scr.clear(&self.DARK_BLUE);
    }

    pub fn render(&mut self, cpu: &mut MOS6510) {
        self.scr.display();
    } 

    pub fn poll(&mut self) -> bool {
        while let Some(event) = self.scr.poll_event() {
            match event {
                Closed => { self.scr.close(); return false; }
                KeyPressed { code, .. } => match code {
                    Key::Escape => { self.scr.close(); return false;},
                    _ => {}
                },
                _ => {}
            }
        }
        true
    }
}
