import { WasmMemory } from '../engine';
import './display.scss';

const CELL_SIZE = 5; // px
const OFF_COLOR = '#FFFFFF';
const ON_COLOR = '#000000';

export class Display {
    canvas: HTMLCanvasElement;
    ctx: CanvasRenderingContext2D;

    memory: WasmMemory;
    displayPtr: number;
    registerPtr: number;
    pixels: Uint8Array;

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
        this.canvas.height = CELL_SIZE * this.height;
        this.canvas.width = CELL_SIZE * this.width;

        this.pixels = new Uint8Array(this.memory.buffer, this.displayPtr, this.width * this.height);

        this.drawPixels = this.drawPixels.bind(this);
    }

    private getIndex(row: number, col: number) {
        return row * this.width + col;
    }

    public drawPixels() {
        this.ctx.beginPath();

        this.ctx.fillStyle = ON_COLOR;
        for (let row = 0; row < this.height; row++) {
            for (let col = 0; col < this.width; col++) {
                const idx = this.getIndex(row, col);
                if (this.pixels[idx] === 0) { continue; }
                this.ctx.fillRect(
                    col * CELL_SIZE,
                    row * CELL_SIZE,
                    CELL_SIZE,
                    CELL_SIZE
                );
            }
        }

        this.ctx.fillStyle = OFF_COLOR;
        for (let row = 0; row < this.height; row++) {
            for (let col = 0; col < this.width; col++) {
                const idx = this.getIndex(row, col);
                if (this.pixels[idx] === 1) { continue; }
                this.ctx.fillRect(
                    col * CELL_SIZE,
                    row * CELL_SIZE,
                    CELL_SIZE,
                    CELL_SIZE
                );
            }
        }

        this.ctx.stroke();
    }
}