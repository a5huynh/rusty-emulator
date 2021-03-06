// CHIP8 programs may refer to a group of sprites representing the
// hexadecimal digits 0 through F.
//
// These sprites are 5 bytes long, or 8x5 pixels. The data should be stored
// in the interpreter area of Chip-8 memory (0x000 to 0x1FF). Below is a
// listing of each character's bytes, in binary and hexadecimal:
const ZERO: [u8; 5]  = [0xF0, 0x90, 0x90, 0x90, 0xF0];
const ONE: [u8; 5]   = [0x20, 0x60, 0x20, 0x20, 0x70];
const TWO: [u8; 5]   = [0xF0, 0x10, 0xF0, 0x80, 0xF0];
const THREE: [u8; 5] = [0xF0, 0x10, 0xF0, 0x10, 0xF0];
const FOUR: [u8; 5]  = [0x90, 0x90, 0xF0, 0x10, 0x10];
const FIVE: [u8; 5]  = [0xF0, 0x80, 0xF0, 0x10, 0xF0];
const SIX: [u8; 5]   = [0xF0, 0x80, 0xF0, 0x90, 0xF0];
const SEVEN: [u8; 5] = [0xF0, 0x10, 0x20, 0x40, 0x40];
const EIGHT: [u8; 5] = [0xF0, 0x90, 0xF0, 0x90, 0xF0];
const NINE: [u8; 5]  = [0xF0, 0x90, 0xF0, 0x10, 0xF0];
const A: [u8; 5]     = [0xF0, 0x90, 0xF0, 0x90, 0x90];
const B: [u8; 5]     = [0xE0, 0x90, 0xE0, 0x90, 0xE0];
const C: [u8; 5]     = [0xF0, 0x80, 0x80, 0x80, 0xF0];
const D: [u8; 5]     = [0xE0, 0x90, 0x90, 0x90, 0xE0];
const E: [u8; 5]     = [0xF0, 0x80, 0xF0, 0x80, 0xF0];
const F: [u8; 5]     = [0xF0, 0x80, 0xF0, 0x80, 0x80];

pub const FONT: [[u8; 5]; 16] = [
    ZERO, ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE,
    A, B, C, D, E, F,
];