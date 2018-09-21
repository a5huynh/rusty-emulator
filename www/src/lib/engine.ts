import { CHIP8, Register } from "z80-emulator";
import Display from "./ui/display";

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

    constructor(memory: WasmMemory) {
        this.memory = memory;
        this.display = new Display(
            'engine-display',
            this.memory,
            this.engine.display(),
            this.width,
            this.height
        );

        this.render = this.render.bind(this);
    }

    public isPaused() {
        return this.animationId === null;
    }

    public render() {
        // this..tick();

        this.display.drawGrid();
        this.display.drawPixels();

        this.animationId = requestAnimationFrame(this.render);
    }
}