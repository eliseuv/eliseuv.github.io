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

<div class="ising-wrap" style="display:flex; flex-wrap:wrap; gap:1.5rem; align-items:flex-start; font-family:'JetBrains Mono','Fira Code',monospace; color:#aaaaaa;">
    <div style="border:1px solid #333; background:#000; overflow:auto;">
        <canvas id="ising-canvas" oncontextmenu="return false;"></canvas>
    </div>
    <div class="ising-controls" style="min-width:220px;">
        <button id="ising-play-pause" style="background:#111; color:#aaaaaa; border:1px solid #333; padding:0.5rem 1rem; font-family:inherit; font-size:1.2rem; cursor:pointer;">&#9654;</button>
        <p>Lattice size:
            <select id="ising-l-select" style="background:#111; color:#aaaaaa; border:1px solid #333; font-family:inherit; padding:0.2rem 0.4rem;">
                <option value="32">32 &times; 32</option>
                <option value="64">64 &times; 64</option>
                <option value="128" selected>128 &times; 128</option>
                <option value="256">256 &times; 256</option>
            </select>
        </p>
        <p>Temperature: <output id="ising-t-value">1.00</output> T<sub>c</sub> <span style="color:#666;">(T &approx; <output id="ising-t-abs">2.269</output>)</span></p>
        <input type="range" id="ising-t-slider" min="0.1" max="3" step="0.01" value="1" style="width:100%;">
        <p>
            <button id="ising-randomize" style="background:#111; color:#aaaaaa; border:1px solid #333; padding:0.3rem 0.8rem; font-family:inherit; cursor:pointer;">Randomize</button>
            <button id="ising-tc" style="background:#111; color:#aaaaaa; border:1px solid #333; padding:0.3rem 0.8rem; font-family:inherit; cursor:pointer;">Set T = Tc</button>
        </p>
        <p style="color:#888; font-size:0.9rem;">Magnetization: <output id="ising-m-value">0.00</output></p>
        <ul style="color:#888; font-size:0.9rem; padding-left:1.2rem;">
            <li>P &rarr; start/stop sampling</li>
            <li>R &rarr; randomize</li>
            <li>C &rarr; set T = T<sub>c</sub></li>
        </ul>
    </div>
</div>

<script type="module">
    import init, { IsingModel } from '/wasm/ising_2d.js';

    async function run() {
        try {
            const wasm = await init(); // `--target web` resolves init() to instance.exports

            const UP_COLOR = "#7aa2f7";
            const DOWN_COLOR = "#ff0055";
            // Onsager's exact critical temperature for the 2D square lattice.
            const T_CRITICAL = 2.269185314213022;
            // Canvas is drawn at 1px/site, then CSS-scaled to a fixed
            // on-screen size with `image-rendering: pixelated` — so
            // switching lattice length never changes the display footprint.
            const DISPLAY_SIZE = 512;

            const canvas = document.getElementById("ising-canvas");
            canvas.style.width = `${DISPLAY_SIZE}px`;
            canvas.style.height = `${DISPLAY_SIZE}px`;
            canvas.style.imageRendering = "pixelated";
            const ctx = canvas.getContext('2d');

            let model;

            const buildModel = (length, temperature) => {
                model = IsingModel.new(length, length, temperature);
                canvas.width = length;
                canvas.height = length;
            };

            const drawCells = () => {
                const nrows = model.nrows();
                const ncols = model.ncols();
                const spinsPtr = model.spins();
                // `--target web` glue doesn't export `memory` separately;
                // it's an own-property of the raw instance exports.
                const spins = new Int8Array(wasm.memory.buffer, spinsPtr, nrows * ncols);

                for (let row = 0; row < nrows; row++) {
                    for (let col = 0; col < ncols; col++) {
                        const idx = row * ncols + col;
                        ctx.fillStyle = spins[idx] > 0 ? UP_COLOR : DOWN_COLOR;
                        ctx.fillRect(col, row, 1, 1);
                    }
                }
            };

            const mValue = document.getElementById("ising-m-value");
            const updateReadouts = () => {
                mValue.textContent = model.magnetization().toFixed(3);
            };

            let animationId = null;
            const isPaused = () => animationId === null;

            const renderLoop = () => {
                model.step();
                drawCells();
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
                drawCells();
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
                drawCells();
                updateReadouts();
            });

            document.addEventListener("keydown", (event) => {
                if (event.code === "KeyP") playOrPause();
                else if (event.code === "KeyR") randomizeState();
                else if (event.code === "KeyC") setCriticalTemperature();
            });

            buildModel(parseInt(lSelect.value, 10), temperatureFromSlider());
            pause();
            drawCells();
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
