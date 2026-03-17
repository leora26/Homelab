<script lang="ts">
    import FolderStructure from "$lib/components/FolderStructure.svelte";
    import ContentSection from "$lib/components/ContentSection.svelte";
    import FormModal, { type FormField } from "$lib/components/common/FormModal.svelte";

    import { invoke } from "@tauri-apps/api/core";
    import { userId } from "$lib/types/tempUserId";
    import type { FolderView } from "$lib/types/models";

    let activeFolderId = $state<string | null>(null);
    let isNewFolderModalOpen = $state(false);
    let targetParentFolderId = $state<string | null>(null);

    let treeVersion = $state(0);

    const handleActiveFolderChange = (folderId: string) => {
        activeFolderId = folderId;
    }

    const openNewFolderModal = (targetId?: string) => {
        const idToUse = targetId || activeFolderId;
        if (!idToUse) {
            alert("Please select a parent folder first.");
            return;
        }
        targetParentFolderId = idToUse;
        isNewFolderModalOpen = true;
    }

    const newFolderFields: FormField[] = [
        {
            name: "folderName",
            label: "Folder Name",
            type: "text",
            placeholder: "e.g., Vacation Photos",
            required: true
        }
    ];

    const handleCreateFolder = async (data: Record<string, string | number>) => {
        if (!targetParentFolderId) return;

        const newFolder = await invoke<FolderView>('create_folder', {
            parentFolderId: targetParentFolderId,
            userId: userId,
            name: String(data.folderName).trim()
        });

        console.log("Successfully created folder:", newFolder);
        isNewFolderModalOpen = false;
        targetParentFolderId = null;

        treeVersion++;
    }
</script>

<div class="app-layout">
    <header class="toolbar">
        <h2>NAS Storage</h2>
        <div class="actions">
            <button onclick={() => openNewFolderModal()} class="btn secondary" disabled={!activeFolderId}>
                📁 New Folder
            </button>
            <button class="btn primary">⬆️ Upload File</button>
        </div>
    </header>

    <main class="split-view">
        <FolderStructure
                bind:treeVersion={treeVersion}
                {activeFolderId}
                onActiveFolderChange={handleActiveFolderChange}
                onRequestNewFolder={openNewFolderModal}
        />

        {#if activeFolderId}
            <ContentSection
                    {activeFolderId}
            />
        {/if}
    </main>
</div>

<FormModal
        isOpen={isNewFolderModalOpen}
        title="Create New Folder"
        fields={newFolderFields}
        submitText="Create Folder"
        loadingText="Creating..."
        onClose={() => isNewFolderModalOpen = false}
        onSubmit={handleCreateFolder}
/>

<style>
    .app-layout {
        display: flex;
        flex-direction: column;
        height: calc(100vh - 4rem);
        color: #1e1e2f;
    }

    .toolbar {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding-bottom: 1rem;
        margin-bottom: 1rem;
        border-bottom: 1px solid #e1e4e8;
        flex-shrink: 0;
    }

    .toolbar h2 { margin: 0; font-size: 1.5rem; }
    .actions { display: flex; gap: 1rem; }

    .btn {
        padding: 0.5rem 1rem;
        border-radius: 6px;
        font-weight: 500;
        cursor: pointer;
        border: none;
        transition: opacity 0.2s;
    }
    .btn:disabled { opacity: 0.5; cursor: not-allowed; }
    .btn.primary { background: #007bff; color: white; }
    .btn.secondary { background: #f0f2f5; border: 1px solid #d1d5db; }

    .split-view {
        display: grid;
        grid-template-columns: 260px 1fr;
        gap: 1.5rem;
        flex: 1;
        min-height: 0;
    }
</style>