// I couldn't quite figure out how to do this import correctly in Typescript,
// so for now we have a normal javascript bootstrap piece which loads the
// main application written in Typescript.
import { memory } from "z80-emulator/z80_emulator_bg";
import Renderer from './lib/render';

let renderer = new Renderer(memory);
const playPauseButton = document.getElementById("play-pause");

const play = () => {
  playPauseButton.textContent = "⏸";
  renderer.render();
};

const pause = () => {
  playPauseButton.textContent = "▶";
  cancelAnimationFrame(renderer.animationId);
  renderer.animationId = null;
};

playPauseButton.addEventListener("click", event => {
  if (renderer.isPaused()) {
    play();
  } else {
    pause();
  }
});

play();