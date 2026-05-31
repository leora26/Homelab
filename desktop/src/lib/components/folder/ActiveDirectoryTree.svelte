<script lang="ts">
    import { fly } from "svelte/transition";
    import type { FolderView } from "$lib/types/models";
    import { safeInvoke } from "$lib/components/helpers/safeInvoke";
    import { userId } from "$lib/types/tempUserId";
    import FolderTreeItem from "$lib/components/folder/FolderTreeItem.svelte";
    import { onMount } from "svelte";

    interface ActiveDirectoryTreeProps {
        activeFolderId: string | null;
        treeVersion: number;
        onSelect: (folderId: string, isTrash: boolean) => void;
        onContextMenu: (e: MouseEvent, folderId: string, folderName: string) => void;
        onRootFetched: (rootFolder: FolderView) => void;
    }

    const {
        activeFolderId,
        treeVersion,
        onSelect,
        onContextMenu,
        onRootFetched
    }: ActiveDirectoryTreeProps = $props();

    let error = $state<string | null>(null);
    let isLoading = $state(true);
    let rootFolder = $state<FolderView | null>(null);

    onMount(async () => {
        await fetchActiveRoot();
    });

    const fetchActiveRoot = async () => {
        try {
            const result = await safeInvoke<FolderView>('get_root_folder', {userId});

            if (result.ok) {
                rootFolder = result.data;
                onSelect(rootFolder.id, false);
                onRootFetched(result.data);
            } else {
                error = result.error;
                console.error("Failed to fetch root folder", error);
            }
        } finally {
            isLoading = false;
        }
    }
</script>

<div class="tree-container" in:fly={{ y: 20, duration: 300 }} out:fly={{ y: -20, duration: 300 }}>
    {#if isLoading}
        <div class="full-center">
            <div class="spinner"></div>
        </div>
    {:else if error}
        <div class="full-center error">⚠️ {error}</div>
    {:else}
        {#if rootFolder}
            <FolderTreeItem
                    folder={rootFolder}
                    {activeFolderId}
                    onSelect={(id) => onSelect(id, false)}
                    onContextMenu={onContextMenu}
                    {treeVersion}
            />
        {/if}
    {/if} </div>

<style>
    .tree-container {
        padding: 0.5rem;
        overflow-y: auto;
        height: 100%;
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