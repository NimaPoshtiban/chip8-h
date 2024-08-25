#![allow(arithmetic_overflow)]
use std::ffi::c_void;

use chip8::{VIDEO_HEIGHT, VIDEO_WIDTH};

#[allow(non_snake_case)]
#[allow(dead_code)]
mod chip8;
mod platform;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {:?}  <Scale> <Delay> <ROM>", args[0]);
    }
    let videoScale = args[1].parse::<i32>().unwrap();
    let cycle_delay = args[2].parse::<i32>().unwrap();
    let romFilename = &args[3];
    let mut platform = platform::Platform::new(
        "CHIP-8 Emulator".to_owned(),
        (VIDEO_WIDTH as i32) * videoScale,
        (VIDEO_HEIGHT as i32) * videoScale,
        VIDEO_WIDTH as i32,
        VIDEO_HEIGHT as i32,
    );

    let mut chip8 = chip8::Chip8::new();
    chip8.load_ROM(romFilename.to_owned()).unwrap();

    let video_pitch = std::mem::size_of::<u32>() * (VIDEO_WIDTH as usize) ;

    let mut last_cycle_time = std::time::Instant::now();
    let mut quit = false;
    while !quit {
        quit = unsafe {
            platform.process(chip8.keypad.as_mut_ptr() as *mut i8)
        };
        let current_time = std::time::Instant::now();
        let duration = current_time.duration_since(last_cycle_time);
        let dt: f32 = duration.as_secs_f32() * 1000.0;
        if dt as i32 > cycle_delay{
			last_cycle_time = current_time;
			chip8.cycle();
            unsafe {
			    // platform.update(chip8.video.as_ptr() as *const c_void, video_pitch as i32);
            }
		}
    }
    unsafe {platform.destroy()};
}
