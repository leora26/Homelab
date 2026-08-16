<script lang="ts">
    import { Trash2, TriangleAlert } from "@lucide/svelte";

    import Button from "$lib/components/ui/Button.svelte";
    import EmptyState from "$lib/components/ui/EmptyState.svelte";
    import ExtBadge from "$lib/components/ui/ExtBadge.svelte";
    import ConfirmDialog from "$lib/components/dialogs/ConfirmDialog.svelte";

    import { safeInvoke } from "$lib/utils/safeInvoke";
    import { toasts } from "$lib/stores/toasts.svelte";
    import { session } from "$lib/stores/session.svelte";
    import { ancestorsOf } from "$lib/utils/folderPath.svelte";
    import { pathString, truncatePath } from "$lib/utils/paths";
    import { formatBytes, formatDate, pluralise } from "$lib/utils/format";
    import type { FileView, FolderView } from "$lib/types/models";

    interface Row {
        kind: "file" | "folder";
        id: string;
        name: string;
        /** Folders have no size of their own — rendered as an em dash. */
        size: number | null;
        deletedAt: number | null;
        parentId: string | null;
        location: string;
    }

    let rows = $state<Row[]>([]);
    let loading = $state(true);
    let error = $state<string | null>(null);

    let confirmTarget = $state<Row | null>(null);
    let showEmptyConfirm = $state(false);
    let busy = $state(false);

    async function load() {
        loading = true;
        error = null;

        const [fileResult, folderResult] = await Promise.all([
            safeInvoke<FileView[]>("get_deleted_files"),
            safeInvoke<FolderView[]>("get_deleted_folder"),
        ]);

        if (!fileResult.ok) {
            error = fileResult.error;
            loading = false;
            return;
        }

        /*
         * Both calls already exclude nested items — anything whose parent is itself
         * trashed is not its own row — so the two lists concatenate without dedup.
         */
        const fileRows: Row[] = fileResult.data.map((file) => ({
            kind: "file",
            id: file.id,
            name: file.name,
            size: file.size,
            deletedAt: file.deleted_at,
            parentId: file.parent_folder_id,
            location: "",
        }));

        const folderRows: Row[] = (folderResult.ok ? folderResult.data : []).map((folder) => ({
            kind: "folder",
            id: folder.id,
            name: folder.name,
            size: null,
            deletedAt: folder.deleted_at,
            parentId: folder.parent_folder_id,
            location: "",
        }));

        rows = [...folderRows, ...fileRows].sort(
            (a, b) => (b.deletedAt ?? 0) - (a.deletedAt ?? 0),
        );

        loading = false;

        // Resolve original locations after painting — each is an ancestor walk, and the
        // table is readable without them.
        for (const row of rows) {
            if (!row.parentId) continue;
            ancestorsOf(row.parentId).then((segments) => {
                const path = pathString(segments);
                rows = rows.map((existing) =>
                    existing.id === row.id ? { ...existing, location: path } : existing,
                );
            });
        }
    }

    $effect(() => {
        load();
    });

    /*
     * Folders contribute no bytes client-side, so a summed total would understate what
     * emptying actually frees. The server computes it recursively over the whole tree.
     */
    const reclaimable = $derived(session.stats?.trashed_bytes ?? null);

    async function restore(row: Row) {
        const result =
            row.kind === "file"
                ? await safeInvoke("restore_file", { fileId: row.id })
                : await safeInvoke("restore_folder", { folderId: row.id });

        if (!result.ok) {
            toasts.error("Restore failed", result.error);
            return;
        }

        await load();
        await session.refreshStorage();

        if (row.kind === "folder") {
            // The backend re-parents to the root when the original parent is also
            // trashed, which would otherwise be a silent surprise.
            toasts.success(
                "Folder restored",
                "If its original folder is still in the trash, it was restored to My Files.",
            );
        } else {
            toasts.success("File restored", row.name);
        }
    }

    async function deleteForever() {
        if (!confirmTarget) return;

        busy = true;
        const result =
            confirmTarget.kind === "file"
                ? await safeInvoke("remove_deleted_file", { fileId: confirmTarget.id })
                : await safeInvoke("cleanup_deleted_folder", { deletedFolderId: confirmTarget.id });
        busy = false;

        if (!result.ok) {
            toasts.error("Delete failed", result.error);
            return;
        }

        confirmTarget = null;
        await load();
        await session.refreshStorage();
        toasts.success("Deleted permanently");
    }

    async function emptyTrash() {
        busy = true;
        const result = await safeInvoke("cleanup_trash");

        if (!result.ok) {
            busy = false;
            toasts.error("Could not empty trash", result.error);
            return;
        }

        showEmptyConfirm = false;

        // `cleanup_trash` only publishes a message; the deletion happens in a consumer.
        // Poll until it lands, then reload — otherwise the table redraws unchanged and
        // the action looks like it did nothing.
        const cleared = await session.awaitTrashCleared();
        await load();
        busy = false;

        if (cleared) toasts.success("Trash emptied");
        else toasts.info("Emptying the trash", "This is still running in the background.");
    }
</script>

<div class="page">
    <header class="head">
        <div class="heading">
            <h1 class="page-title">Trash</h1>
            <p class="page-subtitle">
                Deleted items stay here until you empty the trash. They still take up disk space.
            </p>
        </div>

        <div class="actions">
            <Button onclick={() => history.back()}>Back to files</Button>
            <Button
                variant="destructive"
                onclick={() => (showEmptyConfirm = true)}
                disabled={rows.length === 0}
            >
                Empty trash{reclaimable ? ` · ${formatBytes(reclaimable)}` : ""}
            </Button>
        </div>
    </header>

    {#if rows.length > 0}
        <div class="warning">
            <TriangleAlert size={16} strokeWidth={1.8} />
            <span>Emptying the trash is permanent — files cannot be recovered afterwards.</span>
        </div>
    {/if}

    <div class="card">
        {#if loading}
            <div class="skeletons">
                {#each Array(4) as _, index (index)}
                    <div class="skeleton"></div>
                {/each}
            </div>
        {:else if error}
            <EmptyState icon={Trash2} title="Couldn't load the trash" body={error} />
        {:else if rows.length === 0}
            <EmptyState
                icon={Trash2}
                title="Trash is empty"
                body="Nothing to recover. Deleted files will appear here."
            >
                {#snippet action()}
                    <Button onclick={() => (window.location.href = "/nas")}>Back to files</Button>
                {/snippet}
            </EmptyState>
        {:else}
            <div class="thead">
                <span>Name</span>
                <span>Original location</span>
                <span>Deleted</span>
                <span class="right">Size</span>
                <span>Actions</span>
            </div>

            <div class="tbody">
                {#each rows as row (row.id)}
                    <div class="trow">
                        <span class="cell-name">
                            <ExtBadge name={row.name} folder={row.kind === "folder"} />
                            <span class="truncate">{row.name}</span>
                        </span>

                        <span class="mono muted truncate" title={row.location}>
                            {row.location ? truncatePath(row.location) : "—"}
                        </span>

                        <span class="mono muted">{formatDate(row.deletedAt)}</span>

                        <span class="mono muted right">
                            {#if row.size === null}
                                <span class="empty-value">—</span>
                            {:else}
                                {formatBytes(row.size)}
                            {/if}
                        </span>

                        <span class="cell-actions">
                            <Button size="sm" onclick={() => restore(row)}>Restore</Button>
                            <Button
                                size="sm"
                                variant="destructive"
                                onclick={() => (confirmTarget = row)}
                            >
                                Delete forever
                            </Button>
                        </span>
                    </div>
                {/each}
            </div>

            <div class="tfoot">
                {pluralise(rows.length, "item")}
                {#if reclaimable !== null}
                    · {formatBytes(reclaimable)} reclaimable
                {/if}
            </div>
        {/if}
    </div>
</div>

<ConfirmDialog
    open={confirmTarget !== null}
    title="Delete forever"
    body={confirmTarget?.kind === "folder"
        ? `${confirmTarget.name} and everything inside it will be deleted permanently. This cannot be undone.`
        : `${confirmTarget?.name ?? "This file"} will be deleted permanently. This cannot be undone.`}
    confirmLabel="Delete forever"
    {busy}
    onconfirm={deleteForever}
    onclose={() => (confirmTarget = null)}
/>

<ConfirmDialog
    open={showEmptyConfirm}
    title="Empty trash"
    body="All {rows.length} items will be deleted permanently{reclaimable
        ? ` and ${formatBytes(reclaimable)} of disk space will be reclaimed`
        : ''}. This cannot be undone."
    confirmLabel="Empty trash"
    {busy}
    onconfirm={emptyTrash}
    onclose={() => (showEmptyConfirm = false)}
/>

<style>
    .page {
        flex: 1;
        min-height: 0;
        padding: 24px 28px;
        display: flex;
        flex-direction: column;
        gap: 16px;
        overflow: hidden;
    }

    .head {
        display: flex;
        align-items: flex-end;
        justify-content: space-between;
        gap: 20px;
        flex: none;
    }

    .heading {
        display: flex;
        flex-direction: column;
        gap: 6px;
    }

    .actions {
        display: flex;
        gap: 9px;
        flex: none;
    }

    .warning {
        display: flex;
        align-items: center;
        gap: 10px;
        padding: 12px 16px;
        border-radius: var(--r-inset);
        background: var(--danger-bg);
        border: 1px solid var(--danger-bd);
        font-size: var(--fs-btn);
        color: var(--danger-tx-soft);
        flex: none;
    }

    .warning :global(svg) {
        color: var(--danger);
        flex: none;
    }

    .card {
        flex: 1;
        min-height: 0;
        border: 1px solid var(--bd);
        border-radius: var(--r-card);
        background: var(--card);
        display: flex;
        flex-direction: column;
        overflow: hidden;
    }

    .thead,
    .trow {
        display: grid;
        grid-template-columns: minmax(200px, 1fr) 170px 110px 90px 210px;
        gap: 12px;
        align-items: center;
    }

    .thead {
        padding: 9px 18px;
        border-bottom: 1px solid var(--bd);
        font-size: var(--fs-label);
        text-transform: uppercase;
        letter-spacing: var(--track-th);
        color: var(--tx-faint-2);
        flex: none;
    }

    .tbody {
        flex: 1;
        overflow-y: auto;
        min-height: 0;
    }

    .trow {
        padding: 11px 18px;
        font-size: var(--fs-base);
        color: var(--tx);
        border-bottom: 1px solid var(--bd-row);
        transition: background var(--t-hover);
    }

    .trow:hover {
        background: var(--hover-row);
    }

    .cell-name {
        display: flex;
        align-items: center;
        gap: 10px;
        min-width: 0;
    }

    .muted {
        color: var(--tx-mut);
        font-size: var(--fs-sm);
    }

    .right {
        text-align: right;
    }

    .cell-actions {
        display: flex;
        gap: 8px;
        justify-content: flex-end;
        white-space: nowrap;
    }

    .tfoot {
        padding: 9px 18px;
        border-top: 1px solid var(--bd-row);
        font-size: var(--fs-sm);
        color: var(--tx-faint-2);
        flex: none;
    }

    .skeletons {
        padding: 14px 18px;
        display: flex;
        flex-direction: column;
        gap: 14px;
    }

    .skeleton {
        height: 20px;
        border-radius: var(--r-badge);
        background: var(--bd-row);
        animation: shimmer 1.3s ease-in-out infinite;
    }

    @keyframes shimmer {
        0%, 100% { opacity: 1; }
        50% { opacity: 0.5; }
    }
</style>
