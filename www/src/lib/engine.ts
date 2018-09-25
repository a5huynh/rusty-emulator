import { CHIP8, Register } from "z80-emulator";
import { Display, MemoryDisplay } from "./ui/display";
import { TextEncoder } from 'text-encoding';

export interface WasmMemory {
    buffer: ArrayBuffer;
}

export default class Engine {
    public animationId: number;

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
        this.render = this.render.bind(this);
        this.isPaused = this.isPaused.bind(this);
    }

    public isPaused() {
        return this.animationId === null;
    }

    public render() {
        // this..tick();

        this.display.drawGrid();
        this.display.drawPixels();
        this.memDisplay.drawRegisters();
        this.animationId = requestAnimationFrame(this.render);
    }
}