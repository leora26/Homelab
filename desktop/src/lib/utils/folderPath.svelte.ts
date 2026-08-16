/*
 * Breadcrumb and path resolution.
 *
 * There is no "give me the ancestors" RPC, so the chain is walked one `get_folder` call
 * at a time. Depth is small in practice and results are cached, so a breadcrumb is
 * usually zero round trips after the first visit.
 */

import { safeInvoke } from "$lib/utils/safeInvoke";
import { toSegment, type PathSegment } from "$lib/utils/paths";
import type { FolderView } from "$lib/types/models";

/** Folders change rarely; a rename or delete clears the affected entry. */
const cache = new Map<string, FolderView>();

export async function loadFolder(folderId: string): Promise<FolderView | null> {
    const cached = cache.get(folderId);
    if (cached) return cached;

    const result = await safeInvoke<FolderView>("get_folder", { folderId });
    if (!result.ok) return null;

    cache.set(folderId, result.data);
    return result.data;
}

/**
 * Root-first chain ending at `folderId` itself.
 *
 * Guards against a cycle in `parent_folder_id` — a malformed tree would otherwise spin
 * here forever rather than surfacing as a bad breadcrumb.
 */
export async function ancestorsOf(folderId: string): Promise<PathSegment[]> {
    const chain: PathSegment[] = [];
    const visited = new Set<string>();

    let currentId: string | null = folderId;

    while (currentId && !visited.has(currentId)) {
        visited.add(currentId);

        const folder: FolderView | null = await loadFolder(currentId);
        if (!folder) break;

        chain.unshift(toSegment(folder));
        currentId = folder.parent_folder_id;
    }

    return chain;
}

/** Drops cached entries so the next breadcrumb reflects a rename, move or delete. */
export function forgetFolder(folderId: string): void {
    cache.delete(folderId);
}

export function forgetAllFolders(): void {
    cache.clear();
}

/**
 * Bumped whenever the folder structure changes, so mounted tree nodes refetch their
 * children instead of showing a stale cache.
 */
class TreeRevision {
    value = $state(0);

    bump = () => {
        forgetAllFolders();
        this.value++;
    };
}

export const treeRevision = new TreeRevision();
