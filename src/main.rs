extern crate minifb;
extern crate rand;
use minifb::{Key, WindowOptions, Window};
mod cpu;

const WIDTH: usize = 640;
const HEIGHT: usize = 320;

fn main() {
	// Initialization
	let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new("Chip8 Emulator - ESC to exit",
                                 WIDTH,
                                 HEIGHT,
                                 WindowOptions::default()).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut cpu = cpu::Cpu {
    	pc: 0x200,
    	i: 0,
    	sp: 0,
    	stack: [0; 16],
    	v: [0; 16],
    	memory: [0; 4096],
    	dt: 60,
    	st: 0,
    	display: [[0; 64]; 32],
    	key: [0; 16]
    };

    // Main loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
    	// Run the CPU
    	cpu.emulate_cycle();

    	// Write to screen
        for i in buffer.iter_mut() {
            *i = 0; // write something more funny here!
        }
        println!("{}", cpu.v[0]);
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer).unwrap();
    }
}
