pub mod bus;
pub mod cartridge;
pub mod cpu;
pub mod opcodes;
pub mod trace;
pub mod ppu;

use bus::Bus;
use cartridge::Rom;
use cpu::CPU;
use trace::trace;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate bitflags;

fn main() {
    // init sdl2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Dave's Nintendo Emulator", (32.0 * 10.0) as u32, (32.0 * 10.0) as u32)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    canvas.set_scale(10.0, 10.0).unwrap();

    // Load Game
    // let bytes: Vec<u8> = std::fs::read("pacman.nes").unwrap();
    let bytes: Vec<u8> = std::fs::read("nestest.nes").unwrap();
    let rom = Rom::new(&bytes).unwrap();
    let bus = Bus::new(rom);
    let mut cpu = CPU::new(bus);
    cpu.reset();
    cpu.program_counter = 0xC000;

    // Run Game Cycle
    cpu.run_with_callback(move |cpu| {
        println!("{}", trace(cpu));
    });
}
