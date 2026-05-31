<script lang="ts">
    import FormModal, { type FormField } from "$lib/components/common/FormModal.svelte";
    import { safeInvoke } from "$lib/components/helpers/safeInvoke";

    let { isOpen = $bindable(), folderId, currentName, onSuccess } = $props();

    let renameFields = $derived<FormField[]>([
        { name: "newFolderName", label: "Folder Name", type: "text", required: true, defaultValue: currentName }
    ]);

    const confirmRename = async (data: Record<string, string | number>) => {
        const newName = String(data.newFolderName).trim();
        if (newName !== currentName) {
            await safeInvoke('rename_folder', { folderId, newName });
            onSuccess();
        }
        isOpen = false;
    };
</script>

<FormModal {isOpen} title="Rename Folder" fields={renameFields} onClose={() => isOpen = false} onSubmit={confirmRename} />