<script lang="ts">
    import Button from "$lib/components/ui/Button.svelte";
    import Dialog from "$lib/components/ui/Dialog.svelte";
    import FolderTreeNode from "$lib/components/folder/FolderTreeNode.svelte";
    import { ancestorsOf } from "$lib/utils/folderPath.svelte";
    import { pathString } from "$lib/utils/paths";
    import type { FolderView } from "$lib/types/models";

    interface Props {
        open: boolean;
        /** "move" and "copy" share this dialog; only the title and CTA differ. */
        mode: "move" | "copy";
        fileName: string;
        root: FolderView | null;
        /** The file's current folder — shown disabled, since it's a no-op target. */
        currentFolderId: string | null;
        onsubmit: (folderId: string) => Promise<void> | void;
        onclose: () => void;
    }

    const { open, mode, fileName, root, currentFolderId, onsubmit, onclose }: Props = $props();

    let selected = $state<FolderView | null>(null);
    let destination = $state("");
    let busy = $state(false);

    $effect(() => {
        if (open) {
            selected = null;
            destination = "";
            busy = false;
        }
    });

    // Resolve the chosen folder to a readable path for the footer.
    $effect(() => {
        const target = selected;
        if (!target) {
            destination = "";
            return;
        }

        ancestorsOf(target.id).then((segments) => {
            if (selected?.id === target.id) destination = pathString(segments);
        });
    });

    const canSubmit = $derived(selected !== null && !busy);

    async function submit() {
        if (!selected || busy) return;

        busy = true;
        try {
            await onsubmit(selected.id);
        } finally {
            busy = false;
        }
    }
</script>

<Dialog
    {open}
    title={mode === "move" ? "Move file" : "Copy file"}
    subtitle={fileName}
    {onclose}
    width={460}
>
    {#snippet children()}
        <p class="prompt">Choose a destination folder</p>

        <div class="tree" role="tree" aria-label="Destination folder">
            {#if root}
                <FolderTreeNode
                    folder={root}
                    depth={0}
                    selectedId={selected?.id ?? null}
                    disabledId={currentFolderId}
                    expandTo={root ? [root.id] : []}
                    onselect={(folder) => (selected = folder)}
                />
            {/if}
        </div>
    {/snippet}

    {#snippet footer()}
        <span class="destination mono truncate">{destination}</span>
        <Button onclick={onclose} disabled={busy}>Cancel</Button>
        <Button variant="primary" onclick={submit} disabled={!canSubmit}>
            {busy ? "Working…" : mode === "move" ? "Move here" : "Copy here"}
        </Button>
    {/snippet}
</Dialog>

<style>
    .prompt {
        font-size: var(--fs-btn);
        color: var(--tx-mut);
    }

    .tree {
        max-height: 280px;
        overflow-y: auto;
        border: 1px solid var(--bd-meta);
        border-radius: var(--r-inset);
        background: var(--inset);
        padding: 6px;
    }

    .destination {
        margin-right: auto;
        font-size: var(--fs-caption);
        color: var(--tx-faint-2);
        min-width: 0;
        max-width: 220px;
    }
</style>
