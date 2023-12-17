extern crate sfml;
use sfml::window::*;
use sfml::window::mouse::*;
use sfml::window::Event::*;
use super::debugger::Debugger;

pub fn poll_keyboard(debugger: &mut Debugger, event: Event) -> u8 {
    match event {
        KeyPressed { code, .. } => match code {
            Key::Escape => { 
                debugger.dbg.close(); 
                return debugger.events.EXIT; },
            Key::Space => { 
                debugger.is_paused = !debugger.is_paused; 
                return debugger.events.PAUSE; },
            Key::R => { 
                if (!debugger.is_paused) { debugger.clear_snapshots(); }
                return debugger.events.RESET; },
            Key::Down | Key::J => { 
                if (debugger.is_paused) {
                    if (debugger.line_count - 1 > debugger.active_state) {
                        debugger.active_state += 1;
                        if (debugger.active_state > 48) {
                            debugger.selected_state = 48;
                        } else {
                            debugger.selected_state = debugger.active_state;
                        }
                    }
                } 
                return debugger.events.OK;
            },
            Key::Up | Key::K => {
                if (debugger.is_paused) {
                    if (debugger.active_state > 0) {
                        debugger.active_state -= 1;
                        if (debugger.active_state > 48) {
                            debugger.selected_state = 48;
                        } else {
                            debugger.selected_state = debugger.active_state;
                        }
                    }
                }
                debugger.events.OK
            },
            _ => debugger.events.OK
        },
        _ => debugger.events.OK
    }
}

pub fn poll_mouse(debugger: &mut Debugger, event: Event) -> u8 {
    match event {
        MouseWheelScrolled { wheel, delta, .. } => match wheel {
            Wheel::VerticalWheel => {
                if (delta > 0.0 && debugger.is_paused) {
                    if (debugger.line_count - 1 > debugger.active_state) {
                        debugger.active_state += 1;
                        if (debugger.active_state > 48) {
                            debugger.selected_state = 48;
                        } else {
                            debugger.selected_state = debugger.active_state;
                        }
                    }
                }
                if (delta < 0.0 && debugger.is_paused) {
                    if (debugger.active_state > 0) {
                        debugger.active_state -= 1;
                        if (debugger.active_state > 48) {
                            debugger.selected_state = 48;
                        } else {
                            debugger.selected_state = debugger.active_state;
                        }
                    }
                }
                debugger.events.OK
            },
            _ => debugger.events.OK
        }
        _ => debugger.events.OK
    }
}
