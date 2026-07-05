<script lang="ts">
    import { fly } from "svelte/transition";
    import type { FolderView } from "$lib/types/models";
    import { safeInvoke } from "$lib/components/helpers/safeInvoke";
    import FolderTreeItem from "$lib/components/folder/FolderTreeItem.svelte";

    interface TrashDirectoryTreeProps {
        activeFolderId: string | null;
        treeVersion: number;
        onSelect: (folderId: string, isTrash: boolean) => void;
        onContextMenu: (e: MouseEvent, folderId: string, folderName: string) => void;
    }

    let {
        activeFolderId,
        treeVersion,
        onSelect,
        onContextMenu
    }: TrashDirectoryTreeProps = $props();

    let trashedFolders = $state<FolderView[]>([]);
    let isLoading = $state(true);
    let error = $state<string | null>(null);

    $effect(() => {
        const _trigger = treeVersion;
        fetchTrashedFolders();
    });

    const fetchTrashedFolders = async () => {
        isLoading = true;
        const result = await safeInvoke<FolderView[]>('get_deleted_folder');

        if (result.ok) {
            trashedFolders = result.data;
        } else {
            error = result.error;
        }

        isLoading = false;
    }
</script>

<div class="tree-container trash-theme" in:fly={{ y: 20, duration: 300 }} out:fly={{ y: -20, duration: 300 }}>
    {#if isLoading}
        <div class="full-center">
            <div class="spinner"></div>
        </div>
    {:else if error}
        <div class="full-center error">⚠️ {error}</div>
    {:else if trashedFolders.length === 0}
        <div class="empty-state-small">No folders in trash</div>
    {:else}
        {#each trashedFolders as trashFolder (trashFolder.id)}
            <FolderTreeItem
                    folder={trashFolder}
                    {activeFolderId}
                    onSelect={(id) => onSelect(id, true)}
                    onContextMenu={onContextMenu}
                    {treeVersion}
            />
        {/each}
    {/if}
</div>

<style>
    .tree-container {
        padding: 0.5rem;
        overflow-y: auto;
        height: 100%;
    }

    .trash-theme {
        background: #fafafa;
    }

    .full-center {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        height: 100%;
        padding: 2rem;
        text-align: center;
        color: #666;
    }

    .empty-state-small {
        padding: 1rem;
        text-align: center;
        color: #888;
        font-size: 0.9rem;
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
        0% { transform: rotate(0deg); }
        100% { transform: rotate(360deg); }
    }
</style>