import { Universe, Cell } from "z80-emulator";
import { memory } from "z80-emulator/z80_emulator_bg";

const CELL_SIZE = 5; // px
const GRID_COLOR = '#CCCCCC';
const DEAD_COLOR = '#FFFFFF';
const ALIVE_COLOR = '#000000';

const universe = Universe.new();
const width = universe.width();
const height = universe.height();

const canvas = document.getElementById('game-of-life-canvas');
// Give the canvas room for all of the cells and a 1px border
// around each of them.
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');
let animationId = null;
const renderLoop = () => {
    universe.tick();

    drawGrid();
    drawCells();

    animationId = requestAnimationFrame(renderLoop);
};

const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    // Vertical lines
    for (let i = 0; i <= width; i++) {
        const x = i * (CELL_SIZE + 1 ) + 1;
        const y = (CELL_SIZE + 1) * height + 1;
        ctx.moveTo(x, 0);
        ctx.lineTo(x, y);
    }

    // Horizontal lines
    for (let j = 0; j <= height; j++) {
        const x = (CELL_SIZE + 1) * width + 1;
        const y = j * (CELL_SIZE + 1) + 1;
        ctx.moveTo(0, y);
        ctx.lineTo(x, y);
    }

    ctx.stroke();
};

const isPaused = () => {
    return animationId === null;
}

const getIndex = (row, column) => {
    return row * width + column;
};

const drawCells = () => {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

    ctx.beginPath();

    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);
            ctx.fillStyle = cells[idx] === Cell.DEAD ? DEAD_COLOR : ALIVE_COLOR;
            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }

    ctx.stroke();
};

const playPauseButton = document.getElementById("play-pause");

const play = () => {
  playPauseButton.textContent = "⏸";
  renderLoop();
};

const pause = () => {
  playPauseButton.textContent = "▶";
  cancelAnimationFrame(animationId);
  animationId = null;
};

playPauseButton.addEventListener("click", event => {
  if (isPaused()) {
    play();
  } else {
    pause();
  }
});

play();