pub mod frame;
pub mod palette;

use crate::ppu::NesPPU;
use rand::{thread_rng, Rng};
use frame::Frame;

pub fn render(ppu: &NesPPU, frame: &mut Frame) {
	let bank = ppu.ctrl.bknd_pattern_addr();
	let mut rng = thread_rng();
    let palette_rng: Vec<usize> = (0..4 as usize).map(|i| {
        println!("{}", i);
        rng.gen_range(0, 63)
    }).collect();

	for i in 0..0x03c0 {
		let tile = ppu.vram[i] as u16;
		let tile_x = i % 32;
		let tile_y = i / 32;
		let tile = &ppu.chr_rom[(bank + tile * 16) as usize..=(bank + tile * 16 + 15) as usize];

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
				frame.set_pixel(tile_x * 8 + x, tile_y * 8 + y, rgb)
			}
		}
	}
}
