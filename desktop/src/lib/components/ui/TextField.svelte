<script lang="ts">
    import type { Component } from "svelte";

    interface Props {
        value: string;
        label?: string;
        placeholder?: string;
        /** Explanatory line under the field, e.g. naming rules. */
        hint?: string;
        /** Replaces the hint and turns the border red. */
        error?: string;
        /** Paths, names and IDs render in the mono face. */
        mono?: boolean;
        /** Leading icon — the magnifier on search and filter fields. */
        icon?: Component;
        disabled?: boolean;
        readonly?: boolean;
        autofocus?: boolean;
        /** Fixed width; omit to fill the container. */
        width?: number;
        onenter?: () => void;
    }

    let {
        value = $bindable(),
        label,
        placeholder,
        hint,
        error,
        mono = false,
        icon: Icon,
        disabled = false,
        readonly = false,
        autofocus = false,
        width,
        onenter,
    }: Props = $props();

    let input = $state<HTMLInputElement | null>(null);

    $effect(() => {
        if (autofocus && input) {
            input.focus();
            input.select();
        }
    });
</script>

<div class="field" style={width ? `width:${width}px` : undefined}>
    {#if label}<label class="label" for={label}>{label}</label>{/if}

    <div class="shell" class:invalid={!!error} class:disabled>
        {#if Icon}
            <span class="icon"><Icon size={13} strokeWidth={2} /></span>
        {/if}
        <input
            bind:this={input}
            bind:value
            id={label}
            class="input selectable"
            class:mono
            {placeholder}
            {disabled}
            {readonly}
            onkeydown={(event) => {
                if (event.key === "Enter") onenter?.();
            }}
        />
    </div>

    {#if error}
        <p class="error">{error}</p>
    {:else if hint}
        <p class="hint">{hint}</p>
    {/if}
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
        display: flex;
        align-items: center;
        gap: 8px;
        padding: 8px 11px;
        background: var(--inset);
        border: 1px solid var(--bd-alt);
        border-radius: var(--r-control);
        transition: border-color var(--t-hover);
    }

    .shell:focus-within {
        border-color: var(--accent);
    }

    .shell.invalid {
        border-color: var(--danger-bd);
    }

    .shell.disabled {
        opacity: 0.55;
    }

    .icon {
        display: flex;
        color: var(--tx-faint-2);
        flex: none;
    }

    .input {
        flex: 1;
        min-width: 0;
        background: none;
        border: none;
        outline: none;
        font-size: var(--fs-btn);
        color: var(--tx);
    }

    .input.mono {
        font-family: var(--font-mono);
    }

    .input::placeholder {
        color: var(--tx-faint-2);
    }

    .hint,
    .error {
        font-size: var(--fs-caption);
        text-wrap: pretty;
    }

    .hint {
        color: var(--tx-faint-2);
    }

    .error {
        color: var(--danger);
    }
</style>
