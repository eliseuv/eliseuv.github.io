+++
title = "Simulated Annealing for the TSP"
date = 2026-09-25
description = "Simulated annealing with Metropolis sampling and geometric cooling applied to the traveling salesman problem on random domains, running in WebAssembly."

[extra]
references = [
    "kirkpatrick1983",
    "cerny1985",
    "metropolis1953",
    "geman1984",
    "nourani1998",
    "croes1958",
    "dasilva2020",
    "10.1016/j.physa.2021.126067",
    "dantzig1954",
    "lin1973",
    "beardwood1959",
    "kirkpatrick1984",
    "vanlaarhoven1987",
    "aarts1989",
    "stanley2001",
    "newman1999",
    "landau2014",
]

[extra.symbols]
'N' = 'Number of cities.'
'\sigma' = 'A cycle: a cyclic ordering of the cities.'
'\sigma_i' = 'The $i$-th city visited by the cycle $\sigma$.'
'\sigma^{\prime}' = 'Candidate cycle proposed by a move.'
'C' = 'Cost: total length of a cycle.'
'\Delta C' = 'Cost change of a candidate move, computed in $O(1)$ from the two replaced edges.'
'd' = 'Euclidean distance between two cities.'
'H' = 'Energy in the physical analogy, equal to the cost $C$.'
'\pi_T' = 'Gibbs measure at temperature $T$: cheaper cycles are exponentially more likely.'
'A' = 'Metropolis acceptance probability of a proposed move.'
'T' = 'Temperature. It has units of distance, like the cost.'
'T_t' = 'Temperature after $t$ cooling steps.'
'T_0' = 'Initial temperature, where each anneal starts.'
'T_f' = 'Final temperature, where cooling stops and the chain keeps sampling.'
't' = 'Annealing step: how many times $T$ has been cooled since the last Anneal.'
'\alpha' = 'Cooling factor: each step multiplies $T$ by $\alpha$. Closer to $1$ cools more slowly.'
'n_{\text{iter}}' = 'Metropolis iterations at each temperature before cooling again.'
'n_{\text{steps}}' = 'Number of cooling steps from $T_0$ down to $T_f$.'
'\rho' = 'Correlation between the $x$ and $y$ coordinates of the cities.'
'\varphi' = 'Mixing angle that sets the correlation $\rho$.'
'z_1' = 'First independent variable mixed into the city coordinates.'
'z_2' = 'Second independent variable mixed into the city coordinates.'
'\gamma' = 'Power-law exponent. Smaller means heavier tails; at $\gamma \leq 3$ the variance is infinite.'
'x_0' = 'Lower cutoff of the power law.'
'p(x)' = 'Density of each coordinate in the power-law domain.'
'\langle C_0\rangle' = 'Expected cost of a uniformly random cycle on the same cities.'
'C/\langle C_0\rangle' = 'Performance: cost relative to a random cycle. Lower is better.'

[extra.links]
context = "msc"
projects = ["tsp-sa"]
publications = ["10.1016/j.physa.2021.126067"]
skills = ["Rust"]
+++

The **traveling salesman problem** (TSP) asks for the shortest closed route visiting each of $N$ cities exactly once. A route is a Hamiltonian cycle $\sigma$, i.e. a cyclic ordering of the cities, and its cost is the total length

$$
C(\sigma) = \sum_{i=1}^{N} d\left(\sigma_i, \sigma_{i+1}\right), \qquad \sigma_{N+1} \equiv \sigma_1.
$$

There are $(N-1)!/2$ distinct cycles, and the problem is NP-hard: no known algorithm finds the optimum in polynomial time. **Simulated annealing** is a heuristic borrowed from statistical physics {{ cite(ids=["kirkpatrick1983", "cerny1985"]) }}. It treats the cost as an energy $H = C$ and samples cycles from the Gibbs measure $\pi_T(\sigma) \propto e^{-C(\sigma)/T}$ at a slowly decreasing temperature $T$, the way a slowly cooled metal settles into a low-energy crystal.

### Metropolis sampling

Sampling uses a Markov chain. At each iteration a **move** proposes a neighboring cycle $\sigma^{\prime}$, and the Metropolis prescription {{ cite(ids=["metropolis1953"]) }} accepts it with probability

$$
A(\sigma \to \sigma^{\prime}) = \min\left(1,\\; e^{-\left(C(\sigma^{\prime}) - C(\sigma)\right)/T}\right).
$$

Downhill moves are always taken. Uphill moves are taken with a probability that shrinks with $T$, which lets the chain climb out of local minima while $T$ is still high, unlike a greedy local search.

### Geometric cooling

The annealing loop samples $n_{\text{iter}}$ iterations at each temperature and then cools:

$$
T_{t+1} = \alpha\\, T_t \quad\Longrightarrow\quad T_t = \alpha^t\\, T_0, \qquad \alpha \in (0, 1),
$$

until the final temperature $T_f$ is reached after $n_{\text{steps}} = 1 + \log(T_f/T_0)/\log\alpha$ steps, for a total of $n_{\text{steps}} \times n_{\text{iter}}$ iterations. Logarithmic schedules $T_t \propto 1/\log(t+1)$ provably reach the global minimum, but only in infinite time {{ cite(ids=["geman1984"]) }}. The geometric schedule has no such guarantee, yet in practice it gives very good cycles in reasonable time {{ cite(ids=["nourani1998"]) }}.

### Moves

Both moves replace two edges of the cycle, so the cost difference $\Delta C$ of a candidate costs $O(1)$ to compute and no cycle ever has to be summed from scratch. Writing $h$ and $k$ for the cities just before and just after the affected stretch $i \dots j$,

$$
\Delta C = d(h, j) + d(i, k) - d(h, i) - d(j, k).
$$

- **Swap** exchanges two cities adjacent in the cycle ($j = i + 1$). The moves are tiny, so many of them are needed to change the cycle's shape.
- **2-opt** {{ cite(ids=["croes1958"]) }} reverses the whole stretch between $i$ and $j$: it cuts two edges and reconnects the two resulting paths the other way around. One move can undo a crossing of two edges of any length, which is why 2-opt reaches far cheaper cycles than swap, and the gap grows with $N$.

### Domains

Cities are drawn at random from one of the following two-dimensional domains:

- **Uniform**: uniform in the unit square $[0, 1]^2$.
- **Correlated**: coordinates with correlation $\rho$ {{ cite(ids=["dasilva2020", "10.1016/j.physa.2021.126067"]) }}, mixed from two independent variables $z_1, z_2$ (uniform on $[-\tfrac12, \tfrac12]$ or normal with the same variance $\tfrac{1}{12}$):

$$
x = z_1 \sin\varphi + z_2 \cos\varphi, \qquad y = z_1 \cos\varphi + z_2 \sin\varphi, \qquad \varphi = \tfrac12 \arcsin\rho.
$$

<p style="font-size:0.85rem; color:#888;">The mixing keeps the mean and variance of each coordinate, so $\rho$ is the only thing that changes. At $\rho \to 1$ the cities collapse onto the diagonal and the TSP turns into sorting points on a line, which is easy. Tuning $\rho$ moves the instance from a hard problem to one in P.</p>

- **Power law**: each coordinate independently drawn from the two-tailed power law

$$
p(x) = \frac{\gamma - 1}{2 x_0^{1-\gamma}} \\, |x|^{-\gamma}, \qquad |x| \geq x_0,
$$

<p style="font-size:0.85rem; color:#888;">with $x_0 = 0.1$ fixed here. For $\gamma \leq 3$ the variance is infinite and a few far outliers dominate the picture. The bulk of the cities then gets squeezed into the middle of the view, which is an honest rendering of the domain rather than a glitch. Note also that $T$ has units of distance: a power-law domain has a different scale than the unit square, so the same $T$ means something different on it.</p>

### Performance

The measure of performance is the ratio $C/\langle C_0\rangle$ between the cost reached and the expected cost of a uniformly random cycle on the same cities, which is $N$ times the mean distance between two cities:

$$
\langle C_0 \rangle = \frac{2}{N - 1} \sum_{i < j} d(i, j).
$$

<p style="font-size:0.85rem; color:#888;">For the unit square this is known in closed form, $\langle C_0\rangle = \frac{N}{15}\left(2 + \sqrt2 + 5\ln(1 + \sqrt2)\right) \approx 0.52\\,N$. The shortest cycle only grows like $\sqrt N$, so good final ratios shrink as $N$ grows.</p>

This is a live version of the simulations in my M.Sc. dissertation {{ cite(ids=["10.1016/j.physa.2021.126067"]) }}, whose research code is at [eliseuv/tsp-sa](https://github.com/eliseuv/tsp-sa). Every control acts on the running simulation. The schedule runs on its own, but you can grab the current $T$ at any time to reheat or quench the cycle, or hold it fixed to watch the chain at equilibrium.

<div class="tsp-wrap" style="display:flex; flex-direction:column; gap:0.8rem; font-family:'JetBrains Mono','Fira Code',monospace; color:#aaaaaa; font-size:0.9rem; line-height:1.3;">
    <div style="display:flex; flex-wrap:wrap; gap:0.5rem; align-items:center;">
        <button id="tsp-play-pause" style="background:#111; color:#aaaaaa; border:1px solid #333; padding:0.3rem 0.9rem; font-family:inherit; font-size:1.1rem; cursor:pointer;">&#9654;</button>
        <button id="tsp-anneal" style="background:#111; color:#aaaaaa; border:1px solid #333; padding:0.3rem 0.8rem; font-family:inherit; cursor:pointer;">Anneal</button>
        <button id="tsp-randomize" style="background:#111; color:#aaaaaa; border:1px solid #333; padding:0.3rem 0.8rem; font-family:inherit; cursor:pointer;">Randomize</button>
        <span style="color:#666; margin-left:auto;">P start/stop &middot; A anneal &middot; R randomize</span>
    </div>
    <div class="tsp-controls" style="display:grid; grid-template-columns:repeat(auto-fill, minmax(170px, 1fr)); gap:0.4rem 1.2rem;">
        <div>
            <p style="margin:0 0 0.2rem;"><span data-sym="N" tabindex="0">N</span> = <output id="tsp-n-value"></output></p>
            <input type="range" id="tsp-n-slider" min="3" max="11" step="0.25" value="8" style="width:100%;">
        </div>
        <div>
            <p style="margin:0 0 0.2rem;"><span data-tooltip="Distribution the cities are drawn from. Changing it draws a new instance." tabindex="0">Domain</span></p>
            <select id="tsp-domain" style="width:100%; background:#111; color:#aaaaaa; border:1px solid #333; font-family:inherit; padding:0.2rem;">
                <option value="uniform" selected>uniform</option>
                <option value="correlated-uniform">correlated (uniform z)</option>
                <option value="correlated-normal">correlated (normal z)</option>
                <option value="power-law">power law</option>
            </select>
        </div>
        <div id="tsp-rho-row" style="display:none;">
            <p style="margin:0 0 0.2rem;"><span data-sym="\rho" tabindex="0">&rho;</span> = <output id="tsp-rho-value"></output></p>
            <input type="range" id="tsp-rho-slider" min="0" max="1" step="0.01" value="0.9" style="width:100%;">
        </div>
        <div id="tsp-gamma-row" style="display:none;">
            <p style="margin:0 0 0.2rem;"><span data-sym="\gamma" tabindex="0">&gamma;</span> = <output id="tsp-gamma-value"></output></p>
            <input type="range" id="tsp-gamma-slider" min="1.5" max="6" step="0.1" value="3.4" style="width:100%;">
        </div>
        <div>
            <p style="margin:0 0 0.2rem;"><span data-tooltip="Proposal move. Swap exchanges two adjacent cities; 2-opt reverses a whole stretch of the cycle and anneals far better." tabindex="0">Move</span></p>
            <select id="tsp-move" style="width:100%; background:#111; color:#aaaaaa; border:1px solid #333; font-family:inherit; padding:0.2rem;">
                <option value="2-opt" selected>2-opt</option>
                <option value="swap">swap</option>
            </select>
        </div>
        <div>
            <p style="margin:0 0 0.2rem;"><span data-sym="T_0" tabindex="0">T<sub>0</sub></span> = <output id="tsp-t0-value"></output></p>
            <input type="range" id="tsp-t0-slider" min="-2" max="2" step="0.1" value="0" style="width:100%;">
        </div>
        <div>
            <p style="margin:0 0 0.2rem;"><span data-sym="T_f" tabindex="0">T<sub>f</sub></span> = <output id="tsp-tf-value"></output></p>
            <input type="range" id="tsp-tf-slider" min="-8" max="-1" step="0.1" value="-4" style="width:100%;">
        </div>
        <div>
            <p style="margin:0 0 0.2rem;"><span data-sym="\alpha" tabindex="0">&alpha;</span> = <output id="tsp-alpha-value"></output></p>
            <input type="range" id="tsp-alpha-slider" min="1" max="4" step="0.05" value="1.7" style="width:100%;">
        </div>
        <div>
            <p style="margin:0 0 0.2rem;"><span data-sym="n_{\text{iter}}" tabindex="0">n<sub>iter</sub></span> = <output id="tsp-iter-value"></output></p>
            <input type="range" id="tsp-iter-slider" min="1" max="5" step="0.1" value="3.3" style="width:100%;">
        </div>
        <div>
            <p style="margin:0 0 0.2rem;"><span data-tooltip="Metropolis iterations per animation frame, i.e. playback speed." tabindex="0">Moves/frame</span> = <output id="tsp-speed-value"></output></p>
            <input type="range" id="tsp-speed-slider" min="1" max="5" step="0.1" value="3.3" style="width:100%;">
        </div>
        <div>
            <p style="margin:0 0 0.2rem; display:flex; justify-content:space-between;">
                <span><span data-sym="T" tabindex="0">T</span> = <output id="tsp-t-value"></output></span>
                <label style="cursor:pointer;"><input type="checkbox" id="tsp-hold"> <span data-tooltip="Stop cooling and keep sampling at the current T." tabindex="0">Hold</span></label>
            </p>
            <input type="range" id="tsp-t-slider" min="-8" max="2" step="0.01" value="0" style="width:100%;">
        </div>
    </div>
    <p style="color:#888; margin:0; display:flex; flex-wrap:wrap; gap:0.2rem 1.2rem;">
        <span><span data-sym="t" tabindex="0">t</span> = <output id="tsp-step-value">0</output> &rarr; <output id="tsp-state-value"></output></span>
        <span><span data-sym="C" tabindex="0">C</span> = <output id="tsp-cost-value"></output></span>
        <span><span data-sym="\langle C_0\rangle" tabindex="0">&lang;C<sub>0</sub>&rang;</span> = <output id="tsp-c0-value"></output></span>
        <span><span data-sym="C/\langle C_0\rangle" tabindex="0">C/&lang;C<sub>0</sub>&rang;</span> = <output id="tsp-ratio-value"></output></span>
        <span><span data-tooltip="Lowest C/⟨C₀⟩ seen since the last Anneal." tabindex="0">best</span> = <output id="tsp-best-value"></output></span>
        <span><span data-tooltip="Fraction of proposed moves accepted during the last frame." tabindex="0">accepted</span> = <output id="tsp-acceptance-value"></output></span>
    </p>
    <div style="display:flex; flex-wrap:wrap; gap:1rem; align-items:flex-start;">
        <div style="width:480px; max-width:100%;">
            <p style="font-size:0.85rem; margin:0 0 0.25rem;"><span data-tooltip="Current cycle: gray edges in visiting order, red dots are the cities. At high T it is a random tangle; as T drops, crossings disappear and the cycle unfolds." tabindex="0">Cycle</span></p>
            <div style="border:1px solid #333; background:#000;">
                <canvas id="tsp-tour" style="display:block; width:100%; aspect-ratio:1/1; height:auto;"></canvas>
            </div>
        </div>
        <div style="flex:1; min-width:280px; display:flex; flex-direction:column; gap:0.5rem;">
            <div>
                <p style="font-size:0.85rem; margin:0 0 0.25rem;"><span data-tooltip="One point per frame since the last Anneal, on log-log axes. Cooling moves the trace leftwards; the cost falls smoothly and then freezes at low T." tabindex="0">C/&lang;C<sub>0</sub>&rang; vs T</span> <span style="color:#666;">(log-log)</span></p>
                <div style="border:1px solid #333; background:#000;">
                    <canvas id="tsp-plot-cost-temperature" style="display:block; width:100%; height:180px;"></canvas>
                </div>
            </div>
            <div>
                <p style="font-size:0.85rem; margin:0 0 0.25rem;"><span data-tooltip="Current (gray) and best-so-far (red) cost relative to a random cycle, one sample per frame." tabindex="0"><span style="color:#aaaaaa;">C/&lang;C<sub>0</sub>&rang;</span> <span style="color:#666;">/</span> <span style="color:#ff0055;">best</span></span></p>
                <div style="border:1px solid #333; background:#000;">
                    <canvas id="tsp-plot-cost" style="display:block; width:100%; height:110px;"></canvas>
                </div>
            </div>
            <div>
                <p style="color:#ff0055; font-size:0.85rem; margin:0 0 0.25rem;"><span data-tooltip="Fraction of proposed moves accepted in each frame, on a fixed 0–1 scale. Drops toward zero as the cycle freezes." tabindex="0">Acceptance rate</span></p>
                <div style="border:1px solid #333; background:#000;">
                    <canvas id="tsp-plot-acceptance" style="display:block; width:100%; height:110px;"></canvas>
                </div>
            </div>
        </div>
    </div>
</div>

<script type="module">
    import init, { TspSimulation } from '/wasm/tsp_annealing.js';
    import { ALIVE_COLOR } from '/js/grid-canvas.js';

    async function run() {
        try {
            await init();

            const PRIMARY_COLOR = ALIVE_COLOR;
            const ACCENT_COLOR = "#ff0055";
            const AXIS_COLOR = "#333";
            const LABEL_COLOR = "#666";
            const TOUR_SIZE = 512;
            const TOUR_PADDING = 12;
            // Plots are drawn at a fixed internal resolution matching the
            // plot column, and CSS-stretched when the layout stacks.
            const PLOT_WIDTH = 480;
            const PLOT_HEIGHT = 110;
            const LOG_PLOT_HEIGHT = 180;
            const PLOT_HISTORY = 600;
            const COST_TEMPERATURE_HISTORY = 5000;
            const POWER_LAW_X0 = 0.1;

            const setUpCanvas = (id, width, height) => {
                const canvas = document.getElementById(id);
                canvas.width = width;
                canvas.height = height;
                return canvas.getContext('2d');
            };
            const tourCtx = setUpCanvas("tsp-tour", TOUR_SIZE, TOUR_SIZE);
            const costTemperatureCtx = setUpCanvas("tsp-plot-cost-temperature", PLOT_WIDTH, LOG_PLOT_HEIGHT);
            const costCtx = setUpCanvas("tsp-plot-cost", PLOT_WIDTH, PLOT_HEIGHT);
            const acceptanceCtx = setUpCanvas("tsp-plot-acceptance", PLOT_WIDTH, PLOT_HEIGHT);

            const formatTemperature = (t) => (t >= 0.01 && t < 1000) ? t.toPrecision(3) : t.toExponential(1);

            // Several sliders are on a log scale: the slider holds the
            // exponent and `toValue` maps it back. Defaults live in the HTML
            // `value` attributes.
            const SLIDERS = {
                n: { toValue: (v) => Math.round(2 ** v), format: String },
                rho: { toValue: (v) => v, format: (v) => v.toFixed(2) },
                gamma: { toValue: (v) => v, format: (v) => v.toFixed(1) },
                t0: { toValue: (v) => 10 ** v, format: formatTemperature },
                tf: { toValue: (v) => 10 ** v, format: formatTemperature },
                // Holds −log₁₀(1 − α), so resolution concentrates near 1.
                alpha: { toValue: (v) => 1 - 10 ** -v, format: (v) => v.toFixed(4) },
                iter: { toValue: (v) => Math.round(10 ** v), format: String },
                speed: { toValue: (v) => Math.round(10 ** v), format: String },
                t: { toValue: (v) => 10 ** v, format: formatTemperature },
            };
            const slider = (name) => document.getElementById(`tsp-${name}-slider`);
            const sliderValue = (name) => SLIDERS[name].toValue(parseFloat(slider(name).value));
            const readout = (name) => document.getElementById(`tsp-${name}-value`);
            const showSliderValue = (name) => {
                readout(name).textContent = SLIDERS[name].format(sliderValue(name));
            };

            const domainSelect = document.getElementById("tsp-domain");
            const moveSelect = document.getElementById("tsp-move");
            const holdCheckbox = document.getElementById("tsp-hold");

            const model = TspSimulation.new(
                sliderValue("n"),
                domainSelect.value,
                sliderValue("rho"),
                sliderValue("gamma"),
                POWER_LAW_X0,
                moveSelect.value,
                sliderValue("t0"),
                sliderValue("tf"),
                sliderValue("alpha"),
                sliderValue("iter"),
            );

            // City coordinates only change on randomize; the tour order is
            // fetched every frame.
            let pointsX = [];
            let pointsY = [];
            let toTourX = (x) => x;
            let toTourY = (y) => y;

            const cachePoints = () => {
                pointsX = model.points_x();
                pointsY = model.points_y();
                let [xMin, xMax, yMin, yMax] = [Infinity, -Infinity, Infinity, -Infinity];
                for (let i = 0; i < pointsX.length; i++) {
                    xMin = Math.min(xMin, pointsX[i]);
                    xMax = Math.max(xMax, pointsX[i]);
                    yMin = Math.min(yMin, pointsY[i]);
                    yMax = Math.max(yMax, pointsY[i]);
                }
                // One scale for both axes, so the geometry isn't distorted.
                const span = Math.max(xMax - xMin, yMax - yMin, 1e-12);
                const scale = (TOUR_SIZE - 2 * TOUR_PADDING) / span;
                const xOffset = (TOUR_SIZE - (xMax - xMin) * scale) / 2;
                const yOffset = (TOUR_SIZE - (yMax - yMin) * scale) / 2;
                toTourX = (x) => xOffset + (x - xMin) * scale;
                toTourY = (y) => TOUR_SIZE - yOffset - (y - yMin) * scale;
            };

            let costTemperatureTrail = [];
            let ratioHistory = [];
            let bestHistory = [];
            let acceptanceHistory = [];

            const pushRolling = (history, value, limit) => {
                history.push(value);
                if (history.length > limit) history.shift();
            };

            const resetHistory = () => {
                costTemperatureTrail = [];
                ratioHistory = [];
                bestHistory = [];
                acceptanceHistory = [];
                // Discard moves counted before the reset.
                model.take_acceptance_rate();
            };

            const sampleHistory = () => {
                const expected = model.expected_random_cost();
                const ratio = model.cost() / expected;
                pushRolling(costTemperatureTrail, [model.temperature(), ratio], COST_TEMPERATURE_HISTORY);
                pushRolling(ratioHistory, ratio, PLOT_HISTORY);
                pushRolling(bestHistory, model.best_cost() / expected, PLOT_HISTORY);
                pushRolling(acceptanceHistory, model.take_acceptance_rate(), PLOT_HISTORY);
            };

            const clearPanel = (ctx, width, height) => {
                ctx.fillStyle = "#000";
                ctx.fillRect(0, 0, width, height);
            };

            const drawTour = () => {
                clearPanel(tourCtx, TOUR_SIZE, TOUR_SIZE);
                const order = model.tour();
                const cityCount = order.length;
                tourCtx.strokeStyle = PRIMARY_COLOR;
                tourCtx.lineWidth = cityCount > 512 ? 0.6 : 1;
                tourCtx.beginPath();
                tourCtx.moveTo(toTourX(pointsX[order[0]]), toTourY(pointsY[order[0]]));
                for (let i = 1; i < cityCount; i++) {
                    tourCtx.lineTo(toTourX(pointsX[order[i]]), toTourY(pointsY[order[i]]));
                }
                tourCtx.closePath();
                tourCtx.stroke();

                const radius = cityCount > 512 ? 1 : 2;
                tourCtx.fillStyle = ACCENT_COLOR;
                for (let i = 0; i < cityCount; i++) {
                    tourCtx.fillRect(toTourX(pointsX[i]) - radius, toTourY(pointsY[i]) - radius, 2 * radius, 2 * radius);
                }
            };

            // Whole-decade [min, max] of log₁₀ over `values`, never empty.
            const decadeRange = (values) => {
                let low = Infinity;
                let high = -Infinity;
                for (const v of values) {
                    if (!(v > 0)) continue;
                    const exponent = Math.log10(v);
                    low = Math.min(low, exponent);
                    high = Math.max(high, exponent);
                }
                if (!isFinite(low)) return [-1, 0];
                low = Math.floor(low);
                high = Math.ceil(high);
                return high > low ? [low, high] : [low, low + 1];
            };

            const drawCostTemperature = () => {
                const ctx = costTemperatureCtx;
                clearPanel(ctx, PLOT_WIDTH, LOG_PLOT_HEIGHT);
                const [tLow, tHigh] = decadeRange(costTemperatureTrail.map(([t]) => t));
                const [rLow, rHigh] = decadeRange(costTemperatureTrail.map(([, r]) => r));
                const [left, right, top, bottom] = [44, 24, 10, 24];
                const toX = (t) => left + (Math.log10(t) - tLow) / (tHigh - tLow) * (PLOT_WIDTH - left - right);
                const toY = (r) => top + (1 - (Math.log10(r) - rLow) / (rHigh - rLow)) * (LOG_PLOT_HEIGHT - top - bottom);

                ctx.strokeStyle = AXIS_COLOR;
                ctx.lineWidth = 1;
                ctx.beginPath();
                for (let k = tLow; k <= tHigh; k++) {
                    ctx.moveTo(toX(10 ** k), top);
                    ctx.lineTo(toX(10 ** k), LOG_PLOT_HEIGHT - bottom);
                }
                for (let k = rLow; k <= rHigh; k++) {
                    ctx.moveTo(left, toY(10 ** k));
                    ctx.lineTo(PLOT_WIDTH - right, toY(10 ** k));
                }
                ctx.stroke();

                ctx.fillStyle = LABEL_COLOR;
                ctx.font = "12px 'JetBrains Mono', monospace";
                ctx.textAlign = "center";
                for (let k = tLow; k <= tHigh; k++) {
                    ctx.fillText(`1e${k}`, toX(10 ** k), LOG_PLOT_HEIGHT - 8);
                }
                ctx.textAlign = "right";
                for (let k = rLow; k <= rHigh; k++) {
                    ctx.fillText(`1e${k}`, left - 6, toY(10 ** k) + 4);
                }
                ctx.textAlign = "left";

                if (costTemperatureTrail.length === 0) return;
                ctx.fillStyle = PRIMARY_COLOR;
                for (const [t, r] of costTemperatureTrail) {
                    ctx.fillRect(toX(t) - 1, toY(r) - 1, 2, 2);
                }
                const [t, r] = costTemperatureTrail[costTemperatureTrail.length - 1];
                ctx.fillStyle = ACCENT_COLOR;
                ctx.beginPath();
                ctx.arc(toX(t), toY(r), 4, 0, 2 * Math.PI);
                ctx.fill();
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
                if (!isFinite(vMin)) return [0, 1];
                const pad = Math.max((vMax - vMin) * 0.1, 1e-3);
                return [vMin - pad, vMax + pad];
            };

            const drawSeries = (ctx, history, color, vMin, vMax) => {
                if (history.length < 2) return;
                ctx.strokeStyle = color;
                ctx.lineWidth = 1.5;
                ctx.beginPath();
                history.forEach((v, i) => {
                    const x = (i / (PLOT_HISTORY - 1)) * PLOT_WIDTH;
                    const y = PLOT_HEIGHT * (1 - (v - vMin) / (vMax - vMin));
                    if (i === 0) ctx.moveTo(x, y);
                    else ctx.lineTo(x, y);
                });
                ctx.stroke();
            };

            const drawTimeSeriesPanel = (ctx, seriesWithColors, range) => {
                clearPanel(ctx, PLOT_WIDTH, PLOT_HEIGHT);
                const [vMin, vMax] = range ?? seriesRange(seriesWithColors.map(([series]) => series));
                ctx.fillStyle = LABEL_COLOR;
                ctx.font = "12px 'JetBrains Mono', monospace";
                ctx.fillText(vMax.toPrecision(3), 4, 14);
                ctx.fillText(vMin.toPrecision(3), 4, PLOT_HEIGHT - 6);
                for (const [series, color] of seriesWithColors) drawSeries(ctx, series, color, vMin, vMax);
            };

            const drawAll = () => {
                drawTour();
                drawCostTemperature();
                drawTimeSeriesPanel(costCtx, [[ratioHistory, PRIMARY_COLOR], [bestHistory, ACCENT_COLOR]]);
                drawTimeSeriesPanel(acceptanceCtx, [[acceptanceHistory, ACCENT_COLOR]], [0, 1]);
            };

            // The temperature slider follows the model, except while the
            // user is dragging it.
            let draggingTemperature = false;

            const updateReadouts = () => {
                const temperature = model.temperature();
                const expected = model.expected_random_cost();
                if (!draggingTemperature) slider("t").value = Math.log10(temperature);
                readout("t").textContent = formatTemperature(temperature);
                readout("step").textContent = model.annealing_step();
                readout("cost").textContent = model.cost().toPrecision(4);
                readout("c0").textContent = expected.toPrecision(4);
                readout("ratio").textContent = (model.cost() / expected).toFixed(4);
                readout("best").textContent = (model.best_cost() / expected).toFixed(4);
                const acceptance = acceptanceHistory[acceptanceHistory.length - 1];
                readout("acceptance").textContent = acceptance === undefined ? "–" : acceptance.toFixed(3);

                const state = readout("state");
                if (model.is_frozen()) {
                    state.textContent = "frozen";
                    state.style.color = ACCENT_COLOR;
                } else {
                    state.textContent = holdCheckbox.checked ? "held" : "cooling";
                    state.style.color = "#888";
                }
            };

            const redraw = () => {
                drawAll();
                updateReadouts();
            };

            const showDomainParameters = () => {
                const domain = domainSelect.value;
                document.getElementById("tsp-rho-row").style.display = domain.startsWith("correlated") ? "" : "none";
                document.getElementById("tsp-gamma-row").style.display = domain === "power-law" ? "" : "none";
            };

            const randomize = () => {
                model.randomize(sliderValue("n"), domainSelect.value, sliderValue("rho"), sliderValue("gamma"), POWER_LAW_X0);
                cachePoints();
                resetHistory();
                redraw();
            };

            const anneal = () => {
                model.anneal();
                resetHistory();
                redraw();
            };

            for (const name of ["n", "rho", "gamma"]) {
                showSliderValue(name);
                slider(name).addEventListener("input", () => {
                    showSliderValue(name);
                    randomize();
                });
            }
            domainSelect.addEventListener("change", () => {
                showDomainParameters();
                randomize();
            });

            // Slider name → model setter.
            const SCHEDULE_PARAMETERS = [
                ["t0", "set_initial_temperature"],
                ["tf", "set_final_temperature"],
                ["alpha", "set_alpha"],
                ["iter", "set_iterations_per_step"],
            ];
            for (const [name, setter] of SCHEDULE_PARAMETERS) {
                showSliderValue(name);
                slider(name).addEventListener("input", () => {
                    showSliderValue(name);
                    model[setter](sliderValue(name));
                    updateReadouts();
                });
            }

            showSliderValue("speed");
            slider("speed").addEventListener("input", () => showSliderValue("speed"));

            moveSelect.addEventListener("change", () => model.set_move(moveSelect.value));

            slider("t").addEventListener("pointerdown", () => { draggingTemperature = true; });
            window.addEventListener("pointerup", () => { draggingTemperature = false; });
            slider("t").addEventListener("input", () => {
                model.set_temperature(sliderValue("t"));
                updateReadouts();
            });

            holdCheckbox.addEventListener("change", () => {
                model.set_hold(holdCheckbox.checked);
                updateReadouts();
            });

            let animationId = null;
            const isPaused = () => animationId === null;

            const renderLoop = () => {
                model.advance(sliderValue("speed"));
                sampleHistory();
                redraw();
                animationId = requestAnimationFrame(renderLoop);
            };

            const playPauseButton = document.getElementById("tsp-play-pause");

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
            document.getElementById("tsp-anneal").addEventListener("click", anneal);
            document.getElementById("tsp-randomize").addEventListener("click", randomize);

            document.addEventListener("keydown", (event) => {
                if (event.target instanceof HTMLSelectElement) return;
                if (event.code === "KeyP") playOrPause();
                else if (event.code === "KeyA") anneal();
                else if (event.code === "KeyR") randomize();
            });

            showDomainParameters();
            cachePoints();
            pause();
            resetHistory();
            redraw();

        } catch (e) {
            console.error("Failed to load TSP annealing WASM simulation:", e);
            const canvas = document.getElementById("tsp-tour");
            if (canvas) {
                canvas.style.border = "2px solid red";
                canvas.style.background = "red";
            }
        }
    }

    run();
</script>
