<script lang="ts">
    import {onMount} from "svelte";
    import {invoke} from "@tauri-apps/api/core";
    import type {FolderView} from "$lib/types/models";
    import {userId} from "$lib/types/tempUserId";
    import FolderTreeItem from "./FolderTreeItem.svelte";
    import FormModal from "$lib/components/common/FormModal.svelte";

    interface FolderStructureProps {
        activeFolderId: string | null;
        onActiveFolderChange: (folderId: string) => void;
        onRequestNewFolder: (parentId: string) => void;
    }

    let {
        activeFolderId,
        onActiveFolderChange,
        onRequestNewFolder,
    }: FolderStructureProps = $props();

    let error = $state<string | null>(null);
    let isLoading = $state(true);
    let rootFolder = $state<FolderView | null>(null);

    let isDeleteModalOpen = $state(false);
    let folderToDeleteId = $state<string | null>(null);

    let contextMenu = $state({ isOpen: false, x: 0, y: 0, targetId: '' });

    onMount(async () => {
        try {
            rootFolder = await invoke<FolderView>('get_root_folder', {userId});
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

    const handleContextMenu = (e: MouseEvent, folderId: string) => {
        contextMenu = { isOpen: true, x: e.clientX, y: e.clientY, targetId: folderId };
    };

    const closeContextMenu = () => {
        contextMenu.isOpen = false;
    };

    const triggerDelete = (folderId: string) => {
        if (folderId === rootFolder?.id) return;
        folderToDeleteId = folderId;
        isDeleteModalOpen = true;
    };

    const confirmDeleteFolder = async () => {
        if (!activeFolderId) return;

        await invoke('delete_selected_folder', {selectedFolderId: activeFolderId});

        console.log(`Successfully deleted folder: ${activeFolderId}`);

        isDeleteModalOpen = false;

        if (rootFolder) {
            onActiveFolderChange(rootFolder.id);
        }
    };
</script>

<svelte:window onclick={closeContextMenu} onscroll={closeContextMenu} />

<aside class="sidebar">
    {#if isLoading}
        <div class="full-center"><div class="spinner"></div></div>
    {:else if error}
        <div class="full-center error">⚠️ {error}</div>
    {:else if rootFolder}
        <div class="sidebar-header">Directories</div>

        <div class="tree-container">
            <FolderTreeItem
                    folder={rootFolder}
                    {activeFolderId}
                    onSelect={onActiveFolderChange}
                    onContextMenu={handleContextMenu}
            />
        </div>
    {/if}
</aside>

{#if contextMenu.isOpen}
    <div
            class="context-menu"
            style="top: {contextMenu.y}px; left: {contextMenu.x}px;"
    >
        <button onclick={() => onRequestNewFolder(contextMenu.targetId)}>
            📁 New Subfolder
        </button>

        {#if contextMenu.targetId !== rootFolder?.id}
            <button class="danger" onclick={() => triggerDelete(contextMenu.targetId)}>
                🗑️ Delete
            </button>
        {/if}
    </div>
{/if}

<FormModal
        isOpen={isDeleteModalOpen}
        title="Delete Folder"
        description="Are you sure you want to permanently delete this folder? This action cannot be undone."
        fields={[]}
        submitText="Yes, Delete"
        loadingText="Deleting..."
        onClose={() => isDeleteModalOpen = false}
        onSubmit={confirmDeleteFolder}
/>

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
        padding: 0.85rem 1rem;
        font-weight: 600;
        font-size: 0.85rem;
        text-transform: uppercase;
        color: #666;
        border-bottom: 1px solid #f0f2f5;
        background: #f8f9fa;
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    .context-menu {
        position: fixed;
        background: white;
        border-radius: 6px;
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
        border: 1px solid #e1e4e8;
        padding: 0.5rem 0;
        min-width: 160px;
        z-index: 9999;
        display: flex;
        flex-direction: column;
    }

    .context-menu button {
        background: none;
        border: none;
        padding: 0.5rem 1rem;
        text-align: left;
        font-size: 0.9rem;
        cursor: pointer;
        display: flex;
        align-items: center;
        gap: 0.5rem;
        color: #1e1e2f;
    }

    .context-menu button:hover {
        background: #f0f2f5;
    }

    .context-menu button.danger {
        color: #d32f2f;
    }

    .context-menu button.danger:hover {
        background: #ffebee;
    }

    .tree-container {
        padding: 0.5rem;
        overflow-y: auto;
        flex: 1;
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
        0% {
            transform: rotate(0deg);
        }
        100% {
            transform: rotate(360deg);
        }
    }
</style>