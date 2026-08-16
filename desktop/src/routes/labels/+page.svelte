<script lang="ts">
    import { Tag } from "@lucide/svelte";

    import Button from "$lib/components/ui/Button.svelte";
    import EmptyState from "$lib/components/ui/EmptyState.svelte";
    import LabelChip from "$lib/components/ui/LabelChip.svelte";
    import ConfirmDialog from "$lib/components/dialogs/ConfirmDialog.svelte";
    import LabelFormDialog from "$lib/components/dialogs/LabelFormDialog.svelte";

    import { safeInvoke } from "$lib/utils/safeInvoke";
    import { toasts } from "$lib/stores/toasts.svelte";
    import { session } from "$lib/stores/session.svelte";
    import { formatCount, formatDate, pluralise } from "$lib/utils/format";
    import type { LabelView } from "$lib/types/models";

    let labels = $state<LabelView[]>([]);
    let loading = $state(true);
    let error = $state<string | null>(null);

    let editing = $state<LabelView | null>(null);
    let showForm = $state(false);
    let deleteTarget = $state<LabelView | null>(null);
    let busy = $state(false);

    async function load() {
        loading = true;
        error = null;

        const result = await safeInvoke<LabelView[]>("get_labels");
        if (result.ok) labels = result.data;
        else error = result.error;

        loading = false;
    }

    $effect(() => {
        load();
    });

    const labelledTotal = $derived(session.stats?.labelled_file_count ?? null);
    const unlabelledTotal = $derived(session.stats?.unlabelled_file_count ?? null);

    async function submit(name: string, color: string) {
        const result = editing
            ? await safeInvoke<LabelView>("change_label", { id: editing.id, name, color })
            : await safeInvoke<LabelView>("create_label", { name, color });

        if (!result.ok) throw new Error(result.error);

        showForm = false;
        const wasEditing = editing !== null;
        editing = null;

        await load();
        await session.refreshStorage();
        toasts.success(wasEditing ? "Label updated" : "Label created", name);
    }

    async function confirmDelete() {
        if (!deleteTarget) return;

        busy = true;
        const result = await safeInvoke("delete_label", { id: deleteTarget.id });
        busy = false;

        if (!result.ok) {
            toasts.error("Delete failed", result.error);
            return;
        }

        const name = deleteTarget.name;
        deleteTarget = null;

        await load();
        await session.refreshStorage();
        toasts.success("Label deleted", `"${name}" was removed from every file.`);
    }
</script>

<div class="page">
    <header class="head">
        <div class="heading">
            <h1 class="page-title">Labels</h1>
            <p class="page-subtitle">
                Colour-coded tags you can attach to any file. Deleting a label never deletes files.
            </p>
        </div>

        <Button
            variant="primary"
            onclick={() => {
                editing = null;
                showForm = true;
            }}
        >
            New label
        </Button>
    </header>

    <div class="body">
        <div class="card">
            {#if loading}
                <div class="skeletons">
                    {#each Array(4) as _, index (index)}
                        <div class="skeleton"></div>
                    {/each}
                </div>
            {:else if error}
                <EmptyState icon={Tag} title="Couldn't load labels" body={error} />
            {:else if labels.length === 0}
                <EmptyState
                    icon={Tag}
                    title="No labels yet"
                    body="Labels let you group files across folders — a file can carry as many as you like."
                >
                    {#snippet action()}
                        <Button
                            variant="primary"
                            onclick={() => {
                                editing = null;
                                showForm = true;
                            }}
                        >
                            Create your first label
                        </Button>
                    {/snippet}
                </EmptyState>
            {:else}
                <div class="thead">
                    <span>Label</span>
                    <span>Files</span>
                    <span>Created</span>
                    <span class="right">Actions</span>
                </div>

                <div class="tbody">
                    {#each labels as label (label.id)}
                        <div class="trow">
                            <span class="cell-label">
                                <LabelChip name={label.name} color={label.color} />
                            </span>
                            <span class="mono muted">{formatCount(label.file_count)}</span>
                            <span class="mono muted">{formatDate(label.created_at)}</span>
                            <span class="cell-actions">
                                <Button
                                    size="sm"
                                    onclick={() => {
                                        editing = label;
                                        showForm = true;
                                    }}
                                >
                                    Edit
                                </Button>
                                <Button
                                    size="sm"
                                    variant="destructive"
                                    onclick={() => (deleteTarget = label)}
                                >
                                    Delete
                                </Button>
                            </span>
                        </div>
                    {/each}
                </div>
            {/if}
        </div>

        {#if labels.length > 0}
            <p class="footer">
                {pluralise(labels.length, "label")}
                {#if labelledTotal !== null}
                    · {formatCount(labelledTotal)} labelled files
                {/if}
                {#if unlabelledTotal !== null}
                    · {formatCount(unlabelledTotal)} files with no label
                {/if}
            </p>
        {/if}
    </div>
</div>

<LabelFormDialog
    open={showForm}
    initialName={editing?.name ?? ""}
    initialColor={editing?.color}
    onsubmit={submit}
    onclose={() => {
        showForm = false;
        editing = null;
    }}
/>

<ConfirmDialog
    open={deleteTarget !== null}
    title="Delete label"
    body={`"${deleteTarget?.name ?? ""}" will be removed from ${
        deleteTarget ? pluralise(deleteTarget.file_count, "file") : "any files"
    }. The files themselves are not deleted.`}
    confirmLabel="Delete label"
    {busy}
    onconfirm={confirmDelete}
    onclose={() => (deleteTarget = null)}
/>

<style>
    .page {
        flex: 1;
        min-height: 0;
        display: flex;
        flex-direction: column;
        overflow: hidden;
    }

    .head {
        display: flex;
        align-items: flex-end;
        justify-content: space-between;
        gap: 20px;
        padding: 24px 28px 18px;
        flex: none;
    }

    .heading {
        display: flex;
        flex-direction: column;
        gap: 6px;
        max-width: 60ch;
    }

    .body {
        flex: 1;
        min-height: 0;
        padding: 0 28px 22px;
        display: flex;
        flex-direction: column;
        gap: 12px;
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
        grid-template-columns: 1fr 110px 150px 168px;
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
        padding: 12px 18px;
        border-bottom: 1px solid var(--bd-row);
        transition: background var(--t-hover);
    }

    .trow:hover {
        background: var(--hover-row);
    }

    .cell-label {
        display: flex;
        min-width: 0;
    }

    .muted {
        color: var(--tx-mut);
        font-size: var(--fs-btn);
    }

    .cell-actions {
        display: flex;
        gap: 8px;
        justify-content: flex-end;
    }

    .right {
        text-align: right;
    }

    .footer {
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
