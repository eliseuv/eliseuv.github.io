+++
title = "Spectral Signatures of Criticality"
date = 2026-09-26
description = "Random matrix analysis of time series from many independent copies of a system: the spectrum of their correlation matrix locates the phase transitions of the Ising model and the contact process. Compiled to WebAssembly."

[extra.symbols]
'N' = 'Number of time series, i.e. of independent copies (samples) of the system. $N = 100$ here.'
'n' = 'Time steps per series after the initial measurement, so each series has $n + 1$ values.'
'M' = 'Time series matrix: row $i$ holds the series of sample $i$, column $t$ its value at time step $t$.'
'm_{it}' = 'Value of the series of sample $i$ at time step $t$: magnetization for the Ising model, density of active sites for the contact process.'
'm^\ast_{it}' = 'Standardized value: the series of sample $i$ shifted to zero mean and scaled to unit variance over time.'
'\sigma_i' = 'Standard deviation over time of the series of sample $i$.'
'G^\ast' = 'Correlation matrix of the standardized series, $N \times N$. Entry $g^\ast_{ij}$ is the Pearson correlation between samples $i$ and $j$.'
'g^\ast_{ij}' = 'Pearson correlation between the series of samples $i$ and $j$, from $-1$ to $+1$. Diagonal entries are exactly $1$.'
'\lambda' = 'An eigenvalue of $G^\ast$. All are non-negative, since $G^\ast$ is a correlation matrix.'
'\lambda_{\max}' = 'Largest eigenvalue of $G^\ast$. Approaches $N$ when every series follows one common signal.'
'\lambda_\pm' = 'Edges of the Marchenko-Pastur law: $(1 \pm \sqrt{q})^2$.'
'q' = 'Aspect ratio $N/(n+1)$ of the time series matrix. Sets the width of the Marchenko-Pastur law.'
'\rho' = 'Prescribed correlation between the two series of each pair in the toy model.'
'T' = 'Temperature of the Ising model, in units where $k_B = J = 1$.'
'T_c' = 'Critical temperature of the infinite square lattice Ising model, $2/\ln(1+\sqrt{2}) \approx 2.269$ (Onsager).'
'L' = 'Linear size of the lattice, in sites: $L \times L$ for the Ising model, a ring of $L$ sites for the contact process.'
'\alpha' = 'Infection rate of the contact process, relative to a recovery rate of one.'
'\alpha_c' = 'Critical infection rate of the one-dimensional contact process without diffusion, $\approx 3.29785$. Below it activity always dies out.'
'\gamma' = 'Diffusion probability: chance per attempt that a site swaps its state with a random neighbor.'
'\rho_a' = 'Density of active sites of the contact process, from $0$ (absorbed) to $1$ (fully active).'
+++

A phase transition is a collective change: at the critical point, fluctuations are correlated over the whole system and relax slowly. This page shows a way to detect it without measuring any correlation length directly. Run $N$ independent copies of the system, record one global quantity of each over time, and ask how *similar* those $N$ time series are to each other. The answer is read from the spectrum of their correlation matrix, compared against what random matrix theory predicts for series that are not correlated at all.

This is the method of my PhD thesis, *Random matrices approaches for correlated time series: statistical physics and other applications*, and the simulations run on the same code, compiled to WebAssembly.

## From time series to a spectrum

Each of the $N$ samples starts from its own random initial state and produces a series $m_{it}$, $t = 0, \dots, n$. Stacking them as rows gives the $N \times (n+1)$ time series matrix $M$. Every series is standardized over time,

$$
m^\ast_{it} = \frac{m_{it} - \langle m_i \rangle}{\sigma_i},
$$

and the correlation matrix is

$$
G^\ast = \frac{1}{n+1} M^\ast M^{\ast\mathsf{T}}, \qquad g^\ast_{ij} = \frac{1}{n+1} \sum_{t=0}^{n} m^\ast_{it}\\, m^\ast_{jt},
$$

so $g^\ast_{ij}$ is the Pearson correlation between samples $i$ and $j$. Its eigenvalues $\lambda$ summarize all $N(N-1)/2$ correlations at once.

If the series are independent noise, the eigenvalue density does not collapse onto $\lambda = 1$ as one might expect: finite series have spurious correlations of order $1/\sqrt{n+1}$, and they spread the spectrum over the **Marchenko-Pastur** law

$$
\sigma(\lambda) = \frac{\sqrt{(\lambda_+ - \lambda)(\lambda - \lambda_-)}}{2\pi q \lambda}, \qquad \lambda_\pm = (1 \pm \sqrt{q})^2, \qquad q = \frac{N}{n+1}.
$$

Deviations from it are genuine correlations.

## What the spectrum can and cannot tell

Two exact identities fix what is worth measuring. Standardization makes every diagonal entry $g^\ast_{ii} = 1$, so the eigenvalues always sum to $\operatorname{Tr} G^\ast = N$ and their **mean is exactly one**, whatever the system does. The mean carries no information.

The **variance** of the eigenvalues does, and it has a direct meaning:

$$
\operatorname{var}(\lambda) = \frac{1}{N} \operatorname{Tr} {G^\ast}^2 - 1 = \frac{1}{N} \sum_{i \neq j} {g^\ast_{ij}}^2 = (N - 1)\\, \big\langle {g^\ast_{ij}}^2 \big\rangle_{i \neq j},
$$

the mean squared correlation between different samples, scaled by $N - 1$. Uncorrelated series give the Marchenko-Pastur value $q$; series that all move together push it towards $N - 1$.

The **largest eigenvalue** $\lambda_{\max}$ measures how much of the ensemble follows one common signal. When every series is (up to its sign) the same, $G^\ast$ has rank one: $\lambda_{\max} \to N$ and every other eigenvalue drops to zero, opening a gap in the spectrum.

## The sources

- **White noise**: independent Gaussian values. The spectrum settles on the Marchenko-Pastur law.
- **Correlated pairs**: a toy model with a knob. Samples come in pairs whose series have correlation $\rho$, built from two independent Gaussian series $\varphi_1, \varphi_2$ as $\varphi_1 \sin\theta + \varphi_2 \cos\theta$ and $\varphi_1 \cos\theta + \varphi_2 \sin\theta$ with $\sin 2\theta = \rho$. Different pairs are independent. As $\rho$ grows the spectrum splits into two bulks around $1 \pm \rho$.
- **Ising model** on a periodic $L \times L$ lattice with $L = 32$, heat bath dynamics, series of the magnetization over $n = 300$ sweeps from random configurations. The temperature $T$ is set relative to $T_c$.
- **Contact process** on a ring of $L = 128$ sites, series of the density of active sites $\rho_a$ over $n = 500$ steps from fully active chains, with infection rate $\alpha$ and diffusion probability $\gamma$. It has an absorbing state and no Hamiltonian, and belongs to the directed percolation universality class rather than Ising's.

## What the reference scans show

In both models the ordered side has every sample following the same trajectory, so the correlations are strong, $\lambda_{\max}$ approaches $N$ and $\operatorname{var}(\lambda)$ approaches $N - 1$. For the Ising model that trajectory is ordering into one of two domains, up or down, so correlations are near $\pm 1$. For the contact process it is the common decay into the absorbing state, below $\alpha_c$. Across the transition both statistics collapse, and the critical point sits where they change fastest:

- **Ising model**: $\operatorname{var}(\lambda)$ falls steepest, its inflection point, at $T = T_c$ within the $0.02\\,T_c$ spacing of the scan. $\langle \lambda_{\max} \rangle$ has its inflection just above, between $T_c$ and $1.02\\,T_c$.
- **Contact process** without diffusion: $\langle \lambda_{\max} \rangle$ has its inflection and $\operatorname{var}(\lambda_{\max})$ its maximum at $\alpha \approx 3.15$, and $\operatorname{var}(\lambda)$ its inflection at $\alpha \approx 3.10$. That is about $5\\%$ below $\alpha_c \approx 3.298$ at these sizes. With diffusion the same features move down together, to $\alpha \approx 2.55$ for $\gamma = 0.5$ and $\alpha \approx 2.35$ for $\gamma = 1$, following the known decrease of the critical rate with diffusion. For $\gamma > 0$ the dashed critical line is the thesis' fit $\alpha_c = 2.33\\,(\gamma + 0.16)^{-0.19}$ of the spectral and power law estimates, not an independent value.

On the other side of the transition neither model reaches the Marchenko-Pastur law: the series are short and remember their initial state, so they stay autocorrelated and $\operatorname{var}(\lambda)$ remains well above $q$.

Each change of source or parameter starts a new ensemble. The time series matrix fills in as the samples evolve; when it is complete its correlation matrix is diagonalized, its eigenvalues are added to the histogram, and a new matrix begins. The spectral statistics at every parameter value you visit are kept as points on the scan plots, over reference curves precomputed with the same sizes and $1000$ matrices per point.

<div class="spectral-wrap" style="display:flex; flex-direction:column; gap:0.8rem; font-family:'JetBrains Mono','Fira Code',monospace; color:#aaaaaa; font-size:0.9rem; line-height:1.3;">
    <div style="display:flex; flex-wrap:wrap; gap:0.5rem; align-items:center;">
        <button id="spectral-play-pause" style="background:#111; color:#aaaaaa; border:1px solid #333; padding:0.3rem 0.9rem; font-family:inherit; font-size:1.1rem; cursor:pointer;">&#9654;</button>
        <button id="spectral-reset" style="background:#111; color:#aaaaaa; border:1px solid #333; padding:0.3rem 0.8rem; font-family:inherit; cursor:pointer;">Reset</button>
        <button id="spectral-critical" style="background:#111; color:#aaaaaa; border:1px solid #333; padding:0.3rem 0.8rem; font-family:inherit; cursor:pointer;">Set critical</button>
        <span style="color:#666; margin-left:auto;">P start/stop &middot; R reset &middot; C set critical</span>
    </div>
    <div style="display:grid; grid-template-columns:repeat(auto-fill, minmax(170px, 1fr)); gap:0.4rem 1.2rem;">
        <div>
            <p style="margin:0 0 0.2rem;">Source</p>
            <select id="spectral-source" style="width:100%; background:#111; color:#aaaaaa; border:1px solid #333; font-family:inherit; padding:0.2rem;">
                <option value="noise">White noise</option>
                <option value="pairs">Correlated pairs</option>
                <option value="ising" selected>Ising model</option>
                <option value="contact">Contact process</option>
            </select>
        </div>
        <div>
            <p style="margin:0 0 0.2rem;">Speed</p>
            <select id="spectral-speed" style="width:100%; background:#111; color:#aaaaaa; border:1px solid #333; font-family:inherit; padding:0.2rem;">
                <option value="1">1 step / frame</option>
                <option value="10" selected>10 steps / frame</option>
                <option value="max">As fast as possible</option>
            </select>
        </div>
        <div data-source="pairs">
            <p style="margin:0 0 0.2rem;"><span data-sym="\rho" tabindex="0">&rho;</span> = <output id="spectral-rho-value">0.50</output></p>
            <input type="range" id="spectral-rho" min="0" max="1" step="0.05" value="0.5" style="width:100%;">
        </div>
        <div data-source="ising">
            <p style="margin:0 0 0.2rem;"><span data-sym="T" tabindex="0">T</span> = <output id="spectral-t-value">1.00</output> <span data-sym="T_c" tabindex="0">T<sub>c</sub></span></p>
            <input type="range" id="spectral-t" min="0.6" max="1.4" step="0.02" value="1" style="width:100%;">
        </div>
        <div data-source="contact">
            <p style="margin:0 0 0.2rem;"><span data-sym="\alpha" tabindex="0">&alpha;</span> = <output id="spectral-alpha-value">3.30</output></p>
            <input type="range" id="spectral-alpha" min="2" max="4" step="0.05" value="3.3" style="width:100%;">
        </div>
        <div data-source="contact">
            <p style="margin:0 0 0.2rem;"><span data-sym="\gamma" tabindex="0">&gamma;</span></p>
            <select id="spectral-gamma" style="width:100%; background:#111; color:#aaaaaa; border:1px solid #333; font-family:inherit; padding:0.2rem;">
                <option value="0" selected>0</option>
                <option value="0.5">0.5</option>
                <option value="1">1</option>
            </select>
        </div>
    </div>
    <p style="color:#888; margin:0; display:flex; flex-wrap:wrap; gap:0.2rem 1.2rem;">
        <span><span data-tooltip="Correlation matrices completed and diagonalized at the current parameters." tabindex="0">Matrices</span> = <output id="spectral-matrices">0</output></span>
        <span><span data-tooltip="Variance of all accumulated eigenvalues: N − 1 times the mean squared correlation between different samples." tabindex="0">var(&lambda;)</span> = <output id="spectral-var">–</output></span>
        <span><span data-tooltip="Largest eigenvalue, averaged over the accumulated matrices. At most N = 100." tabindex="0">&lang;&lambda;<sub>max</sub>&rang;</span> = <output id="spectral-max">–</output></span>
        <span><span data-tooltip="Eigenvalue variance of the Marchenko-Pastur law, the value for uncorrelated series." tabindex="0">MP var(&lambda;)</span> = <output id="spectral-mp-var">–</output></span>
    </p>
    <div style="display:flex; flex-wrap:wrap; gap:1rem; align-items:flex-start;">
        <div id="spectral-lattice-panel" style="display:flex; flex-direction:column; gap:0.25rem;">
            <p id="spectral-lattice-title" style="font-size:0.85rem; margin:0;">Sample 1</p>
            <div style="border:1px solid #333; background:#000; max-width:100%;">
                <canvas id="spectral-lattice"></canvas>
            </div>
        </div>
        <div style="flex:1; min-width:280px; display:flex; flex-direction:column; gap:0.25rem;">
            <p style="font-size:0.85rem; margin:0;"><span data-tooltip="Row i is the series of sample i, left to right in time; the current matrix fills in as the samples evolve. Magenta is positive, cyan negative." tabindex="0">Time series matrix M</span> <span style="color:#666;">(t = <output id="spectral-t-step">0</output>)</span></p>
            <div style="border:1px solid #333; background:#000;">
                <canvas id="spectral-series" style="display:block; width:100%; height:200px; image-rendering:pixelated;"></canvas>
            </div>
        </div>
    </div>
    <div style="display:flex; flex-wrap:wrap; gap:1rem; align-items:flex-start;">
        <div style="display:flex; flex-direction:column; gap:0.25rem;">
            <p style="font-size:0.85rem; margin:0;"><span data-tooltip="Correlations g*_ij of the last completed matrix. Magenta is +1, cyan −1, black 0; the diagonal is always +1." tabindex="0">Correlation matrix G*</span></p>
            <div style="border:1px solid #333; background:#000; max-width:100%;">
                <canvas id="spectral-correlations"></canvas>
            </div>
            <label style="font-size:0.8rem; color:#888;"><input type="checkbox" id="spectral-sort"> sort by correlation with sample 1</label>
        </div>
        <div style="flex:1; min-width:280px; display:flex; flex-direction:column; gap:0.25rem;">
            <p style="font-size:0.85rem; margin:0;"><span data-tooltip="Distribution of the off-diagonal correlations of every accumulated matrix (bars), with the precomputed reference at the nearest parameter value (gray line)." tabindex="0">Correlations between samples</span></p>
            <div style="border:1px solid #333; background:#000;">
                <canvas id="spectral-correlation-hist" style="display:block; width:100%; height:200px;"></canvas>
            </div>
        </div>
    </div>
    <div style="display:flex; flex-direction:column; gap:0.25rem;">
        <p style="font-size:0.85rem; margin:0;"><span data-tooltip="Density of every accumulated eigenvalue (bars), the Marchenko-Pastur law of uncorrelated series (red) and the precomputed reference at the nearest parameter value (gray)." tabindex="0">Eigenvalue density</span> <label style="font-size:0.8rem; color:#888; margin-left:1rem;"><input type="checkbox" id="spectral-log"> logarithmic &lambda; axis</label></p>
        <div style="border:1px solid #333; background:#000;">
            <canvas id="spectral-spectrum" style="display:block; width:100%; height:240px;"></canvas>
        </div>
    </div>
    <div id="spectral-scans" style="display:flex; flex-wrap:wrap; gap:1rem; align-items:flex-start;">
        <div style="flex:1; min-width:280px; display:flex; flex-direction:column; gap:0.25rem;">
            <p style="font-size:0.85rem; margin:0;"><span data-tooltip="Eigenvalue variance against the control parameter: reference scan (gray line) and the parameter values visited live (magenta points). The dashed line marks the critical value." tabindex="0">var(&lambda;)</span></p>
            <div style="border:1px solid #333; background:#000;">
                <canvas id="spectral-scan-var" style="display:block; width:100%; height:200px;"></canvas>
            </div>
        </div>
        <div style="flex:1; min-width:280px; display:flex; flex-direction:column; gap:0.25rem;">
            <p style="font-size:0.85rem; margin:0;"><span data-tooltip="Average largest eigenvalue against the control parameter: reference scan (gray line) and the parameter values visited live (magenta points)." tabindex="0">&lang;&lambda;<sub>max</sub>&rang;</span></p>
            <div style="border:1px solid #333; background:#000;">
                <canvas id="spectral-scan-max" style="display:block; width:100%; height:200px;"></canvas>
            </div>
        </div>
        <div style="flex:1; min-width:280px; display:flex; flex-direction:column; gap:0.25rem;">
            <p style="font-size:0.85rem; margin:0;"><span data-tooltip="Variance of the largest eigenvalue over the matrices, against the control parameter: reference scan (gray line) and the parameter values visited live (magenta points)." tabindex="0">var(&lambda;<sub>max</sub>)</span></p>
            <div style="border:1px solid #333; background:#000;">
                <canvas id="spectral-scan-max-var" style="display:block; width:100%; height:200px;"></canvas>
            </div>
        </div>
    </div>
</div>

<script type="module">
    import init, {
        SpectralLab,
        marchenko_pastur_density,
        decode_reference,
        contact_process_alpha_critical,
    } from '/wasm/spectral_criticality.js';
    import { setUpGridCanvas, resizeGrid, drawGrid } from '/js/grid-canvas.js';

    async function run() {
        try {
            const wasm = await init(); // `--target web` resolves init() to instance.exports

            const POSITIVE = [255, 0, 85];   // #ff0055
            const NEGATIVE = [0, 183, 255];  // #00b7ff
            const BAR_COLOR = "#aaaaaa";
            const LIVE_COLOR = "#ff0055";
            const REFERENCE_COLOR = "#777777";
            const MP_COLOR = "#ff3333";
            const AXIS_COLOR = "#333333";
            const LABEL_COLOR = "#888888";
            const SPECTRUM_BINS = 80;
            // Eigenvalues below this are drawn in the first bin of the logarithmic axis;
            // absorbed or perfectly ordered samples give eigenvalues that are zero up to rounding.
            const LOG_LAMBDA_MIN = 1e-3;
            // Budget per frame for the fastest speed, leaving time for drawing.
            const FRAME_BUDGET_MS = 12;

            // `scale` maps a raw series value to [-1, 1] for the diverging colors.
            const SOURCES = {
                noise: {
                    make: () => SpectralLab.white_noise(),
                    key: () => "noise",
                    scale: (v) => v / 3,
                    logAxis: false,
                },
                pairs: {
                    make: (p) => SpectralLab.correlated_pairs(p.rho),
                    key: (p) => p.rho.toFixed(2),
                    x: (p) => p.rho,
                    xLabel: "ρ",
                    xRange: [0, 1],
                    scale: (v) => v / 3,
                    logAxis: false,
                },
                ising: {
                    make: (p) => SpectralLab.ising(p.t),
                    key: (p) => p.t.toFixed(2),
                    x: (p) => p.t,
                    xLabel: "T/Tc",
                    xRange: [0.6, 1.4],
                    critical: () => 1,
                    scale: (v) => v,
                    logAxis: true,
                    reference: () => referenceScans.ising?.[0],
                },
                contact: {
                    make: (p) => SpectralLab.contact_process(p.alpha, p.gamma),
                    key: (p) => `${p.gamma}:${p.alpha.toFixed(2)}`,
                    x: (p) => p.alpha,
                    xLabel: "α",
                    xRange: [2, 4],
                    critical: (p) => contactProcessAlphaCritical(p.gamma),
                    scale: (v) => v,
                    logAxis: true,
                    reference: (p) => referenceScans.contact?.find(
                        (scan) => scan.fixed.some(([name, value]) => name === "gamma" && value === p.gamma)),
                },
            };

            // Without diffusion, the best known estimate; with it, the thesis' fit
            // α_c = a (γ - b)^c of both the spectral and the power law estimates.
            const contactProcessAlphaCritical = (gamma) =>
                gamma === 0 ? contact_process_alpha_critical() : 2.33 * (gamma + 0.16) ** -0.19;

            const referenceScans = {};
            const loadReference = async (name, file) => {
                try {
                    const response = await fetch(`/data/spectral/${file}`);
                    if (!response.ok) throw new Error(`HTTP ${response.status}`);
                    const bytes = new Uint8Array(await response.arrayBuffer());
                    referenceScans[name] = JSON.parse(decode_reference(bytes));
                } catch (e) {
                    console.warn(`Spectral reference ${file} unavailable:`, e);
                }
            };
            await Promise.all([
                loadReference("ising", "ising_2d.cbor.gz"),
                loadReference("contact", "contact_process_1d.cbor.gz"),
            ]);

            const el = (id) => document.getElementById(id);
            const sourceSelect = el("spectral-source");
            const speedSelect = el("spectral-speed");
            const rhoSlider = el("spectral-rho");
            const tSlider = el("spectral-t");
            const alphaSlider = el("spectral-alpha");
            const gammaSelect = el("spectral-gamma");
            const logCheckbox = el("spectral-log");
            const sortCheckbox = el("spectral-sort");

            const params = () => ({
                rho: parseFloat(rhoSlider.value),
                t: parseFloat(tSlider.value),
                alpha: parseFloat(alphaSlider.value),
                gamma: parseFloat(gammaSelect.value),
            });

            let sourceName;
            let source;
            let lab;
            // Live scan points, per source and fixed parameters: key -> {x, variance, maxMean}.
            const livePoints = {};
            const liveScanKey = () => sourceName === "contact" ? `contact:${params().gamma}` : sourceName;

            // ---- Canvases ---------------------------------------------------------------

            // Plots are drawn at their displayed size, so text is not stretched; they are
            // resized (and redrawn) with the window.
            const plotCanvases = [];
            const setUpPlot = (id) => {
                const canvas = el(id);
                plotCanvases.push(canvas);
                return canvas.getContext("2d");
            };
            const resizePlots = () => {
                for (const canvas of plotCanvases) {
                    canvas.width = Math.max(1, canvas.clientWidth);
                    canvas.height = parseInt(canvas.style.height, 10);
                }
            };
            const correlationHistCtx = setUpPlot("spectral-correlation-hist");
            const spectrumCtx = setUpPlot("spectral-spectrum");
            const scanVarCtx = setUpPlot("spectral-scan-var");
            const scanMaxCtx = setUpPlot("spectral-scan-max");
            const scanMaxVarCtx = setUpPlot("spectral-scan-max-var");

            const latticeCanvas = el("spectral-lattice");
            const latticeCtx = setUpGridCanvas(latticeCanvas, 256);
            const correlationsCanvas = el("spectral-correlations");
            const correlationsCtx = setUpGridCanvas(correlationsCanvas, 256);
            const seriesCanvas = el("spectral-series");
            const seriesCtx = seriesCanvas.getContext("2d");

            // Space-time diagram of the first contact process sample, one row per step.
            let spaceTime = null;

            const diverging = (u, pixels, offset) => {
                const s = Math.max(-1, Math.min(1, u));
                const [r, g, b] = s >= 0 ? POSITIVE : NEGATIVE;
                const a = Math.abs(s);
                pixels[offset] = r * a;
                pixels[offset + 1] = g * a;
                pixels[offset + 2] = b * a;
                pixels[offset + 3] = 255;
            };

            const f64View = (ptr, len) => new Float64Array(wasm.memory.buffer, ptr, len);

            // ---- Drawing: matrices and lattices -------------------------------------------

            const drawSeries = () => {
                const nSamples = lab.n_samples();
                const nCols = lab.n_steps() + 1;
                const values = f64View(lab.series(), nSamples * nCols);
                const image = seriesCtx.createImageData(nCols, nSamples);
                const filled = lab.t();
                for (let i = 0; i < nSamples; i++) {
                    for (let t = 0; t <= filled; t++) {
                        diverging(source.scale(values[i * nCols + t]), image.data, 4 * (i * nCols + t));
                    }
                }
                seriesCtx.putImageData(image, 0, 0);
                el("spectral-t-step").textContent = filled;
            };

            const sortedOrder = (g, n) => {
                const order = [...Array(n).keys()];
                if (sortCheckbox.checked) order.sort((a, b) => g[b] - g[a]);
                return order;
            };

            const drawCorrelations = () => {
                const n = lab.n_samples();
                const image = correlationsCtx.createImageData(n, n);
                if (lab.n_matrices() > 0) {
                    const g = f64View(lab.correlations(), n * n);
                    const order = sortedOrder(g, n);
                    for (let i = 0; i < n; i++) {
                        for (let j = 0; j < n; j++) {
                            diverging(g[order[i] * n + order[j]], image.data, 4 * (i * n + j));
                        }
                    }
                } else {
                    for (let k = 3; k < image.data.length; k += 4) image.data[k] = 255;
                }
                correlationsCtx.putImageData(image, 0, 0);
            };

            const drawLattice = () => {
                if (sourceName === "ising") {
                    const rows = lab.lattice_rows();
                    const cols = lab.lattice_cols();
                    const spins = new Int8Array(wasm.memory.buffer, lab.sites(0), rows * cols);
                    drawGrid(latticeCtx, spins, rows, cols, { isAlive: (v) => v > 0 });
                } else if (sourceName === "contact") {
                    latticeCtx.putImageData(spaceTime, 0, 0);
                }
            };

            // Record the current state of the first contact process sample as row `t`.
            const recordSpaceTimeRow = () => {
                const cols = lab.lattice_cols();
                const sites = new Uint8Array(wasm.memory.buffer, lab.sites(0), cols);
                const row = lab.t();
                for (let k = 0; k < cols; k++) {
                    const offset = 4 * (row * cols + k);
                    const value = sites[k] ? 170 : 0;
                    spaceTime.data[offset] = value;
                    spaceTime.data[offset + 1] = value;
                    spaceTime.data[offset + 2] = value;
                    spaceTime.data[offset + 3] = 255;
                }
            };

            const clearSpaceTime = () => {
                spaceTime.data.fill(0);
                for (let k = 3; k < spaceTime.data.length; k += 4) spaceTime.data[k] = 255;
            };

            // ---- Drawing: plots ----------------------------------------------------------

            const MARGIN = { left: 48, right: 10, top: 10, bottom: 24 };

            // Frame with axes and tick labels; returns coordinate maps.
            const frame = (ctx, [x0, x1], [y0, y1], { xTicks, yTicks, xFormat = (v) => v, yFormat = (v) => v }) => {
                const { width, height } = ctx.canvas;
                ctx.fillStyle = "#000";
                ctx.fillRect(0, 0, width, height);
                const px = (x) => MARGIN.left + (x - x0) / (x1 - x0) * (width - MARGIN.left - MARGIN.right);
                const py = (y) => height - MARGIN.bottom - (y - y0) / (y1 - y0) * (height - MARGIN.top - MARGIN.bottom);
                ctx.strokeStyle = AXIS_COLOR;
                ctx.lineWidth = 1;
                ctx.strokeRect(MARGIN.left, MARGIN.top, width - MARGIN.left - MARGIN.right, height - MARGIN.top - MARGIN.bottom);
                ctx.fillStyle = LABEL_COLOR;
                ctx.font = "11px 'JetBrains Mono', monospace";
                ctx.textAlign = "center";
                ctx.textBaseline = "top";
                for (const x of xTicks) ctx.fillText(xFormat(x), px(x), height - MARGIN.bottom + 5);
                ctx.textAlign = "right";
                ctx.textBaseline = "middle";
                for (const y of yTicks) ctx.fillText(yFormat(y), MARGIN.left - 5, py(y));
                return { px, py };
            };

            const niceTicks = (lo, hi, count = 5) => {
                const step0 = (hi - lo) / count;
                const magnitude = 10 ** Math.floor(Math.log10(step0));
                const step = [1, 2, 5, 10].map((f) => f * magnitude).find((s) => s >= step0);
                const ticks = [];
                for (let v = Math.ceil(lo / step) * step; v <= hi + 1e-9; v += step) ticks.push(+v.toFixed(10));
                return ticks;
            };
            const formatTick = (v) => Math.abs(v) >= 100 || v === Math.round(v) ? v.toFixed(0) : +v.toPrecision(3);

            const polyline = (ctx, points, color, width = 1.5, dash = []) => {
                ctx.strokeStyle = color;
                ctx.lineWidth = width;
                ctx.setLineDash(dash);
                ctx.beginPath();
                let started = false;
                for (const [x, y] of points) {
                    if (!Number.isFinite(y)) { started = false; continue; }
                    if (started) ctx.lineTo(x, y); else ctx.moveTo(x, y);
                    started = true;
                }
                ctx.stroke();
                ctx.setLineDash([]);
            };

            // Reference point closest to the current parameter value.
            const nearestReferencePoint = () => {
                const p = params();
                const scan = source.reference?.(p);
                if (!scan) return null;
                const x = source.x(p);
                return scan.points.reduce((best, point) =>
                    Math.abs(point.parameter - x) < Math.abs(best.parameter - x) ? point : best);
            };

            // Density of a fixed-width reference histogram at its bin centers.
            const histogramDensity = (hist) => {
                const width = (hist.high - hist.low) / hist.counts.length;
                const total = hist.counts.reduce((a, b) => a + b, 0);
                return hist.counts.map((c, k) => [hist.low + (k + 0.5) * width, c / (total * width)]);
            };

            const drawCorrelationHistogram = () => {
                const counts = Array.from(lab.correlation_counts());
                const reference = nearestReferencePoint();
                const nBins = counts.length || 200;
                const width = 2 / nBins;
                const total = counts.reduce((a, b) => a + b, 0);
                const live = counts.map((c, k) => [-1 + (k + 0.5) * width, total ? c / (total * width) : 0]);
                const ref = reference ? histogramDensity(reference.correlations) : [];
                const yMax = Math.max(1, ...live.map(([, d]) => d), ...ref.map(([, d]) => d)) * 1.05;
                const { px, py } = frame(correlationHistCtx, [-1, 1], [0, yMax], {
                    xTicks: [-1, -0.5, 0, 0.5, 1], yTicks: niceTicks(0, yMax, 4), xFormat: formatTick, yFormat: formatTick,
                });
                correlationHistCtx.fillStyle = BAR_COLOR;
                const barWidth = px(width) - px(0);
                for (const [x, d] of live) {
                    correlationHistCtx.fillRect(px(x - width / 2), py(d), Math.max(1, barWidth - 1), py(0) - py(d));
                }
                polyline(correlationHistCtx, ref.map(([x, d]) => [px(x), py(d)]), REFERENCE_COLOR);
            };

            const drawSpectrum = () => {
                const nSamples = lab.n_samples();
                const count = Number(lab.n_matrices()) * nSamples;
                const eigenvalues = count ? f64View(lab.accumulated_eigenvalues(), count) : new Float64Array();
                const log = logCheckbox.checked;
                const nSteps = lab.n_steps();
                const q = nSamples / (nSteps + 1);
                const mpHigh = (1 + Math.sqrt(q)) ** 2;
                // Plot coordinate u: λ itself, or log10 λ on the logarithmic axis.
                const toU = (lambda) => log ? Math.log10(Math.max(lambda, LOG_LAMBDA_MIN)) : lambda;
                let lambdaMax = mpHigh * 1.15;
                for (const lambda of eigenvalues) lambdaMax = Math.max(lambdaMax, lambda);
                const [u0, u1] = log ? [Math.log10(LOG_LAMBDA_MIN), Math.log10(nSamples)] : [0, lambdaMax * 1.02];
                // Density per unit u: dλ/du = λ ln 10 on the logarithmic axis.
                const jacobian = (lambda) => log ? lambda * Math.LN10 : 1;

                const width = (u1 - u0) / SPECTRUM_BINS;
                const counts = new Array(SPECTRUM_BINS).fill(0);
                for (const lambda of eigenvalues) {
                    const k = Math.min(SPECTRUM_BINS - 1, Math.max(0, Math.floor((toU(lambda) - u0) / width)));
                    counts[k]++;
                }
                const live = counts.map((c, k) => [u0 + (k + 0.5) * width, count ? c / (count * width) : 0]);

                const mp = [];
                for (let k = 0; k <= 400; k++) {
                    const u = u0 + (u1 - u0) * k / 400;
                    const lambda = log ? 10 ** u : u;
                    mp.push([u, marchenko_pastur_density(nSteps, lambda) * jacobian(lambda)]);
                }

                // The reference is binned linearly: on the logarithmic axis its first bins
                // average over decades of λ, so it is drawn only where the bins resolve it.
                const reference = nearestReferencePoint();
                const referenceWidth = reference
                    ? (reference.eigenvalues.high - reference.eigenvalues.low) / reference.eigenvalues.counts.length
                    : 0;
                const ref = reference
                    ? histogramDensity(reference.eigenvalues)
                        .filter(([lambda]) => !log || lambda > 2 * referenceWidth)
                        .map(([lambda, d]) => [toU(lambda), d * jacobian(lambda)])
                    : [];

                // Scale to the histograms, ignoring their two tallest bins: a single pile of
                // near-zero eigenvalues would otherwise flatten everything else. The
                // Marchenko-Pastur curve sets the scale only before any data.
                const peaks = [...live, ...ref].map(([, d]) => d).sort((a, b) => b - a);
                const dataMax = peaks[Math.min(2, peaks.length - 1)] ?? 0;
                const yMax = (dataMax > 0 ? dataMax : Math.max(...mp.map(([, d]) => d))) * 1.1;
                const xTicks = log ? [-3, -2, -1, 0, 1, 2] : niceTicks(u0, u1, 6);
                const { px, py } = frame(spectrumCtx, [u0, u1], [0, yMax], {
                    xTicks, yTicks: niceTicks(0, yMax, 4),
                    xFormat: log ? (u) => `1e${u}` : formatTick, yFormat: formatTick,
                });
                spectrumCtx.save();
                spectrumCtx.beginPath();
                spectrumCtx.rect(MARGIN.left, MARGIN.top, spectrumCtx.canvas.width - MARGIN.left - MARGIN.right, spectrumCtx.canvas.height - MARGIN.top - MARGIN.bottom);
                spectrumCtx.clip();
                spectrumCtx.fillStyle = BAR_COLOR;
                const barWidth = px(u0 + width) - px(u0);
                for (const [u, d] of live) {
                    spectrumCtx.fillRect(px(u - width / 2), py(d), Math.max(1, barWidth - 1), py(0) - py(d));
                }
                polyline(spectrumCtx, ref.map(([u, d]) => [px(u), py(d)]), REFERENCE_COLOR);
                polyline(spectrumCtx, mp.map(([u, d]) => [px(u), py(d)]), MP_COLOR);
                spectrumCtx.restore();
            };

            const drawScan = (ctx, field, referenceField) => {
                const p = params();
                const scan = source.reference?.(p);
                const ref = scan ? scan.points.map((point) => [point.parameter, point[referenceField]]) : [];
                const live = Object.values(livePoints[liveScanKey()] ?? {}).map((point) => [point.x, point[field]]);
                const values = [...ref, ...live].map(([, y]) => y).filter(Number.isFinite);
                const yMax = Math.max(0.1, ...values) * 1.08;
                const { px, py } = frame(ctx, source.xRange, [0, yMax], {
                    xTicks: niceTicks(...source.xRange, 5), yTicks: niceTicks(0, yMax, 4),
                    xFormat: formatTick, yFormat: formatTick,
                });
                if (source.critical) {
                    const critical = source.critical(p);
                    polyline(ctx, [[px(critical), py(0)], [px(critical), py(yMax)]], LABEL_COLOR, 1, [4, 4]);
                }
                polyline(ctx, ref.map(([x, y]) => [px(x), py(y)]), REFERENCE_COLOR);
                // Current parameter value.
                polyline(ctx, [[px(source.x(p)), py(0)], [px(source.x(p)), py(yMax)]], "#553344", 1);
                ctx.fillStyle = LIVE_COLOR;
                for (const [x, y] of live) {
                    ctx.beginPath();
                    ctx.arc(px(x), py(y), 3.5, 0, 2 * Math.PI);
                    ctx.fill();
                }
                ctx.fillStyle = LABEL_COLOR;
                ctx.textAlign = "right";
                ctx.textBaseline = "bottom";
                ctx.fillText(source.xLabel, ctx.canvas.width - MARGIN.right - 4, ctx.canvas.height - MARGIN.bottom - 4);
            };

            const drawScans = () => {
                if (!source.x) return;
                drawScan(scanVarCtx, "variance", "eigenvalue_variance");
                drawScan(scanMaxCtx, "maxMean", "max_eigenvalue_mean");
                drawScan(scanMaxVarCtx, "maxVariance", "max_eigenvalue_variance");
            };

            const updateReadouts = () => {
                const n = Number(lab.n_matrices());
                el("spectral-matrices").textContent = n;
                el("spectral-var").textContent = n ? lab.eigenvalue_variance().toFixed(3) : "–";
                el("spectral-max").textContent = n ? lab.max_eigenvalue_mean().toFixed(2) : "–";
                el("spectral-mp-var").textContent = (lab.n_samples() / (lab.n_steps() + 1)).toFixed(3);
            };

            const recordLivePoint = () => {
                if (!source.x || lab.n_matrices() === 0n) return;
                const p = params();
                const scanPoints = (livePoints[liveScanKey()] ??= {});
                scanPoints[source.key(p)] = {
                    x: source.x(p),
                    variance: lab.eigenvalue_variance(),
                    maxMean: lab.max_eigenvalue_mean(),
                    maxVariance: lab.max_eigenvalue_variance(),
                };
            };

            const drawAnalysis = () => {
                drawCorrelations();
                drawCorrelationHistogram();
                drawSpectrum();
                drawScans();
                updateReadouts();
            };

            // ---- Lifecycle ---------------------------------------------------------------

            const rebuild = () => {
                lab?.free();
                source = SOURCES[sourceName];
                lab = source.make(params());
                resizeGrid(correlationsCanvas, lab.n_samples(), lab.n_samples());
                seriesCanvas.width = lab.n_steps() + 1;
                seriesCanvas.height = lab.n_samples();
                const hasLattice = lab.lattice_cols() > 0;
                el("spectral-lattice-panel").style.display = hasLattice ? "" : "none";
                // The space-time diagram is squeezed into the square the lattice uses, so
                // its 501 steps don't dwarf the other panels.
                latticeCanvas.style.height = sourceName === "contact" ? latticeCanvas.style.width : "auto";
                if (sourceName === "ising") {
                    resizeGrid(latticeCanvas, lab.lattice_rows(), lab.lattice_cols());
                    el("spectral-lattice-title").textContent = "Lattice of sample 1";
                } else if (sourceName === "contact") {
                    resizeGrid(latticeCanvas, lab.n_steps() + 1, lab.lattice_cols());
                    spaceTime = latticeCtx.createImageData(lab.lattice_cols(), lab.n_steps() + 1);
                    clearSpaceTime();
                    recordSpaceTimeRow();
                    el("spectral-lattice-title").textContent = "Space-time diagram of sample 1";
                }
                el("spectral-scans").style.display = source.x ? "flex" : "none";
                // Hidden plots have no width until shown.
                resizePlots();
                drawSeries();
                drawLattice();
                drawAnalysis();
            };

            window.addEventListener("resize", () => {
                resizePlots();
                drawAnalysis();
            });

            const selectSource = (name) => {
                sourceName = name;
                document.querySelectorAll(".spectral-wrap [data-source]").forEach((node) => {
                    node.style.display = node.dataset.source === name ? "" : "none";
                });
                logCheckbox.checked = SOURCES[name].logAxis;
                rebuild();
            };

            // Advance one step; returns whether a matrix was completed.
            const stepOnce = () => {
                const completed = lab.advance(1) > 0;
                if (sourceName === "contact") {
                    if (lab.t() === 0) clearSpaceTime();
                    recordSpaceTimeRow();
                }
                if (completed) recordLivePoint();
                return completed;
            };

            let animationId = null;
            const isPaused = () => animationId === null;

            const renderLoop = () => {
                let completed = false;
                if (speedSelect.value === "max") {
                    const start = performance.now();
                    while (performance.now() - start < FRAME_BUDGET_MS) completed = stepOnce() || completed;
                } else {
                    for (let k = 0; k < parseInt(speedSelect.value, 10); k++) completed = stepOnce() || completed;
                }
                drawSeries();
                drawLattice();
                if (completed) drawAnalysis();
                animationId = requestAnimationFrame(renderLoop);
            };

            const playPauseButton = el("spectral-play-pause");
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

            el("spectral-reset").addEventListener("click", rebuild);

            // Moves the slider to the step nearest the critical value.
            const setCritical = () => {
                const slider = { ising: tSlider, contact: alphaSlider }[sourceName];
                if (!slider) return;
                const step = parseFloat(slider.step);
                const min = parseFloat(slider.min);
                slider.value = min + Math.round((source.critical(params()) - min) / step) * step;
                slider.dispatchEvent(new Event("input"));
            };
            el("spectral-critical").addEventListener("click", setCritical);

            const bindSlider = (slider, output, digits) => {
                slider.addEventListener("input", () => {
                    output.textContent = parseFloat(slider.value).toFixed(digits);
                    rebuild();
                });
            };
            bindSlider(rhoSlider, el("spectral-rho-value"), 2);
            bindSlider(tSlider, el("spectral-t-value"), 2);
            bindSlider(alphaSlider, el("spectral-alpha-value"), 2);
            gammaSelect.addEventListener("change", rebuild);
            sourceSelect.addEventListener("change", () => selectSource(sourceSelect.value));
            logCheckbox.addEventListener("change", drawSpectrum);
            sortCheckbox.addEventListener("change", drawCorrelations);

            document.addEventListener("keydown", (event) => {
                // Letters also pick options of a focused select.
                if (event.target instanceof HTMLSelectElement) return;
                if (event.code === "KeyP") playOrPause();
                else if (event.code === "KeyR") rebuild();
                else if (event.code === "KeyC") setCritical();
            });

            selectSource(sourceSelect.value);
        } catch (e) {
            console.error("Failed to load spectral criticality WASM simulation:", e);
            const canvas = document.getElementById("spectral-series");
            if (canvas) {
                canvas.style.border = "2px solid red";
                canvas.style.background = "red";
            }
        }
    }

    run();
</script>
