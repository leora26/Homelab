<script lang="ts">
    import FormModal from "$lib/components/common/FormModal.svelte";
    import {safeInvoke} from "$lib/components/helpers/safeInvoke";

    let {
        isOpen = $bindable(),
        activeFolderId,
        folderId,
        rootFolder,
        onActiveFolderChange,
        onSuccess
    } = $props();


    const confirmDeleteFolder = async () => {
        if (!activeFolderId) return;

        await safeInvoke('delete_selected_folder', {selectedFolderId: folderId});

        console.log(`Successfully deleted folder: ${folderId}`);

        isOpen = false;

        if (activeFolderId === folderId && rootFolder) {
            onActiveFolderChange(rootFolder.id, false);
        }

        folderId = null;

        onSuccess();
    };
</script>

<FormModal
        isOpen={isOpen}
        title="Delete Folder"
        description="Are you sure you want to permanently delete this folder? This action cannot be undone and subfolder and files withing this folder will be permanently deleted."
        fields={[]}
        submitText="Yes, Delete"
        loadingText="Deleting..."
        onClose={() => isOpen = false}
        onSubmit={confirmDeleteFolder}
/>