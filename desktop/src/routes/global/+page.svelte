<script lang="ts">
    import type { GlobalFileView, UserProfileView } from "$lib/types/models";
    import { safeInvoke } from "$lib/components/helpers/safeInvoke";
    import { getFileIcon } from "$lib/components/helpers/file/getFileIcon";
    import { formatBytes } from "$lib/components/helpers/file/formatBytes";
    import PreviewSection from "$lib/components/file/PreviewSection.svelte";
    import NotificationManager from "$lib/components/common/NotificationManager.svelte";

    let globalFiles = $state<GlobalFileView[]>([]);
    let isLoading = $state(false);
    let error = $state<string | null>(null);
    let currentUserId = $state<string | null>(null);
    let selectedFile = $state<GlobalFileView | null>(null);

    const fetchGlobalFiles = async () => {
        isLoading = true;
        error = null;

        const res = await safeInvoke<GlobalFileView[]>("get_global_files");
        if (res.ok) {
            globalFiles = res.data;

            // Keep the current selection if it's still shared, otherwise clear it.
            if (selectedFile && !globalFiles.some((g) => g.id === selectedFile!.id)) {
                selectedFile = null;
            }
        } else {
            error = res.error;
        }

        isLoading = false;
    };

    $effect(() => {
        fetchGlobalFiles();

        safeInvoke<UserProfileView>("get_user_profile").then((res) => {
            if (res.ok) currentUserId = res.data.id;
        });
    });

    const handleGlobalChange = (isGlobal: boolean) => {
        // A file made private is no longer global; refresh the list and drop the preview.
        if (!isGlobal) {
            selectedFile = null;
        }
        fetchGlobalFiles();
    };
</script>

<div class="global-page">
    <header class="page-header">
        <h2>🌐 Global Files</h2>
        <p>Files shared with every user. Anyone can view and download them; only the owner can un-share.</p>
    </header>

    <main class="split-view">
        <section class="content-pane">
            <div class="table-wrapper">
                {#if isLoading}
                    <div class="status-message">
                        <div class="spinner"></div>
                        <p>Loading global files...</p>
                    </div>
                {:else if error}
                    <div class="status-message error">
                        <p>⚠️ {error}</p>
                    </div>
                {:else if globalFiles.length === 0}
                    <div class="status-message empty-state">
                        <p>No global files have been shared yet.</p>
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
                        {#each globalFiles as gf (gf.id)}
                            <tr
                                    class:selected={selectedFile?.id === gf.id}
                                    onclick={() => selectedFile = gf}
                            >
                                <td class="col-name">
                                    <span class="icon">{getFileIcon(gf.file.file_type)}</span>
                                    {gf.file.name}
                                </td>
                                <td class="col-date">{gf.file.updated_at}</td>
                                <td class="col-size">{formatBytes(gf.file.size)}</td>
                            </tr>
                        {/each}
                        </tbody>
                    </table>
                {/if}
            </div>
        </section>

        {#if selectedFile}
            <PreviewSection
                    selectedFile={selectedFile.file}
                    closePreview={() => selectedFile = null}
                    showManagementActions={false}
                    canToggleGlobal={currentUserId === selectedFile.file.owner_id}
                    onGlobalChange={handleGlobalChange}
            />
        {/if}
    </main>
</div>

<NotificationManager />

<style>
    .global-page {
        display: flex;
        flex-direction: column;
        height: calc(100vh - 4rem);
        color: #1e1e2f;
    }

    .page-header {
        margin-bottom: 1.5rem;
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

    .split-view {
        display: flex;
        gap: 1.5rem;
        flex: 1;
        min-height: 0;
        overflow: hidden;
    }

    .content-pane {
        background: white;
        border-radius: 8px;
        border: 1px solid #e1e4e8;
        display: flex;
        flex-direction: column;
        overflow: hidden;
        flex: 1;
    }

    .table-wrapper {
        flex: 1;
        overflow-y: auto;
    }

    .file-table {
        width: 100%;
        border-collapse: collapse;
        text-align: left;
    }

    .file-table th {
        background: #f8f9fa;
        padding: 0.75rem 1.5rem;
        font-size: 0.85rem;
        color: #666;
        font-weight: 600;
        border-bottom: 1px solid #e1e4e8;
        position: sticky;
        top: 0;
        z-index: 10;
        user-select: none;
    }

    .file-table td {
        padding: 0.75rem 1.5rem;
        border-bottom: 1px solid #f0f2f5;
        font-size: 0.9rem;
    }

    .file-table tbody tr {
        cursor: pointer;
        transition: background 0.15s ease;
    }

    .file-table tbody tr:hover {
        background: #f8f9fa;
    }

    .file-table tbody tr.selected {
        background: #eaf3ff;
    }

    .col-name { width: 55%; }
    .col-name .icon { margin-right: 0.5rem; }
    .col-date { width: 30%; color: #666; }
    .col-size { width: 15%; color: #666; text-align: right; }

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
        width: 30px;
        height: 30px;
        border: 3px solid #f3f3f3;
        border-top: 3px solid #007bff;
        border-radius: 50%;
        animation: spin 1s linear infinite;
        margin-bottom: 1rem;
    }
    @keyframes spin {
        0% { transform: rotate(0deg); }
        100% { transform: rotate(360deg); }
    }
</style>
