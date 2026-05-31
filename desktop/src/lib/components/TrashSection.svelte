<script lang="ts">
    import {safeInvoke} from "$lib/components/helpers/safeInvoke";
    import {userId} from "$lib/types/tempUserId";
    import type {FileView, FolderView} from "$lib/types/models";
    import DeletedItemsTable from "$lib/components/folder/DeletedItemsTable.svelte";
    import FormModal from "$lib/components/common/FormModal.svelte";

    interface TrashSectionProps {
        activeFolderId: string | null;
    }

    const {activeFolderId}: TrashSectionProps = $props();

    let isLoading = $state(true);
    let error = $state<string | null>(null);
    let deletedFiles = $state<FileView[]>([]);
    let deletedSubfolder = $state<FolderView[]>([]);
    let fetchId = 0;

    let showRestoreModal = $state(false);
    let itemToRestore = $state<{ id: string, type: 'file' | 'folder' } | null>(null);

    async function fetchDeletedFiles(folderId: string | null) {
        const currentFetchId = ++fetchId;
        isLoading = true;
        error = null;

        if (!folderId) {
            const result = await safeInvoke<FileView[]>('get_deleted_files', {userId});
            if (currentFetchId !== fetchId) return;

            if (result.ok) {
                deletedFiles = result.data;
            } else {
                error = result.error;
                console.error("Failed to fetch deleted files:", error);
            }
            deletedSubfolder = [];
        } else {
            const [filesResult, folderResult] = await Promise.all([
                safeInvoke<FileView[]>('get_trash_files_by_folder', {folderId}),
                safeInvoke<FolderView[]>('get_trash_subfolders_by_folder', {folderId})
            ]);

            if (currentFetchId !== fetchId) return;

            if (filesResult.ok) deletedFiles = filesResult.data;
            else error = filesResult.error;

            if (folderResult.ok) deletedSubfolder = folderResult.data;
            else error = folderResult.error;
        }

        isLoading = false;
    }

    $effect(() => {
        fetchDeletedFiles(activeFolderId);
    });

    const requestRestore = (id: string, type: 'file' | 'folder') => {
        if (activeFolderId) {
            itemToRestore = { id, type };
            showRestoreModal = true;
        } else {
            executeRestore(id, type);
        }
    };

    const executeRestore = async (id: string, type: 'file' | 'folder') => {
        const endpoint = type === 'file' ? 'restore_file' : 'restore_folder';
        const payload = type === 'file' ? { fileId: id } : { folderId: id };

        const result = await safeInvoke(endpoint, payload);

        if (!result.ok) {
            error = result.error as string;
            console.error(`Failed to restore ${type}:`, error);
        }

        await fetchDeletedFiles(activeFolderId);
    };

    const confirmRestore = async (formData: Record<string, string | number>) => {
        if (itemToRestore) {
            await executeRestore(itemToRestore.id, itemToRestore.type);
        }
        closeModal();
    };

    const closeModal = () => {
        showRestoreModal = false;
        itemToRestore = null;
    };

    const permanentlyDeleteFile = async (fileId: string) => {
        const result = await safeInvoke('remove_deleted_file', {fileId});
        if (!result.ok) error = result.error;
        fetchDeletedFiles(activeFolderId);
    };
    const permanentlyDeleteFolder = async (folderId: string) => {
        const result = await safeInvoke('cleanup_deleted_folder', {folderId});
        if (!result.ok) error = result.error;
        fetchDeletedFiles(activeFolderId);
    };

    const emptyTrash = async () => {
        const result = await safeInvoke('empty_trash', {userId});
        if (!result.ok) error = result.error;
        fetchDeletedFiles(activeFolderId);
    }
</script>

<div class="trash-container">
    <header class="trash-header">
        <div>
            <h2>Trash Bin</h2>
            <p class="subtitle">Items here will be kept until you empty the trash.</p>
        </div>
        <button
                class="btn danger"
                disabled={(deletedFiles.length === 0 && deletedSubfolder.length === 0) || isLoading}
                onclick={emptyTrash}
        >
            Empty Trash
        </button>
    </header>

    <div class="trash-content">
        {#if isLoading}
            <div class="full-center">
                <div class="spinner"></div>
                <p>Loading trash...</p>
            </div>
        {:else if error}
            <div class="full-center error">
                ⚠️ {error}
                <button class="btn secondary mt-1" onclick={() => fetchDeletedFiles(activeFolderId)}>Retry</button>
            </div>
        {:else if deletedFiles.length === 0 && deletedSubfolder.length === 0}
            <div class="full-center empty-state">
                <span class="large-icon">🗑️</span>
                <h3>Trash is empty</h3>
                <p>No deleted items found.</p>
            </div>
        {:else}
            <DeletedItemsTable
                    files={deletedFiles}
                    folders={deletedSubfolder}
                    onRestoreFile={(id) => requestRestore(id, 'file')}
                    onDeleteFile={permanentlyDeleteFile}
                    onRestoreFolder={(id) => requestRestore(id, 'folder')}
                    onDeleteFolder={permanentlyDeleteFolder}
            />
        {/if}
    </div>
</div>

<FormModal
        isOpen={showRestoreModal}
        title="Confirm Restore"
        description={`The parent folder for this item is currently in the trash. Restoring this ${itemToRestore?.type || 'item'} will move it directly to your Root Folder. Do you want to proceed?`}
        fields={[]}
        submitText="Confirm Restore"
        loadingText="Restoring..."
        onClose={closeModal}
        onSubmit={confirmRestore}
/>

<style>
    .trash-container {
        display: flex;
        flex-direction: column;
        height: 100%;
        background: white;
        border-radius: 8px;
        border: 1px solid #e1e4e8;
        overflow: hidden;
    }

    .trash-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 1.5rem;
        border-bottom: 1px solid #e1e4e8;
        background: #ffd3d3;
    }

    .trash-header h2 {
        margin: 0 0 0.25rem 0;
        font-size: 1.25rem;
        color: #1e1e2f;
    }

    .subtitle {
        margin: 0;
        font-size: 0.85rem;
        color: #666;
    }

    .trash-content {
        flex: 1;
        overflow-y: auto;
        padding-bottom: 2rem;
    }

    .btn {
        padding: 0.5rem 1rem;
        border-radius: 6px;
        font-weight: 500;
        cursor: pointer;
        border: 1px solid transparent;
        transition: opacity 0.2s;
    }

    .btn:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }

    .btn.danger {
        background: #d32f2f;
        color: white;
    }

    .btn.danger:hover:not(:disabled) {
        background: #b71c1c;
    }

    .btn.secondary {
        background: #f0f2f5;
        border-color: #d1d5db;
        color: #1e1e2f;
    }

    .mt-1 {
        margin-top: 1rem;
    }

    .full-center {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        height: 100%;
        padding: 3rem;
        text-align: center;
        color: #666;
    }

    .empty-state .large-icon {
        font-size: 3rem;
        margin-bottom: 1rem;
        opacity: 0.5;
    }

    .error {
        color: #d32f2f;
    }

    .spinner {
        width: 30px;
        height: 30px;
        border: 3px solid #f3f3f3;
        border-top: 3px solid #007bff;
        border-radius: 50%;
        animation: spin 1s linear infinite;
        margin-bottom: 1rem;
    }

    @keyframes spin {
        0% {
            transform: rotate(0deg);
        }
        100% {
            transform: rotate(360deg);
        }
    }
</style>