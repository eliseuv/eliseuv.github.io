+++
title = "2D Ising Model"
date = 2026-09-13
description = "Ferromagnetic Ising model on a 2D lattice, sampled by Metropolis Monte Carlo, compiled to WebAssembly."
+++

The Ising model places a spin $s_i \in \{+1, -1\}$ on every site of a lattice. Neighboring spins interact through the Hamiltonian

$$
H = -J \sum_{\langle i,j \rangle} s_i s_j,
$$

summed over nearest-neighbor bonds $\langle i,j \rangle$, with $J = 1$ favoring aligned neighbors (ferromagnetic coupling).

The system is sampled by single-spin-flip **Metropolis** Monte Carlo. Each attempt picks a random site $i$, computes the energy change a flip would cost,

$$
\Delta E = 2 s_i \sum_{j \in \text{nn}(i)} s_j,
$$

and accepts the flip with probability $\min(1, e^{-\beta \Delta E})$, where $\beta = 1/T$. One *sweep* (one call to `step`) makes as many such attempts as there are sites.

<p style="font-size:0.85rem; color:#888;">On the infinite 2D square lattice this model has an exact (Onsager) critical temperature $T_c = 2 / \ln(1 + \sqrt{2}) \approx 2.269$, separating an ordered ferromagnetic phase ($T < T_c$) from a disordered paramagnetic one ($T > T_c$).</p>

This simulation runs on a **toroidal** lattice: the edges wrap around, so interactions cross the boundary seamlessly.

<div class="ising-wrap" style="display:flex; flex-direction:column; gap:0.8rem; font-family:'JetBrains Mono','Fira Code',monospace; color:#aaaaaa; font-size:0.9rem; line-height:1.3;">
    <div style="display:flex; flex-wrap:wrap; gap:0.5rem; align-items:center;">
        <button id="ising-play-pause" style="background:#111; color:#aaaaaa; border:1px solid #333; padding:0.3rem 0.9rem; font-family:inherit; font-size:1.1rem; cursor:pointer;">&#9654;</button>
        <button id="ising-randomize" style="background:#111; color:#aaaaaa; border:1px solid #333; padding:0.3rem 0.8rem; font-family:inherit; cursor:pointer;">Randomize</button>
        <button id="ising-tc" style="background:#111; color:#aaaaaa; border:1px solid #333; padding:0.3rem 0.8rem; font-family:inherit; cursor:pointer;">Set T = Tc</button>
        <span style="color:#666; margin-left:auto;">P start/stop sampling &middot; R randomize &middot; C set T = T<sub>c</sub></span>
    </div>
    <div class="ising-controls" style="display:grid; grid-template-columns:repeat(auto-fill, minmax(170px, 1fr)); gap:0.4rem 1.2rem;">
        <div>
            <p style="margin:0 0 0.2rem;"><span data-tooltip="Side length L of the L×L toroidal lattice. Changing it rebuilds the lattice and pauses." tabindex="0">Lattice size</span></p>
            <select id="ising-l-select" style="width:100%; background:#111; color:#aaaaaa; border:1px solid #333; font-family:inherit; padding:0.2rem;">
                <option value="32">32 &times; 32</option>
                <option value="64">64 &times; 64</option>
                <option value="128" selected>128 &times; 128</option>
                <option value="256">256 &times; 256</option>
            </select>
        </div>
        <div>
            <p style="margin:0 0 0.2rem;"><span data-tooltip="Temperature in units of the critical temperature Tc. Below 1, spins order into large domains; above 1, thermal noise keeps them disordered." tabindex="0">T</span> = <output id="ising-t-value">1.00</output> T<sub>c</sub> <span style="color:#666;">(&approx; <output id="ising-t-abs">2.269</output>)</span></p>
            <input type="range" id="ising-t-slider" min="0.1" max="3" step="0.01" value="1" style="width:100%;">
        </div>
    </div>
    <p style="color:#888; margin:0; display:flex; flex-wrap:wrap; gap:0.2rem 1.2rem;">
        <span><span data-tooltip="Average spin, from −1 (all down) to +1 (all up). Near ±1 in the ordered phase, near 0 in the disordered one." tabindex="0">Magnetization</span> = <output id="ising-m-value">0.00</output></span>
        <span><span data-tooltip="Energy per site. −2 when every neighbor pair is aligned; rises toward 0 with disorder." tabindex="0">Energy</span> = <output id="ising-e-value">0.00</output></span>
    </p>
    <div style="display:flex; flex-wrap:wrap; gap:1rem; align-items:flex-start;">
        <div style="border:1px solid #333; background:#000; overflow:auto; max-width:100%;">
            <canvas id="ising-canvas" oncontextmenu="return false;"></canvas>
        </div>
        <div style="flex:1; min-width:280px; display:flex; flex-direction:column; gap:0.5rem;">
            <div>
                <p style="font-size:0.85rem; margin:0 0 0.25rem;"><span data-tooltip="Magnetization per sweep, on a fixed −1 to +1 scale. Large fluctuations near Tc signal the phase transition." tabindex="0">Magnetization</span></p>
                <div style="border:1px solid #333; background:#000;">
                    <canvas id="ising-plot-mag" style="display:block; width:100%; height:216px;"></canvas>
                </div>
            </div>
            <div>
                <p style="color:#ff0055; font-size:0.85rem; margin:0 0 0.25rem;"><span data-tooltip="Energy per site per sweep, on a fixed −2 to 0 scale. It relaxes to its equilibrium value after a temperature change." tabindex="0">Energy per site</span></p>
                <div style="border:1px solid #333; background:#000;">
                    <canvas id="ising-plot-energy" style="display:block; width:100%; height:216px;"></canvas>
                </div>
            </div>
        </div>
    </div>
</div>

<script type="module">
    import init, { IsingModel } from '/wasm/ising_2d.js';
    import { setUpGridCanvas, resizeGrid, drawGrid, ALIVE_COLOR } from '/js/grid-canvas.js';

    async function run() {
        try {
            const wasm = await init(); // `--target web` resolves init() to instance.exports

            const MAG_COLOR = ALIVE_COLOR;
            const ENERGY_COLOR = "#ff0055";
            // Onsager's exact critical temperature for the 2D square lattice.
            const T_CRITICAL = 2.269185314213022;
            // Canvas is drawn at 1px/site, then CSS-scaled to a fixed
            // on-screen size with `image-rendering: pixelated` — so
            // switching lattice length never changes the display footprint.
            const DISPLAY_SIZE = 480;
            // Each plot is drawn at a fixed internal resolution matching the
            // plot column, and CSS-stretched when the layout stacks.
            const PLOT_WIDTH = 480;
            const PLOT_HEIGHT = 216;
            const PLOT_HISTORY = 300;

            const canvas = document.getElementById("ising-canvas");
            const ctx = setUpGridCanvas(canvas, DISPLAY_SIZE);

            const setUpPlotCanvas = (id) => {
                const plotCanvas = document.getElementById(id);
                plotCanvas.width = PLOT_WIDTH;
                plotCanvas.height = PLOT_HEIGHT;
                return plotCanvas.getContext('2d');
            };
            const magCtx = setUpPlotCanvas("ising-plot-mag");
            const energyCtx = setUpPlotCanvas("ising-plot-energy");

            let model;
            let magHistory = [];
            let energyHistory = [];

            const buildModel = (length, temperature) => {
                model = IsingModel.new(length, length, temperature);
                resizeGrid(canvas, length, length);
            };

            const drawCells = () => {
                const nrows = model.nrows();
                const ncols = model.ncols();
                const spinsPtr = model.spins();
                // `--target web` glue doesn't export `memory` separately;
                // it's an own-property of the raw instance exports.
                const spins = new Int8Array(wasm.memory.buffer, spinsPtr, nrows * ncols);

                drawGrid(ctx, spins, nrows, ncols, { isAlive: (v) => v > 0 });
            };

            const yForValue = (v, vMin, vMax) => PLOT_HEIGHT * (1 - (v - vMin) / (vMax - vMin));

            const drawPanel = (panelCtx, history, color, vMin, vMax) => {
                panelCtx.fillStyle = "#000";
                panelCtx.fillRect(0, 0, PLOT_WIDTH, PLOT_HEIGHT);

                panelCtx.strokeStyle = "#333";
                panelCtx.beginPath();
                panelCtx.moveTo(0, yForValue(0, vMin, vMax));
                panelCtx.lineTo(PLOT_WIDTH, yForValue(0, vMin, vMax));
                panelCtx.stroke();

                if (history.length < 2) return;
                panelCtx.strokeStyle = color;
                panelCtx.lineWidth = 1.5;
                panelCtx.beginPath();
                history.forEach((v, i) => {
                    const x = (i / (history.length - 1)) * PLOT_WIDTH;
                    const y = yForValue(v, vMin, vMax);
                    if (i === 0) panelCtx.moveTo(x, y);
                    else panelCtx.lineTo(x, y);
                });
                panelCtx.stroke();
            };

            // Magnetization ranges over [-1, 1]. Per-site energy (J = 1) is
            // mathematically bounded by [-2, 2], but for this ferromagnetic
            // model equilibrium energy at any reachable temperature stays
            // in [-2, 0] — the all-anti-aligned state that reaches positive
            // energy is thermodynamically disfavored and never sampled.
            const drawPlots = () => {
                drawPanel(magCtx, magHistory, MAG_COLOR, -1, 1);
                drawPanel(energyCtx, energyHistory, ENERGY_COLOR, -2, 0);
            };

            const mValue = document.getElementById("ising-m-value");
            const eValue = document.getElementById("ising-e-value");
            const updateReadouts = () => {
                mValue.textContent = model.magnetization().toFixed(3);
                eValue.textContent = model.energy().toFixed(3);
            };

            const resetHistory = () => {
                magHistory = [];
                energyHistory = [];
            };

            // Rolling window: drop the oldest sample once the buffer is full,
            // so the plot scrolls with a fixed-size history.
            const sampleHistory = () => {
                magHistory.push(model.magnetization());
                energyHistory.push(model.energy());
                if (magHistory.length > PLOT_HISTORY) magHistory.shift();
                if (energyHistory.length > PLOT_HISTORY) energyHistory.shift();
            };

            let animationId = null;
            const isPaused = () => animationId === null;

            const renderLoop = () => {
                model.step();
                sampleHistory();
                drawCells();
                drawPlots();
                updateReadouts();
                animationId = requestAnimationFrame(renderLoop);
            };

            const playPauseButton = document.getElementById("ising-play-pause");

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

            const tSlider = document.getElementById("ising-t-slider");
            const tValue = document.getElementById("ising-t-value");
            const tAbs = document.getElementById("ising-t-abs");

            // The slider holds T in units of T_c; the model wants an
            // absolute temperature.
            const temperatureFromSlider = () => parseFloat(tSlider.value) * T_CRITICAL;

            tSlider.addEventListener("input", (event) => {
                tValue.textContent = event.target.value;
                const temperature = temperatureFromSlider();
                tAbs.textContent = temperature.toFixed(3);
                model.set_temperature(temperature);
            });

            const randomizeState = () => {
                model.randomize();
                resetHistory();
                sampleHistory();
                drawCells();
                drawPlots();
                updateReadouts();
            };

            document.getElementById("ising-randomize").addEventListener("click", randomizeState);

            const setCriticalTemperature = () => {
                tSlider.value = 1;
                tValue.textContent = "1.00";
                tAbs.textContent = T_CRITICAL.toFixed(3);
                model.set_temperature(T_CRITICAL);
            };

            document.getElementById("ising-tc").addEventListener("click", setCriticalTemperature);

            const lSelect = document.getElementById("ising-l-select");
            lSelect.addEventListener("change", () => {
                pause();
                buildModel(parseInt(lSelect.value, 10), temperatureFromSlider());
                resetHistory();
                sampleHistory();
                drawCells();
                drawPlots();
                updateReadouts();
            });

            document.addEventListener("keydown", (event) => {
                if (event.code === "KeyP") playOrPause();
                else if (event.code === "KeyR") randomizeState();
                else if (event.code === "KeyC") setCriticalTemperature();
            });

            buildModel(parseInt(lSelect.value, 10), temperatureFromSlider());
            pause();
            resetHistory();
            sampleHistory();
            drawCells();
            drawPlots();
            updateReadouts();

        } catch (e) {
            console.error("Failed to load Ising WASM simulation:", e);
            const canvas = document.getElementById("ising-canvas");
            if (canvas) {
                canvas.style.border = "2px solid red";
                canvas.style.background = "red";
            }
        }
    }

    run();
</script>
