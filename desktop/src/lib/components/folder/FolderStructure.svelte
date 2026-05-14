<script lang="ts">
    import type {FolderView} from "$lib/types/models";
    import RenameFolderModal from "$lib/components/folder/RenameFolderModal.svelte";
    import DeleteFormModal from "$lib/components/folder/DeleteFormModal.svelte";
    import TrashDirectoryTree from "$lib/components/folder/TrashDirectoryTree.svelte";
    import ActiveDirectoryTree from "$lib/components/folder/ActiveDirectoryTree.svelte";
    import FolderContextMenu from "$lib/components/folder/FolderContextMenu.svelte";

    interface FolderStructureProps {
        activeFolderId: string | null;
        onActiveFolderChange: (folderId: string, isTrash: boolean) => void;
        onRequestNewFolder: (parentId: string) => void;
        treeVersion: number;
    }

    let {
        activeFolderId,
        onActiveFolderChange,
        onRequestNewFolder,
        treeVersion = $bindable()
    }: FolderStructureProps = $props();

    // --- State ---
    let viewMode = $state<'active' | 'trash'>('active');
    let rootFolder = $state<FolderView | null>(null);

    // Modal & Menu State
    let isDeleteModalOpen = $state(false);
    let isRenameModalOpen = $state(false);
    let contextMenu = $state({isOpen: false, x: 0, y: 0, targetId: '', targetName: ''});

    // --- View Mode Logic ---
    const toggleTrashMode = () => {
        if (viewMode === "active") {
            viewMode = "trash";
            onActiveFolderChange('TRASH_ROOT', true);
        } else {
            viewMode = "active";
            if (rootFolder) {
                onActiveFolderChange(rootFolder.id, false);
            }
        }
    };

    // --- Context Menu Handlers ---
    const handleContextMenu = (e: MouseEvent, folderId: string, folderName: string) => {
        contextMenu = {
            isOpen: true,
            x: e.clientX,
            y: e.clientY,
            targetId: folderId,
            targetName: folderName
        };
    };

    const closeContextMenu = () => {
        contextMenu.isOpen = false;
    };

    const triggerDelete = () => {
        if (contextMenu.targetId === rootFolder?.id) return;
        isDeleteModalOpen = true;
    };

    const triggerRename = () => {
        if (contextMenu.targetId === rootFolder?.id) return;
        isRenameModalOpen = true;
    };
</script>

<svelte:window onclick={closeContextMenu} onscroll={closeContextMenu}/>

<aside class="sidebar">
    <div class="sidebar-header">
        {viewMode === 'active' ? 'Directories' : 'Trash Bin'}

        {#if viewMode === 'trash'}
            <button class="back-btn" onclick={toggleTrashMode} title="Back to Files">
                ↩️
            </button>
        {/if}
    </div>

    <div class="tree-container-wrapper">
        {#if viewMode === 'active'}
            <ActiveDirectoryTree
                    activeFolderId={activeFolderId}
                    treeVersion={treeVersion}
                    onSelect={onActiveFolderChange}
                    onContextMenu={handleContextMenu}
                    onRootFetched={(folder) => rootFolder = folder}
            />
        {:else}
            <TrashDirectoryTree
                    activeFolderId={activeFolderId}
                    treeVersion={treeVersion}
                    onSelect={onActiveFolderChange}
                    onContextMenu={handleContextMenu}
            />
        {/if}
    </div>

    <div class="sidebar-footer">
        <button
                class="trash-button"
                class:active={viewMode === 'trash'}
                onclick={toggleTrashMode}
        >
            <span class="icon">🗑️</span>
            <span class="label">{viewMode === 'active' ? 'Open Trash' : 'Close Trash'}</span>
        </button>
    </div>
</aside>

{#if contextMenu.isOpen}
    <FolderContextMenu
            x={contextMenu.x}
            y={contextMenu.y}
            targetId={contextMenu.targetId}
            rootFolderId={rootFolder?.id || ''}
            {viewMode}
            onNewFolder={(id) => {
                onRequestNewFolder(id);
                closeContextMenu();
            }}
            onRename={() => {
                triggerRename();
                closeContextMenu();
            }}
            onDelete={() => {
                triggerDelete();
                closeContextMenu();
            }}
            onRestore={() => {
                // TODO: Implement Restore API call here
                console.log("Restoring folder:", contextMenu.targetId);
                closeContextMenu();
                treeVersion++; // Trigger tree refresh after restore
            }}
            onRemove={() => {
                // TODO: Open Hard Delete Confirmation Modal
                console.log("Permanently deleting:", contextMenu.targetId);
                closeContextMenu();
            }}
    />
{/if}

<RenameFolderModal
        bind:isOpen={isRenameModalOpen}
        folderId={contextMenu.targetId}
        currentName={contextMenu.targetName}
        onSuccess={() => treeVersion++}
/>

<DeleteFormModal
        bind:isOpen={isDeleteModalOpen}
        activeFolderId={activeFolderId}
        folderId={contextMenu.targetId}
        rootFolder={rootFolder}
        onActiveFolderChange={onActiveFolderChange}
        onSuccess={() => treeVersion++}
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

    .back-btn {
        background: none;
        border: none;
        cursor: pointer;
        font-size: 1.1rem;
        padding: 0.2rem;
        border-radius: 4px;
        transition: background 0.2s;
    }

    .back-btn:hover {
        background: #e0e0e0;
    }

    .tree-container-wrapper {
        position: relative;
        flex: 1;
        overflow: hidden;
        display: grid;
    }

    .sidebar-footer {
        padding: 0.5rem;
        border-top: 1px solid #e1e4e8;
        background: #fdfdfd;
    }

    .trash-button {
        display: flex;
        align-items: center;
        gap: 0.6rem;
        width: 100%;
        padding: 0.6rem 0.8rem;
        background: transparent;
        border: none;
        border-radius: 6px;
        cursor: pointer;
        font-size: 0.95rem;
        color: #4a5568;
        text-align: left;
        transition: background-color 0.15s ease;
    }

    .trash-button:hover {
        background: #f0f2f5;
        color: #1e1e2f;
    }

    .trash-button.active {
        background: #ffebee;
        color: #d32f2f;
        font-weight: 500;
    }

    .trash-button .icon {
        font-size: 1.1rem;
    }
</style>