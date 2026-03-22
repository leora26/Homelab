<script lang="ts">
    import {onMount} from "svelte";
    import {safeInvoke} from "$lib/components/helpers/safeInvoke";
    import {userId} from "$lib/types/tempUserId";
    import type {FileView} from "$lib/types/models";

    let isLoading = $state(true);
    let error = $state<string | null>(null);
    let deletedFiles = $state<FileView[]>([]);

    async function fetchDeletedFiles() {
        isLoading = true;
        error = null;

        const result = await safeInvoke<FileView[]>('get_deleted_files', {
            userId: userId
        });

        if (result.ok) {
            deletedFiles = result.data;
        } else {
            error = result.error;
            console.error("Failed to fetch deleted files:", error);
        }

        isLoading = false;
    }

    onMount(() => {
        fetchDeletedFiles();
    });

    const restoreFile = async (fileId: string) => {
        const result = await safeInvoke<FileView>('restore_file', {
            fileId
        })

        if (!result.ok) {
            error = result.error;
            console.error("Failed to restore a file: ", error)
        }
    };

    const permanentlyDeleteFile = async (fileId: string) => {
        const result = await safeInvoke('remove_deleted_file', {
            fileId
        });

        if (!result.ok) {
            error = result.error;
            console.error("Failed to permanantry delete a file: ", error)
        }
    };

    const emptyTrash = async () => {
        const result = await safeInvoke('empty_trash', {
            userId: userId
        });

        if (!result.ok) {
            error = result.error;
            console.error("Failed to empty trash: ", error)
        }
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
                disabled={deletedFiles.length === 0 || isLoading}
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
                <button class="btn secondary mt-1" onclick={fetchDeletedFiles}>Retry</button>
            </div>
        {:else if deletedFiles.length === 0}
            <div class="full-center empty-state">
                <span class="large-icon">🗑️</span>
                <h3>Trash is empty</h3>
                <p>No deleted files found.</p>
            </div>
        {:else}
            <table class="file-table">
                <thead>
                <tr>
                    <th>Name</th>
                    <th>Type</th>
                    <th>Size (Bytes)</th>
                    <th class="actions-col">Actions</th>
                </tr>
                </thead>
                <tbody>
                {#each deletedFiles as file (file.id)}
                    <tr>
                        <td class="file-name">
                            <span class="icon">📄</span>
                            {file.name}
                        </td>
                        <td>{file.file_type || 'Unknown'}</td>
                        <td>{file.size}</td>
                        <td class="actions-col">
                            <button class="action-btn restore" onclick={() => restoreFile(file.id)} title="Restore">
                                ↩️
                            </button>
                            <button class="action-btn delete" onclick={() => permanentlyDeleteFile(file.id)}
                                    title="Permanently Delete">
                                ❌
                            </button>
                        </td>
                    </tr>
                {/each}
                </tbody>
            </table>
        {/if}
    </div>
</div>

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
        background: #fdfdfd;
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
    }

    /* --- Table Styles --- */
    .file-table {
        width: 100%;
        border-collapse: collapse;
        text-align: left;
    }

    .file-table th {
        position: sticky;
        top: 0;
        background: #f8f9fa;
        padding: 0.75rem 1.5rem;
        font-size: 0.85rem;
        font-weight: 600;
        color: #4a5568;
        border-bottom: 1px solid #e1e4e8;
    }

    .file-table td {
        padding: 0.75rem 1.5rem;
        border-bottom: 1px solid #f0f2f5;
        font-size: 0.9rem;
        color: #1e1e2f;
        vertical-align: middle;
    }

    .file-table tbody tr:hover {
        background: #fdfdfd;
    }

    .file-name {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        font-weight: 500;
    }

    .actions-col {
        text-align: right;
        width: 120px;
    }

    .action-btn {
        background: none;
        border: none;
        cursor: pointer;
        padding: 0.4rem;
        border-radius: 4px;
        font-size: 1.1rem;
        transition: background 0.2s;
    }

    .action-btn.restore:hover {
        background: #e6f4ea;
    }

    .action-btn.delete:hover {
        background: #ffebee;
    }

    /* --- Utility Styles --- */
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