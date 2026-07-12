<script lang="ts">
    import type { LabelView } from "$lib/types/models";
    import { safeInvoke } from "$lib/components/helpers/safeInvoke";
    import { notifications } from "$lib/stores/notificationStore";
    import LabelChip from "$lib/components/label/LabelChip.svelte";
    import LabelFormModal from "$lib/components/label/LabelFormModal.svelte";
    import FormModal from "$lib/components/common/FormModal.svelte";
    import NotificationManager from "$lib/components/common/NotificationManager.svelte";

    let labels = $state<LabelView[]>([]);
    let isLoading = $state(false);
    let error = $state<string | null>(null);

    // null editing => create mode; otherwise editing that label.
    let isFormOpen = $state(false);
    let editing = $state<LabelView | null>(null);

    let isDeleteOpen = $state(false);
    let toDelete = $state<LabelView | null>(null);

    const fetchLabels = async () => {
        isLoading = true;
        error = null;

        const res = await safeInvoke<LabelView[]>("get_labels");
        if (res.ok) {
            labels = res.data;
        } else {
            error = res.error;
        }

        isLoading = false;
    };

    $effect(() => {
        fetchLabels();
    });

    const openCreate = () => {
        editing = null;
        isFormOpen = true;
    };

    const openEdit = (label: LabelView) => {
        editing = label;
        isFormOpen = true;
    };

    // Shared handler for the create/edit modal. Throws on failure so the modal keeps
    // itself open and shows the error inline.
    const handleSubmit = async (name: string, color: string) => {
        if (editing) {
            const res = await safeInvoke<LabelView>("change_label", { id: editing.id, name, color });
            if (!res.ok) throw new Error(res.error);
            notifications.notify("SUCCESS", "Label updated", `"${name}" was saved.`);
        } else {
            const res = await safeInvoke<LabelView>("create_label", { name, color });
            if (!res.ok) throw new Error(res.error);
            notifications.notify("SUCCESS", "Label created", `"${name}" was added.`);
        }

        isFormOpen = false;
        editing = null;
        await fetchLabels();
    };

    const openDelete = (label: LabelView) => {
        toDelete = label;
        isDeleteOpen = true;
    };

    const confirmDelete = async () => {
        if (!toDelete) return;

        const res = await safeInvoke("delete_label", { id: toDelete.id });
        if (res.ok) {
            notifications.notify("SUCCESS", "Label deleted", `"${toDelete.name}" was removed.`);
            await fetchLabels();
        } else {
            notifications.notify("FAILURE", "Delete failed", res.error);
        }

        isDeleteOpen = false;
        toDelete = null;
    };
</script>

<div class="labels-page">
    <header class="page-header">
        <div>
            <h2>🏷️ Labels</h2>
            <p>Create and organize labels you can later attach to your files.</p>
        </div>
        <button class="btn primary" onclick={openCreate}>➕ New Label</button>
    </header>

    <div class="list-wrapper">
        {#if isLoading}
            <div class="status-message">
                <div class="spinner"></div>
                <p>Loading labels...</p>
            </div>
        {:else if error}
            <div class="status-message error">
                <p>⚠️ {error}</p>
            </div>
        {:else if labels.length === 0}
            <div class="status-message empty-state">
                <p>You haven't created any labels yet.</p>
                <button class="btn primary" onclick={openCreate}>Create your first label</button>
            </div>
        {:else}
            <ul class="label-list">
                {#each labels as label (label.id)}
                    <li class="label-row">
                        <LabelChip name={label.name} color={label.color} />
                        <div class="row-actions">
                            <button class="btn small" onclick={() => openEdit(label)}>✏️ Edit</button>
                            <button class="btn small danger" onclick={() => openDelete(label)}>🗑️ Delete</button>
                        </div>
                    </li>
                {/each}
            </ul>
        {/if}
    </div>
</div>

<LabelFormModal
        isOpen={isFormOpen}
        title={editing ? "Edit Label" : "New Label"}
        submitText={editing ? "Save Changes" : "Create Label"}
        loadingText={editing ? "Saving..." : "Creating..."}
        initialName={editing?.name ?? ""}
        initialColor={editing?.color ?? "#3B82F6"}
        onClose={() => { isFormOpen = false; editing = null; }}
        onSubmit={handleSubmit}
/>

<FormModal
        isOpen={isDeleteOpen}
        title="Delete Label"
        description={`Delete "${toDelete?.name ?? ''}"? It will be removed from any files it's attached to. This cannot be undone.`}
        fields={[]}
        submitText="Yes, Delete"
        loadingText="Deleting..."
        onClose={() => { isDeleteOpen = false; toDelete = null; }}
        onSubmit={confirmDelete}
/>

<NotificationManager />

<style>
    .labels-page {
        display: flex;
        flex-direction: column;
        height: calc(100vh - 4rem);
        color: #1e1e2f;
    }

    .page-header {
        display: flex;
        justify-content: space-between;
        align-items: flex-start;
        margin-bottom: 1.5rem;
        gap: 1rem;
    }

    .page-header h2 {
        margin: 0 0 0.25rem 0;
        font-size: 1.5rem;
    }

    .page-header p {
        margin: 0;
        color: #666;
        font-size: 0.9rem;
    }

    .list-wrapper {
        flex: 1;
        overflow-y: auto;
        background: white;
        border: 1px solid #e1e4e8;
        border-radius: 8px;
    }

    .label-list {
        list-style: none;
        margin: 0;
        padding: 0;
    }

    .label-row {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 0.85rem 1.5rem;
        border-bottom: 1px solid #f0f2f5;
        gap: 1rem;
    }

    .label-row:last-child {
        border-bottom: none;
    }

    .row-actions {
        display: flex;
        gap: 0.5rem;
        flex-shrink: 0;
    }

    .btn {
        padding: 0.5rem 1rem;
        border-radius: 6px;
        font-weight: 500;
        cursor: pointer;
        border: 1px solid #d1d5db;
        background: #f0f2f5;
        transition: opacity 0.2s;
    }

    .btn.primary {
        background: #007bff;
        color: white;
        border-color: #0069d9;
    }

    .btn.small {
        padding: 0.35rem 0.7rem;
        font-size: 0.8rem;
    }

    .btn.danger {
        color: #d32f2f;
        border-color: #f2c2c2;
        background: #fdf0f0;
    }

    .status-message {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        gap: 1rem;
        padding: 4rem 2rem;
        color: #888;
        height: 100%;
    }

    .status-message.error {
        color: #d32f2f;
    }

    .empty-state {
        font-style: italic;
    }

    .spinner {
        width: 30px;
        height: 30px;
        border: 3px solid #f3f3f3;
        border-top: 3px solid #007bff;
        border-radius: 50%;
        animation: spin 1s linear infinite;
    }

    @keyframes spin {
        0% { transform: rotate(0deg); }
        100% { transform: rotate(360deg); }
    }
</style>
