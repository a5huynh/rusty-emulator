import { Universe, Cell } from "z80-emulator";

const CELL_SIZE = 5; // px
const GRID_COLOR = '#CCCCCC';
const DEAD_COLOR = '#FFFFFF';
const ALIVE_COLOR = '#000000';

interface WasmMemory {
    buffer: ArrayBuffer;
}

export default class Renderer {
    public animationId:number;

    canvas:HTMLCanvasElement;
    ctx:CanvasRenderingContext2D;
    universe:Universe;
    memory: WasmMemory;

    width:number;
    height: number

    constructor(memory: WasmMemory) {
        this.canvas = <HTMLCanvasElement>document.getElementById('game-of-life-canvas');
        this.ctx = this.canvas.getContext('2d');

        this.memory = memory;
        this.universe = Universe.new();

        // Give the canvas room for all of the cells and a 1px border
        // around each of them.
        this.width = this.universe.width();
        this.height = this.universe.height();
        this.canvas.height = (CELL_SIZE + 1) * this.height + 1;
        this.canvas.width = (CELL_SIZE + 1) * this.width + 1;

        this.drawCells = this.drawCells.bind(this);
        this.drawGrid = this.drawGrid.bind(this);
        this.render = this.render.bind(this);
    }

    private getIndex(row: number, col: number) {
        return row * this.width + col;
    }

    private drawCells() {
        const cellsPtr = this.universe.cells();
        const cells = new Uint8Array(this.memory.buffer, cellsPtr, this.width * this.height);

        this.ctx.beginPath();

        for (let row = 0; row < this.height; row++) {
            for (let col = 0; col < this.width; col++) {
                const idx = this.getIndex(row, col);
                this.ctx.fillStyle = cells[idx] === Cell.DEAD ? DEAD_COLOR : ALIVE_COLOR;
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

    private drawGrid() {
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

    public isPaused() {
        return this.animationId === null;
    }

    public render() {
        this.universe.tick();

        this.drawGrid();
        this.drawCells();

        this.animationId = requestAnimationFrame(this.render);
    }
}