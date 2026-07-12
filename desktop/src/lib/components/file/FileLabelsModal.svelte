<script lang="ts">
    import type { LabelView } from "$lib/types/models";
    import { safeInvoke } from "$lib/components/helpers/safeInvoke";
    import { notifications } from "$lib/stores/notificationStore";
    import LabelChip from "$lib/components/label/LabelChip.svelte";
    import ColorWheel from "$lib/components/label/ColorWheel.svelte";

    interface Props {
        isOpen: boolean;
        fileId: string;
        fileName: string;
        onClose: () => void;
        // Notifies the parent (the preview) that this file's labels changed, so it can
        // refresh the chips it shows.
        onChanged?: () => void;
    }

    let { isOpen, fileId, fileName, onClose, onChanged }: Props = $props();

    let allLabels = $state<LabelView[]>([]);
    let assignedIds = $state<string[]>([]);
    let search = $state("");
    let loading = $state(false);
    let busyId = $state<string | null>(null);

    // Inline "create a new label" panel state.
    let showCreate = $state(false);
    let createColor = $state("#3B82F6");
    let isCreating = $state(false);

    const query = $derived(search.trim().toLowerCase());
    const filtered = $derived(
        query === "" ? allLabels : allLabels.filter((l) => l.name.toLowerCase().includes(query))
    );
    const exactMatch = $derived(allLabels.some((l) => l.name.toLowerCase() === query));
    // Offer to create when the search text is a genuinely new name.
    const canCreate = $derived(query !== "" && !exactMatch);

    async function loadData() {
        loading = true;

        const [all, forFile] = await Promise.all([
            safeInvoke<LabelView[]>("get_labels"),
            safeInvoke<LabelView[]>("get_labels_for_file", { fileId }),
        ]);

        if (all.ok) allLabels = all.data;
        if (forFile.ok) assignedIds = forFile.data.map((l) => l.id);

        loading = false;
    }

    $effect(() => {
        if (!isOpen) return;
        // Reset transient state each time the modal opens.
        search = "";
        showCreate = false;
        createColor = "#3B82F6";
        busyId = null;
        loadData();
    });

    async function toggle(label: LabelView) {
        if (busyId) return;
        busyId = label.id;

        const isAssigned = assignedIds.includes(label.id);
        const command = isAssigned ? "delete_fl" : "create_fl";

        const res = await safeInvoke(command, { fileId, labelId: label.id });
        if (res.ok) {
            assignedIds = isAssigned
                ? assignedIds.filter((id) => id !== label.id)
                : [...assignedIds, label.id];
            onChanged?.();
        } else {
            notifications.notify("FAILURE", "Could not update label", res.error);
        }

        busyId = null;
    }

    async function createAndAdd() {
        const name = search.trim();
        if (name === "") return;

        isCreating = true;

        const created = await safeInvoke<LabelView>("create_label", { name, color: createColor });
        if (!created.ok) {
            notifications.notify("FAILURE", "Could not create label", created.error);
            isCreating = false;
            return;
        }

        const label = created.data;
        allLabels = [...allLabels, label];

        const assign = await safeInvoke("create_fl", { fileId, labelId: label.id });
        if (assign.ok) {
            assignedIds = [...assignedIds, label.id];
            notifications.notify("SUCCESS", "Label created", `"${name}" was created and added.`);
            onChanged?.();
        } else {
            notifications.notify("FAILURE", "Label created but not added", assign.error);
        }

        search = "";
        showCreate = false;
        createColor = "#3B82F6";
        isCreating = false;
    }
</script>

{#if isOpen}
    <div class="modal-backdrop" onclick={onClose}>
        <div class="modal-content" onclick={(e) => e.stopPropagation()}>
            <div class="modal-header">
                <h3>Labels</h3>
                <span class="subtitle">{fileName}</span>
                <button class="close-btn" onclick={onClose}>✕</button>
            </div>

            <div class="modal-body">
                <input
                        class="search"
                        type="text"
                        bind:value={search}
                        placeholder="Search or create a label…"
                />

                <div class="label-list">
                    {#if loading}
                        <p class="hint">Loading labels…</p>
                    {:else if filtered.length === 0 && !canCreate}
                        <p class="hint">
                            {allLabels.length === 0 ? "No labels yet. Type a name to create one." : "No matches."}
                        </p>
                    {:else}
                        {#each filtered as label (label.id)}
                            {@const assigned = assignedIds.includes(label.id)}
                            <button
                                    class="label-row"
                                    class:assigned
                                    onclick={() => toggle(label)}
                                    disabled={busyId !== null}
                            >
                                <LabelChip name={label.name} color={label.color} />
                                <span class="check">{busyId === label.id ? "⏳" : assigned ? "✓" : ""}</span>
                            </button>
                        {/each}
                    {/if}
                </div>

                {#if canCreate}
                    {#if !showCreate}
                        <button class="create-trigger" onclick={() => (showCreate = true)}>
                            ➕ Create “{search.trim()}”
                        </button>
                    {:else}
                        <div class="create-panel">
                            <div class="create-preview">
                                <LabelChip name={search.trim()} color={createColor} />
                            </div>
                            <ColorWheel color={createColor} onChange={(c) => (createColor = c)} size={150} />
                            <div class="create-actions">
                                <button class="btn secondary" onclick={() => (showCreate = false)} disabled={isCreating}>
                                    Cancel
                                </button>
                                <button class="btn primary" onclick={createAndAdd} disabled={isCreating}>
                                    {isCreating ? "Creating…" : "Create & add"}
                                </button>
                            </div>
                        </div>
                    {/if}
                {/if}
            </div>
        </div>
    </div>
{/if}

<style>
    .modal-backdrop {
        position: fixed;
        inset: 0;
        background: rgba(0, 0, 0, 0.4);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 1000;
        backdrop-filter: blur(2px);
    }

    .modal-content {
        background: white;
        border-radius: 10px;
        width: 100%;
        max-width: 380px;
        box-shadow: 0 10px 25px rgba(0, 0, 0, 0.15);
        display: flex;
        flex-direction: column;
        overflow: hidden;
    }

    .modal-header {
        padding: 1rem 1.5rem;
        border-bottom: 1px solid #e1e4e8;
        display: grid;
        grid-template-columns: 1fr auto;
        align-items: center;
        column-gap: 0.5rem;
    }

    .modal-header h3 {
        margin: 0;
        font-size: 1.15rem;
        color: #1e1e2f;
    }

    .modal-header .subtitle {
        grid-column: 1 / 2;
        font-size: 0.8rem;
        color: #888;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
    }

    .close-btn {
        grid-column: 2;
        grid-row: 1 / 3;
        background: none;
        border: none;
        cursor: pointer;
        font-size: 1.2rem;
        color: #666;
    }

    .modal-body {
        padding: 1.25rem 1.5rem 1.5rem;
        display: flex;
        flex-direction: column;
        gap: 1rem;
    }

    .search {
        padding: 0.6rem 0.8rem;
        border: 1px solid #ccc;
        border-radius: 6px;
        font-size: 0.95rem;
        outline: none;
    }

    .search:focus {
        border-color: #007bff;
        box-shadow: 0 0 0 3px rgba(0, 123, 255, 0.1);
    }

    .label-list {
        display: flex;
        flex-direction: column;
        max-height: 220px;
        overflow-y: auto;
        gap: 0.25rem;
    }

    .label-row {
        display: flex;
        align-items: center;
        justify-content: space-between;
        gap: 0.5rem;
        padding: 0.4rem 0.5rem;
        border: none;
        background: none;
        border-radius: 6px;
        cursor: pointer;
        text-align: left;
        width: 100%;
    }

    .label-row:hover:not(:disabled) {
        background: #f2f4f7;
    }

    .label-row.assigned {
        background: #eaf3ff;
    }

    .label-row:disabled {
        cursor: default;
    }

    .check {
        color: #007bff;
        font-weight: 700;
        min-width: 1rem;
        text-align: center;
    }

    .hint {
        color: #999;
        font-size: 0.85rem;
        font-style: italic;
        margin: 0.5rem 0;
        text-align: center;
    }

    .create-trigger {
        border: 1px dashed #c4c9d2;
        background: #fafbfc;
        color: #333;
        border-radius: 6px;
        padding: 0.6rem;
        cursor: pointer;
        font-size: 0.9rem;
        font-weight: 500;
    }

    .create-trigger:hover {
        background: #f2f4f7;
    }

    .create-panel {
        border: 1px solid #e1e4e8;
        border-radius: 8px;
        padding: 1rem;
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 0.9rem;
    }

    .create-preview {
        display: flex;
        justify-content: center;
    }

    .create-actions {
        display: flex;
        gap: 0.75rem;
        width: 100%;
        justify-content: flex-end;
    }

    .btn {
        padding: 0.5rem 1rem;
        border-radius: 6px;
        font-weight: 500;
        cursor: pointer;
        border: none;
        transition: opacity 0.2s;
    }

    .btn:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }

    .btn.primary {
        background: #007bff;
        color: white;
    }

    .btn.secondary {
        background: #f0f2f5;
        border: 1px solid #d1d5db;
    }
</style>
