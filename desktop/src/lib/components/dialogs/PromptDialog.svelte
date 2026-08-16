<script lang="ts">
    import Button from "$lib/components/ui/Button.svelte";
    import Dialog from "$lib/components/ui/Dialog.svelte";
    import TextField from "$lib/components/ui/TextField.svelte";

    interface Props {
        open: boolean;
        title: string;
        /** Mono line under the title — the target path, or the file being renamed. */
        subtitle?: string;
        fieldLabel: string;
        hint?: string;
        initialValue?: string;
        confirmLabel: string;
        placeholder?: string;
        onsubmit: (value: string) => Promise<void> | void;
        onclose: () => void;
    }

    const {
        open,
        title,
        subtitle,
        fieldLabel,
        hint,
        initialValue = "",
        confirmLabel,
        placeholder,
        onsubmit,
        onclose,
    }: Props = $props();

    let value = $state("");
    let busy = $state(false);
    let error = $state<string | null>(null);

    // Reset whenever the dialog is opened, so a reused instance doesn't show the
    // previous entry or a stale error.
    $effect(() => {
        if (open) {
            value = initialValue;
            error = null;
            busy = false;
        }
    });

    const canSubmit = $derived(value.trim().length > 0 && !busy);

    async function submit() {
        if (!canSubmit) return;

        busy = true;
        error = null;

        try {
            await onsubmit(value.trim());
        } catch (e) {
            error = e instanceof Error ? e.message : String(e);
            busy = false;
        }
    }
</script>

<Dialog {open} {title} {subtitle} {onclose} width={420}>
    {#snippet children()}
        <TextField
            bind:value
            label={fieldLabel}
            {placeholder}
            {hint}
            error={error ?? undefined}
            autofocus
            onenter={submit}
        />
    {/snippet}

    {#snippet footer()}
        <Button onclick={onclose} disabled={busy}>Cancel</Button>
        <Button variant="primary" onclick={submit} disabled={!canSubmit}>
            {busy ? "Working…" : confirmLabel}
        </Button>
    {/snippet}
</Dialog>
