// Previews a reference in the floating tooltip when hovering or focusing an
// inline citation (`cite` shortcode). The content is the page's own entry in
// its reference list, so the two never disagree; clicking still jumps there.
//
// Load (deferred) after floating-tooltip.js.
FloatingTooltip.register(".cite a", (link) => {
    const entry = document.getElementById(decodeURIComponent(link.hash.slice(1)));
    return entry ? `<span class="tooltip-cite-number">[${link.textContent}]</span> ${entry.innerHTML}` : null;
}, "cite");
