<script lang="ts">
    import {safeInvoke} from "$lib/components/helpers/safeInvoke";
    import {userId} from "$lib/types/tempUserId";
    import type {FolderView} from "$lib/types/models";
    import FolderTreeItem from "$lib/components/FolderTreeItem.svelte";

    interface Props {
        isOpen: boolean;
        title: string;
        description?: string;
        submitText?: string;
        treeVersion: number;
        onClose: () => void;
        onSubmit: (selectedFolderId: string) => Promise<void>;
    }

    let {
        isOpen,
        title,
        description,
        submitText = "Select Folder",
        treeVersion,
        onClose,
        onSubmit
    }: Props = $props();

    let rootFolder = $state<FolderView | null>(null);
    let activeFolderId = $state<string | null>(null);
    let isLoading = $state(false);
    let error = $state<string | null>(null);
    let isSubmitting = $state(false);

    $effect(() => {
        if (isOpen && !rootFolder) {
            loadRootFolder();
        }
    });

    async function loadRootFolder() {
        isLoading = true;
        const result = await safeInvoke<FolderView>('get_root_folder', {userId});
        if (result.ok) {
            rootFolder = result.data;
            activeFolderId = rootFolder.id;
        } else {
            error = result.error;
        }
        isLoading = false;
    }

    const handleSelect = (folderId: string) => {
        activeFolderId = folderId;
    };

    const dummyContextMenu = (e: MouseEvent) => {
        e.preventDefault();
    };

    async function handleSubmit() {
        if (!activeFolderId) return;

        isSubmitting = true;
        try {
            await onSubmit(activeFolderId);
        } catch (err) {
            error = String(err);
        } finally {
            isSubmitting = false;
        }
    }
</script>

{#if isOpen}
    <div class="modal-backdrop" onclick={onClose}>
        <div class="modal-content" onclick={(e) => e.stopPropagation()}>
            <div class="modal-header">
                <h3>{title}</h3>
            </div>

            <div class="modal-body tree-body">
                {#if error}
                    <div class="error-banner">{error}</div>
                {/if}

                {#if description}
                    <p class="modal-description">{description}</p>
                {/if}

                <div class="tree-scroll-container">
                    {#if isLoading}
                        <div class="spinner-container">
                            <div class="spinner"></div>
                        </div>
                    {:else if rootFolder}
                        <FolderTreeItem
                                folder={rootFolder}
                                {activeFolderId}
                                onSelect={handleSelect}
                                onContextMenu={dummyContextMenu}
                                {treeVersion}
                        />
                    {/if}
                </div>
            </div>

            <div class="modal-actions">
                <button class="btn secondary" onclick={onClose} disabled={isSubmitting}>
                    Cancel
                </button>
                <button class="btn primary" onclick={handleSubmit} disabled={isSubmitting || !activeFolderId}>
                    {#if isSubmitting}
                        Processing...
                    {:else}
                        {submitText}
                    {/if}
                </button>
            </div>
        </div>
    </div>
{/if}

<style>
    .modal-backdrop {
        position: fixed;
        top: 0;
        left: 0;
        width: 100vw;
        height: 100vh;
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
        max-width: 400px;
        box-shadow: 0 10px 25px rgba(0, 0, 0, 0.15);
        display: flex;
        flex-direction: column;
        overflow: hidden;
    }

    .modal-header {
        padding: 1.25rem 1.5rem;
        border-bottom: 1px solid #e1e4e8;
    }

    .tree-body {
        padding: 1rem;
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    .tree-scroll-container {
        max-height: 300px;
        overflow-y: auto;
        border: 1px solid #e1e4e8;
        border-radius: 6px;
        padding: 0.5rem;
        background: #fdfdfd;
    }

    .spinner-container {
        display: flex;
        justify-content: center;
        padding: 2rem;
    }

    .error-banner {
        background: #ffebee;
        color: #c62828;
        padding: 0.75rem;
        border-radius: 6px;
        font-size: 0.85rem;
        margin-bottom: 0.5rem;
    }

    .modal-actions {
        padding: 1.25rem 1.5rem;
        border-top: 1px solid #e1e4e8;
        background: #f8f9fa;
        display: flex;
        justify-content: flex-end;
        gap: 1rem;
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