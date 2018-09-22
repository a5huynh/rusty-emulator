// CHIP-8 emulator
// ---------------
// I tried my best to heavily comment sections that may seem confusing or
// have some interesting/neat implementation details.
//
// Check out the `research` section of the README to learn more.

use wasm_bindgen::prelude::*;
use utils;

// Various dimensions used in this emulator implementation.
const NUM_REGISTERS: usize = 18;
const MEM_SIZE: usize = 4000;
const STACK_SIZE: usize = 16;
const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;

// Mapping of register names to the register bank
#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Register {
    // General purpose registers.
    V0, V1, V2, V3, V4, V5, V6, V7, V8, V9, VA, VB, VC, VD, VE, VF,
    // The delay timer is active when the delay timer reg is non-zero.
    // The timer will subtract at 60Hz and stop at 0.
    DT,
    // The sound timer is active when the sound timer reg is non-zero.
    // As long as it's greater than 0, the buzzer will sound.
    ST,
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct CHIP8 {
    // 16-bit register called "I". This register is generally used to store
    // memory addresses.
    i_reg: u16,
    // Program counter.
    pc: u16,
    // Stack pointer.
    sp: u8,
    // The CHIP-8 has 16 general purpose 8-bit registers, usually referred to as Vx
    // where x is a hexadecimal digit (0 through F).
    registers: [u8; NUM_REGISTERS],
    // The language is capable of accessing up to 4KB of RAM.
    // The first 512 bytes (0x200) are where the original interpreter was located
    // thus most CHIP8 programs start at location 0x200.
    memory: [u8; MEM_SIZE],
    // The stack is an array of 16 16-bit values, used to store the address that
    // interpreter should return to when finished. Thus only really allowing up
    // to 16 levels of nested function calls.
    stack: [u16; STACK_SIZE],
    // Display
    display: [u8; DISPLAY_HEIGHT * DISPLAY_WIDTH],
}

#[wasm_bindgen]
impl CHIP8 {
    pub fn new() -> CHIP8 {
        utils::set_panic_hook();

        CHIP8 {
            i_reg: 0,
            pc: 0x200,
            sp: 0,
            registers: [0; NUM_REGISTERS],
            memory: [0; MEM_SIZE],
            stack: [0; STACK_SIZE],
            display: [0; DISPLAY_HEIGHT * DISPLAY_WIDTH],
        }
    }

    // Handy access to emu constants
    pub fn display_height() -> usize { DISPLAY_HEIGHT }
    pub fn display_width() -> usize { DISPLAY_WIDTH }
    pub fn mem_size() -> usize { MEM_SIZE }
    pub fn num_registers() -> usize { NUM_REGISTERS }
    pub fn stack_size() -> usize { STACK_SIZE }

    // Retrieves the current opcode pointed to by the program counter.
    // All instrs are 2 bytes long and are stored most-sig byte first.
    fn fetch(&mut self) -> u16 {
        // Shift first byte to upper 8 bits and OR second byte into lower 8 bits
        let opcode = u16::from(self.memory[self.pc as usize]) << 8 | u16::from(self.memory[(self.pc + 1) as usize]);
        // Increment program counter
        self.pc += 2;
        return opcode;
    }

    // Executes an opcode.
    fn execute(&mut self, opcode: u16) {
        let instr = opcode & 0xF000;
        let subinstr  = opcode & 0x000F;
        let addr  = opcode & 0x0FFF;
        let lower = (opcode & 0x00FF) as u8;
        // Register positions
        let vx = ((opcode & 0x0F00) >> 8) as usize;
        let vy = ((opcode & 0x00F0) >> 4) as usize;

        match instr {
            0x0000 => {
                match lower {
                    // Clear display.
                    0xE0 => {
                        for idx in 0..MEM_SIZE {
                            self.display[idx] = 0;
                        }
                    },
                    // Return from subroutine.
                    0xEE => {
                        // Sets the program counter to the address at the top
                        // of the stack.
                        self.pc = self.stack[self.sp as usize];
                        // Subtract 1 from the stack pointer.
                        self.sp -= 1;
                    },
                    _ => println!("Unknown opcode {:#X}", opcode)
                }
            },
            // JP <addr>: Jump to <addr>
            0x1000 => self.pc = addr as u16,
            // CALL <addr>: call subroutine at <addr>
            0x2000 => {
                // Increments the stack pointer and adds the current program counter
                // to the top of the stack.
                self.sp += 1;
                self.stack[self.sp as usize] = self.pc;
                // Program counter set to <addr>.
                self.pc = addr as u16;
            },
            // SE vx, byte
            // Skips next instruction if Vx = lower byte.
            0x3000 => {
                if self.registers[vx] == lower {
                    self.pc += 2;
                }
            },
            // SNE vx, byte
            // Skips next instruction if vx != lower byte.
            0x4000 => {
                if self.registers[vx] != lower {
                    self.pc += 2;
                }
            },
            // SE vx, vy
            // Skip next instruction if vx == vy
            0x5000 => {
                if self.registers[vx] == self.registers[vy] {
                    self.pc += 2
                }
            },
            // LD vx, byte (vx = byte)
            // Puts the value of the lower byte into the register vx.
            0x6000 => self.registers[vx] = lower,
            // ADD vx, byte (vx = vx + byte)
            // Adds the value of lower to the value in vx, storing the result in vx.
            0x7000 => self.registers[vx] += lower,
            0x8000 => {
                match subinstr {
                    // LD vx, vy
                    0 => self.registers[vx] = self.registers[vy],
                    // OR vx, vy
                    1 => self.registers[vx] = self.registers[vx] | self.registers[vy],
                    // AND vx, vy
                    2 => self.registers[vx] = self.registers[vx] & self.registers[vy],
                    // XOR vx, vy
                    3 => self.registers[vx] = self.registers[vx] ^ self.registers[vy],
                    // ADD vx, vy
                    4 => self.registers[vx] = self.registers[vx] + self.registers[vy],
                    // SUB vx, vy
                    5 => self.registers[vx] = self.registers[vx] - self.registers[vy],
                    // SHR vx {, vy} (bit shift right)
                    // vx = vx shr 1.
                    // If the least significant bit of vx is 1, then vf is set to 1, otherwise 0.
                    // Then vx is divided by 2.
                    6 => {
                        let lsb = self.registers[vx] & 1;
                        // Set VF to least-significant bit before shift
                        self.registers[Register::VF as usize] = lsb;
                        self.registers[vx] >>= 1;
                    },
                    // SUBN vx, vy
                    // vx = vy - vx, set vf = 1 if vy > vx
                    7 => {
                        if self.registers[vy] > self.registers[vx] {
                            self.registers[Register::VF as usize] = 1;
                        } else {
                            self.registers[Register::VF as usize] = 0;
                        }
                        self.registers[vx] = self.registers[vy] - self.registers[vx];
                    },
                    0xE => {
                        let msb = (self.registers[vx] & 0b1000_0000) >> 7;
                        // Set VF to most-significant bit before shift
                        self.registers[Register::VF as usize] = msb;
                        self.registers[vx] <<= 1;
                    },
                    _ => println!("Unknown opcode {:#X}", opcode),
                }
            },
            _ => println!("Unknown opcode {:#X}", opcode)
        }
    }

    // Retrieves a pointer to the display memory.
    pub fn display(&self) -> *const u8 {
        self.display.as_ptr()
    }

    // Retrieves a pointer to the register bank.
    pub fn registers(&self) -> *const u8 {
        self.registers.as_ptr()
    }

    // Retrieves a pointer to the RAM memory.
    pub fn memory(&self) -> *const u8 {
        self.memory.as_ptr()
    }

    // Retrieves a pointer to the stack
    pub fn stack(&self) -> *const u16 {
        self.stack.as_ptr()
    }

    // Utility functions to get program counter & stack pointer.
    pub fn pc(&self) -> u16 { self.pc }
    pub fn sp(&self) -> u8 { self.sp }

    // Loads a rom (an array of bytes) in the CHIP8 memory and sets the
    // program counter to the beginning.
    pub fn load_rom(&mut self, rom: Option<Box<[u8]>>) -> usize {
        let mut start = 0;
        self.pc = 0x200;
        if let Some(data) = rom {
            for byte in data.iter() {
                self.memory[self.pc as usize + start] = *byte;
                start += 1;
            }
        }
        return start;
    }

    pub fn tick(&mut self) {
        // Fetch opcode
        let opcode = self.fetch();
        // Execute opcode
        self.execute(opcode);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialization() {
        let emu = CHIP8::new();
        assert_eq!(emu.pc, 512);
    }

    #[test]
    fn test_fetch() {
        let mut emu = CHIP8::new();
        // Seed memory at PC with a fake opcode.
        emu.memory[0x200] = 0xAB;
        emu.memory[0x201] = 0xCD;
        // Op-code should be read with most-sig byte first.
        let opcode = emu.fetch();
        assert_eq!(opcode, 0xABCD);
    }

    #[test]
    fn test_load_rom() {
        let mut emu = CHIP8::new();
        // Test the number of bytes written.
        let bytes_written = emu.load_rom(Some(Box::new([1; 8])));
        assert_eq!(bytes_written, 8);
        // Test that the rom was written in the right place.
        let start = 0x200;
        for idx in 0..8 {
            assert_eq!(emu.memory[start + idx], 1);
        }
    }

    #[test]
    fn test_execute_0x1000() {
        let mut emu = CHIP8::new();
        // Test basic jump
        emu.execute(0x1FED);
        assert_eq!(emu.pc, 0x0FED);
    }

    #[test]
    fn test_execute_0x2000() {
        let mut emu = CHIP8::new();
        // Test function call
        emu.pc = 0xDEAD;
        emu.execute(0x2FED);
        assert_eq!(emu.pc, 0x0FED);
        assert_eq!(emu.stack[emu.sp as usize], 0xDEAD);
    }

    #[test]
    fn test_execute_0x3000() {
        let mut emu = CHIP8::new();
        // Test skip instruction
        emu.pc = 0;
        emu.registers[0] = 0xAD;
        emu.execute(0x30AD);
        assert_eq!(emu.pc, 2);

        emu.pc = 0;
        emu.registers[0] = 0;
        emu.execute(0x30AD);
        assert_eq!(emu.pc, 0);
    }

    #[test]
    fn test_execute_0x4000() {
        let mut emu = CHIP8::new();
        // Test ne skip instruction
        emu.pc = 0;
        emu.registers[0] = 0xAD;
        emu.execute(0x40AD);
        assert_eq!(emu.pc, 0);

        emu.pc = 0;
        emu.registers[0] = 0;
        emu.execute(0x40AD);
        assert_eq!(emu.pc, 2);
    }

    #[test]
    fn test_execute_0x5000() {
        let mut emu = CHIP8::new();
        emu.pc = 0;
        emu.registers[0x0] = 0xAB;
        emu.registers[0x1] = 0xAB;
        emu.execute(0x5010);
        assert_eq!(emu.pc, 2);

        emu.pc = 0;
        emu.registers[0x0] = 0xAB;
        emu.registers[0x1] = 0xCD;
        emu.execute(0x5010);
        assert_eq!(emu.pc, 0);
    }

    #[test]
    fn test_execute_0x6000() {
        let mut emu = CHIP8::new();
        emu.execute(0x60AB);
        assert_eq!(emu.registers[0], 0xAB);
    }

    #[test]
    fn test_execute_0x7000() {
        let mut emu = CHIP8::new();
        emu.registers[0] = 2;
        emu.execute(0x7002);
        assert_eq!(emu.registers[0], 4);
    }

    #[test]
    fn test_execute_0x8000() {
        let mut emu = CHIP8::new();
        // LD
        emu.registers[1] = 0xAD;
        emu.execute(0x8010);
        assert_eq!(emu.registers[0], 0xAD);
        // OR
        emu.registers[0] = 0xF0;
        emu.registers[1] = 0x0F;
        emu.execute(0x8011);
        assert_eq!(emu.registers[0], 0xFF);
        // AND
        emu.registers[0] = 0xF0;
        emu.registers[1] = 0x0F;
        emu.execute(0x8012);
        assert_eq!(emu.registers[0], 0x00);
        // XOR
        emu.registers[0] = 0xF0;
        emu.registers[1] = 0x0F;
        emu.execute(0x8013);
        assert_eq!(emu.registers[0], 0xFF);
        // ADD
        emu.registers[0] = 0x02;
        emu.registers[1] = 0x02;
        emu.execute(0x8014);
        assert_eq!(emu.registers[0], 4);
        // SUB
        emu.registers[0] = 0x02;
        emu.registers[1] = 0x02;
        emu.execute(0x8015);
        assert_eq!(emu.registers[0], 0);
        // SHR
        emu.registers[0] = 0x01;
        emu.execute(0x8006);
        // Right shifting 1 should result in VF = 1, V1 = 0
        assert_eq!(emu.registers[Register::VF as usize], 1);
        assert_eq!(emu.registers[0], 0);
        // Right shifting 2 should result in VF = 0, V1 = 1
        emu.registers[0] = 0b0010;
        emu.execute(0x8006);
        assert_eq!(emu.registers[Register::VF as usize], 0);
        assert_eq!(emu.registers[0], 0b0001);
        // SUBN vx, vy
        emu.registers[0] = 0x02;
        emu.registers[1] = 0x04;
        emu.execute(0x8017);
        assert_eq!(emu.registers[Register::VF as usize], 1);
        assert_eq!(emu.registers[0], 2);
        // SHL vx
        emu.registers[0] = 0b1000_0000;
        emu.execute(0x800E);
        assert_eq!(emu.registers[Register::VF as usize], 1);
        assert_eq!(emu.registers[0], 0);

        emu.registers[0] = 0b0000_0001;
        emu.execute(0x800E);
        assert_eq!(emu.registers[Register::VF as usize], 0);
        assert_eq!(emu.registers[0], 0b0010);
    }
}