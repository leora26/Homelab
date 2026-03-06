<script lang="ts">
    import {onMount} from "svelte";
    import {invoke} from "@tauri-apps/api/core";
    import type {FolderView} from "$lib/types/models";

    interface FolderStructureProps {
        onActiveFolderChange: (folderId: string) => void
    }

    const {
        onActiveFolderChange
    }: FolderStructureProps = $props()

    let userId = "4a352510-842b-40dd-8810-7227b6b4c2c0";

    let error = $state<string | null>(null);
    let isLoading = $state(true);

    let rootFolder = $state<FolderView | null>(null);

    onMount(async () => {
        try {
            rootFolder = await invoke<FolderView>('get_root_folder', {userId});
            if (rootFolder) {
                onActiveFolderChange(rootFolder.id)
            }
        } catch (err) {
            error = String(err);
            console.error("Failed to fetch root folder", error);
        } finally {
            isLoading = false;
        }
    });
</script>

<aside class="sidebar">
    {#if isLoading}
        <div class="full-center">
            <div class="spinner"></div>
            <p>Loading filesystem...</p>
        </div>
    {:else if error}
        <div class="full-center error">⚠️ {error}</div>
    {:else if rootFolder}
        <div class="sidebar-header">Directories</div>
        <div class="tree-container">
            <div class="tree-item active">
                <span class="icon">🗂️</span>
                {rootFolder.name} (Root)
            </div>
        </div>
    {/if}if
</aside>

<style>

    .sidebar {
        background: white;
        border-radius: 8px;
        border: 1px solid #e1e4e8;
        display: flex;
        flex-direction: column;
        overflow: hidden;
    }

    .sidebar-header {
        padding: 1rem;
        font-weight: 600;
        font-size: 0.85rem;
        text-transform: uppercase;
        color: #666;
        border-bottom: 1px solid #f0f2f5;
        background: #f8f9fa;
    }

    .tree-container {
        padding: 0.5rem;
        overflow-y: auto;
        flex: 1;
    }

    .tree-item {
        padding: 0.5rem;
        border-radius: 6px;
        cursor: pointer;
        display: flex;
        align-items: center;
        gap: 0.5rem;
        font-size: 0.95rem;
        transition: background 0.1s;
    }

    .tree-item:hover {
        background: #f0f2f5;
    }

    .tree-item.active {
        background: #e6f2ff;
        color: #0056b3;
        font-weight: 500;
    }
</style>