<script lang="ts">
    import type {FileView} from "$lib/types/models";
    import {invoke} from "@tauri-apps/api/core";
    import ContextMenu, {type ContextMenuOption} from "$lib/components/common/ContextMenu.svelte";
    import ContentSectionItem from "$lib/components/ContentSectionItem.svelte";
    import FormModal, {type FormField} from "$lib/components/common/FormModal.svelte";
    import {safeInvoke} from "$lib/components/helpers/safeInvoke";
    import FolderSelectionModal from "$lib/components/common/FolderSelectionModal.svelte";

    interface ContentSectionProps {
        activeFolderId: string
    }

    const {activeFolderId}: ContentSectionProps = $props();

    let files = $state<FileView[]>([]);
    let isLoading = $state(false);
    let error = $state<string | null>(null);
    let contextMenu = $state({isOpen: false, x: 0, y: 0, targetId: '', targetName: ''});
    let isMoveModalOpen = $state(false);
    let fileToMove = $state<string | null>(null);
    let treeVersion = $state(0);
    let isRenameModalOpen = $state(false);
    let isDeleteModalOpen = $state(false);
    let fileToDelete = $state<string | null>(null);


    const fetchFiles = async () => {
        if (!activeFolderId) {
            return;
        }

        isLoading = true;
        error = null;

        try {
            files = await invoke<FileView[]>('get_files_for_folder', {folderId: activeFolderId});
            console.log(`Fetched files for folder ${activeFolderId}:`, files);
        } catch (err) {
            error = String(err);
            console.error("Failed to fetch files:", err);
        } finally {
            isLoading = false;
        }
    }

    $effect(() => {
        fetchFiles();
    });

    const triggerMove = () => {
        isMoveModalOpen = true;
        fileToMove = contextMenu.targetId;
    }

    const confirmMoveFile = async (selectedFolderId: string) => {
        if (!fileToMove) {return;}

        if (selectedFolderId === activeFolderId) {
            console.warn("File is already in the current folder");
            isMoveModalOpen = false;
            return;
        }

        await safeInvoke('move_file', {
            fileId: fileToMove,
            folderId: selectedFolderId
        });

        console.log(`Successfully moved file ${fileToMove} to folder ${selectedFolderId}`);

        isMoveModalOpen = false;
        fileToMove = null;

        fetchFiles();
    }

    const closeContextMenu = () => {
        contextMenu.isOpen = false;
    };

    const triggerRename = () => {
        isRenameModalOpen = true;
    };

    const triggerDelete = (folderId: string) => {
        fileToDelete = folderId;
        isDeleteModalOpen = true;
    };

    const confirmRenameFile = async (data: Record<string, string | number>) => {
        const newName = String(data.newFileName).trim();

        if (newName === contextMenu.targetName) {
            isRenameModalOpen = false;
            return
        }

        await safeInvoke('rename_file', {
            fileId: contextMenu.targetId,
            newName: newName
        });

        console.log(`Successfully renamed file to ${newName}`);
        isRenameModalOpen = false;
    }

    const confirmDeleteFile = async () => {
        if (!fileToDelete) return;

        await safeInvoke('delete_file', {
            fileId: fileToDelete
        });

        console.log(`Successfully deleted file: ${contextMenu.targetId}`);


        isDeleteModalOpen = false;

        fileToDelete = null;
    }

    let menuOptions = $derived.by<ContextMenuOption[]>(() => {
        const options: ContextMenuOption[] = [
            {
                label: 'Rename',
                icon: '✏️',
                action: () => {
                    closeContextMenu();
                    triggerRename()
                }
            },
            {
                label: 'Delete',
                icon: '🗑️',
                danger: true,
                action: () => {
                    closeContextMenu();
                    triggerDelete(contextMenu.targetId);
                }
            },
            {
                label: 'Move',
                icon: '➡️',
                danger: false,
                action: () => {
                    triggerMove();
                    closeContextMenu();
                }
            }
        ];

        return options;
    });

    let renameFields = $derived<FormField[]>([
        {
            name: "newFileName",
            label: "File Name",
            type: "text",
            required: true,
            defaultValue: contextMenu.targetName
        }
    ]);

    const handleContextMenu = (e: MouseEvent, fileId: string, fileName: string) => {
        contextMenu = {
            isOpen: true,
            x: e.clientX,
            y: e.clientY,
            targetId: fileId,
            targetName: fileName
        };
    };
</script>

<svelte:window onclick={closeContextMenu} onscroll={closeContextMenu}/>

<section class="content-pane">
    <div class="content-header">
        <h3>Folder Contents</h3>
    </div>

    <div class="table-wrapper">
        {#if isLoading}
            <div class="status-message">
                <div class="spinner"></div>
                <p>Loading files...</p>
            </div>
        {:else if error}
            <div class="status-message error">
                <p>⚠️ {error}</p>
            </div>
        {:else if files.length === 0}
            <div class="status-message empty-state">
                <p>This folder is empty.</p>
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
                {#each files as file (file.id)}
                    <ContentSectionItem
                            file={file}
                            onContextMenu={handleContextMenu}
                    />
                {/each}
                </tbody>
            </table>
        {/if}
    </div>
</section>

{#if contextMenu.isOpen}
    <ContextMenu
            x={contextMenu.x}
            y={contextMenu.y}
            options={menuOptions}
    />
{/if}

<FormModal
        isOpen={isRenameModalOpen}
        title="Rename File"
        fields={renameFields}
        submitText="Save Changes"
        loadingText="Saving..."
        onClose={() => isRenameModalOpen = false}
        onSubmit={confirmRenameFile}
/>

<FormModal
        isOpen={isDeleteModalOpen}
        title="Delete File"
        description="Are you sure you want to permanently delete this folder? This action cannot be undone."
        fields={[]}
        submitText="Yes, Delete"
        loadingText="Deleting..."
        onClose={() => isDeleteModalOpen = false}
        onSubmit={confirmDeleteFile}
/>

<FolderSelectionModal
        isOpen={isMoveModalOpen}
        title="Move File"
        description="Select a destination folder for this file"
        submitText="Move Here"
        treeVersion={treeVersion}
        onClose={() => {
            isMoveModalOpen = false;
            fileToMove = null;
        }}
        onSubmit={confirmMoveFile}
/>

<style>
    .content-pane {
        background: white;
        border-radius: 8px;
        border: 1px solid #e1e4e8;
        display: flex;
        flex-direction: column;
        overflow: hidden;
        height: 100%;
    }

    .content-header {
        padding: 1rem 1.5rem;
        border-bottom: 1px solid #f0f2f5;
        background: #f8f9fa;
    }

    .content-header h3 {
        margin: 0;
        font-size: 1rem;
        font-weight: 600;
        color: #1e1e2f;
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
        background: white;
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


    .col-name {
        width: 55%;
    }

    .col-date {
        width: 30%;
        color: #666;
    }

    .col-size {
        width: 15%;
        color: #666;
        text-align: right;
    }

    .file-table th.col-size {
        text-align: right;
    }

    .status-message {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        padding: 4rem 2rem;
        color: #888;
        height: 100%;
    }

    .status-message.error {
        color: #d32f2f;
    }

    .empty-state {
        font-style: italic;
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