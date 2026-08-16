<script lang="ts">
    import { ChevronDown } from "@lucide/svelte";

    interface Option {
        value: string;
        label: string;
    }

    interface Props {
        value: string;
        options: Option[];
        label?: string;
        /** Fixed width; omit to size to content. */
        width?: number;
        disabled?: boolean;
    }

    let { value = $bindable(), options, label, width, disabled = false }: Props = $props();
</script>

<div class="field" style={width ? `width:${width}px` : undefined}>
    {#if label}<span class="label">{label}</span>{/if}

    <div class="shell" class:disabled>
        <select bind:value {disabled} aria-label={label}>
            {#each options as option (option.value)}
                <option value={option.value}>{option.label}</option>
            {/each}
        </select>
        <span class="chevron"><ChevronDown size={13} strokeWidth={2} /></span>
    </div>
</div>

<style>
    .field {
        display: flex;
        flex-direction: column;
        gap: 6px;
        min-width: 0;
    }

    .label {
        font-size: var(--fs-sm);
        color: var(--tx-mut);
    }

    .shell {
        position: relative;
        display: flex;
        align-items: center;
        background: var(--inset);
        border: 1px solid var(--bd-alt);
        border-radius: var(--r-control);
        transition: border-color var(--t-hover);
    }

    .shell:focus-within {
        border-color: var(--accent);
    }

    .shell.disabled {
        opacity: 0.55;
    }

    select {
        appearance: none;
        background: none;
        border: none;
        outline: none;
        /* Right padding leaves room for the chevron. */
        padding: 7px 28px 7px 11px;
        font-size: var(--fs-btn);
        color: var(--tx);
        width: 100%;
        cursor: pointer;
    }

    /* The native dropdown list renders with the OS palette, so its options need
       explicit colours to match the app rather than inheriting a white popup. */
    select option {
        background: var(--dialog);
        color: var(--tx);
    }

    .chevron {
        position: absolute;
        right: 9px;
        display: flex;
        color: var(--tx-faint-2);
        pointer-events: none;
    }
</style>
