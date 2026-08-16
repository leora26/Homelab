<script lang="ts">
    import type { Snippet } from "svelte";

    /** Semantic chip sets from the handoff, plus `custom` for label colours. */
    type Tone = "neutral" | "private" | "shared" | "success" | "warning" | "danger" | "custom";

    interface Props {
        tone?: Tone;
        /** Required when tone is "custom" — supplies bg / text / border. */
        bg?: string;
        color?: string;
        border?: string;
        /** Renders as a button with hover and pressed states. */
        interactive?: boolean;
        selected?: boolean;
        title?: string;
        onclick?: () => void;
        children: Snippet;
    }

    const {
        tone = "neutral",
        bg,
        color,
        border,
        interactive = false,
        selected = false,
        title,
        onclick,
        children,
    }: Props = $props();

    const customStyle = $derived(
        tone === "custom"
            ? `background:${bg ?? "transparent"};color:${color ?? "inherit"};border-color:${border ?? "transparent"};`
            : "",
    );
</script>

{#if interactive}
    <button
        type="button"
        {title}
        {onclick}
        class="chip {tone}"
        class:interactive
        class:selected
        style={customStyle}
    >
        {@render children()}
    </button>
{:else}
    <span {title} class="chip {tone}" style={customStyle}>
        {@render children()}
    </span>
{/if}

<style>
    .chip {
        display: inline-flex;
        align-items: center;
        gap: 6px;
        padding: 4px 9px;
        border-radius: var(--r-chip);
        font-size: var(--fs-label);
        font-weight: 500;
        line-height: 1.3;
        white-space: nowrap;
        border: 1px solid transparent;
    }

    .interactive {
        cursor: pointer;
        /* Unselected filter chips sit back so the active ones read first. */
        opacity: 0.62;
        transition: opacity var(--t-hover), box-shadow var(--t-hover);
    }

    .interactive:hover {
        opacity: 0.85;
    }

    .interactive.selected {
        opacity: 1;
        box-shadow: inset 0 0 0 1px currentColor;
    }

    .neutral {
        background: var(--slate-bg);
        color: var(--slate);
        border-color: var(--slate-bd);
    }

    .private {
        background: var(--private-bg);
        color: var(--private-tx);
        border-color: var(--private-bd);
    }

    .shared,
    .success {
        background: var(--success-bg);
        color: var(--success);
        border-color: var(--success-bd);
    }

    .warning {
        background: var(--warning-bg);
        color: var(--warning);
        border-color: var(--warning-bd);
    }

    .danger {
        background: var(--danger-bg);
        color: var(--danger);
        border-color: var(--danger-bd);
    }
</style>
