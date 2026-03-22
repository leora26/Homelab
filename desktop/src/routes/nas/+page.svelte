<script lang="ts">
    import FolderStructure from "$lib/components/FolderStructure.svelte";
    import ContentSection from "$lib/components/ContentSection.svelte";
    import FormModal, {type FormField} from "$lib/components/common/FormModal.svelte";

    import {userId} from "$lib/types/tempUserId";
    import type {FolderView} from "$lib/types/models";
    import NasToolbar from "$lib/components/NasToolbar.svelte";
    import {safeInvoke} from "$lib/components/helpers/safeInvoke";
    import TrashSection from "$lib/components/TrashSection.svelte";

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

        const newFolder = await safeInvoke<FolderView>('create_folder', {
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
    <NasToolbar
            openNewFolderModal={openNewFolderModal}
            activeFolderId={activeFolderId}
    />

    <main class="split-view">
        <FolderStructure
                bind:treeVersion={treeVersion}
                {activeFolderId}
                onActiveFolderChange={handleActiveFolderChange}
                onRequestNewFolder={openNewFolderModal}
        />

        {#if activeFolderId === 'TRASH'}
            <TrashSection />
        {:else if activeFolderId}
            <ContentSection {activeFolderId} />
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

    .split-view {
        display: grid;
        grid-template-columns: 260px 1fr;
        gap: 1.5rem;
        flex: 1;
        min-height: 0;
    }
</style>