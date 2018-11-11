# rusty-emulator

A bunch of emulators written in rust 🦀 and WebAssembly 🕸.

## chip8 emulator

See `src/chip8` for implementation.


## z80 emulator

This project aims to create a browser based emulator of the [Zilog Z80][z80-wiki]
microprocessor with handy introspection tools so that you can run and debug
natively assembled code.

[z80-wiki]: https://en.wikipedia.org/wiki/Zilog_Z80


### research

- [Decoding Z80 opcodes](http://z80.info/decoding.htm#intro)
- [Z80 Instruction Set](http://clrhome.org/table/)

Another person's attempt at a Z80 emulator and some history:
- https://floooh.github.io/2017/12/10/z80-emu-evolution.html
- https://floooh.github.io/2016/06/15/the-amazing-z80.html
