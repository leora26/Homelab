<script lang="ts">
    import type { FileView } from "$lib/types/models";
    import { invoke } from "@tauri-apps/api/core";

    interface ContentSectionProps {
        activeFolderId: string
    }

    const { activeFolderId }: ContentSectionProps = $props();

    let files = $state<FileView[]>([]);
    let isLoading = $state(false);
    let error = $state<string | null>(null);

    // --- HELPERS ---
    function formatBytes(bytes: number) {
        if (bytes === 0) return '0 B';
        const k = 1024;
        const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
        const i = Math.floor(Math.log(bytes) / Math.log(k));
        return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
    }

    function getFileIcon(type: string) {
        switch (type.toLowerCase()) {
            case 'image': return '🖼️';
            case 'video': return '🎞️';
            case 'audio': return '🎵';
            case 'text': return '📄';
            case 'pdf': return '📕';
            default: return '📎';
        }
    }

    $effect(() => {
        if (!activeFolderId) {
            return;
        }

        const fetchFiles = async () => {
            isLoading = true;
            error = null;

            try {
                files = await invoke<FileView[]>('get_files_for_folder', { folderId: activeFolderId });
                console.log(`Fetched files for folder ${activeFolderId}:`, files);
            } catch (err) {
                error = String(err);
                console.error("Failed to fetch files:", err);
            } finally {
                isLoading = false;
            }
        }

        fetchFiles();
    });
</script>

<section class="content-pane">
    <div class="content-header">
        <h3>Folder Contents</h3>
    </div>

    <div class="table-wrapper">
        {#if isLoading}
            <div class="status-message">
                <div class="spinner"></div>
                <p>Loading files...</p>
            </div>
        {:else if error}
            <div class="status-message error">
                <p>⚠️ {error}</p>
            </div>
        {:else if files.length === 0}
            <div class="status-message empty-state">
                <p>This folder is empty.</p>
            </div>
        {:else}
            <table class="file-table">
                <thead>
                <tr>
                    <th class="col-name">Name</th>
                    <th class="col-date">Date Modified</th>
                    <th class="col-size">Size</th>
                </tr>
                </thead>
                <tbody>
                {#each files as file (file.id)}
                    <tr class="file-row">
                        <td class="col-name">
                            <div class="name-cell">
                                <span class="icon" aria-hidden="true">{getFileIcon(file.file_type)}</span>
                                <span class="file-name">{file.name}</span>
                            </div>
                        </td>
                        <td class="col-date">{file.updated_at}</td>
                        <td class="col-size">{formatBytes(file.size)}</td>
                    </tr>
                {/each}
                </tbody>
            </table>
        {/if}
    </div>
</section>

<style>
    /* --- PANE LAYOUT --- */
    .content-pane {
        background: white;
        border-radius: 8px;
        border: 1px solid #e1e4e8;
        display: flex;
        flex-direction: column;
        overflow: hidden;
        height: 100%; /* Ensure it fills its parent in the grid */
    }

    .content-header {
        padding: 1rem 1.5rem;
        border-bottom: 1px solid #f0f2f5;
        background: #f8f9fa; /* Slight contrast from the table body */
    }

    .content-header h3 {
        margin: 0;
        font-size: 1rem;
        font-weight: 600;
        color: #1e1e2f;
    }

    /* --- TABLE STYLES --- */
    .table-wrapper {
        flex: 1;
        overflow-y: auto; /* Scroll ONLY the table body, not the whole pane */
    }

    .file-table {
        width: 100%;
        border-collapse: collapse;
        text-align: left;
    }

    .file-table th {
        background: white;
        padding: 0.75rem 1.5rem;
        font-size: 0.85rem;
        color: #666;
        font-weight: 600;
        border-bottom: 1px solid #e1e4e8;
        position: sticky;
        top: 0;
        z-index: 10;
        /* Prevent text selection on headers */
        user-select: none;
    }

    .file-table td {
        padding: 0.75rem 1.5rem;
        border-bottom: 1px solid #f0f2f5;
        font-size: 0.95rem;
        color: #333;
    }

    .file-row {
        cursor: pointer;
        transition: background-color 0.15s ease;
    }

    .file-row:hover {
        background-color: #f4f6f8;
    }

    /* --- COLUMNS --- */
    .col-name { width: 55%; }
    .col-date { width: 30%; color: #666; }
    .col-size { width: 15%; color: #666; text-align: right; }

    .file-table th.col-size { text-align: right; }

    .name-cell {
        display: flex;
        align-items: center;
        gap: 0.75rem;
    }

    .icon {
        font-size: 1.25rem;
    }

    .file-name {
        font-weight: 500;
        color: #1e1e2f;
        /* Truncate long file names cleanly */
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
        max-width: 400px;
    }

    /* --- STATUS STATES (Loading, Empty, Error) --- */
    .status-message {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        padding: 4rem 2rem;
        color: #888;
        height: 100%;
    }

    .status-message.error { color: #d32f2f; }
    .empty-state { font-style: italic; }

    .spinner {
        width: 30px; height: 30px;
        border: 3px solid #f3f3f3; border-top: 3px solid #007bff;
        border-radius: 50%;
        animation: spin 1s linear infinite;
        margin-bottom: 1rem;
    }

    @keyframes spin {
        0% { transform: rotate(0deg); }
        100% { transform: rotate(360deg); }
    }
</style>