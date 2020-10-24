pub mod bus;
pub mod cartridge;
pub mod cpu;
pub mod joypad;
pub mod opcodes;
pub mod ppu;
pub mod render;

use bus::Bus;
use cartridge::Rom;
use cpu::CPU;
use ppu::NesPPU;
use render::frame::Frame;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use std::collections::HashMap;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate bitflags;

fn main() {
    // Initialize sdl2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Dave's Nintendo Emulator", (256.0 * 3.0) as u32, (240.0 * 3.0) as u32)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    canvas.set_scale(3.0, 3.0).unwrap();
    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGB24, 256, 240)
        .unwrap();

    // Load Game
    let bytes: Vec<u8> = std::fs::read("galaga.nes").unwrap();
    let rom = Rom::new(&bytes).unwrap();
    let mut frame = Frame::new();
    let mut key_map = HashMap::new();

    key_map.insert(Keycode::Down, joypad::JoypadButton::DOWN);
    key_map.insert(Keycode::Up, joypad::JoypadButton::UP);
    key_map.insert(Keycode::Right, joypad::JoypadButton::RIGHT);
    key_map.insert(Keycode::Left, joypad::JoypadButton::LEFT);
    key_map.insert(Keycode::Space, joypad::JoypadButton::SELECT);
    key_map.insert(Keycode::Return, joypad::JoypadButton::START);
    key_map.insert(Keycode::A, joypad::JoypadButton::BUTTON_A);
    key_map.insert(Keycode::S, joypad::JoypadButton::BUTTON_B);


    // Run Game Cycle
    let bus = Bus::new(rom, move |ppu: &NesPPU, joypad: &mut joypad::Joypad| {
        render::render(ppu, &mut frame);
        texture.update(None, &frame.data, 256 * 2 * 3).unwrap();
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => std::process::exit(0),
                Event::KeyDown { keycode, .. } => {
                    if let Some(key) = key_map.get(&keycode.unwrap_or(Keycode::Ampersand)) {
                        joypad.set_button_pressed_status(*key, true);
                    }
                },
                Event::KeyUp { keycode, .. } => {
                    if let Some(key) = key_map.get(&keycode.unwrap_or(Keycode::Ampersand)) {
                        joypad.set_button_pressed_status(*key, false);
                    }
                },
                _ => { /* Do Nothing */ }
            }
        }
    });
    let mut cpu = CPU::new(bus);
    cpu.reset();
    cpu.run_with_callback(|_cpu| {
        // println!("Trace: {}", trace(cpu));
    });
}
