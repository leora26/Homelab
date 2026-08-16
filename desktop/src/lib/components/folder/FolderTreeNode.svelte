<script lang="ts">
    import { ChevronDown, ChevronRight } from "@lucide/svelte";
    import Self from "$lib/components/folder/FolderTreeNode.svelte";
    import { safeInvoke } from "$lib/utils/safeInvoke";
    import { treeRevision } from "$lib/utils/folderPath.svelte";
    import { ROOT_LABEL } from "$lib/utils/paths";
    import type { FolderView } from "$lib/types/models";

    interface Props {
        folder: FolderView;
        depth: number;
        selectedId: string | null;
        /** Rendered muted and unselectable — the move dialog's current folder. */
        disabledId?: string | null;
        /** Ids whose subtree should be open on first render. */
        expandTo?: string[];
        onselect: (folder: FolderView) => void;
        oncontextmenu?: (event: MouseEvent, folder: FolderView) => void;
    }

    const {
        folder,
        depth,
        selectedId,
        disabledId = null,
        expandTo = [],
        onselect,
        oncontextmenu,
    }: Props = $props();

    let expanded = $state(false);
    let children = $state<FolderView[]>([]);

    const isRoot = $derived(folder.parent_folder_id === null);
    const label = $derived(isRoot ? ROOT_LABEL : folder.name);
    const disabled = $derived(folder.id === disabledId);

    // Open the branch containing the folder the caller wants revealed.
    $effect(() => {
        if (expandTo.includes(folder.id) && !expanded) {
            expanded = true;
        }
    });

    // Reading `treeRevision.value` here is what subscribes this node to structural
    // changes — a folder created or deleted anywhere re-runs the fetch.
    $effect(() => {
        treeRevision.value;

        if (!expanded) return;

        safeInvoke<FolderView[]>("get_subfolders", { folderId: folder.id }).then((result) => {
            if (result.ok) children = result.data;
        });
    });

    function toggle(event: MouseEvent) {
        event.stopPropagation();
        expanded = !expanded;
    }

    function select() {
        if (!disabled) onselect(folder);
    }
</script>

<div
    class="row"
    class:selected={selectedId === folder.id}
    class:disabled
    style="padding-left:{6 + depth * 16}px"
    role="treeitem"
    aria-selected={selectedId === folder.id}
    aria-expanded={expanded}
    tabindex="0"
    onclick={select}
    onkeydown={(event) => {
        if (event.key === "Enter" || event.key === " ") {
            event.preventDefault();
            select();
        }
    }}
    oncontextmenu={(event) => oncontextmenu?.(event, folder)}
>
    <button class="twisty" onclick={toggle} tabindex="-1" aria-label={expanded ? "Collapse" : "Expand"}>
        {#if expanded}
            <ChevronDown size={13} strokeWidth={2} />
        {:else}
            <ChevronRight size={13} strokeWidth={2} />
        {/if}
    </button>

    <span class="name truncate">{label}</span>

    {#if disabled}
        <span class="marker">· current folder</span>
    {/if}
</div>

{#if expanded}
    {#each children as child (child.id)}
        <Self
            folder={child}
            depth={depth + 1}
            {selectedId}
            {disabledId}
            {expandTo}
            {onselect}
            {oncontextmenu}
        />
    {/each}
{/if}

<style>
    .row {
        display: flex;
        align-items: center;
        gap: 4px;
        padding-top: 6px;
        padding-bottom: 6px;
        padding-right: 8px;
        border-radius: var(--r-control);
        font-size: var(--fs-base);
        color: var(--tx-2);
        cursor: pointer;
        transition: background var(--t-hover), color var(--t-hover);
    }

    .row:hover:not(.disabled) {
        background: var(--hover-nav);
        color: var(--tx-hi);
    }

    .row.selected {
        background: var(--nav-active);
        color: var(--tx-hi);
        font-weight: 500;
    }

    .row.disabled {
        color: var(--tx-faint-2);
        cursor: not-allowed;
    }

    .twisty {
        display: flex;
        color: var(--tx-ghost);
        flex: none;
        border-radius: var(--r-dot);
    }

    .twisty:hover {
        color: var(--tx-2);
    }

    .name {
        min-width: 0;
    }

    .marker {
        font-size: var(--fs-caption);
        color: var(--tx-faint-2);
        flex: none;
    }
</style>
