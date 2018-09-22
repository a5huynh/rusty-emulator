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
}