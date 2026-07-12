<script lang="ts">
    import FolderStructure from "$lib/components/folder/FolderStructure.svelte";
    import ContentSection from "$lib/components/file/ContentSection.svelte";
    import FormModal, {type FormField} from "$lib/components/common/FormModal.svelte";

    import type {FolderView, LabelView} from "$lib/types/models";
    import NasToolbar from "$lib/components/NasToolbar.svelte";
    import {safeInvoke} from "$lib/components/helpers/safeInvoke";
    import TrashSection from "$lib/components/TrashSection.svelte";
    import DateRangePicker from "$lib/components/file/DateRangePicker.svelte";
    import LabelChip from "$lib/components/label/LabelChip.svelte";
    import SearchResults from "$lib/components/file/SearchResults.svelte";

    let activeFolderId = $state<string | null>(null);
    let activeTrashFolder = $state<string | null>(null);
    let isNewFolderModalOpen = $state(false);
    let targetParentFolderId = $state<string | null>(null);
    let isTrashActive = $state(false);

    let treeVersion = $state(0);
    let fileVersion = $state(0);

    let searchName = $state("");
    let fromTs = $state<number | null>(null);
    let toTs = $state<number | null>(null);
    let allLabels = $state<LabelView[]>([]);
    let selectedLabelIds = $state<string[]>([]);
    let datePicker: DateRangePicker;

    const searchActive = $derived(
        searchName.trim() !== "" || fromTs !== null || toTs !== null || selectedLabelIds.length > 0
    );

    $effect(() => {
        safeInvoke<LabelView[]>("get_labels").then((res) => {
            if (res.ok) allLabels = res.data;
        });
    });

    const toggleLabel = (id: string) => {
        selectedLabelIds = selectedLabelIds.includes(id)
            ? selectedLabelIds.filter((x) => x !== id)
            : [...selectedLabelIds, id];
    };

    const onDateChange = (from: Date | null, to: Date | null) => {
        fromTs = from ? Math.floor(from.getTime() / 1000) : null;
        toTs = to
            ? Math.floor(new Date(to.getFullYear(), to.getMonth(), to.getDate(), 23, 59, 59).getTime() / 1000)
            : null;
    };

    const clearFilters = () => {
        searchName = "";
        selectedLabelIds = [];
        fromTs = null;
        toTs = null;
        datePicker?.clear();
    };

    const handleActiveFolderChange = (folderId: string | null, isTrash: boolean) => {
        if (isTrash) {
            activeTrashFolder = folderId;
        } else {
            activeFolderId = folderId;
        }

        isTrashActive = isTrash;
        clearFilters();
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
            onUploadComplete={() => fileVersion++}
    />

    <div class="filter-bar">
        <div class="filter-row">
            <input class="name-input" type="text" bind:value={searchName} placeholder="Search files by name…" />
            <DateRangePicker bind:this={datePicker} onChange={onDateChange} />
            {#if searchActive}
                <button class="clear-filters" onclick={clearFilters}>Clear</button>
            {/if}
        </div>

        {#if allLabels.length > 0}
            <div class="label-filter">
                {#each allLabels as label (label.id)}
                    <button
                            class="label-toggle"
                            class:selected={selectedLabelIds.includes(label.id)}
                            onclick={() => toggleLabel(label.id)}
                    >
                        <LabelChip name={label.name} color={label.color} />
                    </button>
                {/each}
            </div>
        {/if}
    </div>

    <main class="split-view">
        <FolderStructure
                bind:treeVersion={treeVersion}
                {activeFolderId}
                onActiveFolderChange={handleActiveFolderChange}
                onRequestNewFolder={openNewFolderModal}
        />

        {#if searchActive}
            <SearchResults name={searchName} {fromTs} {toTs} labelIds={selectedLabelIds} />
        {:else if isTrashActive}
            <TrashSection activeFolderId={activeTrashFolder} />
        {:else if activeFolderId}
            <ContentSection {activeFolderId} fileVersion={fileVersion} />
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

    .filter-bar {
        display: flex;
        flex-direction: column;
        gap: 0.6rem;
        padding-bottom: 1rem;
        margin-bottom: 1rem;
        border-bottom: 1px solid #e1e4e8;
        flex-shrink: 0;
    }

    .filter-row {
        display: flex;
        gap: 0.75rem;
        align-items: center;
        flex-wrap: wrap;
    }

    .name-input {
        flex: 1;
        min-width: 220px;
        padding: 0.5rem 0.8rem;
        border: 1px solid #ccc;
        border-radius: 6px;
        font-size: 0.95rem;
        outline: none;
    }

    .name-input:focus {
        border-color: #007bff;
        box-shadow: 0 0 0 3px rgba(0, 123, 255, 0.1);
    }

    .clear-filters {
        border: none;
        background: none;
        color: #007bff;
        cursor: pointer;
        font-size: 0.85rem;
        font-weight: 500;
        white-space: nowrap;
    }

    .clear-filters:hover {
        text-decoration: underline;
    }

    .label-filter {
        display: flex;
        flex-wrap: wrap;
        gap: 0.5rem;
    }

    .label-toggle {
        border: 2px solid transparent;
        background: none;
        padding: 0;
        border-radius: 999px;
        cursor: pointer;
        opacity: 0.55;
        transition: opacity 0.15s ease;
    }

    .label-toggle:hover {
        opacity: 0.85;
    }

    .label-toggle.selected {
        opacity: 1;
        border-color: #1e1e2f;
    }

    .split-view {
        display: grid;
        grid-template-columns: 260px 1fr;
        gap: 1.5rem;
        flex: 1;
        min-height: 0;
    }
</style>
