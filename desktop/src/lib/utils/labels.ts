/*
 * Label colour handling.
 *
 * The design fixes six preset colours so chips stay readable in tables. Labels created
 * before the redesign came from a free colour wheel, so any stored colour is mapped to
 * its nearest preset at display time — see REDESIGN_CONTRACT.md §C3. Nothing here
 * changes stored data; the raw hex stays in the database.
 */

export type PresetKey = "amber" | "green" | "blue" | "purple" | "red" | "slate";

export interface LabelPalette {
    /** The value written to `labels.color` when this preset is chosen. */
    swatch: string;
    /** Chip background, text and border, and the dot in the file table. */
    bg: string;
    text: string;
    border: string;
}

export const LABEL_PRESETS: Record<PresetKey, LabelPalette> = {
    amber: { swatch: "#e0a341", bg: "#2b2211", text: "#e0a341", border: "#4a3a1c" },
    green: { swatch: "#3ddc97", bg: "#12291f", text: "#3ddc97", border: "#1f4033" },
    blue: { swatch: "#3b6fff", bg: "#1c2536", text: "#5b9dff", border: "#27385a" },
    purple: { swatch: "#a86cff", bg: "#241d33", text: "#a86cff", border: "#3a2e52" },
    red: { swatch: "#ff7a8a", bg: "#31191f", text: "#ff7a8a", border: "#3d2229" },
    slate: { swatch: "#7b8194", bg: "#1e232e", text: "#7b8194", border: "#2b3342" },
};

/** Swatch order as drawn in the New/Edit label dialog. */
export const PRESET_ORDER: PresetKey[] = ["amber", "green", "blue", "purple", "red", "slate"];

function parseHex(hex: string): [number, number, number] | null {
    const cleaned = hex.trim().replace(/^#/, "");

    const expanded =
        cleaned.length === 3
            ? cleaned.split("").map((c) => c + c).join("")
            : cleaned;

    if (!/^[0-9a-fA-F]{6}$/.test(expanded)) return null;

    return [
        parseInt(expanded.slice(0, 2), 16),
        parseInt(expanded.slice(2, 4), 16),
        parseInt(expanded.slice(4, 6), 16),
    ];
}

/**
 * Closest preset to an arbitrary colour, by squared distance in RGB.
 *
 * RGB distance is crude next to a perceptual space, but the six presets are far enough
 * apart in hue that it never picks a surprising neighbour — and it keeps this dependency-free.
 */
export function nearestPreset(color: string): PresetKey {
    const rgb = parseHex(color);
    if (!rgb) return "slate";

    let best: PresetKey = "slate";
    let bestDistance = Number.POSITIVE_INFINITY;

    for (const key of PRESET_ORDER) {
        const target = parseHex(LABEL_PRESETS[key].swatch);
        if (!target) continue;

        const distance =
            (rgb[0] - target[0]) ** 2 +
            (rgb[1] - target[1]) ** 2 +
            (rgb[2] - target[2]) ** 2;

        if (distance < bestDistance) {
            bestDistance = distance;
            best = key;
        }
    }

    return best;
}

/** Palette a stored label colour should render with. */
export function paletteFor(color: string): LabelPalette {
    return LABEL_PRESETS[nearestPreset(color)];
}
