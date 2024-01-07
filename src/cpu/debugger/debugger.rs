#![allow(non_snake_case)]
#![allow(unused)]

extern crate sfml;
use sfml::graphics::*;
use sfml::graphics::Image;
use sfml::SfBox;
use sfml::window::*;
use crate::cpu::MOS6510;
use crate::cpu::MMU;
use std::collections::VecDeque;
use sfml::window::Event::*;
use super::input_handler::poll_keyboard;
use super::input_handler::poll_mouse;
use super::renderer::render_memory_banks;
use super::renderer::render_registers;
use super::renderer::render_instructions;
use super::renderer::memory_map;

pub struct EventCodes {
	pub OK		: u8,
	pub EXIT	: u8,
	pub PAUSE	: u8,
	pub RESET	: u8,
}

pub struct Snapshot {
	pub instruction : String,
	pub	AX			: String,
	pub SY			: String,
	pub PC			: String,
	pub Pb			: String,
	pub cycle		: String,
    pub memory_map  : String,
}

impl Snapshot {
	pub fn new() -> Snapshot {
		let empty = "".to_string();
		Snapshot {
			instruction : empty.clone(),
			AX			: empty.clone(),
			SY			: empty.clone(),
			PC			: empty.clone(),
			Pb			: empty.clone(),
			cycle		: empty.clone(),
            memory_map  : empty.clone(),
		}
	}
}

pub struct Debugger {
	pub dbg				: RenderWindow,
	text				: Vec<String>,
	pub font			: SfBox<Font>,
	pub active_state	: u8,
	pub selected_state	: u8,
	pub line_count		: u8,
	pub LIGHT_BLUE		: Color,
	pub DARK_BLUE		: Color,
	pub PURPLE			: Color,
	pub LIGHT_RED		: Color,
	BLACK				: Color,
	pub snapshots		: VecDeque<Snapshot>,
	pub events			: EventCodes,
	pub is_paused		: bool,
    pub scale           : f32,
}

impl Debugger {
	pub fn new(scale: f32) -> Debugger {
		Debugger {
			dbg				: RenderWindow::new (
							((1850.0 * scale) as u32, (1600.0 * scale) as u32),
							"C64 DBG",
							Style::TITLEBAR | Style::CLOSE,
							&Default::default(),),
			text			: Vec::new(),
			font			: Font::from_file("res/C64_pro.ttf").unwrap(),
			active_state	: 0,
			selected_state	: 0,
			line_count		: 0,
			LIGHT_BLUE		: Color::rgb(134, 122, 221),
			DARK_BLUE		: Color::rgb(72, 59, 170),
			PURPLE			: Color::rgb(221, 136, 85),
			LIGHT_RED		: Color::rgb(255, 119, 119),
			BLACK			: Color::rgb(51, 51, 51),
			snapshots		: VecDeque::new(),
			events			: EventCodes { OK: 0, EXIT: 1, PAUSE: 2, RESET: 3 },
			is_paused		: false,
            scale,
		}
	}

	pub fn clear(&mut self) {
		self.dbg.clear(self.DARK_BLUE);
	}

    pub fn init(&mut self) {
        let image = Image::from_file("res/icon.png").unwrap();
        unsafe {
            self.dbg.set_icon(512, 512, image.pixel_data());
        }
    }

	pub fn render(&mut self, mmu: &mut MMU) {
		render_instructions(self);
		render_registers(self);
        render_memory_banks(self, mmu);
		memory_map(self, mmu);
		self.dbg.display();
	}

	pub fn poll(&mut self) -> u8 {
		while let Some(event) = self.dbg.poll_event() {
			let keyboard_event = poll_keyboard(self, event);
			if keyboard_event != 0 { return keyboard_event; }

			let mouse_event = poll_mouse(self, event);
			if mouse_event != 0 { return mouse_event; }

			match event {
				Closed => { self.dbg.close(); return self.events.EXIT; }
				_ => {}
			}
		}
		self.events.OK
	}

	pub fn create_snapshot(&mut self, text: String, cpu: &mut MOS6510) {
		if self.snapshots.len() == 255 {
			self.snapshots.pop_back();
		}

		let mut snapshot: Snapshot = Snapshot::new();
		snapshot.instruction = text;
		snapshot.AX = format!("A:  {}		X:	{}", format!("0x{:02X}", cpu.A), format!("0x{:02X}", cpu.X));
		snapshot.SY = format!("S:  {}		Y:	{}", format!("0x{:02X}", cpu.S), format!("0x{:02X}", cpu.Y));
		snapshot.PC = format!("PC: {}",format!("0x{:04X}", cpu.PC));
		snapshot.Pb = format!("{:08b}", cpu.P);
		snapshot.cycle = format!("CYCLE:   {}", cpu.cycle);
        snapshot.memory_map = format!("Memory map config: {:08b}", cpu.mmu.read(0x0001 as u16));
		self.snapshots.push_front(snapshot);
	}

	pub fn clear_snapshots(&mut self) {
		self.active_state = 0;
		self.selected_state = 0;
		self.snapshots.clear();
	}
}
