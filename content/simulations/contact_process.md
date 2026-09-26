+++
title = "Contact Process"
date = 2026-09-13
description = "Directed-percolation contact process on a 2D lattice, compiled to WebAssembly."
+++

The contact process models activity spreading and dying out on a lattice: every site is either **Active** or **Inactive**. Unlike the Ising model, there is no Hamiltonian here — the dynamics are defined directly by per-site rates, not by an energy function.

Each attempt picks a random site $i$:

- if $i$ is Active, it heals to Inactive with probability $p$;
- if $i$ is Inactive, it adopts the current state of one uniformly random nearest neighbor — this is the only infection mechanism, there is no separate infection-probability parameter.

One *sweep* (one call to `step`) makes as many such attempts as there are sites.

<p style="font-size:0.85rem; color:#888;">This produces the classic directed-percolation phase transition: for large $p$ (fast healing), activity always dies out (an absorbing phase). For small $p$, activity can survive and spread indefinitely (a percolating phase). A critical $p_c$ separates the two — explore it with the slider below.</p>

This simulation runs on a **toroidal** lattice: the edges wrap around, so interactions cross the boundary seamlessly.

<div class="contact-wrap" style="display:flex; flex-direction:column; gap:0.8rem; font-family:'JetBrains Mono','Fira Code',monospace; color:#aaaaaa; font-size:0.9rem; line-height:1.3;">
    <div style="display:flex; flex-wrap:wrap; gap:0.5rem; align-items:center;">
        <button id="contact-play-pause" style="background:#111; color:#aaaaaa; border:1px solid #333; padding:0.3rem 0.9rem; font-family:inherit; font-size:1.1rem; cursor:pointer;">&#9654;</button>
        <button id="contact-randomize" style="background:#111; color:#aaaaaa; border:1px solid #333; padding:0.3rem 0.8rem; font-family:inherit; cursor:pointer;">Randomize</button>
        <button id="contact-seed" style="background:#111; color:#aaaaaa; border:1px solid #333; padding:0.3rem 0.8rem; font-family:inherit; cursor:pointer;">Seed center</button>
        <span style="color:#666; margin-left:auto;">P start/stop sampling &middot; R randomize &middot; S seed center</span>
    </div>
    <div class="contact-controls" style="display:grid; grid-template-columns:repeat(auto-fill, minmax(170px, 1fr)); gap:0.4rem 1.2rem;">
        <div>
            <p style="margin:0 0 0.2rem;"><span data-tooltip="Side length L of the L×L toroidal lattice. Changing it rebuilds the lattice and pauses." tabindex="0">Lattice size</span></p>
            <select id="contact-l-select" style="width:100%; background:#111; color:#aaaaaa; border:1px solid #333; font-family:inherit; padding:0.2rem;">
                <option value="32">32 &times; 32</option>
                <option value="64">64 &times; 64</option>
                <option value="128" selected>128 &times; 128</option>
                <option value="256">256 &times; 256</option>
            </select>
        </div>
        <div>
            <p style="margin:0 0 0.2rem;"><span data-tooltip="Chance an active site heals when picked. High values kill activity off; low values let it spread and persist." tabindex="0">p</span> = <output id="contact-p-value">0.50</output></p>
            <input type="range" id="contact-p-slider" min="0.01" max="1" step="0.01" value="0.5" style="width:100%;">
        </div>
    </div>
    <p style="color:#888; margin:0; display:flex; flex-wrap:wrap; gap:0.2rem 1.2rem;">
        <span><span data-tooltip="Fraction of sites currently active. Zero is absorbing: once activity dies out it never returns." tabindex="0">Active fraction</span> = <output id="contact-active-value">0.00</output></span>
    </p>
    <div style="display:flex; flex-wrap:wrap; gap:1rem; align-items:flex-start;">
        <div style="border:1px solid #333; background:#000; overflow:auto; max-width:100%;">
            <canvas id="contact-canvas" oncontextmenu="return false;"></canvas>
        </div>
        <div style="flex:1; min-width:280px;">
            <p style="font-size:0.85rem; margin:0 0 0.25rem;"><span data-tooltip="Active fraction per sweep, on a fixed 0 to 1 scale. It settles to a plateau when activity survives, or drops to 0 when it dies out." tabindex="0"><span style="color:#ff0055;">Active fraction</span></span></p>
            <div style="border:1px solid #333; background:#000;">
                <canvas id="contact-plot-active" style="display:block; width:100%; height:463px;"></canvas>
            </div>
        </div>
    </div>
</div>

<script type="module">
    import init, { ContactProcessModel } from '/wasm/contact_process.js';
    import { setUpGridCanvas, resizeGrid, drawGrid, ALIVE_COLOR } from '/js/grid-canvas.js';

    async function run() {
        try {
            const wasm = await init(); // `--target web` resolves init() to instance.exports

            const ACTIVE_COLOR = ALIVE_COLOR;
            // Canvas is drawn at 1px/site, then CSS-scaled to a fixed
            // on-screen size with `image-rendering: pixelated` — so
            // switching lattice length never changes the display footprint.
            const DISPLAY_SIZE = 480;
            // The plot is drawn at a fixed internal resolution matching the
            // plot column, and CSS-stretched when the layout stacks.
            const PLOT_WIDTH = 480;
            const PLOT_HEIGHT = 463;
            const PLOT_HISTORY = 300;

            const canvas = document.getElementById("contact-canvas");
            const ctx = setUpGridCanvas(canvas, DISPLAY_SIZE);

            const plotCanvas = document.getElementById("contact-plot-active");
            plotCanvas.width = PLOT_WIDTH;
            plotCanvas.height = PLOT_HEIGHT;
            const plotCtx = plotCanvas.getContext('2d');

            let model;
            let activeHistory = [];

            const buildModel = (length, p) => {
                model = ContactProcessModel.new(length, length, p);
                resizeGrid(canvas, length, length);
            };

            const drawCells = () => {
                const nrows = model.nrows();
                const ncols = model.ncols();
                const activePtr = model.active();
                // `--target web` glue doesn't export `memory` separately;
                // it's an own-property of the raw instance exports.
                const active = new Uint8Array(wasm.memory.buffer, activePtr, nrows * ncols);

                drawGrid(ctx, active, nrows, ncols);
            };

            const yForValue = (v) => PLOT_HEIGHT * (1 - v);

            const drawPlot = () => {
                plotCtx.fillStyle = "#000";
                plotCtx.fillRect(0, 0, PLOT_WIDTH, PLOT_HEIGHT);

                if (activeHistory.length < 2) return;
                plotCtx.strokeStyle = ACTIVE_COLOR;
                plotCtx.lineWidth = 1.5;
                plotCtx.beginPath();
                activeHistory.forEach((v, i) => {
                    const x = (i / (activeHistory.length - 1)) * PLOT_WIDTH;
                    const y = yForValue(v);
                    if (i === 0) plotCtx.moveTo(x, y);
                    else plotCtx.lineTo(x, y);
                });
                plotCtx.stroke();
            };

            const activeValue = document.getElementById("contact-active-value");
            const updateReadouts = () => {
                activeValue.textContent = model.active_fraction().toFixed(3);
            };

            const resetHistory = () => {
                activeHistory = [];
            };

            // Rolling window: drop the oldest sample once the buffer is full,
            // so the plot scrolls with a fixed-size history.
            const sampleHistory = () => {
                activeHistory.push(model.active_fraction());
                if (activeHistory.length > PLOT_HISTORY) activeHistory.shift();
            };

            let animationId = null;
            const isPaused = () => animationId === null;

            const renderLoop = () => {
                model.step();
                sampleHistory();
                drawCells();
                drawPlot();
                updateReadouts();
                animationId = requestAnimationFrame(renderLoop);
            };

            const playPauseButton = document.getElementById("contact-play-pause");

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

            const pSlider = document.getElementById("contact-p-slider");
            const pValue = document.getElementById("contact-p-value");
            pSlider.addEventListener("input", (event) => {
                pValue.textContent = event.target.value;
                model.set_healing_probability(parseFloat(pSlider.value));
            });

            const randomizeState = () => {
                model.randomize();
                resetHistory();
                sampleHistory();
                drawCells();
                drawPlot();
                updateReadouts();
            };

            document.getElementById("contact-randomize").addEventListener("click", randomizeState);

            const seedCenterState = () => {
                model.seed_center();
                resetHistory();
                sampleHistory();
                drawCells();
                drawPlot();
                updateReadouts();
            };

            document.getElementById("contact-seed").addEventListener("click", seedCenterState);

            const lSelect = document.getElementById("contact-l-select");
            lSelect.addEventListener("change", () => {
                pause();
                buildModel(parseInt(lSelect.value, 10), parseFloat(pSlider.value));
                resetHistory();
                sampleHistory();
                drawCells();
                drawPlot();
                updateReadouts();
            });

            document.addEventListener("keydown", (event) => {
                if (event.code === "KeyP") playOrPause();
                else if (event.code === "KeyR") randomizeState();
                else if (event.code === "KeyS") seedCenterState();
            });

            buildModel(parseInt(lSelect.value, 10), parseFloat(pSlider.value));
            pause();
            resetHistory();
            sampleHistory();
            drawCells();
            drawPlot();
            updateReadouts();

        } catch (e) {
            console.error("Failed to load Contact Process WASM simulation:", e);
            const canvas = document.getElementById("contact-canvas");
            if (canvas) {
                canvas.style.border = "2px solid red";
                canvas.style.background = "red";
            }
        }
    }

    run();
</script>
