+++
title = "Chiarella Model"
date = 2026-09-25
description = "Extended Chiarella model of a market with fundamentalists, trend followers and noise traders, integrated as a stochastic differential equation in WebAssembly."
+++

The Chiarella model describes the log-price $p$ of an asset traded by three kinds of agents:

- **fundamentalists** buy when the price is below a fundamental value $V$ and sell when it is above, pulling the price back at rate $\kappa$;
- **trend followers** chase a moving average $M$ of recent price changes, with a saturating demand $\beta \tanh(\gamma M)$;
- **noise traders** add a random component of volatility $\sigma_N$.

In the extended version of Majewski, Ciliberti and Bouchaud (2020), the fundamental value itself follows a random walk. The dynamics are

$$
\begin{aligned}
dV &= g\\,dt + \sigma_V\\,dW_1, \\\\
dp &= \kappa (V - p)\\,dt + \beta \tanh(\gamma M)\\,dt + \sigma_N\\,dW_2, \\\\
dM &= \alpha\\,(dp - M\\,dt),
\end{aligned}
$$

where $1/\alpha$ is the memory time of the trend signal. The system is integrated with the Euler–Maruyama scheme at time step $dt$.

### State variables

- $p$ — **log-price** of the asset. Working in logs makes price increments $dp$ returns, so the dynamics don't depend on the price level. The simulation starts at $p = 0$, i.e. a price normalized to $e^0 = 1$.
- $V$ — **log fundamental value**: what the asset is "really worth" according to fundamentalists, e.g. discounted future cash flows. It is exogenous: news moves $V$, but the market price never feeds back into it.
- $\delta = p - V$ — **mispricing**. $\delta > 0$ means the asset is overvalued (the bubble side), $\delta < 0$ undervalued. It is the natural variable for regime behavior, since $p$ and $V$ both wander without bound but their difference does not.
- $M$ — **trend signal**: an exponentially weighted moving average of past price changes, $M_t = \alpha \int_{-\infty}^{t} e^{-\alpha (t - s)}\\,dp_s$. It is a return per unit time: $M > 0$ means prices have recently been rising. Noise-driven moves count too, since trend followers can't tell a noise trade from a genuine trend.
- $W_1, W_2$ — independent Wiener processes (Brownian motions): the random news reaching fundamentals and the random order flow of noise traders.

### Parameters

- $\kappa$ — **fundamentalist strength**: the rate at which fundamentalists correct mispricing. Their demand is $-\kappa\delta$, so alone they close a gap with half-life $\ln 2 / \kappa$. Increasing $\kappa$ stabilizes the market.
- $\beta$ — **trend-follower strength**: the largest drift trend followers can impose on the price, since $\beta\tanh(\gamma M) \in (-\beta, \beta)$. Read it as their capital or market share. Above the threshold, it sets how large bubbles and crashes grow.
- $\gamma$ — **trend-follower sensitivity**: how strongly they react to a weak trend. For $|\gamma M| \ll 1$ their demand is linear, $\approx \beta\gamma M$; for strong trends it saturates at $\pm\beta$, because positions can't grow without limit. Only the product $\beta\gamma$, the gain for small trends, decides stability; $\beta$ alone caps the amplitude.
- $\alpha$ — **inverse memory time** of the trend signal: $M$ averages over roughly the last $1/\alpha$ time units. A large $\alpha$ is a short, jittery memory that chases the latest noise; a small $\alpha$ is a long, smooth one that reacts late. A longer memory raises the threshold $1 + \kappa/\alpha$, giving fundamentalists more time to act before a trend builds.
- $\sigma_N$ — **noise-trader volatility**: the size of random order flow unrelated to value or trend. It is the main source of short-term return volatility, and it also seeds spurious trends in $M$.
- $\sigma_V$ — **fundamental volatility**: the size of news shocks to $V$. Fundamentalists chase every jump, and the trend followers amplify the resulting price moves.
- $g$ — **fundamental drift**: steady growth (or decay) of the fundamental value. In a steadily growing market the trend signal settles at $M = g$, and the price settles at a constant offset $\delta^{\ast} = \left(\beta\tanh(\gamma g) - g\right)/\kappa$ from value. When trend followers are strong enough this is positive: a persistent, self-sustaining overvaluation.

### Simulation controls

- $dt$ — the **integration time step**. It is numerical, not part of the model: smaller is more accurate but covers less simulated time per step. Euler–Maruyama needs $\kappa\\,dt \ll 1$ and $\alpha\\,dt \ll 1$ to track the continuous dynamics.
- **Steps/frame** — the number of integration steps per animation frame, i.e. the playback speed. Each frame advances simulated time by $\text{steps} \times dt$. Every plot takes one sample per frame, so "returns" are price changes over that interval.

Time units are arbitrary: if one unit is a trading day, then $\sigma_N$ is a daily volatility, $1/\alpha$ is a memory length in days, and so on.

The interesting variable is the **mispricing** $\delta = p - V$. Without noise, the state $(\delta, M) = (0, 0)$ is stable as long as

$$
\beta\gamma < 1 + \frac{\kappa}{\alpha}.
$$

<p style="font-size:0.85rem; color:#888;">Linearizing around the origin gives a Jacobian with determinant $\alpha\kappa > 0$ and trace $\alpha(\beta\gamma - 1) - \kappa$. When the trace turns positive, a Hopf bifurcation occurs: trend followers overpower fundamentalists and the mispricing locks into a limit cycle of bubbles and crashes. With noise on, the histogram of $\delta$ widens and becomes bimodal, since the price lingers on the overvalued or undervalued side of the cycle.</p>

Every slider acts on the running simulation immediately. Push $\beta$ past the threshold to watch the phase portrait open into a cycle.

<div class="chiarella-wrap" style="display:flex; flex-direction:column; gap:0.8rem; font-family:'JetBrains Mono','Fira Code',monospace; color:#aaaaaa; font-size:0.9rem; line-height:1.3;">
    <div style="display:flex; flex-wrap:wrap; gap:0.5rem; align-items:center;">
        <button id="chiarella-play-pause" style="background:#111; color:#aaaaaa; border:1px solid #333; padding:0.3rem 0.9rem; font-family:inherit; font-size:1.1rem; cursor:pointer;">&#9654;</button>
        <button id="chiarella-reset" style="background:#111; color:#aaaaaa; border:1px solid #333; padding:0.3rem 0.8rem; font-family:inherit; cursor:pointer;">Reset</button>
        <button id="chiarella-clear-histogram" style="background:#111; color:#aaaaaa; border:1px solid #333; padding:0.3rem 0.8rem; font-family:inherit; cursor:pointer;">Clear hist.</button>
        <span style="color:#666; margin-left:auto;">P start/stop &middot; R reset</span>
    </div>
    <div class="chiarella-controls" style="display:grid; grid-template-columns:repeat(auto-fill, minmax(170px, 1fr)); gap:0.4rem 1.2rem;">
        <div>
            <p style="margin:0 0 0.2rem;"><span data-tooltip="Fundamentalist strength: how fast mispricing gets corrected. On its own, a gap halves every ln2/κ." tabindex="0">&kappa;</span> = <output id="chiarella-kappa-value"></output></p>
            <input type="range" id="chiarella-kappa-slider" min="0" max="0.5" step="0.005" value="0.1" style="width:100%;">
        </div>
        <div>
            <p style="margin:0 0 0.2rem;"><span data-tooltip="Trend-follower strength: the largest drift they can put on the price. Caps how big bubbles get." tabindex="0">&beta;</span> = <output id="chiarella-beta-value"></output></p>
            <input type="range" id="chiarella-beta-slider" min="0" max="5" step="0.01" value="1.3" style="width:100%;">
        </div>
        <div>
            <p style="margin:0 0 0.2rem;"><span data-tooltip="Trend-follower sensitivity: how hard they react to weak trends. Stability depends on the product βγ, not on β or γ separately." tabindex="0">&gamma;</span> = <output id="chiarella-gamma-value"></output></p>
            <input type="range" id="chiarella-gamma-slider" min="0" max="5" step="0.01" value="1" style="width:100%;">
        </div>
        <div>
            <p style="margin:0 0 0.2rem;"><span data-tooltip="Inverse memory of the trend signal: M averages roughly the last 1/α time units." tabindex="0">&alpha;</span> = <output id="chiarella-alpha-value"></output></p>
            <input type="range" id="chiarella-alpha-slider" min="0.01" max="2" step="0.01" value="0.2" style="width:100%;">
        </div>
        <div>
            <p style="margin:0 0 0.2rem;"><span data-tooltip="Noise-trader volatility: random order flow, the main source of short-term volatility." tabindex="0">&sigma;<sub>N</sub></span> = <output id="chiarella-sigma-noise-value"></output></p>
            <input type="range" id="chiarella-sigma-noise-slider" min="0" max="1" step="0.01" value="0.1" style="width:100%;">
        </div>
        <div>
            <p style="margin:0 0 0.2rem;"><span data-tooltip="Fundamental volatility: size of the news shocks that move V." tabindex="0">&sigma;<sub>V</sub></span> = <output id="chiarella-sigma-fundamental-value"></output></p>
            <input type="range" id="chiarella-sigma-fundamental-slider" min="0" max="1" step="0.01" value="0.05" style="width:100%;">
        </div>
        <div>
            <p style="margin:0 0 0.2rem;"><span data-tooltip="Fundamental drift: steady growth (or decay) of V." tabindex="0">g</span> = <output id="chiarella-drift-value"></output></p>
            <input type="range" id="chiarella-drift-slider" min="-0.1" max="0.1" step="0.001" value="0" style="width:100%;">
        </div>
        <div>
            <p style="margin:0 0 0.2rem;"><span data-tooltip="Integration time step. Numerical only: smaller is more accurate but advances less time per step." tabindex="0">dt</span> = <output id="chiarella-dt-value"></output></p>
            <input type="range" id="chiarella-dt-slider" min="0.005" max="0.2" step="0.005" value="0.05" style="width:100%;">
        </div>
        <div>
            <p style="margin:0 0 0.2rem;"><span data-tooltip="Integration steps per animation frame, i.e. playback speed. Each frame covers steps × dt." tabindex="0">Steps/frame</span> = <output id="chiarella-speed-value"></output></p>
            <input type="range" id="chiarella-speed-slider" min="1" max="100" step="1" value="10" style="width:100%;">
        </div>
    </div>
    <p style="color:#888; margin:0; display:flex; flex-wrap:wrap; gap:0.2rem 1.2rem;">
        <span><span data-tooltip="Trend-follower gain βγ against the stability threshold 1+κ/α. Above it, the market cycles between bubbles and crashes." tabindex="0">&beta;&gamma;</span> = <output id="chiarella-bg-value"></output> vs 1+&kappa;/&alpha; = <output id="chiarella-threshold-value"></output> &rarr; <output id="chiarella-regime-value"></output></span>
        <span><span data-tooltip="Log-price of the asset." tabindex="0">p</span> = <output id="chiarella-p-value">0.000</output></span>
        <span><span data-tooltip="Log fundamental value: what fundamentalists think the asset is worth." tabindex="0">V</span> = <output id="chiarella-v-value">0.000</output></span>
        <span><span data-tooltip="Mispricing p − V. Positive means overvalued." tabindex="0">&delta;</span> = <output id="chiarella-delta-value">0.000</output></span>
        <span><span data-tooltip="Trend signal: moving average of recent price changes." tabindex="0">M</span> = <output id="chiarella-m-value">0.000</output></span>
    </p>
    <div style="display:flex; flex-wrap:wrap; gap:1rem; align-items:flex-start;">
        <div style="width:480px; max-width:100%;">
            <p style="font-size:0.85rem; margin:0 0 0.25rem;"><span data-tooltip="Trajectory in the (mispricing, trend) plane; the red dot is the current state. Settling near the center means mean reversion, a loop means a bubble–crash cycle." tabindex="0">Phase portrait (&delta;, M)</span></p>
            <div style="border:1px solid #333; background:#000;">
                <canvas id="chiarella-phase" style="display:block; width:100%; aspect-ratio:1/1; height:auto;"></canvas>
            </div>
        </div>
        <div style="flex:1; min-width:280px; display:flex; flex-direction:column; gap:0.5rem;">
            <div>
                <p style="font-size:0.85rem; margin:0 0 0.25rem;"><span data-tooltip="Log-price p (gray) and fundamental value V (red). Fundamentalists pull p toward V; trend followers push it away." tabindex="0"><span style="color:#aaaaaa;">Price p</span> <span style="color:#666;">/</span> <span style="color:#ff0055;">fundamental V</span></span></p>
                <div style="border:1px solid #333; background:#000;">
                    <canvas id="chiarella-plot-price" style="display:block; width:100%; height:90px;"></canvas>
                </div>
            </div>
            <div>
                <p style="font-size:0.85rem; margin:0 0 0.25rem;"><span data-tooltip="How far the price is from value. Scale is symmetric around the zero line, where the asset is fairly priced." tabindex="0">Mispricing &delta; = p &minus; V</span></p>
                <div style="border:1px solid #333; background:#000;">
                    <canvas id="chiarella-plot-mispricing" style="display:block; width:100%; height:90px;"></canvas>
                </div>
            </div>
            <div>
                <p style="color:#ff0055; font-size:0.85rem; margin:0 0 0.25rem;"><span data-tooltip="Price change over each frame, i.e. over steps × dt of simulated time." tabindex="0">Returns &Delta;p per frame</span></p>
                <div style="border:1px solid #333; background:#000;">
                    <canvas id="chiarella-plot-returns" style="display:block; width:100%; height:90px;"></canvas>
                </div>
            </div>
            <div>
                <p style="font-size:0.85rem; margin:0 0 0.25rem;"><span data-tooltip="Distribution of mispricing since the last clear. One central peak when mean-reverting; wider and two-humped in the oscillatory regime." tabindex="0">Histogram of &delta;</span> <span style="color:#666;">(range &plusmn;<output id="chiarella-histogram-range">1</output>, <output id="chiarella-histogram-count">0</output> samples)</span></p>
                <div style="border:1px solid #333; background:#000;">
                    <canvas id="chiarella-plot-histogram" style="display:block; width:100%; height:90px;"></canvas>
                </div>
            </div>
        </div>
    </div>
</div>

<script type="module">
    import init, { ChiarellaSimulation } from '/wasm/chiarella.js';
    import { ALIVE_COLOR } from '/js/grid-canvas.js';

    async function run() {
        try {
            await init();

            const PRIMARY_COLOR = ALIVE_COLOR;
            const ACCENT_COLOR = "#ff0055";
            const AXIS_COLOR = "#333";
            const PHASE_SIZE = 512;
            // Each plot is drawn at a fixed internal resolution matching the
            // plot column, and CSS-stretched when the layout stacks.
            const PLOT_WIDTH = 480;
            const PLOT_HEIGHT = 90;
            const PLOT_HISTORY = 600;
            const PHASE_TRAIL = 2000;
            // Trail is drawn in this many chunks of equal opacity rather
            // than one stroke per segment, to keep the draw call count low.
            const PHASE_TRAIL_CHUNKS = 20;
            // Must be divisible by 4: doubling the range merges bin pairs
            // into the middle half of the new bins (see `widenHistogram`).
            const HISTOGRAM_BINS = 80;

            const setUpCanvas = (id, width, height) => {
                const canvas = document.getElementById(id);
                canvas.width = width;
                canvas.height = height;
                return canvas.getContext('2d');
            };
            const phaseCtx = setUpCanvas("chiarella-phase", PHASE_SIZE, PHASE_SIZE);
            const priceCtx = setUpCanvas("chiarella-plot-price", PLOT_WIDTH, PLOT_HEIGHT);
            const mispricingCtx = setUpCanvas("chiarella-plot-mispricing", PLOT_WIDTH, PLOT_HEIGHT);
            const returnsCtx = setUpCanvas("chiarella-plot-returns", PLOT_WIDTH, PLOT_HEIGHT);
            const histogramCtx = setUpCanvas("chiarella-plot-histogram", PLOT_WIDTH, PLOT_HEIGHT);

            // Slider id suffix → model setter. Defaults live in the HTML
            // `value` attributes.
            const PARAMETERS = [
                ["kappa", "set_kappa"],
                ["beta", "set_beta"],
                ["gamma", "set_gamma"],
                ["alpha", "set_alpha"],
                ["sigma-noise", "set_sigma_noise"],
                ["sigma-fundamental", "set_sigma_fundamental"],
                ["drift", "set_drift"],
                ["dt", "set_dt"],
            ];
            const slider = (name) => document.getElementById(`chiarella-${name}-slider`);
            const sliderValue = (name) => parseFloat(slider(name).value);

            const model = ChiarellaSimulation.new(
                sliderValue("kappa"),
                sliderValue("beta"),
                sliderValue("gamma"),
                sliderValue("alpha"),
                sliderValue("sigma-noise"),
                sliderValue("sigma-fundamental"),
                sliderValue("drift"),
                sliderValue("dt"),
            );

            let priceHistory = [];
            let fundamentalHistory = [];
            let mispricingHistory = [];
            let returnsHistory = [];
            let trail = [];
            let histogramCounts = new Array(HISTOGRAM_BINS).fill(0);
            let histogramRange = 1;
            let histogramTotal = 0;
            let lastPrice = 0;

            const pushRolling = (history, value, limit) => {
                history.push(value);
                if (history.length > limit) history.shift();
            };

            // Double the symmetric range until `value` fits, merging each
            // pair of old bins into one new bin.
            const widenHistogram = (value) => {
                while (Math.abs(value) >= histogramRange) {
                    const widened = new Array(HISTOGRAM_BINS).fill(0);
                    const offset = HISTOGRAM_BINS / 4;
                    for (let i = 0; i < HISTOGRAM_BINS; i++) {
                        widened[offset + Math.floor(i / 2)] += histogramCounts[i];
                    }
                    histogramCounts = widened;
                    histogramRange *= 2;
                }
            };

            const addToHistogram = (value) => {
                // Otherwise `widenHistogram` never terminates.
                if (!Number.isFinite(value)) return;
                widenHistogram(value);
                const bin = Math.floor((value + histogramRange) / (2 * histogramRange) * HISTOGRAM_BINS);
                histogramCounts[Math.min(bin, HISTOGRAM_BINS - 1)] += 1;
                histogramTotal += 1;
            };

            const clearHistogram = () => {
                histogramCounts = new Array(HISTOGRAM_BINS).fill(0);
                histogramRange = 1;
                histogramTotal = 0;
            };

            const resetHistory = () => {
                priceHistory = [];
                fundamentalHistory = [];
                mispricingHistory = [];
                returnsHistory = [];
                trail = [];
                lastPrice = model.price();
                clearHistogram();
            };

            const sampleHistory = () => {
                const price = model.price();
                const mispricing = model.mispricing();
                pushRolling(priceHistory, price, PLOT_HISTORY);
                pushRolling(fundamentalHistory, model.fundamental(), PLOT_HISTORY);
                pushRolling(mispricingHistory, mispricing, PLOT_HISTORY);
                pushRolling(returnsHistory, price - lastPrice, PLOT_HISTORY);
                pushRolling(trail, [mispricing, model.trend()], PHASE_TRAIL);
                addToHistogram(mispricing);
                lastPrice = price;
            };

            // Padded [min, max] over several series; never degenerate, so a
            // flat line still gets a visible vertical scale.
            const seriesRange = (seriesList) => {
                let vMin = Infinity;
                let vMax = -Infinity;
                for (const series of seriesList) {
                    for (const v of series) {
                        if (v < vMin) vMin = v;
                        if (v > vMax) vMax = v;
                    }
                }
                if (!isFinite(vMin)) return [-1, 1];
                const pad = Math.max((vMax - vMin) * 0.1, 1e-3);
                return [vMin - pad, vMax + pad];
            };

            const clearPanel = (ctx, width, height) => {
                ctx.fillStyle = "#000";
                ctx.fillRect(0, 0, width, height);
            };

            const yForValue = (v, vMin, vMax) => PLOT_HEIGHT * (1 - (v - vMin) / (vMax - vMin));

            const drawZeroLine = (ctx, vMin, vMax) => {
                if (vMin > 0 || vMax < 0) return;
                ctx.strokeStyle = AXIS_COLOR;
                ctx.lineWidth = 1;
                ctx.beginPath();
                ctx.moveTo(0, yForValue(0, vMin, vMax));
                ctx.lineTo(PLOT_WIDTH, yForValue(0, vMin, vMax));
                ctx.stroke();
            };

            const drawSeries = (ctx, history, color, vMin, vMax) => {
                if (history.length < 2) return;
                ctx.strokeStyle = color;
                ctx.lineWidth = 1.5;
                ctx.beginPath();
                history.forEach((v, i) => {
                    const x = (i / (PLOT_HISTORY - 1)) * PLOT_WIDTH;
                    const y = yForValue(v, vMin, vMax);
                    if (i === 0) ctx.moveTo(x, y);
                    else ctx.lineTo(x, y);
                });
                ctx.stroke();
            };

            const drawTimeSeriesPanel = (ctx, seriesWithColors, { symmetric = false } = {}) => {
                clearPanel(ctx, PLOT_WIDTH, PLOT_HEIGHT);
                let [vMin, vMax] = seriesRange(seriesWithColors.map(([series]) => series));
                if (symmetric) {
                    const bound = Math.max(Math.abs(vMin), Math.abs(vMax));
                    [vMin, vMax] = [-bound, bound];
                }
                drawZeroLine(ctx, vMin, vMax);
                for (const [series, color] of seriesWithColors) drawSeries(ctx, series, color, vMin, vMax);
            };

            const drawPhasePortrait = () => {
                clearPanel(phaseCtx, PHASE_SIZE, PHASE_SIZE);
                let deltaBound = 1e-3;
                let trendBound = 1e-3;
                for (const [delta, trend] of trail) {
                    deltaBound = Math.max(deltaBound, Math.abs(delta));
                    trendBound = Math.max(trendBound, Math.abs(trend));
                }
                deltaBound *= 1.1;
                trendBound *= 1.1;
                const half = PHASE_SIZE / 2;
                const toX = (delta) => half + (delta / deltaBound) * half;
                const toY = (trend) => half - (trend / trendBound) * half;

                phaseCtx.strokeStyle = AXIS_COLOR;
                phaseCtx.lineWidth = 1;
                phaseCtx.beginPath();
                phaseCtx.moveTo(0, half);
                phaseCtx.lineTo(PHASE_SIZE, half);
                phaseCtx.moveTo(half, 0);
                phaseCtx.lineTo(half, PHASE_SIZE);
                phaseCtx.stroke();

                phaseCtx.fillStyle = "#666";
                phaseCtx.font = "12px 'JetBrains Mono', monospace";
                phaseCtx.fillText(`δ ∈ ±${deltaBound.toPrecision(2)}`, 6, PHASE_SIZE - 8);
                phaseCtx.fillText(`M ∈ ±${trendBound.toPrecision(2)}`, 6, 16);

                if (trail.length < 2) return;
                phaseCtx.strokeStyle = PRIMARY_COLOR;
                phaseCtx.lineWidth = 1.2;
                const chunkLength = Math.ceil(trail.length / PHASE_TRAIL_CHUNKS);
                for (let start = 0; start < trail.length - 1; start += chunkLength) {
                    const end = Math.min(start + chunkLength, trail.length - 1);
                    phaseCtx.globalAlpha = 0.1 + 0.9 * (end / (trail.length - 1));
                    phaseCtx.beginPath();
                    phaseCtx.moveTo(toX(trail[start][0]), toY(trail[start][1]));
                    for (let i = start + 1; i <= end; i++) {
                        phaseCtx.lineTo(toX(trail[i][0]), toY(trail[i][1]));
                    }
                    phaseCtx.stroke();
                }
                phaseCtx.globalAlpha = 1;

                const [delta, trend] = trail[trail.length - 1];
                phaseCtx.fillStyle = ACCENT_COLOR;
                phaseCtx.beginPath();
                phaseCtx.arc(toX(delta), toY(trend), 4, 0, 2 * Math.PI);
                phaseCtx.fill();
            };

            const histogramRangeValue = document.getElementById("chiarella-histogram-range");
            const histogramCountValue = document.getElementById("chiarella-histogram-count");

            const drawHistogram = () => {
                clearPanel(histogramCtx, PLOT_WIDTH, PLOT_HEIGHT);
                histogramCtx.strokeStyle = AXIS_COLOR;
                histogramCtx.beginPath();
                histogramCtx.moveTo(PLOT_WIDTH / 2, 0);
                histogramCtx.lineTo(PLOT_WIDTH / 2, PLOT_HEIGHT);
                histogramCtx.stroke();

                const maxCount = Math.max(...histogramCounts, 1);
                const binWidth = PLOT_WIDTH / HISTOGRAM_BINS;
                histogramCtx.fillStyle = PRIMARY_COLOR;
                histogramCounts.forEach((count, i) => {
                    const barHeight = (count / maxCount) * (PLOT_HEIGHT - 4);
                    histogramCtx.fillRect(i * binWidth + 1, PLOT_HEIGHT - barHeight, binWidth - 2, barHeight);
                });

                histogramRangeValue.textContent = histogramRange;
                histogramCountValue.textContent = histogramTotal;
            };

            const drawAll = () => {
                drawPhasePortrait();
                drawTimeSeriesPanel(priceCtx, [[priceHistory, PRIMARY_COLOR], [fundamentalHistory, ACCENT_COLOR]]);
                drawTimeSeriesPanel(mispricingCtx, [[mispricingHistory, PRIMARY_COLOR]], { symmetric: true });
                drawTimeSeriesPanel(returnsCtx, [[returnsHistory, ACCENT_COLOR]], { symmetric: true });
                drawHistogram();
            };

            const readout = (id) => document.getElementById(`chiarella-${id}-value`);
            const updateReadouts = () => {
                readout("p").textContent = model.price().toFixed(3);
                readout("v").textContent = model.fundamental().toFixed(3);
                readout("delta").textContent = model.mispricing().toFixed(3);
                readout("m").textContent = model.trend().toFixed(3);
            };

            const updateRegime = () => {
                const trendGain = sliderValue("beta") * sliderValue("gamma");
                const threshold = 1 + sliderValue("kappa") / sliderValue("alpha");
                readout("bg").textContent = trendGain.toFixed(2);
                readout("threshold").textContent = threshold.toFixed(2);
                const regime = readout("regime");
                regime.textContent = model.is_oscillatory() ? "oscillatory" : "mean-reverting";
                regime.style.color = model.is_oscillatory() ? ACCENT_COLOR : "#888";
            };

            for (const [name, setter] of PARAMETERS) {
                const output = readout(name);
                output.textContent = slider(name).value;
                slider(name).addEventListener("input", () => {
                    output.textContent = slider(name).value;
                    model[setter](sliderValue(name));
                    updateRegime();
                });
            }

            const speedOutput = readout("speed");
            speedOutput.textContent = slider("speed").value;
            slider("speed").addEventListener("input", () => {
                speedOutput.textContent = slider("speed").value;
            });

            let animationId = null;
            const isPaused = () => animationId === null;

            const renderLoop = () => {
                model.advance(parseInt(slider("speed").value, 10));
                sampleHistory();
                drawAll();
                updateReadouts();
                animationId = requestAnimationFrame(renderLoop);
            };

            const playPauseButton = document.getElementById("chiarella-play-pause");

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

            const resetState = () => {
                model.reset();
                resetHistory();
                sampleHistory();
                drawAll();
                updateReadouts();
            };

            document.getElementById("chiarella-reset").addEventListener("click", resetState);
            document.getElementById("chiarella-clear-histogram").addEventListener("click", () => {
                clearHistogram();
                drawHistogram();
            });

            document.addEventListener("keydown", (event) => {
                if (event.code === "KeyP") playOrPause();
                else if (event.code === "KeyR") resetState();
            });

            pause();
            updateRegime();
            resetState();

        } catch (e) {
            console.error("Failed to load Chiarella WASM simulation:", e);
            const canvas = document.getElementById("chiarella-phase");
            if (canvas) {
                canvas.style.border = "2px solid red";
                canvas.style.background = "red";
            }
        }
    }

    run();
</script>
