// Shared canvas setup/rendering for this site's lattice simulations:
// uniform gray/black cells, no cell-division markings, drawn at 1 device
// pixel per site and CSS-scaled (with `image-rendering: pixelated`) to a
// fixed on-screen size, so switching grid dimensions never changes the
// display footprint.

export const ALIVE_COLOR = "#aaaaaa";
export const DEAD_COLOR = "#000000";

// One-time canvas setup: fixed CSS display size, pixelated scaling.
// Returns the 2D rendering context.
export function setUpGridCanvas(canvas, displaySize) {
    canvas.style.width = `${displaySize}px`;
    canvas.style.height = `${displaySize}px`;
    canvas.style.imageRendering = "pixelated";
    return canvas.getContext('2d');
}

// Resize the canvas's internal resolution to one pixel per site.
export function resizeGrid(canvas, nrows, ncols) {
    canvas.width = ncols;
    canvas.height = nrows;
}

// Fill the canvas from a row-major buffer, one entry per site. `isAlive`
// decides whether a site's raw value counts as "on" (defaults to a plain
// truthy check, which fits 0/1 buffers; pass e.g. `v => v > 0` for a
// signed +1/-1 buffer).
export function drawGrid(ctx, cells, nrows, ncols, {
    aliveColor = ALIVE_COLOR,
    deadColor = DEAD_COLOR,
    isAlive = (v) => !!v,
} = {}) {
    for (let row = 0; row < nrows; row++) {
        for (let col = 0; col < ncols; col++) {
            const idx = row * ncols + col;
            ctx.fillStyle = isAlive(cells[idx]) ? aliveColor : deadColor;
            ctx.fillRect(col, row, 1, 1);
        }
    }
}

// Map a mouse/pointer event to a (row, col) cell, accounting for the
// canvas's CSS scale factor. Clamped to the grid bounds.
export function cellFromEvent(event, canvas, nrows, ncols) {
    const rect = canvas.getBoundingClientRect();
    const scaleX = canvas.width / rect.width;
    const scaleY = canvas.height / rect.height;
    const x = (event.clientX - rect.left) * scaleX;
    const y = (event.clientY - rect.top) * scaleY;
    const row = Math.min(Math.max(Math.floor(y), 0), nrows - 1);
    const col = Math.min(Math.max(Math.floor(x), 0), ncols - 1);
    return { row, col };
}
