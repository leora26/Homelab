<script lang="ts">
    import Button from "$lib/components/ui/Button.svelte";
    import Checkbox from "$lib/components/ui/Checkbox.svelte";
    import Dialog from "$lib/components/ui/Dialog.svelte";
    import LabelChip from "$lib/components/ui/LabelChip.svelte";
    import { safeInvoke } from "$lib/utils/safeInvoke";
    import { toasts } from "$lib/stores/toasts.svelte";
    import type { LabelView } from "$lib/types/models";

    interface Props {
        open: boolean;
        fileId: string;
        fileName: string;
        /** Labels already on the file, so the dialog opens pre-ticked. */
        current: LabelView[];
        allLabels: LabelView[];
        onsaved: () => void;
        onnewlabel: () => void;
        onclose: () => void;
    }

    const { open, fileId, fileName, current, allLabels, onsaved, onnewlabel, onclose }: Props =
        $props();

    let selected = $state<Set<string>>(new Set());
    let busy = $state(false);

    $effect(() => {
        if (open) {
            selected = new Set(current.map((label) => label.id));
            busy = false;
        }
    });

    function toggle(labelId: string) {
        const next = new Set(selected);
        if (next.has(labelId)) next.delete(labelId);
        else next.add(labelId);
        selected = next;
    }

    /*
     * There is no "set labels" call, so the change is expressed as the difference
     * between the original set and the new one — one create or delete per change.
     */
    async function save() {
        busy = true;

        const before = new Set(current.map((label) => label.id));
        const added = [...selected].filter((id) => !before.has(id));
        const removed = [...before].filter((id) => !selected.has(id));

        const results = await Promise.all([
            ...added.map((labelId) => safeInvoke("create_fl", { fileId, labelId })),
            ...removed.map((labelId) => safeInvoke("delete_fl", { fileId, labelId })),
        ]);

        const failed = results.filter((result) => !result.ok);

        if (failed.length > 0) {
            toasts.error(
                "Some labels didn't save",
                `${failed.length} of ${results.length} changes failed.`,
            );
        }

        busy = false;
        onsaved();
    }
</script>

<Dialog {open} title="Labels on this file" subtitle={fileName} {onclose} width={420}>
    {#snippet children()}
        {#if allLabels.length === 0}
            <p class="empty">No labels yet. Create one to start tagging files.</p>
        {:else}
            <div class="rows">
                {#each allLabels as label (label.id)}
                    <div class="row">
                        <Checkbox
                            checked={selected.has(label.id)}
                            label={label.name}
                            onchange={() => toggle(label.id)}
                        />
                        <LabelChip name={label.name} color={label.color} />
                    </div>
                {/each}
            </div>
        {/if}
    {/snippet}

    {#snippet footer()}
        <button class="new-label" onclick={onnewlabel}>+ New label</button>
        <Button onclick={onclose} disabled={busy}>Cancel</Button>
        <Button variant="primary" onclick={save} disabled={busy}>
            {busy ? "Saving…" : "Save labels"}
        </Button>
    {/snippet}
</Dialog>

<style>
    .rows {
        display: flex;
        flex-direction: column;
    }

    .row {
        display: flex;
        align-items: center;
        justify-content: space-between;
        gap: 12px;
        padding: 10px 0;
        border-bottom: 1px solid var(--bd-meta);
    }

    .row:last-child {
        border-bottom: none;
    }

    /* The checkbox already renders the name; the chip shows its colour, so the
       checkbox's own text label is redundant here. */
    .row :global(.text) {
        display: none;
    }

    .new-label {
        margin-right: auto;
        font-size: var(--fs-btn);
        color: var(--link);
    }

    .new-label:hover {
        color: var(--link-hover);
    }

    .empty {
        font-size: var(--fs-btn);
        color: var(--tx-mut-2);
    }
</style>
