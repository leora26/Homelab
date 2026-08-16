<script lang="ts">
    import { Check } from "@lucide/svelte";

    interface Props {
        checked: boolean;
        label?: string;
        /** 14px on the sign-in row, 15px in the label picker. */
        size?: number;
        disabled?: boolean;
        onchange?: (checked: boolean) => void;
    }

    let { checked = $bindable(), label, size = 15, disabled = false, onchange }: Props = $props();

    function toggle() {
        if (disabled) return;
        checked = !checked;
        onchange?.(checked);
    }
</script>

<button
    type="button"
    class="row"
    class:disabled
    role="checkbox"
    aria-checked={checked}
    aria-label={label}
    {disabled}
    onclick={toggle}
>
    <span class="box" class:checked style="width:{size}px;height:{size}px">
        {#if checked}<Check size={size - 4} strokeWidth={3} />{/if}
    </span>
    {#if label}<span class="text">{label}</span>{/if}
</button>

<style>
    .row {
        display: inline-flex;
        align-items: center;
        gap: 9px;
        cursor: pointer;
    }

    .row.disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }

    .box {
        display: flex;
        align-items: center;
        justify-content: center;
        flex: none;
        border-radius: var(--r-badge);
        border: 1px solid #3a4354;
        color: #ffffff;
        transition: background var(--t-hover), border-color var(--t-hover);
    }

    .box.checked {
        background: var(--accent);
        border-color: var(--accent);
    }

    .text {
        font-size: var(--fs-btn);
        color: var(--tx-2);
    }
</style>
