pub mod bus;
pub mod cartridge;
pub mod cpu;
pub mod opcodes;
pub mod trace;
pub mod ppu;
pub mod render;

// use std::{thread, time};
use rand::{thread_rng, Rng};
use bus::Bus;
use ppu::NesPPU;
use cpu::CPU;
use cartridge::Rom;
use render::frame::Frame;
use render::palette;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate bitflags;

fn show_tile_bank(chr_rom: &Vec<u8>, bank: usize) ->Frame {
    assert!(bank <= 1);

    let mut rng = thread_rng();
    let palette_rng: Vec<usize> = (0..4 as usize).map(|i| {
        println!("{}", i);
        rng.gen_range(0, 63)
    }).collect();
    eprintln!("PR: {:#?}", palette_rng);

    let mut frame = Frame::new();
    let mut tile_y = 0;
    let mut tile_x = 0;
    let bank = (bank * 0x1000) as usize;

    for tile_n in 0..255 {
        if tile_n != 0 && tile_n % 20 == 0 {
            tile_y += 10;
            tile_x = 0;
        }
        let tile = &chr_rom[(bank + tile_n * 16)..=(bank + tile_n * 16 + 15)];

        for y in 0..=7 {
            let mut upper = tile[y];
            let mut lower = tile[y + 8];

            for x in (0..=7).rev() {
                let value = (1 & upper) << 1 | (1 & lower);

                upper = upper >> 1;
                lower = lower >> 1;

                let rgb = match value {
                    0 => palette::SYSTEM_PALETTE[palette_rng[0]],
                    1 => palette::SYSTEM_PALETTE[palette_rng[1]],
                    2 => palette::SYSTEM_PALETTE[palette_rng[2]],
                    3 => palette::SYSTEM_PALETTE[palette_rng[3]],
                    _ => panic!("What in the wide wide world of sports is a' goin' on?!"),
                };
                frame.set_pixel(tile_x + x, tile_y + y, rgb)
            }
        }
        tile_x += 10;
    }
    frame
}

fn main() {
    // let wait_seconds = time::Duration::from_millis(15);

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

    //Load the Game
    let bytes: Vec<u8> = std::fs::read("pacman.nes").unwrap();
    let rom = Rom::new(&bytes).unwrap();
    let mut frame = Frame::new();

    // Run Game Cycle
    let bus = Bus::new(rom, move |ppu: &NesPPU| {
        render::render(ppu, &mut frame);
        texture.update(None, &frame.data, 256 * 3).unwrap();
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
              Event::Quit { .. }
              | Event::KeyDown {
                  keycode: Some(Keycode::Escape),
                  ..
              } => std::process::exit(0),
              _ => { /* Do Nothing */ }
            }
         }
    });
    let mut cpu = CPU::new(bus);

    cpu.reset();
    cpu.run();

    /*loop {
        let right_bank = show_tile_bank(&rom.chr_rom, 1);
        let left_bank = show_tile_bank(&rom.chr_rom, 0);

        texture.update(None, &right_bank.data, 256 * 3).unwrap();
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();
        thread::sleep(wait_seconds);
        texture.update(None, &left_bank.data, 256 * 3).unwrap();
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();
        thread::sleep(wait_seconds);

        for event in event_pump.poll_iter() {
            match event {
              Event::Quit { .. }
              | Event::KeyDown {
                  keycode: Some(Keycode::Escape),
                  ..
              } => std::process::exit(0),
              _ => { /* Do Nothing */ }
            }
         }
    }*/
}
