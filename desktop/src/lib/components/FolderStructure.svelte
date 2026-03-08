<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import type { FolderView } from "$lib/types/models";
    import { userId } from "$lib/types/tempUserId";
    import FolderTreeItem from "./FolderTreeItem.svelte";

    interface FolderStructureProps {
        activeFolderId: string | null; // Add this!
        onActiveFolderChange: (folderId: string) => void;
    }

    let {
        activeFolderId,
        onActiveFolderChange
    }: FolderStructureProps = $props();

    let error = $state<string | null>(null);
    let isLoading = $state(true);

    let rootFolder = $state<FolderView | null>(null);

    onMount(async () => {
        try {
            rootFolder = await invoke<FolderView>('get_root_folder', { userId });
            if (rootFolder) {
                onActiveFolderChange(rootFolder.id);
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
            <FolderTreeItem
                    folder={rootFolder}
                    {activeFolderId}
                    onSelect={onActiveFolderChange}
            />
        </div>
    {/if}
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

    .full-center { display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100%; padding: 2rem; text-align: center; color: #666; }
    .error { color: #d32f2f; }
    .spinner { width: 30px; height: 30px; border: 3px solid #f3f3f3; border-top: 3px solid #007bff; border-radius: 50%; animation: spin 1s linear infinite; margin-bottom: 1rem; }
    @keyframes spin { 0% { transform: rotate(0deg); } 100% { transform: rotate(360deg); } }
</style>