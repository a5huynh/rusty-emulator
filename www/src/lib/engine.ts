import { CHIP8, Key } from 'z80-emulator';

import { Display } from './ui/display';
import { FPS } from './ui/fps';
import { MemoryDisplay } from './ui/memory';

export interface WasmMemory {
    buffer: ArrayBuffer;
}

const SPEED_UP = 4;
// Maps keycode -> CHIP-8 key.
const KEY_MAP: { [key: number]: Key } = {
    49: 0x1,  // 1
    50: 0x2,  // 2
    51: 0x3,  // 3
    52: 0xC,  // 4
    81: 0x4,  // Q
    87: 0x5,  // W
    69: 0x6,  // E
    82: 0xD,  // R
    65: 0x7,  // A
    83: 0x8,  // S
    68: 0x9,  // D
    70: 0xE,  // F
    90: 0xA,  // Z
    88: 0x0,  // X
    67: 0xB,  // C
    86: 0xF   // V
};

export default class Engine {
    public animationId: number = null;

    engine: CHIP8 = CHIP8.new();
    memory: WasmMemory;

    width: number = CHIP8.display_width();
    height: number = CHIP8.display_height();

    fps: FPS;
    display: Display;
    memDisplay: MemoryDisplay;

    showMemDisplay: boolean;

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
        this.fps = new FPS();

        this.handleKeyPress = this.handleKeyPress.bind(this);
        this.handleKeyUp = this.handleKeyUp.bind(this);
        this.isPaused = this.isPaused.bind(this);
        this.render = this.render.bind(this);
        this.tick = this.tick.bind(this);

        window.addEventListener('keydown', this.handleKeyPress);
        window.addEventListener('keyup', this.handleKeyUp);
    }

    private _Base64toBytes(data: string) {
        let decoded = atob(data);
        let array = new Uint8Array(decoded.length);
        for (let i = 0; i < decoded.length; i++) {
            array[i] = decoded.charCodeAt(i);
        }

        return array;
    }

    private _parseURLParams() {
        let urlParams = new URLSearchParams(window.location.search);
        this.showMemDisplay = urlParams.has('memDisplay') ? urlParams.get('memDisplay') === 'true' : false;
    }

    public handleKeyPress(ev: KeyboardEvent) {
        if (ev.keyCode in KEY_MAP) {
            this.engine.key_press(KEY_MAP[ev.keyCode]);
        }
    }

    public handleKeyUp(ev: KeyboardEvent) {
        if (ev.keyCode in KEY_MAP) {
            this.engine.key_up(KEY_MAP[ev.keyCode]);
        }
    }

    public isPaused() {
        return this.animationId === null;
    }

    public tick() {
        this.fps.render();

        this.render();
        for (let i = 0; i < SPEED_UP; i++) {
            this.engine.tick();
        }
        this.animationId = requestAnimationFrame(this.tick);
    }

    public render() {
        this.display.drawPixels();

        this.memDisplay.drawRegisters();
        if (this.showMemDisplay) {
            this.memDisplay.drawMemory(this.isPaused());
        }
    }
}