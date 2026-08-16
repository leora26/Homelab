/*
 * Folder path presentation.
 *
 * `Folder::new_root` stores the owner's email as the root folder's name, so a raw
 * breadcrumb would read "leonid@mayten.com / photos". Every path shown to the user
 * renders that segment as "My Files" instead — display-time only, the stored name is
 * untouched. See REDESIGN_CONTRACT.md §C3.
 */

import type { FolderView } from "$lib/types/models";

export const ROOT_LABEL = "My Files";

export interface PathSegment {
    id: string;
    name: string;
    isRoot: boolean;
}

/** A folder is the root when it has no parent. */
export function isRoot(folder: FolderView): boolean {
    return folder.parent_folder_id === null;
}

export function toSegment(folder: FolderView): PathSegment {
    const root = isRoot(folder);
    return { id: folder.id, name: root ? ROOT_LABEL : folder.name, isRoot: root };
}

/**
 * Joins segments into the mono path shown in the details rail and the move/copy dialog.
 * The root contributes the leading slash rather than its own name, so a file at the top
 * level reads "/report.pdf" instead of "/My Files/report.pdf".
 */
export function pathString(segments: PathSegment[]): string {
    const named = segments.filter((segment) => !segment.isRoot).map((segment) => segment.name);
    return `/${named.join("/")}`;
}

/** Path with a filename appended — the "Path" row in the details rail. */
export function filePathString(segments: PathSegment[], fileName: string): string {
    const base = pathString(segments);
    return base === "/" ? `/${fileName}` : `${base}/${fileName}`;
}

/**
 * Shortens a path from the left, keeping the deepest segments — those identify the
 * location, while the ancestors closer to the root are the least distinguishing part.
 * Used by Trash's narrow "Original location" column.
 */
export function truncatePath(path: string, maxSegments = 2): string {
    const parts = path.split("/").filter(Boolean);
    if (parts.length <= maxSegments) return `/${parts.join("/")}`;
    return `…/${parts.slice(-maxSegments).join("/")}`;
}
