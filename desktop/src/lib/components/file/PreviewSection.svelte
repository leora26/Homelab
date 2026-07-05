<script lang="ts">
    import {invoke} from "@tauri-apps/api/core";
    import type {FileView} from "$lib/types/models";
    import {formatBytes} from "$lib/components/helpers/file/formatBytes";
    import {getFileIcon} from "$lib/components/helpers/file/getFileIcon";
    import isFileArchived from "$lib/components/helpers/file/isFileArchived";
    import {notifications} from "$lib/stores/notificationStore";

    interface PreviewSectionProps {
        selectedFile: FileView;
        closePreview: () => void;
        triggerRename: () => void;
        triggerCopy: () => void;
        triggerDelete: () => void;
        triggerMove: () => void;
        triggerArchive: () => void;
        triggerUnarchive: () => void;
    }

    const {
        selectedFile,
        closePreview,
        triggerRename,
        triggerCopy,
        triggerDelete,
        triggerMove,
        triggerArchive,
        triggerUnarchive
    }: PreviewSectionProps = $props();

    let targetIsArchived = $derived(isFileArchived(selectedFile.name));

    // Preview bytes are fetched (with the auth token) through a Tauri command and
    // returned as a data URL, since an <img> tag cannot send an Authorization header.
    let previewSrc = $state<string | null>(null);
    let previewFailed = $state(false);

    $effect(() => {
        const id = selectedFile.id;
        previewSrc = null;
        previewFailed = false;

        invoke<string>('get_file_preview', {fileId: id})
            .then((dataUrl) => {
                // Ignore a response for a file the user already navigated away from.
                if (selectedFile.id === id) previewSrc = dataUrl;
            })
            .catch((e) => {
                if (selectedFile.id === id) previewFailed = true;
                console.error("Failed to load preview:", e);
            });
    });

    const handleDownload = async () => {
        try {
            await invoke('download_file', {
                fileId: selectedFile.id,
                fileName: selectedFile.name
            });
            notifications.notify("SUCCESS", "File downloaded", "You can find your file in the Downloads folder on your system");
        } catch (e) {
            notifications.notify("FAILURE", "Download failed", String(e));
        }
    };
</script>

<aside class="preview-pane">
    <div class="preview-header">
        <h3>Preview</h3>
        <button class="close-btn" onclick={closePreview}>✕</button>
    </div>

    <div class="preview-content">
        {#if previewSrc}
            <img
                    src={previewSrc}
                    alt={selectedFile.name}
                    class="preview-image"
            />
        {:else if previewFailed}
            <div class="no-preview-fallback">
                <span class="icon">{getFileIcon(selectedFile.file_type)}</span>
                <p>No preview available</p>
            </div>
        {:else}
            <div class="no-preview-fallback">
                <span class="icon">⏳</span>
                <p>Loading preview…</p>
            </div>
        {/if}
    </div>

    <div class="preview-details">
        <h4>{selectedFile.name}</h4>
        <p>Size: {formatBytes(selectedFile.size)}</p>
        <p>Modified: {selectedFile.updated_at}</p>
    </div>

    <div class="preview-actions">
        <button class="action-btn primary" onclick={handleDownload}>
            <span class="btn-icon">⬇️</span> Download File
        </button>
        <button class="action-btn" onclick={triggerRename}>
            <span class="btn-icon">✏️</span> Rename
        </button>
        <button class="action-btn" onclick={triggerCopy}>
            <span class="btn-icon">📄</span> Copy
        </button>
        <button class="action-btn" onclick={triggerMove}>
            <span class="btn-icon">📁</span> Move
        </button>

        {#if targetIsArchived}
            <button class="action-btn" onclick={triggerUnarchive}>
                <span class="btn-icon">📤</span> Extract
            </button>
        {:else}
            <button class="action-btn" onclick={triggerArchive}>
                <span class="btn-icon">📦</span> Archive
            </button>
        {/if}

        <button class="action-btn danger" onclick={triggerDelete}>
            <span class="btn-icon">🗑️</span> Delete
        </button>
    </div>
</aside>

<style>
    .preview-pane {
        width: 340px;
        background: white;
        border-radius: 8px;
        border: 1px solid #e1e4e8;
        display: flex;
        flex-direction: column;
        overflow-y: auto;
        flex-shrink: 0;
    }

    .preview-header {
        padding: 1rem 1.5rem;
        border-bottom: 1px solid #f0f2f5;
        background: #f8f9fa;
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    .preview-header h3 {
        margin: 0;
        font-size: 1rem;
        color: #1e1e2f;
    }

    .close-btn {
        background: none;
        border: none;
        cursor: pointer;
        font-size: 1.2rem;
        color: #666;
    }

    .preview-content {
        width: 100%;
        aspect-ratio: 1 / 1;
        display: flex;
        align-items: center;
        justify-content: center;
        background: #f0f2f5;
        padding: 1.5rem;
        box-sizing: border-box;
        border-bottom: 1px solid #e1e4e8;
    }
    .preview-image {
        width: 100%;
        height: 100%;
        object-fit: contain;
        filter: drop-shadow(0 4px 12px rgba(0, 0, 0, 0.15));
    }

    .no-preview-fallback {
        display: flex;
        flex-direction: column;
        align-items: center;
        color: #888;
    }

    .no-preview-fallback .icon {
        font-size: 3rem;
        margin-bottom: 0.5rem;
    }

    .hidden {
        display: none !important;
    }

    .preview-details {
        padding: 0.75rem 1.5rem 1rem;
    }

    .preview-details h4 {
        margin: 0 0 0.5rem 0;
        font-size: 0.95rem;
        word-break: break-word;
    }

    .preview-details p {
        margin: 0.25rem 0;
        font-size: 0.85rem;
        color: #666;
    }

    .preview-actions {
        padding: 1rem 1.5rem 1.5rem;
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: 0.75rem;
        background: #f8f9fa;
        border-top: 1px solid #f0f2f5;
    }

    .action-btn.primary {
        background: #007bff;
        color: white;
        border-color: #0069d9;
        grid-column: span 2;
    }

    .action-btn {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 0.5rem;
        padding: 0.6rem;
        background: white;
        border: 1px solid #e1e4e8;
        border-radius: 6px;
        font-size: 0.85rem;
        font-weight: 500;
        color: #333;
        cursor: pointer;
        transition: all 0.2s ease;
    }

    .action-btn:hover {
        background: #f0f2f5;
        border-color: #d1d5da;
    }

    .action-btn.danger {
        color: #d32f2f;
        grid-column: span 2;
    }

    .action-btn.danger:hover {
        background: #ffebee;
        border-color: #ffcdd2;
    }

    .btn-icon {
        font-size: 1.1rem;
    }
</style>