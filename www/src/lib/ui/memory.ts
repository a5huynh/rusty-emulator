import { WasmMemory } from '../engine';

const MEM_PER_ROW = 32;

export class MemoryDisplay {
    memory: WasmMemory;
    // Pointers to various memory elements.
    registers: Uint8Array;

    memoryPtr: number;
    stackPtr: number;
    pcPtr: number;
    spPtr: number;

    // Sizes of the register bank, memory bank, and stack.
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

        this.registers = new Uint8Array(this.memory.buffer, registersPtr, numRegisters);

        this.memoryPtr = memoryPtr;
        this.memorySize = memorySize;

        this.stackPtr = stackPtr;
        this.stackSize = stackSize;

        this.pcPtr = pcPtr;
        this.spPtr = spPtr;

        this._toHex = this._toHex.bind(this);
        this.drawMemory = this.drawMemory.bind(this);
        this.drawRegisters = this.drawRegisters.bind(this);
    }

    private _toHex(number: number, len: number = 2) {
        const hex = number.toString(16);
        return '0'.repeat(len - hex.length) + hex;
    }

    public drawRegisters() {
        let disp = '';
        for (let idx = 0; idx < this.registers.length - 2; idx++) {
            disp += `<div>V${this._toHex(idx)}: 0x${this._toHex(this.registers[idx])}</div>`;
        }

        document.getElementById('registers').innerHTML = disp;
    }

    public drawMemory(isPaused: boolean) {
        const element = document.getElementById('memory');
        if (!isPaused) {
            element.innerHTML = '<code>Memory only shown on pause.</code>';
            return;
        }

        const memory = new Uint8Array(this.memory.buffer, this.memoryPtr, this.memorySize);

        let disp = '';

        for (let row = 0; row < (this.memorySize / MEM_PER_ROW); row++) {
            let rowStart = row * MEM_PER_ROW;
            disp += `<div>${this._toHex(rowStart, 3)}: `;
            for (let col = 0; col < MEM_PER_ROW; col++) {
                let idx = (row * MEM_PER_ROW) + col;
                disp += `<span>${this._toHex(memory[idx])}</span>`
            }
            disp += '</div>';
        }

        element.innerHTML = `<code>${disp}</code>`;
    }
}
