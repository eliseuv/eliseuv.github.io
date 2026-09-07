+++
title = "Conway's Game of Life"
date = 2026-09-07
description = "Cellular automaton simulation compiled to WebAssembly, rendered on canvas."
+++

Conway's Game of Life, running as a Rust/WebAssembly cellular automaton on a 128x128 toroidal grid.

<div class="gol-wrap" style="display:flex; flex-wrap:wrap; gap:1.5rem; align-items:flex-start; font-family:'JetBrains Mono','Fira Code',monospace; color:#aaaaaa;">

    <div class="gol-controls" style="min-width:220px;">
        <button id="gol-play-pause" style="background:#111; color:#aaaaaa; border:1px solid #333; padding:0.5rem 1rem; font-family:inherit; font-size:1.2rem; cursor:pointer;">&#9654;</button>

        <p>Fill probability: p = <output id="gol-p-value">0.33</output></p>
        <input type="range" id="gol-p-slider" min="0" max="1" step="0.01" value="0.33" style="width:100%;">

        <p>
            <button id="gol-clear" style="background:#111; color:#aaaaaa; border:1px solid #333; padding:0.3rem 0.8rem; font-family:inherit; cursor:pointer;">Clear</button>
            <button id="gol-randomize" style="background:#111; color:#aaaaaa; border:1px solid #333; padding:0.3rem 0.8rem; font-family:inherit; cursor:pointer;">Randomize</button>
        </p>

        <ul style="color:#888; font-size:0.9rem; padding-left:1.2rem;">
            <li>P &mdash; play / pause</li>
            <li>C &mdash; clear</li>
            <li>R &mdash; randomize</li>
            <li>Click &mdash; toggle cell</li>
            <li>Ctrl+click &mdash; stamp glider</li>
            <li>Shift+click &mdash; stamp pulsar</li>
        </ul>
    </div>

    <div style="border:1px solid #333; background:#000; overflow:auto;">
        <canvas id="game-of-life-canvas" oncontextmenu="return false;"></canvas>
    </div>

</div>

<script type="module">
    import init, { Universe, Cell, Pattern } from '/wasm/game_of_life.js';

    async function run() {
        try {
            const wasm = await init(); // `--target web` resolves init() to instance.exports

            const nrows = 128;
            const ncols = 128;
            const universe = Universe.new(nrows, ncols);

            const CELL_SIZE = 4;
            const GRID_COLOR = "#222222";
            const ALIVE_COLOR = "#aaaaaa";
            const DEAD_COLOR = "#000000";

            const canvas = document.getElementById("game-of-life-canvas");
            canvas.height = (CELL_SIZE + 1) * nrows + 1;
            canvas.width = (CELL_SIZE + 1) * ncols + 1;
            const ctx = canvas.getContext('2d');

            const getIndex = (row, col) => row * ncols + col;

            const drawGrid = () => {
                ctx.beginPath();
                ctx.strokeStyle = GRID_COLOR;
                for (let i = 0; i <= ncols; i++) {
                    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
                    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * nrows + 1);
                }
                for (let j = 0; j <= nrows; j++) {
                    ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
                    ctx.lineTo((CELL_SIZE + 1) * ncols + 1, j * (CELL_SIZE + 1) + 1);
                }
                ctx.stroke();
            };

            const drawCells = () => {
                const statePtr = universe.state();
                // `--target web` glue doesn't export `memory` separately;
                // it's an own-property of the raw instance exports.
                const state = new Uint8Array(wasm.memory.buffer, statePtr, nrows * ncols);

                ctx.beginPath();
                for (let row = 0; row < nrows; row++) {
                    for (let col = 0; col < ncols; col++) {
                        const idx = getIndex(row, col);
                        ctx.fillStyle = state[idx] === Cell.Dead ? DEAD_COLOR : ALIVE_COLOR;
                        ctx.fillRect(col * (CELL_SIZE + 1) + 1, row * (CELL_SIZE + 1) + 1, CELL_SIZE, CELL_SIZE);
                    }
                }
                ctx.stroke();
            };

            let animationId = null;
            const isPaused = () => animationId === null;

            const renderLoop = () => {
                universe.tick();
                drawCells();
                animationId = requestAnimationFrame(renderLoop);
            };

            const playPauseButton = document.getElementById("gol-play-pause");

            const play = () => {
                playPauseButton.textContent = "⏸";
                renderLoop();
            };
            const pause = () => {
                playPauseButton.textContent = "▶";
                cancelAnimationFrame(animationId);
                animationId = null;
            };
            const playOrPause = () => isPaused() ? play() : pause();

            playPauseButton.addEventListener("click", playOrPause);

            const pSlider = document.getElementById("gol-p-slider");
            const pValue = document.getElementById("gol-p-value");
            pSlider.addEventListener("input", (event) => {
                pValue.textContent = event.target.value;
                universe.randomize(parseFloat(pSlider.value));
                drawCells();
            });

            canvas.addEventListener("click", (event) => {
                const rect = canvas.getBoundingClientRect();
                const scaleX = canvas.width / rect.width;
                const scaleY = canvas.height / rect.height;
                const canvasLeft = (event.clientX - rect.left) * scaleX;
                const canvasTop = (event.clientY - rect.top) * scaleY;
                const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), nrows - 1);
                const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), ncols - 1);

                if (event.ctrlKey) {
                    universe.add_pattern(Pattern.Glider, row, col);
                } else if (event.shiftKey) {
                    universe.add_pattern(Pattern.Pulsar, row, col);
                } else {
                    universe.toggle_cell(row, col);
                }
                drawCells();
            });

            const clearState = () => { universe.clear(); drawCells(); };
            const randomizeState = () => { universe.randomize(parseFloat(pSlider.value)); drawCells(); };

            document.getElementById("gol-clear").addEventListener("click", clearState);
            document.getElementById("gol-randomize").addEventListener("click", randomizeState);

            document.addEventListener("keydown", (event) => {
                if (event.code === "KeyP") playOrPause();
                else if (event.code === "KeyC") clearState();
                else if (event.code === "KeyR") randomizeState();
            });

            pause();
            randomizeState();
            drawGrid();

        } catch (e) {
            console.error("Failed to load Game of Life WASM simulation:", e);
            const canvas = document.getElementById("game-of-life-canvas");
            if (canvas) {
                canvas.style.border = "2px solid red";
                canvas.style.background = "red";
            }
        }
    }

    run();
</script>
