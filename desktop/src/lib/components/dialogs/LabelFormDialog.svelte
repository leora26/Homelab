<script lang="ts">
    import Button from "$lib/components/ui/Button.svelte";
    import Dialog from "$lib/components/ui/Dialog.svelte";
    import LabelChip from "$lib/components/ui/LabelChip.svelte";
    import TextField from "$lib/components/ui/TextField.svelte";
    import { LABEL_PRESETS, PRESET_ORDER, nearestPreset } from "$lib/utils/labels";

    interface Props {
        open: boolean;
        /** Absent for create, present for edit. */
        initialName?: string;
        initialColor?: string;
        onsubmit: (name: string, color: string) => Promise<void> | void;
        onclose: () => void;
    }

    const { open, initialName = "", initialColor, onsubmit, onclose }: Props = $props();

    const isEdit = $derived(initialName.length > 0);

    let name = $state("");
    let color = $state(LABEL_PRESETS.amber.swatch);
    let busy = $state(false);
    let error = $state<string | null>(null);

    $effect(() => {
        if (open) {
            name = initialName;
            // An existing label may hold an off-palette colour from the old colour
            // wheel; snap it to the nearest preset so the selection ring has a home.
            color = initialColor
                ? LABEL_PRESETS[nearestPreset(initialColor)].swatch
                : LABEL_PRESETS.amber.swatch;
            error = null;
            busy = false;
        }
    });

    const canSubmit = $derived(name.trim().length > 0 && !busy);

    async function submit() {
        if (!canSubmit) return;

        busy = true;
        error = null;

        try {
            await onsubmit(name.trim(), color);
        } catch (e) {
            error = e instanceof Error ? e.message : String(e);
            busy = false;
        }
    }
</script>

<Dialog
    {open}
    title={isEdit ? "Edit label" : "New label"}
    {onclose}
    width={440}
>
    {#snippet headerAction()}
        <LabelChip name={name.trim() || "Label"} {color} />
    {/snippet}

    {#snippet children()}
        <TextField
            bind:value={name}
            label="Name"
            placeholder="e.g. Invoices"
            error={error ?? undefined}
            autofocus
            onenter={submit}
        />

        <div class="colour">
            <span class="label">Colour</span>
            <div class="swatches">
                {#each PRESET_ORDER as key (key)}
                    {@const preset = LABEL_PRESETS[key]}
                    <button
                        type="button"
                        class="swatch"
                        class:selected={color === preset.swatch}
                        style="background:{preset.swatch}; --ring:{preset.swatch}55"
                        aria-label={key}
                        aria-pressed={color === preset.swatch}
                        onclick={() => (color = preset.swatch)}
                    ></button>
                {/each}
            </div>
            <p class="hint">Six preset colours keep labels readable in tables and chips.</p>
        </div>
    {/snippet}

    {#snippet footer()}
        <Button onclick={onclose} disabled={busy}>Cancel</Button>
        <Button variant="primary" onclick={submit} disabled={!canSubmit}>
            {busy ? "Saving…" : isEdit ? "Save changes" : "Create label"}
        </Button>
    {/snippet}
</Dialog>

<style>
    .colour {
        display: flex;
        flex-direction: column;
        gap: 9px;
    }

    .label {
        font-size: var(--fs-sm);
        color: var(--tx-mut);
    }

    .swatches {
        display: flex;
        gap: 10px;
    }

    .swatch {
        width: 30px;
        height: 30px;
        border-radius: var(--r-inset);
        cursor: pointer;
        transition: box-shadow var(--t-hover);
    }

    .swatch.selected {
        box-shadow: 0 0 0 3px var(--ring);
    }

    .hint {
        font-size: var(--fs-caption);
        color: var(--tx-faint-2);
    }
</style>
