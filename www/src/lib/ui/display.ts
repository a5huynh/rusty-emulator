import { WasmMemory } from '../engine';

const CELL_SIZE = 5; // px
const GRID_COLOR = '#CCCCCC';
const OFF_COLOR = '#FFFFFF';
const ON_COLOR = '#000000';

export class MemoryDisplay {
    memory: WasmMemory;
    // Pointers to various memory elements.
    registersPtr: number;
    memoryPtr: number;
    stackPtr: number;
    pcPtr: number;
    spPtr: number;
    // Sizes of the register bank, memory bank, and stack.
    numRegisters: number;
    memorySize: number;
    stackSize: number;

    constructor(
        memory: WasmMemory,
        registersPtr: number, numRegisters: number,
        memoryPtr: number, memorySize: number,
        stackPtr: number, stackSize: number,
        pcPtr: number, spPtr: number
    ) {
        this.memory = memory;
        this.registersPtr = registersPtr;
        this.numRegisters = numRegisters;

        this.memoryPtr = memoryPtr;
        this.memorySize = memorySize;

        this.stackPtr = stackPtr;
        this.stackSize = stackSize;

        this.pcPtr = pcPtr;
        this.spPtr = spPtr;
    }

    public drawRegisters() {
        const registers = new Uint8Array(this.memory.buffer, this.registersPtr, 16);
        let disp = '';
        for (let idx = 0; idx < 16; idx++) {
            disp += `<div>V${idx.toString(16)}: 0x${registers[idx].toString(16)}</div>`;
        }

        document.getElementById('registers').innerHTML = disp;
    }
}

export class Display {
    canvas: HTMLCanvasElement;
    ctx: CanvasRenderingContext2D;

    memory: WasmMemory;
    displayPtr: number;
    registerPtr: number;

    width: number;
    height: number;

    constructor(elementId: string, memory: WasmMemory, displayPtr: number, width: number, height: number) {
        this.canvas = <HTMLCanvasElement>document.getElementById(elementId);
        this.ctx = this.canvas.getContext('2d');

        this.memory = memory;
        this.displayPtr = displayPtr;

        // Give the canvas room for all of the cells and a 1px border
        // around each of them.
        this.width = width;
        this.height = height;
        this.canvas.height = (CELL_SIZE + 1) * this.height + 1;
        this.canvas.width = (CELL_SIZE + 1) * this.width + 1;

        this.drawPixels = this.drawPixels.bind(this);
        this.drawGrid = this.drawGrid.bind(this);
    }

    private getIndex(row: number, col: number) {
        return row * this.width + col;
    }

    public drawPixels() {
        const pixels = new Uint8Array(this.memory.buffer, this.displayPtr, this.width * this.height);

        this.ctx.beginPath();

        for (let row = 0; row < this.height; row++) {
            for (let col = 0; col < this.width; col++) {
                const idx = this.getIndex(row, col);
                this.ctx.fillStyle = pixels[idx] === 1 ? ON_COLOR : OFF_COLOR;
                this.ctx.fillRect(
                    col * (CELL_SIZE + 1) + 1,
                    row * (CELL_SIZE + 1) + 1,
                    CELL_SIZE,
                    CELL_SIZE
                );
            }
        }

        this.ctx.stroke();
    }

    public drawGrid() {
        this.ctx.beginPath();
        this.ctx.strokeStyle = GRID_COLOR;

        // Vertical lines
        for (let i = 0; i <= this.width; i++) {
            const x = i * (CELL_SIZE + 1 ) + 1;
            const y = (CELL_SIZE + 1) * this.height + 1;
            this.ctx.moveTo(x, 0);
            this.ctx.lineTo(x, y);
        }

        // Horizontal lines
        for (let j = 0; j <= this.height; j++) {
            const x = (CELL_SIZE + 1) * this.width + 1;
            const y = j * (CELL_SIZE + 1) + 1;
            this.ctx.moveTo(0, y);
            this.ctx.lineTo(x, y);
        }

        this.ctx.stroke();
    }
}