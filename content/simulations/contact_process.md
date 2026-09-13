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

<div class="contact-wrap" style="display:flex; flex-wrap:wrap; gap:1.5rem; align-items:flex-start; font-family:'JetBrains Mono','Fira Code',monospace; color:#aaaaaa;">
    <div style="border:1px solid #333; background:#000; overflow:auto;">
        <canvas id="contact-canvas" oncontextmenu="return false;"></canvas>
    </div>
    <div class="contact-controls" style="min-width:220px;">
        <button id="contact-play-pause" style="background:#111; color:#aaaaaa; border:1px solid #333; padding:0.5rem 1rem; font-family:inherit; font-size:1.2rem; cursor:pointer;">&#9654;</button>
        <p>Lattice size:
            <select id="contact-l-select" style="background:#111; color:#aaaaaa; border:1px solid #333; font-family:inherit; padding:0.2rem 0.4rem;">
                <option value="32">32 &times; 32</option>
                <option value="64">64 &times; 64</option>
                <option value="128" selected>128 &times; 128</option>
                <option value="256">256 &times; 256</option>
            </select>
        </p>
        <p>Healing probability: <output id="contact-p-value">0.50</output></p>
        <input type="range" id="contact-p-slider" min="0.01" max="1" step="0.01" value="0.5" style="width:100%;">
        <p>
            <button id="contact-randomize" style="background:#111; color:#aaaaaa; border:1px solid #333; padding:0.3rem 0.8rem; font-family:inherit; cursor:pointer;">Randomize</button>
            <button id="contact-seed" style="background:#111; color:#aaaaaa; border:1px solid #333; padding:0.3rem 0.8rem; font-family:inherit; cursor:pointer;">Seed center</button>
        </p>
        <p style="color:#888; font-size:0.9rem;">Active fraction: <output id="contact-active-value">0.00</output></p>
        <ul style="color:#888; font-size:0.9rem; padding-left:1.2rem;">
            <li>P &rarr; start/stop sampling</li>
            <li>R &rarr; randomize</li>
            <li>S &rarr; seed center</li>
        </ul>
    </div>
</div>

<div style="display:flex; flex-direction:column; gap:1rem; margin-top:1.5rem; font-family:'JetBrains Mono','Fira Code',monospace;">
    <div>
        <p style="color:#7aa2f7; font-size:0.85rem; margin:0 0 0.25rem;">Active fraction</p>
        <div style="border:1px solid #333; background:#000;">
            <canvas id="contact-plot-active" style="display:block; width:100%; height:160px;"></canvas>
        </div>
    </div>
</div>

<script type="module">
    import init, { ContactProcessModel } from '/wasm/contact_process.js';

    async function run() {
        try {
            const wasm = await init(); // `--target web` resolves init() to instance.exports

            const ACTIVE_COLOR = "#7aa2f7";
            const INACTIVE_COLOR = "#000000";
            // Canvas is drawn at 1px/site, then CSS-scaled to a fixed
            // on-screen size with `image-rendering: pixelated` — so
            // switching lattice length never changes the display footprint.
            const DISPLAY_SIZE = 512;
            // The plot is drawn at a fixed, wide internal resolution and
            // CSS-stretched horizontally to fill the available width.
            const PLOT_WIDTH = 900;
            const PLOT_HEIGHT = 160;
            const PLOT_HISTORY = 300;

            const canvas = document.getElementById("contact-canvas");
            canvas.style.width = `${DISPLAY_SIZE}px`;
            canvas.style.height = `${DISPLAY_SIZE}px`;
            canvas.style.imageRendering = "pixelated";
            const ctx = canvas.getContext('2d');

            const plotCanvas = document.getElementById("contact-plot-active");
            plotCanvas.width = PLOT_WIDTH;
            plotCanvas.height = PLOT_HEIGHT;
            const plotCtx = plotCanvas.getContext('2d');

            let model;
            let activeHistory = [];

            const buildModel = (length, p) => {
                model = ContactProcessModel.new(length, length, p);
                canvas.width = length;
                canvas.height = length;
            };

            const drawCells = () => {
                const nrows = model.nrows();
                const ncols = model.ncols();
                const activePtr = model.active();
                // `--target web` glue doesn't export `memory` separately;
                // it's an own-property of the raw instance exports.
                const active = new Uint8Array(wasm.memory.buffer, activePtr, nrows * ncols);

                for (let row = 0; row < nrows; row++) {
                    for (let col = 0; col < ncols; col++) {
                        const idx = row * ncols + col;
                        ctx.fillStyle = active[idx] > 0 ? ACTIVE_COLOR : INACTIVE_COLOR;
                        ctx.fillRect(col, row, 1, 1);
                    }
                }
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
