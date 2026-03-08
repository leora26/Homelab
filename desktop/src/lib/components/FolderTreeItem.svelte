<script lang="ts">
    import {invoke} from "@tauri-apps/api/core";
    import type {FolderView} from "$lib/types/models";
    import FolderTreeItem from "./FolderTreeItem.svelte";

    interface Props {
        folder: FolderView;
        activeFolderId: string | null;
        onSelect: (folderId: string) => void;
        depth?: number;
    }

    let {
        folder,
        activeFolderId,
        onSelect,
        depth = 0
    }: Props = $props();

    let isExpanded = $state(false);
    let subfolders = $state<FolderView[]>([]);
    let isLoading = $state(false);
    let hasLoaded = $state(false);

    async function handleToggle(event: MouseEvent) {
        event.stopPropagation();

        onSelect(folder.id);
        isExpanded = !isExpanded;

        if (isExpanded && !hasLoaded) {
            isLoading = true;
            try {
                subfolders = await invoke<FolderView[]>('get_subfolders', {folderId: folder.id});
                hasLoaded = true;
            } catch (err) {
                console.error("Failed to fetch subfolders:", err);
            } finally {
                isLoading = false;
            }
        }
    }
</script>

<button
        class="tree-item"
        class:active={activeFolderId === folder.id}
        style="padding-left: {0.5 + depth * 1.2}rem;"
        onclick={handleToggle}
        role="button"
        tabindex="0"
>
    <span class="chevron" class:expanded={isExpanded}>
        {#if isLoading}
            <span class="spinner-small"></span>
        {:else}
            ▶
        {/if}
    </span>

    <span class="icon">{isExpanded ? '📂' : '📁'}</span>
    <span class="folder-name">{folder.name}</span>
</button>

{#if isExpanded}
    <div class="sub-tree">
        {#each subfolders as child (child.id)}
            <FolderTreeItem
                    folder={child}
                    {activeFolderId}
                    {onSelect}
                    depth={depth + 1}
            />
        {/each}

        {#if hasLoaded && subfolders.length === 0}
            <div class="empty-node" style="padding-left: {0.5 + (depth + 1) * 1.2}rem;">
                <span class="chevron placeholder"></span>
                <span class="folder-name text-muted">No subfolders</span>
            </div>
        {/if}
    </div>
{/if}

<style>
    .tree-item {
        padding: 0.4rem 0.5rem;
        border-radius: 6px;
        cursor: pointer;
        display: flex;
        align-items: center;
        gap: 0.4rem;
        font-size: 0.95rem;
        transition: background 0.1s;
        margin-bottom: 2px;
        user-select: none;
        border-color: transparent;
        width: 100%;
        background-color: transparent;
    }

    .tree-item:hover {
        background: #f0f2f5;
    }

    .tree-item.active {
        background: #e6f2ff;
        color: #0056b3;
        font-weight: 500;
    }

    .chevron {
        font-size: 0.6rem;
        color: #888;
        width: 16px;
        display: flex;
        justify-content: center;
        transition: transform 0.2s ease;
    }

    .chevron.expanded {
        transform: rotate(90deg);
    }

    .chevron.placeholder {
        visibility: hidden;
    }

    /* Keeps alignment correct */

    .icon {
        font-size: 1.1rem;
    }

    .folder-name {
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
    }

    .empty-node {
        display: flex;
        align-items: center;
        gap: 0.4rem;
        padding: 0.25rem 0.5rem;
        font-size: 0.85rem;
    }

    .text-muted {
        color: #a0aab5;
        font-style: italic;
    }

    .spinner-small {
        width: 10px;
        height: 10px;
        border: 2px solid #ccc;
        border-top: 2px solid #007bff;
        border-radius: 50%;
        animation: spin 1s linear infinite;
    }

    @keyframes spin {
        0% {
            transform: rotate(0deg);
        }
        100% {
            transform: rotate(360deg);
        }
    }
</style>