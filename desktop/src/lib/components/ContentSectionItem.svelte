<script lang="ts">
    import type {FileView} from "$lib/types/models";
    import {getFileIcon} from "$lib/components/helpers/file/getFileIcon";
    import {formatBytes} from "$lib/components/helpers/file/formatBytes";

    interface Props {
        onContextMenu: (e: MouseEvent, fileId: string, fileName: string) => void;
        file: FileView
    }

    const {
        onContextMenu,
        file
    }: Props = $props()
</script>

<tr
        class="file-row"
        oncontextmenu={(e) => {
                                e.preventDefault();
                                onContextMenu(e, file.id, file.name)
                            }}
>
    <td class="col-name">
        <div class="name-cell">
            <span class="icon" aria-hidden="true">{getFileIcon(file.file_type)}</span>
            <span class="file-name">{file.name}</span>
        </div>
    </td>
    <td class="col-date">{file.updated_at}</td>
    <td class="col-size">{formatBytes(file.size)}</td>
</tr>

<style>
    .file-row {
        cursor: pointer;
        transition: background-color 0.15s ease;
    }

    .file-row:hover {
        background-color: #f4f6f8;
    }

    .col-name { width: 55%; }
    .col-date { width: 30%; color: #666; }
    .col-size { width: 15%; color: #666; text-align: right; }


    .name-cell {
        display: flex;
        align-items: center;
        gap: 0.75rem;
    }

    .icon {
        font-size: 1.25rem;
    }

    .file-name {
        font-weight: 500;
        color: #1e1e2f;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
        max-width: 400px;
    }
</style>