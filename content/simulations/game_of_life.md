+++
title = "Conway's Game of Life"
date = 2026-09-07
description = "Cellular automaton simulation compiled to WebAssembly, rendered on canvas."
+++

Conway's Game of Life is a zero-player cellular automaton: a grid of cells, each either **alive** or **dead**, that evolves in discrete steps according to a fixed rule applied to every cell in parallel. There is no player input during a run: the initial configuration alone determines everything that follows.

At each step, every cell looks at its $8$ neighbors (Moore neighborhood) and, writing $n$ for the number of live neighbors, updates according to four rules:

<div class="gol-rules" style="display:flex; flex-wrap:wrap; justify-content:center; gap:1.5rem; margin:1.5rem 0; font-family:'JetBrains Mono','Fira Code',monospace;">
    <div style="width:150px; text-align:center;">
        <svg width="60" height="60" viewBox="0 0 60 60">
            <rect x="0" y="0" width="18" height="18" fill="#000" stroke="#333"/>
            <rect x="21" y="0" width="18" height="18" fill="#aaaaaa" stroke="#333"/>
            <rect x="42" y="0" width="18" height="18" fill="#000" stroke="#333"/>
            <rect x="0" y="21" width="18" height="18" fill="#000" stroke="#333"/>
            <rect x="21" y="21" width="18" height="18" fill="#aaaaaa" stroke="#333"/>
            <rect x="42" y="21" width="18" height="18" fill="#000" stroke="#333"/>
            <rect x="0" y="42" width="18" height="18" fill="#000" stroke="#333"/>
            <rect x="21" y="42" width="18" height="18" fill="#000" stroke="#333"/>
            <rect x="42" y="42" width="18" height="18" fill="#000" stroke="#333"/>
            <rect x="21" y="21" width="18" height="18" fill="none" stroke="#ff0055" stroke-width="3"/>
        </svg>
        <p style="margin:0.6rem 0 0; font-size:0.95rem; color:#aaaaaa;"><strong>Underpopulation</strong></p>
        <p style="margin:0.2rem 0 0; font-size:0.85rem; color:#888;">$n \lt 2$ &rarr; dies</p>
    </div>
    <div style="width:150px; text-align:center;">
        <svg width="60" height="60" viewBox="0 0 60 60">
            <rect x="0" y="0" width="18" height="18" fill="#000" stroke="#333"/>
            <rect x="21" y="0" width="18" height="18" fill="#aaaaaa" stroke="#333"/>
            <rect x="42" y="0" width="18" height="18" fill="#000" stroke="#333"/>
            <rect x="0" y="21" width="18" height="18" fill="#aaaaaa" stroke="#333"/>
            <rect x="21" y="21" width="18" height="18" fill="#aaaaaa" stroke="#333"/>
            <rect x="42" y="21" width="18" height="18" fill="#000" stroke="#333"/>
            <rect x="0" y="42" width="18" height="18" fill="#000" stroke="#333"/>
            <rect x="21" y="42" width="18" height="18" fill="#000" stroke="#333"/>
            <rect x="42" y="42" width="18" height="18" fill="#000" stroke="#333"/>
            <rect x="21" y="21" width="18" height="18" fill="none" stroke="#7aa2f7" stroke-width="3"/>
        </svg>
        <p style="margin:0.6rem 0 0; font-size:0.95rem; color:#aaaaaa;"><strong>Survival</strong></p>
        <p style="margin:0.2rem 0 0; font-size:0.85rem; color:#888;">$2 \le n \le 3$ &rarr; stays alive</p>
    </div>
    <div style="width:150px; text-align:center;">
        <svg width="60" height="60" viewBox="0 0 60 60">
            <rect x="0" y="0" width="18" height="18" fill="#aaaaaa" stroke="#333"/>
            <rect x="21" y="0" width="18" height="18" fill="#aaaaaa" stroke="#333"/>
            <rect x="42" y="0" width="18" height="18" fill="#aaaaaa" stroke="#333"/>
            <rect x="0" y="21" width="18" height="18" fill="#aaaaaa" stroke="#333"/>
            <rect x="21" y="21" width="18" height="18" fill="#aaaaaa" stroke="#333"/>
            <rect x="42" y="21" width="18" height="18" fill="#000" stroke="#333"/>
            <rect x="0" y="42" width="18" height="18" fill="#000" stroke="#333"/>
            <rect x="21" y="42" width="18" height="18" fill="#000" stroke="#333"/>
            <rect x="42" y="42" width="18" height="18" fill="#000" stroke="#333"/>
            <rect x="21" y="21" width="18" height="18" fill="none" stroke="#ff0055" stroke-width="3"/>
        </svg>
        <p style="margin:0.6rem 0 0; font-size:0.95rem; color:#aaaaaa;"><strong>Overpopulation</strong></p>
        <p style="margin:0.2rem 0 0; font-size:0.85rem; color:#888;">$n \gt 3$ &rarr; dies</p>
    </div>
    <div style="width:150px; text-align:center;">
        <svg width="60" height="60" viewBox="0 0 60 60">
            <rect x="0" y="0" width="18" height="18" fill="#aaaaaa" stroke="#333"/>
            <rect x="21" y="0" width="18" height="18" fill="#aaaaaa" stroke="#333"/>
            <rect x="42" y="0" width="18" height="18" fill="#000" stroke="#333"/>
            <rect x="0" y="21" width="18" height="18" fill="#aaaaaa" stroke="#333"/>
            <rect x="21" y="21" width="18" height="18" fill="#000" stroke="#333"/>
            <rect x="42" y="21" width="18" height="18" fill="#000" stroke="#333"/>
            <rect x="0" y="42" width="18" height="18" fill="#000" stroke="#333"/>
            <rect x="21" y="42" width="18" height="18" fill="#000" stroke="#333"/>
            <rect x="42" y="42" width="18" height="18" fill="#000" stroke="#333"/>
            <rect x="21" y="21" width="18" height="18" fill="none" stroke="#7aa2f7" stroke-width="3"/>
        </svg>
        <p style="margin:0.6rem 0 0; font-size:0.95rem; color:#aaaaaa;"><strong>Birth</strong></p>
        <p style="margin:0.2rem 0 0; font-size:0.85rem; color:#888;">$n = 3$ &rarr; becomes alive</p>
    </div>
</div>

<p style="font-size:0.85rem; color:#888;">Gray cells are alive, black cells are dead; the highlighted cell is the one being updated, ringed <span style="color:#7aa2f7;">blue</span> if it is alive after the step or <span style="color:#ff0055;">magenta</span> if it is dead.</p>

This simulation runs on a $128 \times 128$ **toroidal** lattice: the edges wrap around, so a glider leaving the right side reappears on the left.

<div class="gol-wrap" style="display:flex; flex-wrap:wrap; gap:1.5rem; align-items:flex-start; font-family:'JetBrains Mono','Fira Code',monospace; color:#aaaaaa;">
    <div style="border:1px solid #333; background:#000; overflow:auto;">
        <canvas id="game-of-life-canvas" oncontextmenu="return false;"></canvas>
    </div>
    <div class="gol-controls" style="min-width:220px;">
        <button id="gol-play-pause" style="background:#111; color:#aaaaaa; border:1px solid #333; padding:0.5rem 1rem; font-family:inherit; font-size:1.2rem; cursor:pointer;">&#9654;</button>
        <p>Fill probability: <output id="gol-p-value">0.37</output></p>
        <input type="range" id="gol-p-slider" min="0" max="1" step="0.01" value="0.37" style="width:100%;">
        <p>
            <button id="gol-randomize" style="background:#111; color:#aaaaaa; border:1px solid #333; padding:0.3rem 0.8rem; font-family:inherit; cursor:pointer;">Randomize</button>
            <button id="gol-clear" style="background:#111; color:#aaaaaa; border:1px solid #333; padding:0.3rem 0.8rem; font-family:inherit; cursor:pointer;">Clear</button>
        </p>
        <ul style="color:#888; font-size:0.9rem; padding-left:1.2rem;">
            <li>P &rarr; start/stop dynamics</li>
            <li>C &rarr; clear lattice</li>
            <li>R &rarr; randomize</li>
            <li>Click &rarr; toggle cell</li>
            <li>Ctrl+Click &rarr; stamp glider</li>
            <li>Shift+Click &rarr; pulsar</li>
        </ul>
    </div>
</div>

<script type="module">
    import init, { Universe, Pattern } from '/wasm/game_of_life.js';
    import { setUpGridCanvas, resizeGrid, drawGrid, cellFromEvent as gridCellFromEvent } from '/js/grid-canvas.js';

    async function run() {
        try {
            const wasm = await init(); // `--target web` resolves init() to instance.exports

            const nrows = 128;
            const ncols = 128;
            const universe = Universe.new(nrows, ncols);

            // Canvas is drawn at 1px/site, then CSS-scaled to a fixed
            // on-screen size with `image-rendering: pixelated`.
            const DISPLAY_SIZE = 512;

            const canvas = document.getElementById("game-of-life-canvas");
            const ctx = setUpGridCanvas(canvas, DISPLAY_SIZE);
            resizeGrid(canvas, nrows, ncols);

            const drawCells = () => {
                const statePtr = universe.state();
                // `--target web` glue doesn't export `memory` separately;
                // it's an own-property of the raw instance exports.
                const state = new Uint8Array(wasm.memory.buffer, statePtr, nrows * ncols);

                drawGrid(ctx, state, nrows, ncols);
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

            const cellFromEvent = (event) => gridCellFromEvent(event, canvas, nrows, ncols);

            // Dragging paints (toggles) each newly-entered cell once, rather
            // than re-toggling on every mousemove within the same cell.
            let isDragging = false;
            let lastDragCell = null;

            canvas.addEventListener("mousedown", (event) => {
                const { row, col } = cellFromEvent(event);

                if (event.ctrlKey) {
                    universe.add_pattern(Pattern.Glider, row, col);
                    drawCells();
                    return;
                }
                if (event.shiftKey) {
                    universe.add_pattern(Pattern.Pulsar, row, col);
                    drawCells();
                    return;
                }

                universe.toggle_cell(row, col);
                drawCells();
                isDragging = true;
                lastDragCell = { row, col };
                event.preventDefault();
            });

            document.addEventListener("mousemove", (event) => {
                if (!isDragging) return;
                const { row, col } = cellFromEvent(event);
                if (lastDragCell && lastDragCell.row === row && lastDragCell.col === col) return;
                universe.toggle_cell(row, col);
                lastDragCell = { row, col };
                drawCells();
            });

            document.addEventListener("mouseup", () => {
                isDragging = false;
                lastDragCell = null;
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
