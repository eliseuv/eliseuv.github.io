// Per-page symbol descriptions shown as tooltips.
//
// A page declares its symbols in front matter, keyed by TeX pattern:
//
//     [extra.symbols]
//     'T_c'   = 'Critical temperature $2/\ln(1+\sqrt{2})$.'
//     '\beta' = 'Inverse temperature $1/T$.'
//
// Every occurrence in math gets a tooltip unless written as `\nosym{...}`.
// HTML labels opt in with `<span data-sym="T_c" tabindex="0">`.
//
// Math is annotated by rewriting the TeX before KaTeX renders it, not by
// matching rendered glyphs, which are ambiguous (`T` in `T_c` vs. `T`).
// Rendered symbols carry an index (`data-symbol`) rather than the key,
// because keys contain characters (`\`, `%`) unsafe in a KaTeX argument.
//
// Tooltips are shown by floating-tooltip.js. Load (deferred) after it and
// before KaTeX auto-render, and render with
// `renderMathInElement(el, PageSymbols.katexOptions(options))`.
(() => {
    const registryElement = document.getElementById("page-symbols");
    const registry = registryElement ? JSON.parse(registryElement.textContent) : {};
    const keys = Object.keys(registry);

    // Arguments of these commands are text, names or a different symbol
    // (`\mathbf{s}` is not `s`), so they are never annotated.
    const SKIPPED_ARGUMENT_COMMANDS = new Set([
        "\\nosym", "\\text", "\\textrm", "\\textit", "\\textbf", "\\textsf",
        "\\texttt", "\\mbox", "\\hbox", "\\mathrm", "\\operatorname",
        "\\mathbf", "\\boldsymbol", "\\mathcal", "\\mathbb", "\\mathsf",
        "\\mathit", "\\mathfrak", "\\begin", "\\end", "\\color",
        "\\textcolor", "\\htmlData", "\\htmlClass", "\\label", "\\tag",
    ]);

    const TOKEN = /\\[a-zA-Z]+|\\[\s\S]|\s+|[\s\S]/g;

    // Significant tokens with their source offsets. A brace group holding a
    // single token collapses into that token, so `T_{c}` matches `T_c`.
    const toItems = (tex) => {
        const tokens = [...tex.matchAll(TOKEN)]
            .filter((m) => !/^\s/.test(m[0]))
            .map((m) => ({ norm: m[0], start: m.index, end: m.index + m[0].length }));
        const items = [];
        for (let i = 0; i < tokens.length; i++) {
            const inner = tokens[i + 1];
            if (tokens[i].norm === "{" && tokens[i + 2]?.norm === "}" && inner.norm !== "{" && inner.norm !== "}") {
                items.push({ norm: inner.norm, start: tokens[i].start, end: tokens[i + 2].end });
                i += 2;
            } else {
                items.push(tokens[i]);
            }
        }
        return items;
    };

    // Longest first, so `T_c` wins over `T`.
    const patterns = keys
        .map((key, index) => ({ index, norms: toItems(key).map((item) => item.norm) }))
        .filter((pattern) => pattern.norms.length > 0)
        .sort((a, b) => b.norms.length - a.norms.length);

    const skipArgument = (items, i) => {
        if (items[i]?.norm !== "{") return i + 1;
        let depth = 0;
        for (; i < items.length; i++) {
            if (items[i].norm === "{") depth++;
            else if (items[i].norm === "}" && --depth === 0) return i + 1;
        }
        return i;
    };

    // The outer braces keep the result valid as an unbraced script
    // (`x_T`), where KaTeX rejects a bare command taking arguments.
    const annotate = (tex) => {
        const items = toItems(tex);
        let annotated = "";
        let cursor = 0;
        for (let i = 0; i < items.length;) {
            const hit = patterns.find((pattern) =>
                pattern.norms.every((norm, k) => items[i + k]?.norm === norm));
            if (hit) {
                const start = items[i].start;
                const end = items[i + hit.norms.length - 1].end;
                annotated += `${tex.slice(cursor, start)}{\\htmlData{symbol=${hit.index}}{${tex.slice(start, end)}}}`;
                cursor = end;
                i += hit.norms.length;
            } else {
                i = SKIPPED_ARGUMENT_COMMANDS.has(items[i].norm) ? skipArgument(items, i + 1) : i + 1;
            }
        }
        return annotated + tex.slice(cursor);
    };

    let baseOptions = {};

    const katexOptions = (options) => {
        baseOptions = options;
        return {
            ...options,
            preProcess: patterns.length > 0 ? annotate : undefined,
            macros: { "\\nosym": "#1" },
            trust: (context) => context.command === "\\htmlData",
            strict: (code) => (code === "htmlExtension" ? "ignore" : "warn"),
        };
    };

    const keyOf = (anchor) =>
        anchor.dataset.sym ?? keys[Number(anchor.dataset.symbol)];

    // Descriptions are inserted as text and only then math-rendered, so
    // front matter can't inject markup. Rendered once per key.
    const renderedDescriptions = new Map();
    const renderDescription = (key) => {
        if (!renderedDescriptions.has(key)) {
            const scratch = document.createElement("div");
            scratch.textContent = registry[key];
            if (window.renderMathInElement) {
                renderMathInElement(scratch, { ...baseOptions, throwOnError: false });
            }
            renderedDescriptions.set(key, scratch.innerHTML);
        }
        return renderedDescriptions.get(key);
    };

    const warnUndescribed = () => {
        for (const anchor of document.querySelectorAll("[data-sym]")) {
            if (!(anchor.dataset.sym in registry)) {
                console.warn(`No description for symbol "${anchor.dataset.sym}" in [extra.symbols].`);
            }
        }
    };

    if (keys.length > 0) {
        FloatingTooltip.register("[data-sym], [data-symbol]", (anchor) => {
            const key = keyOf(anchor);
            return key in registry ? renderDescription(key) : null;
        }, "symbol");
        if (document.readyState === "loading") document.addEventListener("DOMContentLoaded", warnUndescribed);
        else warnUndescribed();
    }

    window.PageSymbols = { katexOptions };
})();
