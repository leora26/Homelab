<script lang="ts">
    import type { Snippet } from "svelte";

    type Variant = "primary" | "secondary" | "destructive" | "danger-solid" | "ghost";
    type Size = "sm" | "md" | "lg";

    interface Props {
        variant?: Variant;
        size?: Size;
        /** Stretches to the container — the sign-in CTA and details-rail actions. */
        block?: boolean;
        disabled?: boolean;
        type?: "button" | "submit";
        title?: string;
        onclick?: (event: MouseEvent) => void;
        children: Snippet;
    }

    const {
        variant = "secondary",
        size = "md",
        block = false,
        disabled = false,
        type = "button",
        title,
        onclick,
        children,
    }: Props = $props();
</script>

<button
    {type}
    {title}
    {disabled}
    {onclick}
    class="btn {variant} {size}"
    class:block
>
    {@render children()}
</button>

<style>
    .btn {
        display: inline-flex;
        align-items: center;
        justify-content: center;
        gap: 7px;
        border-radius: var(--r-control);
        font-weight: 500;
        white-space: nowrap;
        border: 1px solid transparent;
        transition: background var(--t-hover), border-color var(--t-hover), color var(--t-hover);
    }

    .btn:disabled {
        opacity: 0.45;
        cursor: not-allowed;
    }

    .block {
        width: 100%;
    }

    /* ---- sizes ---- */
    .sm {
        padding: 5px 11px;
        font-size: var(--fs-sm);
    }

    .md {
        padding: 6px 14px;
        font-size: var(--fs-btn);
    }

    .lg {
        padding: 9px 14px;
        font-size: var(--fs-base);
    }

    /* ---- variants ---- */
    .primary {
        background: var(--accent);
        color: #ffffff;
    }

    .primary:hover:not(:disabled) {
        background: var(--accent-hover);
    }

    .secondary {
        background: transparent;
        border-color: var(--bd-strong);
        color: var(--tx-2);
    }

    .secondary:hover:not(:disabled) {
        background: var(--hover-nav);
        color: var(--tx-hi);
    }

    /* Outlined destructive — "Move to trash", "Delete forever". */
    .destructive {
        background: transparent;
        border-color: var(--danger-bd);
        color: var(--danger);
    }

    .destructive:hover:not(:disabled) {
        background: var(--danger-bg);
    }

    /* Filled destructive — the confirming action inside a dialog, and "Empty trash". */
    .danger-solid {
        background: var(--danger-solid);
        color: #ffffff;
    }

    .danger-solid:hover:not(:disabled) {
        background: var(--danger-solid-hover);
    }

    .ghost {
        background: transparent;
        color: var(--tx-mut-2);
    }

    .ghost:hover:not(:disabled) {
        background: var(--hover-nav);
        color: var(--tx-hi);
    }
</style>
