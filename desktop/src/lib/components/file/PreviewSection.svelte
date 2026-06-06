<script lang="ts">
    import type {FileView} from "$lib/types/models";
    import {formatBytes} from "$lib/components/helpers/file/formatBytes";
    import {getFileIcon} from "$lib/components/helpers/file/getFileIcon";

    interface PreviewSectionProps {
        selectedFile: FileView;
        closePreview: () => void;
    }

    const {
        selectedFile,
        closePreview
    }: PreviewSectionProps = $props();
</script>

<aside class="preview-pane">
    <div class="preview-header">
        <h3>Preview</h3>
        <button class="close-btn" onclick={closePreview}>✕</button>
    </div>

    <div class="preview-content">
        <img
                src={`http://127.0.0.1:8080/api/files/${selectedFile.id}/preview`}
                alt={selectedFile.name}
                class="preview-image"
        />
        <div class="no-preview-fallback hidden">
            <span class="icon">{getFileIcon(selectedFile.file_type)}</span>
            <p>No preview available</p>
        </div>
    </div>

    <div class="preview-details">
        <h4>{selectedFile.name}</h4>
        <p>Size: {formatBytes(selectedFile.size)}</p>
        <p>Modified: {selectedFile.updated_at}</p>
    </div>

</aside>

<style>
    .preview-pane {
        width: 320px;
        background: white;
        border-radius: 8px;
        border: 1px solid #e1e4e8;
        display: flex;
        flex-direction: column;
        overflow: hidden;
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
        flex: 1;
        display: flex;
        align-items: center;
        justify-content: center;
        background: #f0f2f5;
        padding: 1rem;
        overflow: hidden;
    }

    .preview-image {
        max-width: 100%;
        max-height: 100%;
        object-fit: contain;
        border-radius: 4px;
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
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
        padding: 1.5rem;
        border-top: 1px solid #f0f2f5;
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
</style>