/*
 * Display formatting. Everything here takes raw values from the Tauri layer — byte
 * counts and unix seconds — and turns them into the strings the design specifies.
 */

const UNITS = ["B", "KB", "MB", "GB", "TB", "PB"] as const;

/**
 * Byte count in the design's style: "249 KB", "6.3 GB", "1.39 TB".
 *
 * Precision follows the mocks rather than a fixed decimal count — large values read as
 * whole numbers ("412 GB", not "412.00 GB") and smaller ones keep up to two decimals
 * with trailing zeros stripped ("6.3 GB", not "6.30 GB").
 */
export function formatBytes(bytes: number | null | undefined): string {
    if (bytes === null || bytes === undefined || Number.isNaN(bytes)) return "—";
    if (bytes === 0) return "0 B";

    const negative = bytes < 0;
    let value = Math.abs(bytes);
    let unit = 0;

    while (value >= 1024 && unit < UNITS.length - 1) {
        value /= 1024;
        unit++;
    }

    const rendered =
        value >= 100
            ? Math.round(value).toString()
            : parseFloat(value.toFixed(2)).toString();

    return `${negative ? "-" : ""}${rendered} ${UNITS[unit]}`;
}

/** Splits a byte count into its number and unit, for the metric cards that style them apart. */
export function splitBytes(bytes: number | null | undefined): { value: string; unit: string } {
    const parts = formatBytes(bytes).split(" ");
    return { value: parts[0] ?? "—", unit: parts[1] ?? "" };
}

const MONTHS = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];

function toDate(unixSeconds: number | null | undefined): Date | null {
    if (unixSeconds === null || unixSeconds === undefined || !Number.isFinite(unixSeconds)) {
        return null;
    }
    const date = new Date(unixSeconds * 1000);
    return Number.isNaN(date.getTime()) ? null : date;
}

/** "12 Aug 2026" — the table-column format. Fixed width so mono columns stay aligned. */
export function formatDate(unixSeconds: number | null | undefined): string {
    const date = toDate(unixSeconds);
    if (!date) return "—";
    const day = date.getDate().toString().padStart(2, "0");
    return `${day} ${MONTHS[date.getMonth()]} ${date.getFullYear()}`;
}

/** "12 Aug 2026, 14:32" — used in the details rail where there's room for the time. */
export function formatDateTime(unixSeconds: number | null | undefined): string {
    const date = toDate(unixSeconds);
    if (!date) return "—";
    const hours = date.getHours().toString().padStart(2, "0");
    const minutes = date.getMinutes().toString().padStart(2, "0");
    return `${formatDate(unixSeconds)}, ${hours}:${minutes}`;
}

/**
 * "just now" / "14 minutes ago" / "3 days ago", falling back to an absolute date past a
 * month, where relative phrasing stops being useful.
 */
export function formatRelative(unixSeconds: number | null | undefined, now = Date.now()): string {
    const date = toDate(unixSeconds);
    if (!date) return "—";

    const seconds = Math.floor((now - date.getTime()) / 1000);
    if (seconds < 0) return formatDate(unixSeconds);
    if (seconds < 45) return "just now";

    const minutes = Math.floor(seconds / 60);
    if (minutes < 60) return `${minutes} minute${minutes === 1 ? "" : "s"} ago`;

    const hours = Math.floor(minutes / 60);
    if (hours < 24) return `${hours} hour${hours === 1 ? "" : "s"} ago`;

    const days = Math.floor(hours / 24);
    if (days < 30) return `${days} day${days === 1 ? "" : "s"} ago`;

    return formatDate(unixSeconds);
}

/** "up 14d 03h" — the machine uptime in the titlebar meta line. */
export function formatUptime(seconds: number | null | undefined): string {
    if (seconds === null || seconds === undefined || seconds < 0) return "up —";

    const days = Math.floor(seconds / 86400);
    const hours = Math.floor((seconds % 86400) / 3600);

    if (days > 0) return `up ${days}d ${hours.toString().padStart(2, "0")}h`;

    const minutes = Math.floor((seconds % 3600) / 60);
    if (hours > 0) return `up ${hours}h ${minutes.toString().padStart(2, "0")}m`;

    return `up ${minutes}m`;
}

/** "3,164" — thousands separators for the metric cards and table footers. */
export function formatCount(value: number | null | undefined): string {
    if (value === null || value === undefined || Number.isNaN(value)) return "—";
    return value.toLocaleString("en-US");
}

/** "5 items" / "1 item" — table footers and the tree's trash row. */
export function pluralise(count: number, singular: string, plural = `${singular}s`): string {
    return `${formatCount(count)} ${count === 1 ? singular : plural}`;
}

/** Percentage clamped to 0–100, for quota bars where taken can exceed allowed. */
export function percentOf(taken: number, allowed: number): number {
    if (!allowed || allowed <= 0) return 0;
    return Math.min(100, Math.max(0, (taken / allowed) * 100));
}
