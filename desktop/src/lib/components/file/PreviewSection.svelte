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
        triggerRename?: () => void;
        triggerCopy?: () => void;
        triggerDelete?: () => void;
        triggerMove?: () => void;
        triggerArchive?: () => void;
        triggerUnarchive?: () => void;
        showManagementActions?: boolean;
        canToggleGlobal?: boolean;
        onGlobalChange?: (isGlobal: boolean) => void;
    }

    const {
        selectedFile,
        closePreview,
        triggerRename,
        triggerCopy,
        triggerDelete,
        triggerMove,
        triggerArchive,
        triggerUnarchive,
        showManagementActions = true,
        canToggleGlobal = true,
        onGlobalChange
    }: PreviewSectionProps = $props();

    let targetIsArchived = $derived(isFileArchived(selectedFile.name));

    // Preview bytes are fetched (with the auth token) through a Tauri command and
    // returned as a data URL, since an <img> tag cannot send an Authorization header.
    let previewSrc = $state<string | null>(null);
    let previewFailed = $state(false);

    // Whether this file is currently shared with every user. `null` while we're still
    // resolving it, so the button doesn't flash the wrong label.
    let isGlobal = $state<boolean | null>(null);
    let isTogglingGlobal = $state(false);

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

    $effect(() => {
        const id = selectedFile.id;
        isGlobal = null;

        invoke<boolean>('is_file_global', {fileId: id})
            .then((result) => {
                if (selectedFile.id === id) isGlobal = result;
            })
            .catch((e) => {
                console.error("Failed to resolve global status:", e);
            });
    });

    const toggleGlobal = async () => {
        if (isGlobal === null) return;

        isTogglingGlobal = true;
        const makePrivate = isGlobal;

        try {
            await invoke(makePrivate ? 'make_file_private' : 'make_file_global', {
                fileId: selectedFile.id
            });
            isGlobal = !makePrivate;

            if (isGlobal) {
                notifications.notify("SUCCESS", "File is now global", "Everyone can see and download this file.");
            } else {
                notifications.notify("SUCCESS", "File is now private", "This file is no longer shared with other users.");
            }

            onGlobalChange?.(isGlobal);
        } catch (e) {
            notifications.notify("FAILURE", makePrivate ? "Could not make private" : "Could not make global", String(e));
        } finally {
            isTogglingGlobal = false;
        }
    };

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

        <span class="global-status" class:shared={isGlobal === true}>
            {#if isGlobal === null}
                ⏳ Checking sharing…
            {:else if isGlobal}
                🌐 Shared with everyone
            {:else}
                🔒 Private
            {/if}
        </span>
    </div>

    <div class="preview-actions">
        <button class="action-btn primary" onclick={handleDownload}>
            <span class="btn-icon">⬇️</span> Download File
        </button>

        {#if canToggleGlobal}
            <button
                    class="action-btn global-action"
                    class:is-global={isGlobal === true}
                    onclick={toggleGlobal}
                    disabled={isGlobal === null || isTogglingGlobal}
                    title={isGlobal ? "Stop sharing this file with other users" : "Share this file with every user"}
            >
                {#if isTogglingGlobal}
                    <span class="btn-icon">⏳</span> Working…
                {:else if isGlobal}
                    <span class="btn-icon">🔒</span> Make Private
                {:else}
                    <span class="btn-icon">🌐</span> Make Global
                {/if}
            </button>
        {/if}

        {#if showManagementActions}
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
        {/if}
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

    .global-status {
        display: inline-flex;
        align-items: center;
        margin-top: 0.6rem;
        padding: 0.2rem 0.6rem;
        border-radius: 999px;
        font-size: 0.75rem;
        font-weight: 600;
        color: #555;
        background: #eceff3;
    }

    .global-status.shared {
        color: #0a6b3b;
        background: #e3f6ec;
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

    .action-btn.global-action {
        grid-column: span 2;
        color: #5b3bc4;
        border-color: #d9d0f5;
        background: #f4f0fe;
    }

    .action-btn.global-action:hover:not(:disabled) {
        background: #ece4fd;
        border-color: #c9bcf0;
    }

    .action-btn.global-action.is-global {
        color: #0a6b3b;
        border-color: #bfe6cf;
        background: #eafaf1;
    }

    .action-btn.global-action.is-global:hover:not(:disabled) {
        background: #dcf5e7;
        border-color: #a9dcbf;
    }

    .action-btn:disabled {
        opacity: 0.6;
        cursor: not-allowed;
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