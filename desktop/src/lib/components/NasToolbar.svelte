<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { open } from "@tauri-apps/plugin-dialog";
    import { stat } from "@tauri-apps/plugin-fs";
    import { userId } from "$lib/types/tempUserId";
    import type { FileView } from "$lib/types/models";


    interface NasToolbarProps {
        openNewFolderModal: (targetId?: string) => void;
        activeFolderId: string | null;
        onUploadComplete?: () => void;
    }

    const {
        openNewFolderModal,
        activeFolderId,
        onUploadComplete
    }: NasToolbarProps = $props();

    let isUploading = $state(false);

    const handleFileUpload = async () => {
        if (!activeFolderId) return;

        try {
            const selectedPath = await open({
                multiple: false,
                title: "Select a file to upload"
            });

            if (!selectedPath) return;

            isUploading = true;

            const fileName = selectedPath.split(/[\\/]/).pop() || "unknown_file";

            console.log(`Preparing to initialize upload for ${fileName}...`);

            const fileView = await invoke<FileView>('init_file', {
                name: fileName,
                destination: activeFolderId,
                ownerId: userId,
                localPath: selectedPath,
                isGlobal: false
            });

            console.log("Initialization successful. Streaming content...");

            await invoke('upload_content', {
                fileId: fileView.id,
                localPath: selectedPath
            });

            console.log("Upload complete!");

            if (onUploadComplete) onUploadComplete();

        } catch (err) {
            console.error("File upload failed:", err);
            alert(`Upload failed: ${err}`);
        } finally {
            isUploading = false;
        }
    }
</script>

<header class="toolbar">
    <h2>NAS Storage</h2>
    <div class="actions">
        <button onclick={() => openNewFolderModal()} class="btn secondary" disabled={!activeFolderId}>
            📁 New Folder
        </button>
        <button
                class="btn primary"
                onclick={handleFileUpload}
                disabled={!activeFolderId || isUploading}
        >
            {isUploading ? '⏳ Uploading...' : '⬆️ Upload File'}
        </button>
    </div>
</header>

<style>
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

</style>