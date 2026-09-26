// One floating tooltip shared by every kind of anchor on the page, positioned
// by script since anchors sit mid-line (often in math) where a CSS popover
// would overflow or clip.
//
// A source registers the anchors it owns and how to fill the tooltip:
//
//     FloatingTooltip.register("[data-sym]", (anchor) => html ?? null, "symbol");
//
// `contentOf` returns trusted HTML, or null to show nothing. The kind lands
// on the tooltip as `data-kind`, for per-source styling.
//
// Load (deferred) before the scripts that register.
(() => {
    const tooltip = document.createElement("div");
    tooltip.id = "floating-tooltip";
    tooltip.className = "floating-tooltip";
    tooltip.setAttribute("role", "tooltip");

    const sources = [];

    const VIEWPORT_MARGIN = 8;
    const ANCHOR_GAP = 6;
    let activeAnchor = null;

    // Centered above the anchor, clamped to the viewport; flips below when
    // there is no room above.
    const position = (anchor) => {
        const rect = anchor.getBoundingClientRect();
        // Layout size: the bounding rect would include the spawn scale.
        const { offsetWidth: width, offsetHeight: height } = tooltip;
        const left = Math.min(
            Math.max(rect.left + rect.width / 2 - width / 2, VIEWPORT_MARGIN),
            window.innerWidth - width - VIEWPORT_MARGIN,
        );
        const above = rect.top - height - ANCHOR_GAP;
        const fitsAbove = above >= VIEWPORT_MARGIN;
        const top = fitsAbove ? above : rect.bottom + ANCHOR_GAP;
        tooltip.classList.toggle("below", !fitsAbove);
        tooltip.style.left = `${Math.max(left, VIEWPORT_MARGIN)}px`;
        tooltip.style.top = `${top}px`;
    };

    const sourceOf = (anchor) => sources.find((source) => anchor.matches(source.selector));

    const show = (anchor) => {
        const source = sourceOf(anchor);
        const content = source?.contentOf(anchor);
        if (content == null) return;
        hide();
        tooltip.dataset.kind = source.kind;
        tooltip.innerHTML = content;
        position(anchor);
        // Flush the removal of `visible` so the spawn animation restarts.
        void tooltip.offsetWidth;
        tooltip.classList.add("visible");
        anchor.classList.add("tooltip-active");
        anchor.setAttribute("aria-describedby", tooltip.id);
        activeAnchor = anchor;
    };

    const hide = () => {
        if (!activeAnchor) return;
        tooltip.classList.remove("visible");
        activeAnchor.classList.remove("tooltip-active");
        activeAnchor.removeAttribute("aria-describedby");
        activeAnchor = null;
    };

    const anchorOf = (target) =>
        target instanceof Element && sources.length > 0
            ? target.closest(sources.map((source) => source.selector).join(", "))
            : null;

    const listen = () => {
        document.body.append(tooltip);

        // Mouse only: a tap also fires `pointerover`, and would then be
        // toggled straight back off by the `click` handler below.
        document.addEventListener("pointerover", (event) => {
            const anchor = anchorOf(event.target);
            if (event.pointerType === "mouse" && anchor && anchor !== activeAnchor) show(anchor);
        });
        document.addEventListener("pointerout", (event) => {
            const anchor = anchorOf(event.target);
            if (event.pointerType === "mouse" && anchor && !anchor.contains(event.relatedTarget)) hide();
        });
        // Tap toggles. Hover already covers the mouse, and a focusable label
        // was just opened by the `focusin` of this same tap, so neither
        // toggles closed here; tapping elsewhere closes them.
        document.addEventListener("click", (event) => {
            const anchor = anchorOf(event.target);
            if (!anchor) hide();
            else if (anchor !== activeAnchor) show(anchor);
            else if (event.pointerType !== "mouse" && !anchor.matches(":focus")) hide();
        });
        document.addEventListener("focusin", (event) => {
            const anchor = anchorOf(event.target);
            if (anchor) show(anchor);
        });
        document.addEventListener("focusout", (event) => {
            if (anchorOf(event.target) === activeAnchor) hide();
        });
        document.addEventListener("keydown", (event) => {
            if (event.key === "Escape") hide();
        });
        // Fixed positioning would leave it behind as the page scrolls.
        window.addEventListener("scroll", hide, { capture: true, passive: true });
        window.addEventListener("resize", hide);
    };

    const register = (selector, contentOf, kind) => {
        sources.push({ selector, contentOf, kind });
        if (sources.length > 1) return;
        if (document.readyState === "loading") document.addEventListener("DOMContentLoaded", listen);
        else listen();
    };

    window.FloatingTooltip = { register };
})();
