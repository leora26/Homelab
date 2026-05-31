<script lang="ts">
    import type {FileView, FolderView} from "$lib/types/models";

    interface Props {
        files: FileView[];
        folders: FolderView[];
        onRestoreFile: (id: string) => void;
        onDeleteFile: (id: string) => void;
        onRestoreFolder: (id: string) => void;
        onDeleteFolder: (id: string) => void;
    }

    const {
        files,
        folders,
        onRestoreFile,
        onDeleteFile,
        onRestoreFolder,
        onDeleteFolder
    }: Props = $props();
</script>

<div class="table-section">
    <table class="item-table">
        <thead>
        <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Size (Bytes)</th>
            <th class="actions-col">Actions</th>
        </tr>
        </thead>
        <tbody>
        {#each folders as folder (folder.id)}
            <tr>
                <td class="item-name">
                    <span class="icon">📁</span>
                    {folder.name}
                </td>
                <td class="text-muted">Folder</td>
                <td class="text-muted">--</td>
                <td class="actions-col">
                    <button class="action-btn restore" onclick={() => onRestoreFolder(folder.id)} title="Restore">
                        ⏪
                    </button>
                    <button class="action-btn delete" onclick={() => onDeleteFolder(folder.id)}
                            title="Permanently Delete">
                        ❌
                    </button>
                </td>
            </tr>
        {/each}

        {#each files as file (file.id)}
            <tr>
                <td class="item-name">
                    <span class="icon">📄</span>
                    {file.name}
                </td>
                <td>{file.file_type || 'Unknown'}</td>
                <td>{file.size}</td>
                <td class="actions-col">
                    <button class="action-btn restore" onclick={() => onRestoreFile(file.id)} title="Restore">
                        ⏪
                    </button>
                    <button class="action-btn delete" onclick={() => onDeleteFile(file.id)} title="Permanently Delete">
                        ❌
                    </button>
                </td>
            </tr>
        {/each}
        </tbody>
    </table>
</div>

<style>
    .table-section {
        margin-bottom: 2rem;
    }

    .item-table {
        width: 100%;
        border-collapse: collapse;
        text-align: left;
    }

    .item-table th {
        position: sticky;
        top: 0;
        background: #f8f9fa;
        padding: 0.75rem 1.5rem;
        font-size: 0.85rem;
        font-weight: 600;
        color: #4a5568;
        border-bottom: 1px solid #e1e4e8;
    }

    .item-table td {
        padding: 0.75rem 1.5rem;
        border-bottom: 1px solid #f0f2f5;
        font-size: 0.9rem;
        color: #1e1e2f;
        vertical-align: middle;
    }

    .item-table tbody tr:hover {
        background: #fdfdfd;
    }

    .item-name {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        font-weight: 500;
    }

    .text-muted {
        color: #888;
        font-style: italic;
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
</style>