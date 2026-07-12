<script lang="ts">
    import type { FileView } from "$lib/types/models";
    import { safeInvoke } from "$lib/components/helpers/safeInvoke";
    import { getFileIcon } from "$lib/components/helpers/file/getFileIcon";
    import { formatBytes } from "$lib/components/helpers/file/formatBytes";
    import PreviewSection from "$lib/components/file/PreviewSection.svelte";

    interface Props {
        name: string;
        fromTs: number | null;
        toTs: number | null;
        labelIds: string[];
    }

    let { name, fromTs, toTs, labelIds }: Props = $props();

    let results = $state<FileView[]>([]);
    let isLoading = $state(false);
    let error = $state<string | null>(null);
    let selectedFile = $state<FileView | null>(null);

    const runSearch = async () => {
        isLoading = true;
        error = null;

        const res = await safeInvoke<FileView[]>("search_files", {
            name: name.trim() === "" ? null : name.trim(),
            labelIds,
            updatedAfter: fromTs,
            updatedBefore: toTs,
        });

        if (res.ok) {
            results = res.data;
            if (selectedFile && !results.some((f) => f.id === selectedFile!.id)) {
                selectedFile = null;
            }
        } else {
            error = res.error;
        }

        isLoading = false;
    };

    // Debounced live search whenever the incoming filters change.
    $effect(() => {
        name;
        fromTs;
        toTs;
        labelIds;

        const timer = setTimeout(runSearch, 300);
        return () => clearTimeout(timer);
    });
</script>

<section class="content-pane-wrapper">
    <section class="content-pane">
        <div class="result-count">
            {#if !isLoading && !error}
                {results.length} {results.length === 1 ? "file" : "files"} found
            {/if}
        </div>

        <div class="table-wrapper">
            {#if isLoading}
                <div class="status-message">
                    <div class="spinner"></div>
                    <p>Searching…</p>
                </div>
            {:else if error}
                <div class="status-message error">
                    <p>⚠️ {error}</p>
                </div>
            {:else if results.length === 0}
                <div class="status-message empty-state">
                    <p>No files match your filters.</p>
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
                    {#each results as file (file.id)}
                        <tr
                                class:selected={selectedFile?.id === file.id}
                                onclick={() => (selectedFile = file)}
                        >
                            <td class="col-name">
                                <span class="icon">{getFileIcon(file.file_type)}</span>
                                {file.name}
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

    {#if selectedFile}
        <PreviewSection
                selectedFile={selectedFile}
                closePreview={() => (selectedFile = null)}
                showManagementActions={false}
                canToggleGlobal={false}
        />
    {/if}
</section>

<style>
    .content-pane-wrapper {
        display: flex;
        gap: 1.5rem;
        height: 100%;
        overflow: hidden;
    }

    .content-pane {
        background: white;
        border-radius: 8px;
        border: 1px solid #e1e4e8;
        display: flex;
        flex-direction: column;
        overflow: hidden;
        height: 100%;
        flex: 1;
    }

    .result-count {
        padding: 0.6rem 1.5rem;
        border-bottom: 1px solid #f0f2f5;
        background: #f8f9fa;
        font-size: 0.8rem;
        color: #666;
        min-height: 1.2rem;
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
