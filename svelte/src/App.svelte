<script lang="ts">
  import { onMount } from 'svelte';
  import type { Universe, Cell } from '../../pkg';

  const rust = import('../../pkg');
  const memory = import('../../pkg/index_bg.wasm');
  const init = (
    wasm: {
      default: typeof import('/home/spicy/git/wasm-game-of-life/pkg/index');
      greet(name: string): void;
      Cell: typeof Cell;
      Universe: typeof Universe;
    },
    hello
  ) => {
    // Initiate canvas
    const canvas = document.getElementById('canvas-wasm') as HTMLCanvasElement;
    const ctx = canvas.getContext('2d');
    let canvasLW = window.innerHeight;
    if (window.innerWidth < window.innerHeight) {
      canvasLW = window.innerWidth;
    }
    canvas.width = canvasLW;
    canvas.height = canvasLW;

    // Initiate universe
    const universe = wasm.Universe.new(64, 64);

    // Parameters
    const width = universe.width();
    const height = universe.height();
    const cwidth = canvas.width;
    const cheight = canvas.height;

    const dw = cwidth / width;
    const dh = cheight / height;

    // Define Grid
    const drawGrid = () => {
      ctx.beginPath();

      // For every row
      for (let row = 0; row < height; row++) {
        // Draw line from beg to end of line
        const offset = row * dh;
        ctx.beginPath();
        ctx.moveTo(0, offset);
        ctx.lineTo(cwidth, offset);
        ctx.stroke();
        ctx.closePath();
      }

      // For every column
      for (let column = 0; column < width; column++) {
        const offset = column * dw;
        ctx.beginPath();
        ctx.moveTo(offset, 0);
        ctx.lineTo(offset, cheight);
        ctx.stroke();
        ctx.closePath();
      }

      ctx.closePath();
    };

    // Define cells
    const drawCells = () => {
      const cellPtr = universe.cells();
      const cells = new Uint8Array(
        hello.memory.buffer,
        cellPtr,
        width * height
      );

      ctx.beginPath();
      for (let row = 0; row < height; row++) {
        for (let column = 0; column < width; column++) {
          switch (cells[universe.get_index(row, column)]) {
            case wasm.Cell.Alive: {
              // Draw square here
              ctx.fillRect(column * dw, row * dh, dw, dh);
              break;
            }
            case wasm.Cell.Dead: {
              break;
            }
          }
        }
      }
      ctx.closePath();
    };

    // Define render
    const render = () => {
      // Clear
      ctx.clearRect(0, 0, cwidth, cheight);
      // Draw grid
      drawGrid();
      // Draw cells
      drawCells();
    };

    // Global var
    let MyGame = {
      frames: 0,
      running: false,
      stopMain: null,
    };

    // Add event listener
    const paint = (e: MouseEvent) => {
      // Calculate cell based off of offset x and offset y
      let row = e.offsetY / dh;
      let column = e.offsetX / dw;

      console.log(row, column);
      console.log(universe.get_index(row, column));
      console.log(universe.toggle_cell(universe.get_index(row, column)));
      
      render();
    };

    // Start game loop
    // (() => {
    const main = () => {
      MyGame.frames++;
      // 5x slower
      if (MyGame.frames > 5) {
        universe.tick();
        render();
        MyGame.frames = 0;
      }
      MyGame.stopMain = window.requestAnimationFrame(main);
    };

    canvas.addEventListener('click', paint);

    const startOrStop = () => {
      if (MyGame.running) {
        MyGame.running = false;
        window.cancelAnimationFrame(MyGame.stopMain);
      } else {
        MyGame.running = true;
        MyGame.stopMain = window.requestAnimationFrame(main);
      }
    };

    document
      .getElementById('start-or-stop')
      .addEventListener('click', startOrStop);


    MyGame.running = true;
    main();
    // })();
  };

  onMount(async () => {
    init(await rust, await memory);
  });
</script>

<main>
  <canvas id="canvas-wasm" />
  <button id="start-or-stop">start/stop</button>
</main>

<style lang="scss">
  * {
    margin: 0;
    padding: 0;
  }

  main {
    width: 100%;
    height: 100%;

    display: grid;
    justify-content: center;
    align-content: center;

    /* font-size: 1rem; */
  }
</style>
