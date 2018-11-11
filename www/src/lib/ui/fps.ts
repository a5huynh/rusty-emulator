
export class FPS {
    fps: HTMLElement;
    frames: Array<number>;
    lastFrame: number;

    constructor() {
        this.fps = document.getElementById('fps');
        this.frames = [];
        this.lastFrame = performance.now();
    }

    render() {
        const now = performance.now();
        const delta = now - this.lastFrame;
        this.lastFrame = now;
        const fps = 1 / delta * 1000;

        // Save only the last 100 timings.
        this.frames.push(fps);
        if (this.frames.length > 100) {
            this.frames.shift();
        }

        let min = Infinity;
        let max = -Infinity;
        let sum = 0;
        for (let i = 0; i < this.frames.length; i++) {
            sum += this.frames[i];
            min = Math.min(this.frames[i], min);
            max = Math.max(this.frames[i], max);
        }

        let mean = sum / this.frames.length;
        this.fps.textContent = `fps: ${Math.round(fps)} (${Math.round(mean)})`;
    }
}