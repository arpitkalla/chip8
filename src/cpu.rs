extern crate rand;
use rand::Rng;

pub struct Cpu {
	pub pc: u16,
	pub i: u16,
	pub memory: [u8; 4096],
	pub v: [u8; 16],
	pub stack: [u16; 16],
	pub sp: u8,
	pub dt: u8,
	pub st: u8,
	pub display: [[u8; 64]; 32],
	pub key: [u8; 16]
}

impl Cpu {


	pub fn reset(&mut self) {
		// Initialize pc, index, stack pointer
		self.pc = 0x200;
		self.i = 0;
		self.sp = 0;

		// Clear stuff
		self.stack = [0; 16];

		// Clear registers
		self.v = [0; 16];  

		// Load fontset
		for (mem, font) in self.memory.iter_mut().zip(FONT_SET.iter()) {
    		*mem = *font
		} 
		// Initialize timers
		self.dt = 60;
		self.st = 0;

		self.display = [[0; 64]; 32];
		self.key = [0; 16];
	}

	pub fn load_game(&mut self, name: &str) {

	}

	pub fn emulate_cycle(&mut self) {
		// Fetch opcode
		// let _opcode: u16 = (self.memory[self.pc as usize] as u16) << 8 | (self.memory[(self.pc + 1) as usize] as u16);
		let _opcode: u16 = 0xC0FF;
		
		// Decode opcode
		CHIP8_TABLE[((_opcode & 0xF000) >> 12) as usize](self ,_opcode);

		// Update timers
		if self.dt > 0 {
			self.dt -= 1;
		}

		if self.st > 0 {
			if self.st == 1 {
				println!("BEEP!");
			}
			self.st -= 1;
		}
	}

}

fn empty(cpu: &mut Cpu, opcode: u16) {
	match opcode & 0x00FF {
		0xE0 => cpu.display = [[0; 64]; 32],
		0xEE => {
			cpu.pc = cpu.sp as u16;
			cpu.sp -= 1;
		},
		_ => ()
	}
} 

fn arithmetic(cpu: &mut Cpu, opcode: u16) {
	let x: usize = ((opcode & 0x0F00) >> 8) as usize;
	let y: usize = ((opcode & 0x00F0) >> 4) as usize;

	match opcode & 0x000F {
		0 => cpu.v[x] = cpu.v[y],
		1 => cpu.v[x] = cpu.v[x] | cpu.v[y],
		2 => cpu.v[x] = cpu.v[x] & cpu.v[y],
		3 => cpu.v[x] = cpu.v[x] ^ cpu.v[y],
		4 => {
			cpu.v[15] = if cpu.v[y] > !cpu.v[x] { 1 } else { 0 };  
			cpu.v[x] = cpu.v[x] + cpu.v[y];
		},
		5 => {
			cpu.v[15] = if cpu.v[x] > cpu.v[y] { 1 } else { 0 };  
			cpu.v[x] = cpu.v[x] - cpu.v[y];
		},
		6 => {
			cpu.v[15] = cpu.v[y] % 2;  
			cpu.v[x] = cpu.v[y] >> 1;
		},
		7 => {
			cpu.v[15] = if cpu.v[y] > cpu.v[x] { 1 } else { 0 };  
			cpu.v[x] = cpu.v[y] - cpu.v[x];
		},
		14 => {
			cpu.v[15] = cpu.v[y] >> 7;  
			cpu.v[x] = cpu.v[y] << 1;
		},
		_ => ()
	}
}

fn flow(cpu: &mut Cpu, opcode: u16) {
	let nnn: u16 = (opcode & 0x0FFF) as u16;
	let x: usize = ((opcode & 0x0F00) >> 8) as usize;
	let y: usize = ((opcode & 0x00F0) >> 4) as usize;
	let k: u8 = (opcode & 0x00FF) as u8;
	let mut rng = rand::thread_rng();

	match (opcode & 0xF000) >> 12 {
		
		1 => cpu.pc = nnn,
		2 => {
			cpu.i += 1;
			cpu.pc = nnn;
		},
		3 => cpu.pc = if cpu.v[x] == k {cpu.pc + 2} else {cpu.pc},
		4 => cpu.pc = if cpu.v[x] != k {cpu.pc + 2} else {cpu.pc},
		5 => cpu.pc = if cpu.v[x] == cpu.v[y] {cpu.pc + 2} else {cpu.pc},
		6 => cpu.v[x] = k,
		7 => cpu.v[x] += k,
		9 => cpu.pc = if cpu.v[x] != cpu.v[y] {cpu.pc + 2} else {cpu.pc},
		0xA => cpu.i = nnn,
		0xB => cpu.pc = cpu.v[0] as u16 + nnn,
		0xC => cpu.v[x] = rng.gen::<u8>() & k,
		_ => ()
	}
}

fn f_type(cpu: &mut Cpu, opcode: u16) {
	let x: usize = ((opcode & 0x0F00) >> 8) as usize;

	match opcode & 0x00FF {
		0x07 => cpu.v[x] = cpu.dt,
		0x0A => {
			cpu.key = [0; 16];
			loop {

			}
		},
		0x15 => cpu.dt = cpu.v[x],
		0x1E => cpu.st = cpu.v[x],
		0x29 => cpu.i += cpu.v[x] as u16,
		0x33 => {
			cpu.memory[cpu.i as usize] = cpu.v[x] / 100;
			cpu.memory[(cpu.i + 1) as usize] = (cpu.v[x] % 100) / 10;
			cpu.memory[(cpu.i + 2) as usize] = cpu.v[x] % 10;
		},
		0x55 => {
			for index in 0..x {
				cpu.memory[(cpu.i as usize + index)] = cpu.v[index];
			}
		},
		0x65 => {
			for index in 0..x {
				cpu.v[index] = cpu.memory[(cpu.i as usize + index)];
			}
		},
		_ => ()
	}
}
fn cpuNULL(cpu: &mut Cpu, opcode: u16) {
	println!("Null");
} 

const CHIP8_TABLE: [fn(&mut Cpu, u16); 16] = [
	empty, flow, flow, flow, flow, flow, flow, flow, 
	arithmetic, flow, flow, flow, flow, cpuNULL,
	cpuNULL,f_type
];
const FONT_SET: [u8; 80] = [
	0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
	0x20, 0x60, 0x20, 0x20, 0x70, // 1
	0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
	0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
	0x90, 0x90, 0xF0, 0x10, 0x10, // 4
	0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
	0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
	0xF0, 0x10, 0x20, 0x40, 0x40, // 7
	0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
	0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
	0xF0, 0x90, 0xF0, 0x90, 0x90, // A
	0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
	0xF0, 0x80, 0x80, 0x80, 0xF0, // C
	0xE0, 0x90, 0x90, 0x90, 0xE0, // D
	0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
	0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];