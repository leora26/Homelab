/*
 * Filename-derived presentation: the extension badge shown wherever a file appears, and
 * the archive test that decides between the Archive and Extract actions.
 */

/** Badge palettes defined in tokens.css. */
export type BadgeKind = "img" | "jpg" | "vid" | "zip" | "doc" | "dir";

const BADGE_BY_EXTENSION: Record<string, BadgeKind> = {
    png: "img", gif: "img", webp: "img", svg: "img",
    jpg: "jpg", jpeg: "jpg", bmp: "jpg",
    mp4: "vid", mov: "vid", mkv: "vid", avi: "vid", webm: "vid",
    zip: "zip", tar: "zip", gz: "zip", rar: "zip", "7z": "zip", bz2: "zip", xz: "zip", iso: "zip",
    pdf: "doc", txt: "doc", md: "doc", json: "doc", xml: "doc", html: "doc", css: "doc", js: "doc", rs: "doc",
};

/** Mirrors the archive extensions the backend's `File::is_archived` recognises. */
const ARCHIVE_EXTENSIONS = new Set(["zip", "gz", "tar", "rar", "7z", "bz2", "xz", "iso"]);

/** Lowercase extension without the dot, or "" when the name has none. */
export function extensionOf(name: string): string {
    const dot = name.lastIndexOf(".");
    if (dot <= 0 || dot === name.length - 1) return "";
    return name.slice(dot + 1).toLowerCase();
}

/**
 * The up-to-three-character label inside the badge. Longer extensions are truncated
 * so the 22×22 badge never has to grow — "jpeg" reads as "JPE", which is still
 * unambiguous next to the filename.
 */
export function badgeLabel(name: string): string {
    const extension = extensionOf(name);
    if (!extension) return "•";
    return extension.slice(0, 3).toUpperCase();
}

export function badgeKind(name: string): BadgeKind {
    return BADGE_BY_EXTENSION[extensionOf(name)] ?? "doc";
}

export function isArchive(name: string): boolean {
    return ARCHIVE_EXTENSIONS.has(extensionOf(name));
}

/** Caption for the details-rail preview box when no thumbnail is available. */
export function previewCaption(fileType: string): string {
    switch (fileType.toLowerCase()) {
        case "image":
            return "image preview";
        case "video":
            return "video preview";
        case "pdf":
        case "text":
            return "document preview";
        default:
            return "no preview available";
    }
}

/** Name without its extension — the portion a rename dialog should pre-select. */
export function stemOf(name: string): string {
    const dot = name.lastIndexOf(".");
    return dot > 0 ? name.slice(0, dot) : name;
}
