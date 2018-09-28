import { CHIP8 } from 'z80-emulator';

import { Display } from './ui/display';
import { MemoryDisplay } from './ui/memory';

export interface WasmMemory {
    buffer: ArrayBuffer;
}

// Test rom base64 encoded.
// Maze test
const TEST_ROM = 'YABhAKIiwgEyAaIe0BRwBDBAEgRgAHEEMSASBBIcgEAgECBAgBA=';

export default class Engine {
    public animationId: number = null;

    engine: CHIP8 = CHIP8.new();
    memory: WasmMemory;

    width: number = CHIP8.display_width();
    height: number = CHIP8.display_height();

    display: Display;
    memDisplay: MemoryDisplay;

    constructor(memory: WasmMemory) {
        this.memory = memory;
        this.display = new Display(
            'engine-display',
            this.memory,
            this.engine.display(),
            this.width,
            this.height
        );

        this.memDisplay = new MemoryDisplay(
            this.memory,
            this.engine.registers(),
            CHIP8.num_registers(),
            this.engine.memory(),
            CHIP8.mem_size(),
            this.engine.stack(),
            CHIP8.stack_size(),
            this.engine.pc(),
            this.engine.sp()
        );

        this.engine.load_rom(this._Base64toBytes(TEST_ROM));

        this.isPaused = this.isPaused.bind(this);
        this.render = this.render.bind(this);
        this.tick = this.tick.bind(this);
    }

    private _Base64toBytes(data: string) {
        let decoded = atob(TEST_ROM);
        let array = new Uint8Array(decoded.length);
        for (let i = 0; i < decoded.length; i++) {
            array[i] = decoded.charCodeAt(i);
        }

        return array;
    }

    public isPaused() {
        return this.animationId === null;
    }

    public tick() {
        this.render();
        this.engine.tick();
        this.animationId = requestAnimationFrame(this.tick);
    }

    public render() {
        this.display.drawGrid();
        this.display.drawPixels();

        this.memDisplay.drawRegisters();
        this.memDisplay.drawMemory(this.isPaused());
    }
}