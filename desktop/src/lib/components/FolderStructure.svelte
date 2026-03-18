<script lang="ts">
    import {onMount} from "svelte";
    import type {FolderView} from "$lib/types/models";
    import {userId} from "$lib/types/tempUserId";
    import FolderTreeItem from "./FolderTreeItem.svelte";
    import FormModal, {type FormField} from "$lib/components/common/FormModal.svelte";
    import ContextMenu, {type ContextMenuOption} from "$lib/components/common/ContextMenu.svelte";
    import {safeInvoke} from "$lib/components/helpers/safeInvoke";

    interface FolderStructureProps {
        activeFolderId: string | null;
        onActiveFolderChange: (folderId: string) => void;
        onRequestNewFolder: (parentId: string) => void;
        treeVersion: number
    }

    let {
        activeFolderId,
        onActiveFolderChange,
        onRequestNewFolder,
        treeVersion = $bindable()
    }: FolderStructureProps = $props();

    let error = $state<string | null>(null);
    let isLoading = $state(true);
    let rootFolder = $state<FolderView | null>(null);

    let isDeleteModalOpen = $state(false);
    let isRenameModalOpen = $state(false);
    let folderToDeleteId = $state<string | null>(null);

    let contextMenu = $state({isOpen: false, x: 0, y: 0, targetId: '', targetName: ''});

    onMount(async () => {
        try {
            const result = await safeInvoke<FolderView>('get_root_folder', {userId});

            if (result.ok) {
                rootFolder = result.data;
                onActiveFolderChange(rootFolder.id);
            } else {
                error = result.error;
                console.error("Failed to fetch root folder", error);
            }
        } finally {
            isLoading = false;
        }
    });

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

    const triggerDelete = (folderId: string) => {
        if (folderId === rootFolder?.id) return;
        folderToDeleteId = folderId;
        isDeleteModalOpen = true;
    };

    const triggerRename = () => {
        if (contextMenu.targetId === rootFolder?.id) return; // Prevent renaming root
        isRenameModalOpen = true;
    };

    let renameFields = $derived<FormField[]>([
        {
            name: "newFolderName",
            label: "Folder Name",
            type: "text",
            required: true,
            defaultValue: contextMenu.targetName
        }
    ]);

    let menuOptions = $derived.by<ContextMenuOption[]>(() => {
        const options: ContextMenuOption[] = [
            {
                label: 'New Subfolder',
                icon: '📁',
                action: () => {
                    onRequestNewFolder(contextMenu.targetId);
                    closeContextMenu();
                }
            }
        ];

        if (contextMenu.targetId !== rootFolder?.id) {
            options.push(
                {
                    label: 'Rename',
                    icon: '✏️',
                    action: () => {
                        triggerRename();
                        closeContextMenu();
                    }
                },
                {
                    label: 'Delete',
                    icon: '🗑️',
                    danger: true,
                    action: () => {
                        triggerDelete(contextMenu.targetId);
                        closeContextMenu();
                    }
                }
            );
        }

        return options;
    });

    const confirmRenameFolder = async (data: Record<string, string | number>) => {
        const newName = String(data.newFolderName).trim();

        if (newName === contextMenu.targetName) {
            isRenameModalOpen = false;
            return;
        }

        await safeInvoke('rename_folder', {
            folderId: contextMenu.targetId,
            newName: newName
        });

        console.log(`Successfully renamed folder to: ${newName}`);
        isRenameModalOpen = false;

        treeVersion++;

    };

    const confirmDeleteFolder = async () => {
        if (!activeFolderId) return;

        await safeInvoke('delete_selected_folder', {selectedFolderId: folderToDeleteId});

        console.log(`Successfully deleted folder: ${folderToDeleteId}`);

        isDeleteModalOpen = false;

        if (activeFolderId === folderToDeleteId && rootFolder) {
            onActiveFolderChange(rootFolder.id);
        }

        folderToDeleteId = null;

        treeVersion++;
    };
</script>

<svelte:window onclick={closeContextMenu} onscroll={closeContextMenu}/>

<aside class="sidebar">
    {#if isLoading}
        <div class="full-center">
            <div class="spinner"></div>
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
                    onContextMenu={handleContextMenu}
                    {treeVersion}
            />
        </div>
    {/if}
</aside>

{#if contextMenu.isOpen}
    <ContextMenu
            x={contextMenu.x}
            y={contextMenu.y}
            options={menuOptions}
    />
{/if}

<FormModal
        isOpen={isRenameModalOpen}
        title="Rename Folder"
        fields={renameFields}
        submitText="Save Changes"
        loadingText="Saving..."
        onClose={() => isRenameModalOpen = false}
        onSubmit={confirmRenameFolder}
/>

<FormModal
        isOpen={isDeleteModalOpen}
        title="Delete Folder"
        description="Are you sure you want to permanently delete this folder? This action cannot be undone and subfolder and files withing this folder will be permanently deleted."
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