// A dependency graph that contains any wasm must all be imported
// asynchronously. This `bootstrap.js` file does the single async import, so
// that no one else needs to worry about it again.
import("./XXX_MODULE.js")
  .then(mod => main(mod))
  .catch(e => console.error("Error:", e));

const HEIGHT = 256;
const WIDTH = 256

let keyDown = false;
window.addEventListener("keydown", () => keyDown = true);

async function main(mod) {
  const frameBuffer = new Uint8ClampedArray(HEIGHT * WIDTH * 4);

  while (true) {
    mod.frame(frameBuffer, keyDown);
    render(frameBuffer);

    keyDown = false;
    await new Promise(resolve => requestAnimationFrame(resolve));
  }
}

const canvas = document.getElementById("canvas");
canvas.width = WIDTH;
canvas.height = HEIGHT;

const ctx = canvas.getContext("2d");

function render(frameBuffer) {
  let data = new ImageData(frameBuffer, WIDTH, HEIGHT);
  ctx.putImageData(data, 0, 0);
}
