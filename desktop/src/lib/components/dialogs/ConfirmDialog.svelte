<script lang="ts">
    import { TriangleAlert } from "@lucide/svelte";
    import Button from "$lib/components/ui/Button.svelte";
    import Dialog from "$lib/components/ui/Dialog.svelte";

    interface Props {
        open: boolean;
        title: string;
        body: string;
        /** Label of the confirming action, e.g. "Move to trash". */
        confirmLabel: string;
        busy?: boolean;
        onconfirm: () => void;
        onclose: () => void;
    }

    const { open, title, body, confirmLabel, busy = false, onconfirm, onclose }: Props = $props();
</script>

<Dialog {open} {title} {onclose} width={420} destructive>
    {#snippet children()}
        <div class="row">
            <span class="tile"><TriangleAlert size={18} strokeWidth={1.8} /></span>
            <p class="body">{body}</p>
        </div>
    {/snippet}

    {#snippet footer()}
        <Button onclick={onclose} disabled={busy}>Cancel</Button>
        <Button variant="danger-solid" onclick={onconfirm} disabled={busy}>
            {busy ? "Working…" : confirmLabel}
        </Button>
    {/snippet}
</Dialog>

<style>
    .row {
        display: flex;
        align-items: flex-start;
        gap: 14px;
    }

    .tile {
        width: 34px;
        height: 34px;
        border-radius: var(--r-card);
        background: var(--danger-bg-tile);
        color: var(--danger);
        display: flex;
        align-items: center;
        justify-content: center;
        flex: none;
    }

    .body {
        font-size: var(--fs-btn);
        color: var(--tx-mut);
        line-height: 1.55;
        text-wrap: pretty;
    }
</style>
