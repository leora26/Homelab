<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import type { FileView, LabelView } from "$lib/types/models";
    import { getFileIcon } from "$lib/components/helpers/file/getFileIcon";
    import { formatBytes } from "$lib/components/helpers/file/formatBytes";
    import LabelChip from "$lib/components/label/LabelChip.svelte";

    interface Props {
        onContextMenu: (e: MouseEvent, fileId: string, fileName: string) => void;
        onClick: (file: FileView) => void;
        file: FileView;
        isSelected?: boolean;
        labelsVersion?: number;
    }

    const {
        onContextMenu,
        onClick,
        file,
        isSelected,
        labelsVersion = 0
    }: Props = $props();

    const NAME_MAX = 15;
    const LABEL_MAX = 10;

    const truncate = (text: string, max: number) =>
        text.length > max ? text.slice(0, max) + "…" : text;

    const nameTruncated = $derived(file.name.length > NAME_MAX);
    const displayName = $derived(truncate(file.name, NAME_MAX));

    let labels = $state<LabelView[]>([]);

    $effect(() => {
        const id = file.id;
        labelsVersion;

        invoke<LabelView[]>("get_labels_for_file", { fileId: id })
            .then((res) => {
                if (file.id === id) labels = res;
            })
            .catch((e) => console.error("Failed to load labels for file:", e));
    });
</script>

<tr
        class="file-row"
        class:selected={isSelected}
        onclick={() => onClick(file)}
        oncontextmenu={(e) => {
            e.preventDefault();
            onContextMenu(e, file.id, file.name)
        }}
>
    <td class="col-name">
        <div class="name-cell">
            <span class="icon" aria-hidden="true">{getFileIcon(file.file_type)}</span>

            <span class="name-wrap">
                <span class="file-name">{displayName}</span>
                {#if nameTruncated}
                    <span class="tooltip name-tooltip">{file.name}</span>
                {/if}
            </span>

            {#if labels.length > 0}
                <span class="row-labels">
                    {#each labels.slice(0, 2) as label (label.id)}
                        <LabelChip name={truncate(label.name, LABEL_MAX)} color={label.color} />
                    {/each}
                    {#if labels.length > 2}
                        <span class="more">+{labels.length - 2}</span>
                    {/if}

                    <span class="tooltip labels-tooltip">
                        {#each labels as label (label.id)}
                            <LabelChip name={label.name} color={label.color} />
                        {/each}
                    </span>
                </span>
            {/if}
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

    .file-row.selected {
        background-color: #e3f2fd;
    }

    .col-name {
        width: 55%;
    }

    .col-date {
        width: 30%;
        color: #666;
    }

    .col-size {
        width: 15%;
        color: #666;
        text-align: right;
    }

    .name-cell {
        display: flex;
        align-items: center;
        gap: 0.6rem;
    }

    .icon {
        font-size: 1.25rem;
    }

    .file-name {
        font-weight: 500;
        color: #1e1e2f;
        white-space: nowrap;
    }

    .name-wrap,
    .row-labels {
        position: relative;
        display: inline-flex;
        align-items: center;
    }

    .row-labels {
        gap: 0.3rem;
    }

    .tooltip {
        display: none;
        position: absolute;
        top: calc(100% + 5px);
        left: 0;
        z-index: 50;
        white-space: nowrap;
    }

    .name-wrap:hover .name-tooltip,
    .row-labels:hover .labels-tooltip {
        display: flex;
    }

    .name-tooltip {
        background: #1e1e2f;
        color: #ffffff;
        padding: 0.3rem 0.55rem;
        border-radius: 5px;
        font-size: 0.75rem;
        font-weight: 500;
        box-shadow: 0 3px 10px rgba(0, 0, 0, 0.28);
    }

    .labels-tooltip {
        flex-direction: column;
        align-items: flex-start;
        gap: 0.3rem;
        background: #ffffff;
        border: 1px solid #e1e4e8;
        border-radius: 6px;
        padding: 0.5rem;
        box-shadow: 0 4px 14px rgba(0, 0, 0, 0.15);
    }

    .more {
        font-size: 0.75rem;
        font-weight: 600;
        color: #888;
    }
</style>
