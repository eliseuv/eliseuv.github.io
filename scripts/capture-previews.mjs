// Records the landing page previews of the simulations: builds and serves the site,
// runs each simulation in headless Chromium through the DevTools protocol and
// encodes the frames with img2webp. Writes <name>.anim.webp for the animation
// and <name>.webp, its last frame, which the cards show until hovered.
//
// Usage: node capture-previews.mjs <output dir> [name...]
// Run from the repository root with the simulations built (static/wasm/); needs
// `zola`, `chromium` and `img2webp` on PATH (see the capture-previews flake app).

import { spawn, execFileSync } from "node:child_process";
import { createServer } from "node:http";
import { mkdtempSync, readFileSync, writeFileSync, rmSync, statSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, extname, normalize } from "node:path";

// The frame is cropped to the union of the canvases' titled wrappers (canvas ->
// bordered div -> block with the title), so the controls stay out of it.
const SIMULATIONS = {
    spectral_criticality: {
        path: "/simulations/spectral-criticality/",
        canvases: ["spectral-lattice", "spectral-series", "spectral-correlations", "spectral-correlation-hist",
            "spectral-spectrum", "spectral-scan-var", "spectral-scan-max", "spectral-scan-max-var"],
        clicks: ["spectral-play-pause"],
        warmupSeconds: 4,
        seconds: 6,
    },
    chiarella: {
        path: "/simulations/chiarella/",
        canvases: ["chiarella-phase", "chiarella-plot-price", "chiarella-plot-mispricing",
            "chiarella-plot-returns", "chiarella-plot-histogram"],
        clicks: ["chiarella-play-pause"],
        warmupSeconds: 3,
        seconds: 6,
    },
    tsp_annealing: {
        path: "/simulations/tsp-annealing/",
        canvases: ["tsp-tour", "tsp-plot-cost-temperature", "tsp-plot-cost", "tsp-plot-acceptance"],
        clicks: ["tsp-play-pause"],
        // Starts from the random tour; the anneal has converged by the end
        warmupSeconds: 0.3,
        seconds: 5,
    },
};

// Matches the stills the cards were laid out for
const OUTPUT_WIDTH = 1050;
const FRAME_INTERVAL_MS = 1000 / 12;
const QUALITY = 70;

const MIME_TYPES = {
    ".html": "text/html", ".js": "text/javascript", ".mjs": "text/javascript", ".css": "text/css",
    ".wasm": "application/wasm", ".json": "application/json", ".svg": "image/svg+xml",
    ".png": "image/png", ".webp": "image/webp", ".woff2": "font/woff2", ".bin": "application/octet-stream",
};

const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

function serve(root) {
    const server = createServer((request, response) => {
        let path = normalize(decodeURIComponent(new URL(request.url, "http://localhost").pathname));
        if (path.endsWith("/")) path += "index.html";
        const file = join(root, path);
        try {
            const body = readFileSync(statSync(file).isDirectory() ? join(file, "index.html") : file);
            response.writeHead(200, { "Content-Type": MIME_TYPES[extname(file)] ?? "application/octet-stream" });
            response.end(body);
        } catch {
            response.writeHead(404).end();
        }
    });
    return new Promise((resolve) => server.listen(0, "127.0.0.1", () => resolve(server)));
}

function launchChromium(profileDir) {
    const chromium = spawn("chromium", [
        "--headless=new", "--remote-debugging-port=0", `--user-data-dir=${profileDir}`,
        "--hide-scrollbars", "--mute-audio", "--no-first-run", "--no-default-browser-check",
    ], { stdio: ["ignore", "ignore", "pipe"] });
    return new Promise((resolve, reject) => {
        let log = "";
        chromium.stderr.on("data", (chunk) => {
            log += chunk;
            const match = log.match(/DevTools listening on (ws:\/\/\S+)/);
            if (match) resolve({ chromium, endpoint: match[1] });
        });
        chromium.on("exit", (code) => reject(new Error(`chromium exited with ${code}:\n${log}`)));
    });
}

// Minimal DevTools protocol client over one flattened browser connection
async function connect(endpoint) {
    const socket = new WebSocket(endpoint);
    await new Promise((resolve, reject) => {
        socket.onopen = resolve;
        socket.onerror = reject;
    });
    let nextId = 1;
    const pending = new Map();
    socket.onmessage = ({ data }) => {
        const message = JSON.parse(data);
        // Page errors explain a widget that never becomes ready
        if (message.method === "Runtime.exceptionThrown") console.error("page:", message.params.exceptionDetails.exception?.description);
        if (message.method === "Runtime.consoleAPICalled" && message.params.type === "error") {
            console.error("page:", message.params.args.map((arg) => arg.value ?? arg.description).join(" "));
        }
        const handlers = pending.get(message.id);
        if (!handlers) return;
        pending.delete(message.id);
        if (message.error) handlers.reject(new Error(`${handlers.method}: ${message.error.message}`));
        else handlers.resolve(message.result);
    };
    const send = (method, params = {}, sessionId) => new Promise((resolve, reject) => {
        const id = nextId++;
        pending.set(id, { resolve, reject, method });
        socket.send(JSON.stringify({ id, method, params, sessionId }));
    });
    return { send, close: () => socket.close() };
}

async function evaluate(send, sessionId, expression) {
    const { result, exceptionDetails } = await send("Runtime.evaluate",
        { expression, awaitPromise: true, returnByValue: true }, sessionId);
    if (exceptionDetails) throw new Error(`${expression}: ${exceptionDetails.exception?.description ?? exceptionDetails.text}`);
    return result.value;
}

async function capture(send, origin, name, simulation, outputDir) {
    const { targetId } = await send("Target.createTarget", { url: "about:blank" });
    const { sessionId } = await send("Target.attachToTarget", { targetId, flatten: true });
    // Tall enough that every widget is laid out on screen at once
    const setScale = (deviceScaleFactor) => send("Emulation.setDeviceMetricsOverride",
        { width: 1200, height: 3000, deviceScaleFactor, mobile: false }, sessionId);
    await setScale(1);
    await send("Page.enable", {}, sessionId);
    await send("Runtime.enable", {}, sessionId);
    await send("Page.navigate", { url: origin + simulation.path }, sessionId);

    // The widgets create their WASM module asynchronously; wait until the play button is wired
    const firstButton = simulation.clicks[0];
    await evaluate(send, sessionId, `new Promise((resolve, reject) => {
        const started = Date.now();
        const poll = () => {
            if (document.readyState === "complete" && document.getElementById(${JSON.stringify(firstButton)})?.textContent.trim() === "▶") resolve();
            else if (Date.now() - started > 20000) reject(new Error("widget did not load: readyState " + document.readyState + ", button " + JSON.stringify(document.getElementById(${JSON.stringify(firstButton)})?.textContent)));
            else setTimeout(poll, 100);
        };
        poll();
    })`);

    const measure = () => evaluate(send, sessionId, `(() => {
        const ids = ${JSON.stringify(simulation.canvases)};
        const blocks = ids.map((id) => {
            const canvas = document.getElementById(id);
            if (!canvas) throw new Error("missing canvas " + id);
            return canvas.parentElement.parentElement;
        });
        blocks[0].scrollIntoView({ block: "start" });
        const rects = blocks.map((block) => block.getBoundingClientRect());
        const left = Math.min(...rects.map((r) => r.left)), top = Math.min(...rects.map((r) => r.top));
        const right = Math.max(...rects.map((r) => r.right)), bottom = Math.max(...rects.map((r) => r.bottom));
        return { x: left + scrollX, y: top + scrollY, width: right - left, height: bottom - top };
    })()`);
    // Scaling through the screenshot clip re-emulates the viewport on every capture, which fires
    // `resize` and clears the canvases the widgets draw on incrementally. Set the scale once instead.
    await setScale(OUTPUT_WIDTH / (await measure()).width);
    const region = await measure();
    const clip = { ...region, scale: 1 };

    for (const id of simulation.clicks) await evaluate(send, sessionId, `document.getElementById(${JSON.stringify(id)}).click()`);
    await sleep(simulation.warmupSeconds * 1000);

    const framesDir = mkdtempSync(join(tmpdir(), `frames-${name}-`));
    const frames = [];
    const start = performance.now();
    while (performance.now() - start < simulation.seconds * 1000) {
        const shotAt = performance.now();
        const { data } = await send("Page.captureScreenshot",
            { format: "png", clip, captureBeyondViewport: false, fromSurface: true }, sessionId);
        const file = join(framesDir, `${String(frames.length).padStart(4, "0")}.png`);
        writeFileSync(file, Buffer.from(data, "base64"));
        frames.push({ file, at: shotAt });
        const wait = FRAME_INTERVAL_MS - (performance.now() - shotAt);
        if (wait > 0) await sleep(wait);
    }
    await send("Target.closeTarget", { targetId });

    // Each frame is shown for as long as it actually took to the next capture, so playback runs in real time
    const args = ["-loop", "0"];
    frames.forEach((frame, index) => {
        const next = frames[index + 1]?.at ?? frame.at + FRAME_INTERVAL_MS;
        args.push("-d", String(Math.round(next - frame.at)), "-lossy", "-q", String(QUALITY), "-m", "6", frame.file);
    });
    const animation = join(outputDir, `${name}.anim.webp`);
    execFileSync("img2webp", [...args, "-o", animation], { stdio: ["ignore", "ignore", "inherit"] });
    execFileSync("img2webp", ["-lossy", "-q", String(QUALITY), "-m", "6", frames.at(-1).file, "-o", join(outputDir, `${name}.webp`)],
        { stdio: ["ignore", "ignore", "inherit"] });
    rmSync(framesDir, { recursive: true, force: true });

    const kib = (file) => `${Math.round(statSync(file).size / 1024)} KiB`;
    console.log(`${name}: ${frames.length} frames, ${Math.round(region.width)}x${Math.round(region.height)} css px, ${kib(animation)}`);
}

async function main() {
    const [outputDir, ...only] = process.argv.slice(2);
    if (!outputDir) throw new Error("usage: capture-previews.mjs <output dir> [name...]");
    const names = only.length ? only : Object.keys(SIMULATIONS);
    for (const name of names) if (!SIMULATIONS[name]) throw new Error(`unknown simulation ${name}`);

    // Built for the local origin, so stylesheets and assets are not fetched from production
    const siteDir = mkdtempSync(join(tmpdir(), "capture-site-"));
    const server = await serve(siteDir);
    const origin = `http://127.0.0.1:${server.address().port}`;
    execFileSync("zola", ["build", "--base-url", origin, "--output-dir", siteDir, "--force"], { stdio: ["ignore", "ignore", "inherit"] });
    const profileDir = mkdtempSync(join(tmpdir(), "capture-previews-"));
    const { chromium, endpoint } = await launchChromium(profileDir);
    const { send, close } = await connect(endpoint);
    try {
        for (const name of names) await capture(send, origin, name, SIMULATIONS[name], outputDir);
    } finally {
        close();
        // The profile can only be removed once Chromium stops writing to it
        const exited = new Promise((resolve) => chromium.once("exit", resolve));
        chromium.kill();
        await exited;
        server.close();
        rmSync(profileDir, { recursive: true, force: true });
        rmSync(siteDir, { recursive: true, force: true });
    }
}

main().catch((error) => {
    console.error(error.message);
    process.exit(1);
});
