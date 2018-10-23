// I couldn't quite figure out how to do this import correctly in Typescript,
// so for now we have a normal javascript bootstrap piece which loads the
// main application written in Typescript.
import { memory } from 'z80-emulator/z80_emulator_bg';
import Engine from './lib/engine';
const ROM_LIST = [
  { path: 'Breakout [Carmelo Cortez, 1979].ch8', name: 'Breakout' },
  { path: 'Brix [Andreas Gustafsson, 1990].ch8', name: 'Brix' },
  { path: 'Chip8 emulator Logo [Garstyciuks].ch8', name: 'Chip8 Logo' },
  { path: 'Maze [David Winter, 199x].ch8', name: 'Maze' }
];

let engine = new Engine(memory);

// Handle ROM loading
const romList = document.getElementById('rom-list');
romList.innerHTML += '<option>Select a ROM</option>';
ROM_LIST.forEach(rom => romList.innerHTML += `<option value="${rom.path}">${rom.name}</option>`);
romList.addEventListener('change', event => {
  const rom = event.target.value;
  return fetch(`/roms/chip8/${rom}`)
    .then((resp) => {
      if (!resp.ok) {
        throw new Error(resp.statusText);
      }

      return resp.arrayBuffer();
    })
    .then((buffer) => {
      let array = new Uint8Array(buffer);
      console.log(array);
      engine.engine.load_rom(array);
    });
});


const playPauseButton = document.getElementById('play-pause');
playPauseButton.textContent = '▶';

const play = () => {
  playPauseButton.textContent = "⏸";
  engine.tick();
};

const pause = () => {
  playPauseButton.textContent = '▶';
  cancelAnimationFrame(engine.animationId);
  engine.animationId = null;
};

playPauseButton.addEventListener('click', event => {
  if (engine.isPaused()) {
    console.log('Starting engine');
    play();
  } else {
    console.log('Pausing engine');
    pause();
    engine.render();
  }
});

// Render UI but don't start engine yet.
engine.render();