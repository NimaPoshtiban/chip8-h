
use rand::{rngs::OsRng, RngCore};
use std::{io::{self, BufReader, Read, Seek}, ptr::null};
const START_ADDRESS: u32 = 0x200;
pub const VIDEO_WIDTH: u32 = 120;
pub const VIDEO_HEIGHT: u32 = 60;
const FONTSET_SIZE: u32 = 80;
const FONTSET_START_ADDRESS: u16 = 0x50;
const fontset: [u8; FONTSET_SIZE as usize] = [
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
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Chip8 {
    /// The CHIP-8 has sixteen 8-bit registers, labeled V0 to VF.
    /// Each register is able to hold any value from 0x00 to 0xFF.
    /// Register VF is a bit special. It’s used as a flag to hold information about the result of operations.
    registers: [u8; 0x0f], // general purpose registers
    /// The CHIP-8 has 4096 bytes of memory, meaning the address space is from 0x000 to 0xFFF.
    /// The address space is segmented into three sections:
    /// 0x000-0x1FF: Originally reserved for the CHIP-8 interpreter, but in our modern emulator we will just never write to or read from that area. Except for…
    /// 0x050-0x0A0: Storage space for the 16 built-in characters (0 through F).
    /// 0x200-0xFFF: Instructions from the ROM will be stored starting at 0x200, and anything left after the ROM’s space is free to use.
    memory: [u8; 4096],
    index: u16,         // index register
    pc: u16,            // program counter reg
    stack: [u16; 0x0f], // stack level
    sp: u8,             // stack pointer reg
    /// The CHIP-8 has a simple timer used for timing.
    /// If the timer value is zero, it stays zero.
    /// If it is loaded with a value, it will decrement at a rate of 60Hz.
    delay_timer: u8,
    /// The CHIP-8 also has another simple timer used for sound.
    /// Its behavior is the same (decrementing at 60Hz if non-zero),
    ///  but a single tone will buzz when it’s non-zero.
    ///  Programmers used this for simple sound emission.
    sound_timer: u8,
    ///  The CHIP-8 has 16 input keys that match the first 16 hex values: 0 through F.
    ///  Each key is either pressed or not pressed
    pub keypad: [u8; 0x0f],
    /// The CHIP-8 has an additional memory buffer used for storing the graphics to display. It is 64 pixels wide and 32 pixels high.
    /// Each pixel is either on or off, so only two colors can be represented.
    pub video: [u32; 64 * 32],

    opcode: u16,

    rand_gen: OsRng,

    /// Function Pointer Table
    /// $0 needs an array that can index up to $E+1
    /// $8 needs an array that can index up to $E+1
    /// $E needs an array that can index up to $E+1
    /// $F needs an array that can index up to $65+1
    table: Vec<fn(&mut Chip8)>,
    table0: Vec<fn(&mut Chip8)>,
    table8: Vec<fn(&mut Chip8)>,
    tableE: Vec<fn(&mut Chip8)>,
    tableF: Vec<fn(&mut Chip8)>,


}
impl Default for Chip8 {
    fn default() -> Self {
        Self {
            registers: Default::default(),
            memory: [0; 4096],
            index: Default::default(),
            pc: Default::default(),
            stack: Default::default(),
            sp: Default::default(),
            delay_timer: Default::default(),
            sound_timer: Default::default(),
            keypad: Default::default(),
            video: [0; 2048],
            opcode: Default::default(),
            rand_gen: OsRng {},
            table: Vec::new(),
            table0: Vec::new(),
            table8: Vec::new(),
            tableE: Vec::new(),
            tableF: Vec::new(),

        }
    }
}

impl Chip8 {
    pub fn new() -> Self {
        let mut chip: Chip8 = Chip8::default();
        chip.pc = START_ADDRESS as u16;
        (0..FONTSET_SIZE).into_iter().for_each(|e| {
            chip.memory[(FONTSET_START_ADDRESS as usize) + (e as usize)] = fontset[e as usize]
        });
        let mut table:Vec<fn(&mut Chip8)> = vec![Chip8::OP_NULL;0xF + 1];
        let mut table0:Vec<fn(&mut Chip8)> = vec![Chip8::OP_NULL;0xE + 1];
        let mut table8:Vec<fn(&mut Chip8)> = vec![Chip8::OP_NULL;0xE + 1];
        let mut tableE:Vec<fn(&mut Chip8)> = vec![Chip8::OP_NULL;0xE + 1];
        let mut tableF:Vec<fn(&mut Chip8)> = vec![Chip8::OP_NULL;0x65 + 1];
        table.fill(Chip8::OP_NULL);
        table0.fill(Chip8::OP_NULL);
        table8.fill(Chip8::OP_NULL);
        tableE.fill(Chip8::OP_NULL);
        tableF.fill(Chip8::OP_NULL);

        table[0x0] = Chip8::Table0;
		table[0x1] = Chip8::OP_1nnn;
		table[0x2] = Chip8::OP_2nnn;
		table[0x3] = Chip8::OP_3xkk;
		table[0x4] = Chip8::OP_4xkk;
		table[0x5] = Chip8::OP_5xy0;
		table[0x6] = Chip8::OP_6xkk;
		table[0x7] = Chip8::OP_7xkk;
		table[0x8] = Chip8::Table8;
		table[0x9] = Chip8::OP_9xy0;
		table[0xA] = Chip8::OP_Annn;
		table[0xB] = Chip8::OP_Bnnn;
		table[0xC] = Chip8::OP_Cxkk;
		table[0xD] = Chip8::OP_Dxyn;
		table[0xE] = Chip8::TableE;
		table[0xF] = Chip8::TableF;
        (0..=0xE).into_iter().for_each(|f| {
            table0[f] = Chip8::OP_NULL;
			table8[f] = Chip8::OP_NULL;
			tableE[f] = Chip8::OP_NULL;
        });
        table0[0x0] = Chip8::OP_00E0;
		table0[0xE] = Chip8::OP_00EE;

		table8[0x0] = Chip8::OP_8xy0;
		table8[0x1] = Chip8::OP_8xy1;
		table8[0x2] = Chip8::OP_8xy2;
		table8[0x3] = Chip8::OP_8xy3;
		table8[0x4] = Chip8::OP_8xy4;
		table8[0x5] = Chip8::OP_8xy5;
		table8[0x6] = Chip8::OP_8xy6;
		table8[0x7] = Chip8::OP_8xy7;
		table8[0xE] = Chip8::OP_8xyE;

		tableE[0x1] = Chip8::OP_ExA1;
		tableE[0xE] = Chip8::OP_Ex9E;

        (0..=0x65).into_iter().for_each(|i|{
            tableF[i] = Chip8::OP_NULL;
        });
        tableF[0x07] = Chip8::OP_Fx07;
		tableF[0x0A] = Chip8::OP_Fx0A;
		tableF[0x15] = Chip8::OP_Fx15;
		tableF[0x18] = Chip8::OP_Fx18;
		tableF[0x1E] = Chip8::OP_Fx1E;
		tableF[0x29] = Chip8::OP_Fx29;
		tableF[0x33] = Chip8::OP_Fx33;
		tableF[0x55] = Chip8::OP_Fx55;
		tableF[0x65] = Chip8::OP_Fx65;
        chip.table = table;
        chip.table0 = table0;
        chip.table8 = table8;
        chip.tableE = tableE;
        chip.tableF = tableF;
        chip
    }

    /// Fetch the next instruction in the form of an opcode
    /// Decode the instruction to determine what operation needs to occur
    /// Execute the instruction
    pub fn cycle(&mut self){

        self.opcode = ((self.memory[(self.pc) as usize]).overflowing_shl(8).0 | self.memory[(self.pc+1) as usize]) as u16; // fetch
        self.pc += 2;

        // Decode and Execute
        (self.table[((self.opcode&0xF000 ) as usize) >> 12])(self);
        if self.delay_timer > 0{
            self.delay_timer -= 1;
        }
        if self.sound_timer >0{
            self.sound_timer -=1;
        }

    }

    fn Table0(&mut self){
		(self.table0[(self.opcode & 0x000F) as usize])(self);
	}

	
	fn Table8(&mut self){
		(self.table8[(self.opcode & 0x000F) as usize])(self);
	}

	fn TableE(&mut self){
		(self.tableE[(self.opcode & 0x000F) as usize])(self);
	}

	fn TableF(&mut self){
		self.tableF[(self.opcode & 0x00FF) as usize](self);
	}

    fn OP_NULL(&mut self){}
    /// CLS
    /// Clear the display.
    fn OP_00E0(&mut self) {
        self.video.fill_with(|| 0);
    }

    /// RST
    /// Return from a subroutine.
    fn OP_00EE(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];
    }

    /// JP addr
    /// Jump to location nnn.
    fn OP_1nnn(&mut self) {
        self.pc = self.opcode & 0x0FFF;
    }

    /// CALL addr
    /// Call subroutine at nnn.
    fn OP_2nnn(&mut self) {
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = self.opcode & 0x0FFF;
    }

    /// SE Vx, byte
    /// Skip next instruction if Vx = kk.
    fn OP_3xkk(&mut self) {
        let Vx = ((self.opcode & 0x0F00) >> 8) as u8;
        let byte = (self.opcode & 0x00FF) as u8;
        if self.registers[Vx as usize] == byte {
            self.pc += 2;
        }
    }

    /// SNE Vx, byte
    /// Skip next instruction if Vx != kk.
    fn OP_4xkk(&mut self) {
        let Vx = ((self.opcode & 0x0F00) >> 8) as u8;
        let byte = (self.opcode & 0x00FF) as u8;
        if self.registers[Vx as usize] != byte {
            self.pc += 2;
        }
    }
    /// SE Vx, Vy
    /// Skip next instruction if Vx = Vy.
    fn OP_5xy0(&mut self) {
        let Vx = ((self.opcode & 0x0F00) >> 8) as u8;
        let Vy = ((self.opcode & 0x00F0) >> 4) as u8;
        if self.registers[Vx as usize] == self.registers[Vy as usize] {
            self.pc += 2;
        }
    }

    /// LD Vx, byte
    /// Set Vx = kk.
    fn OP_6xkk(&mut self) {
        let Vx = ((self.opcode & 0x0F00) >> 8) as u8;
        let byte = (self.opcode & 0x00FF) as u8;
        self.registers[Vx as usize] = byte;
    }

    /// ADD Vx, byte
    /// Set Vx = Vx + kk.
    fn OP_7xkk(&mut self) {
        let Vx = ((self.opcode & 0x0F00) >> 8) as u8;
        let byte = (self.opcode & 0x00FF) as u8;
        self.registers[Vx as usize] += byte;
    }
    /// LD Vx, Vy
    /// Set Vx = Vy.
    fn OP_8xy0(&mut self) {
        let Vx = ((self.opcode & 0x0F00) >> 8) as u8;
        let Vy = ((self.opcode & 0x00F0) >> 4) as u8;
        self.registers[Vx as usize] = self.registers[Vy as usize];
    }

    /// OR Vx, Vy
    /// Set Vx = Vx OR Vy.
    fn OP_8xy1(&mut self) {
        let Vx = ((self.opcode & 0x0F00) >> 8) as u8;
        let Vy = ((self.opcode & 0x00F0) >> 4) as u8;
        self.registers[Vx as usize] |= self.registers[Vy as usize];
    }

    /// AND Vx, Vy
    /// Set Vx = Vx OR Vy.
    fn OP_8xy2(&mut self) {
        let Vx = ((self.opcode & 0x0F00) >> 8) as u8;
        let Vy = ((self.opcode & 0x00F0) >> 4) as u8;
        self.registers[Vx as usize] &= self.registers[Vy as usize];
    }

    /// XOR Vx, Vy
    /// Set Vx = Vx XOR Vy.
    fn OP_8xy3(&mut self) {
        let Vx = ((self.opcode & 0x0F00) >> 8) as u8;
        let Vy = ((self.opcode & 0x00F0) >> 4) as u8;
        self.registers[Vx as usize] ^= self.registers[Vy as usize];
    }
    /// ADD Vx, Vy
    /// Set Vx = Vx + Vy, set VF = carry.
    /// The values of Vx and Vy are added together.
    /// If the result is greater than 8 bits (i.e., > 255,) VF is set to 1,
    /// otherwise 0. Only the lowest 8 bits of the result are kept, and stored in Vx.
    fn OP_8xy4(&mut self) {
        let Vx = ((self.opcode & 0x0F00) >> 8) as u8;
        let Vy = ((self.opcode & 0x00F0) >> 4) as u8;
        let sum = (self.registers[Vx as usize] + self.registers[Vy as usize]) as u16;
        if sum > 255 {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
        self.registers[Vx as usize] = (sum & 0xFF) as u8;
    }
    /// SUB Vx, Vy
    /// Set Vx = Vx - Vy, set VF = NOT borrow.
    /// If Vx > Vy, then VF is set to 1, otherwise 0.
    /// Then Vy is subtracted from Vx, and the results stored in Vx.
    fn OP_8xy5(&mut self) {
        let Vx = ((self.opcode & 0x0F00) >> 8) as u8;
        let Vy = ((self.opcode & 0x00F0) >> 4) as u8;
        if self.registers[Vx as usize] > self.registers[Vy as usize] {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
        self.registers[Vx as usize] -= self.registers[Vy as usize];
    }
    /// SHR Vx
    /// If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0.
    /// Then Vx is divided by 2.
    /// A right shift is performed (division by 2),
    /// and the least significant bit is saved in Register VF.
    fn OP_8xy6(&mut self) {
        let Vx = ((self.opcode & 0x0F00) >> 8) as u8;
        self.registers[0xF] = self.registers[Vx as usize] & 0x1;
        self.registers[Vx as usize] >>= 1;
    }
    /// SUBN Vx, Vy
    /// Set Vx = Vy - Vx, set VF = NOT borrow.
    /// If Vy > Vx, then VF is set to 1, otherwise 0.
    /// Then Vx is subtracted from Vy, and the results stored in Vx.
    fn OP_8xy7(&mut self) {
        let Vx = ((self.opcode & 0x0F00) >> 8) as u8;
        let Vy = ((self.opcode & 0x00F0) >> 4) as u8;
        if self.registers[Vx as usize] < self.registers[Vy as usize] {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
        self.registers[Vx as usize] = self.registers[Vy as usize] - self.registers[Vx as usize];
    }
    /// SHL Vx {, Vy}
    /// Set Vx = Vx SHL 1.
    /// If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
    /// A left shift is performed (multiplication by 2), and the most significant bit is saved in Register VF.
    fn OP_8xyE(&mut self) {
        let Vx = ((self.opcode & 0x0F00) >> 8) as u8;
        // save most significant byte in VF
        self.registers[0xF] = (self.registers[Vx as usize] & 0x80) >> 7;
        self.registers[Vx as usize] <<= 1;
    }

    /// SNE Vx, Vy
    /// Skip next instruction if Vx != Vy.
    fn OP_9xy0(&mut self) {
        let Vx = ((self.opcode & 0x0F00) >> 8) as u8;
        let Vy = ((self.opcode & 0x00F0) >> 4) as u8;
        if self.registers[Vx as usize] != self.registers[Vy as usize] {
            self.pc += 2;
        }
    }

    /// LD I, addr
    /// Set I = nnn.
    fn OP_Annn(&mut self) {
        self.index = (self.opcode & 0x0FFF) as u16;
    }

    /// JP V0, addr
    /// Jump to location nnn + V0.
    fn OP_Bnnn(&mut self) {
        let address = (self.opcode & 0x0FFF) as u16;
        self.pc = (self.registers[0] as u16) + address;
    }

    /// RND Vx, byte
    /// Set Vx = random byte AND kk.
    fn OP_Cxkk(&mut self) {
        let Vx = ((self.opcode & 0x0F00) >> 8) as u8;
        let byte = (self.opcode & 0x00FF) as u8;
        self.registers[Vx as usize] = ((self.rand_gen.next_u32() % 256) as u8) & byte;
    }
    /// DRW Vx, Vy, nibble
    /// Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
    fn OP_Dxyn(&mut self) {
        let Vx = ((self.opcode & 0x0F00) >> 8) as u8;
        let Vy = ((self.opcode & 0x00F0) >> 4) as u8;
        let height = (self.opcode & 0x000F) as u8;

        let x_pos = self.registers[Vx as usize] % (VIDEO_WIDTH as u8);
        let y_pos = self.registers[Vy as usize] % (VIDEO_HEIGHT as u8);

        self.registers[0xF] = 0;
        (0..height).into_iter().for_each(|e| {
            let sprite_byte = self.memory[(self.index + (e as u16)) as usize];
            (0..8).into_iter().for_each(|c| {
                let sprite_pixel = sprite_byte & (0x80 >> c);
                let mut screen_pixel = self.video
                    [((y_pos + e) as usize) * ((VIDEO_WIDTH as u8) + (x_pos + (c) as u8)) as usize];
                if sprite_pixel != 0 {
                    if screen_pixel == 0xFFFFFFFF {
                        self.registers[0xF] = 1;
                    }
                    screen_pixel ^= 0xFFFFFFFF;
                }
            })
        });
    }

    /// SKP Vx
    /// Skip next instruction if key with the value of Vx is pressed.
    fn OP_Ex9E(&mut self) {
        let Vx = ((self.opcode & 0x0F00) >> 8) as u8;
        let key = self.registers[Vx as usize];
        if self.keypad[key as usize] != 0 {
            self.pc += 2;
        }
    }

    /// SKNP Vx
    /// Skip next instruction if key with the value of Vx is not pressed.
    fn OP_ExA1(&mut self) {
        let Vx = ((self.opcode & 0x0F00) >> 8) as u8;
        let key = self.registers[Vx as usize];
        if self.keypad[key as usize] == 0 {
            self.pc += 2;
        }
    }

    /// LD Vx, DT
    /// Set Vx = delay timer value.
    fn OP_Fx07(&mut self) {
        let Vx = ((self.opcode & 0x0F00) >> 8) as u8;
        self.registers[Vx as usize] = self.delay_timer;
    }

    /// LD Vx, K
    /// Wait for a key press, store the value of the key in Vx.
    fn OP_Fx0A(&mut self) {
        let Vx = ((self.opcode & 0x0F00) >> 8) as u8;
        if self.keypad[0] == 1 {
            self.registers[Vx as usize] = 0;
        } else if self.keypad[1] == 1 {
            self.registers[Vx as usize] = 1;
        } else if self.keypad[2] == 1 {
            self.registers[Vx as usize] = 2;
        } else if self.keypad[3] == 1 {
            self.registers[Vx as usize] = 3;
        } else if self.keypad[4] == 1 {
            self.registers[Vx as usize] = 4;
        } else if self.keypad[5] == 1 {
            self.registers[Vx as usize] = 5;
        } else if self.keypad[6] == 1 {
            self.registers[Vx as usize] = 6;
        } else if self.keypad[7] == 1 {
            self.registers[Vx as usize] = 7;
        } else if self.keypad[8] == 1 {
            self.registers[Vx as usize] = 8;
        } else if self.keypad[9] == 1 {
            self.registers[Vx as usize] = 9;
        } else if self.keypad[10] == 1 {
            self.registers[Vx as usize] = 10;
        } else if self.keypad[11] == 1 {
            self.registers[Vx as usize] = 11;
        } else if self.keypad[12] == 1 {
            self.registers[Vx as usize] = 12;
        } else if self.keypad[13] == 1 {
            self.registers[Vx as usize] = 13;
        } else if self.keypad[14] == 1 {
            self.registers[Vx as usize] = 14;
        } else if self.keypad[15] == 1 {
            self.registers[Vx as usize] = 15;
        } else {
            self.pc -= 2;
        }
    }

    /// LD DT, Vx
    /// Set delay timer = Vx.
    fn OP_Fx15(&mut self){
        let Vx = ((self.opcode & 0x0F00) >> 8) as u8;
        self.delay_timer = self.registers[Vx as usize];
    }

    /// LD ST, Vx
    /// Set sound timer = Vx.
    fn OP_Fx18(&mut self){
        let Vx = ((self.opcode & 0x0F00) >> 8) as u8;
        self.sound_timer = self.registers[Vx as usize];
    }

    /// ADD I, Vx
    /// Set I = I + Vx.
    fn OP_Fx1E(&mut self){
        let Vx = ((self.opcode & 0x0F00) >> 8) as u8;
        self.index += (self.registers[Vx as usize]) as u16;
    }

    /// LD F, Vx
    /// Set I = location of sprite for digit Vx.
    fn OP_Fx29(&mut self){
        let Vx = ((self.opcode & 0x0F00) >> 8) as u8;
        let digit = self.registers[Vx as usize] as u16;
        self.index = FONTSET_START_ADDRESS + (5*digit);
    }

    /// LD B, Vx
    /// Store BCD representation of Vx in memory locations I, I+1, and I+2.
    /// The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I,
    /// the tens digit at location I+1, and the ones digit at location I+2.
    fn OP_Fx33(&mut self){
        let Vx = ((self.opcode & 0x0F00) >> 8) as u8;
        let mut value = self.registers[Vx as usize];
        self.memory[(self.index) as usize + 2] = value % 10;
        value /= 10;

        self.memory[(self.index) as usize + 1] = value % 10;
        value /= 10;

        self.memory[self.index as usize] = value % 10;
    }

    /// LD [I], Vx
    /// Store registers V0 through Vx in memory starting at location I.
    fn OP_Fx55(&mut self){
        let Vx = ((self.opcode & 0x0F00) >> 8) as u8;
        (0..=Vx).into_iter().for_each(|i|{
            self.memory[(self.index + (i as u16)) as usize] = self.registers[i as usize];
        });
    }


    /// LD Vx, [I]
    /// Read registers V0 through Vx from memory starting at location I.
    fn OP_Fx65(&mut self){
        let Vx = ((self.opcode & 0x0F00) >> 8) as u8;
        (0..=Vx).into_iter().for_each(|i|{
            self.registers[i as usize] = self.memory[(self.index + (i as u16)) as usize];
        });
    }


    /// loads the contents of a ROM file.
    pub fn load_ROM(&mut self, filename: String) -> io::Result<()> {
        let file = std::fs::File::open(filename).unwrap();
        let file_size = file.metadata().unwrap().len() as usize;
        let mut buffer: Vec<u8> = vec![0;file_size];
        let mut reader = BufReader::new(file);
        reader.seek(io::SeekFrom::Start(0)).unwrap();
        reader.read(&mut buffer).unwrap();
        (0..file_size)
            .into_iter()
            .for_each(|e| self.memory[(START_ADDRESS + ((e) as u32)) as usize] = buffer[e]);
        Ok(())
    }
}
