<script module lang="ts">
    import type { Component } from "svelte";

    export interface MenuItem {
        label: string;
        icon?: Component;
        /** Renders in the danger tone and sits below a divider. */
        destructive?: boolean;
        disabled?: boolean;
        action: () => void;
    }
</script>

<script lang="ts">
    interface Props {
        open: boolean;
        x: number;
        y: number;
        items: MenuItem[];
        onclose: () => void;
    }

    const { open, x, y, items, onclose }: Props = $props();

    let menu = $state<HTMLDivElement | null>(null);
    let position = $state({ left: 0, top: 0 });

    /*
     * Flip the menu back inside the window when opening near an edge, so it never
     * renders half off-screen in the bottom-right corner.
     */
    $effect(() => {
        if (!open || !menu) return;

        const { offsetWidth: width, offsetHeight: height } = menu;
        const margin = 8;

        position = {
            left: x + width + margin > window.innerWidth ? Math.max(margin, x - width) : x,
            top: y + height + margin > window.innerHeight ? Math.max(margin, y - height) : y,
        };
    });

    const ordinary = $derived(items.filter((item) => !item.destructive));
    const destructive = $derived(items.filter((item) => item.destructive));

    function dismiss() {
        if (open) onclose();
    }
</script>

<!-- Any click, scroll or Escape outside dismisses; the menu itself stops propagation. -->
<svelte:window
    onclick={dismiss}
    onscroll={dismiss}
    onkeydown={(event) => {
        if (event.key === "Escape") dismiss();
    }}
/>

{#if open}
    <div
        bind:this={menu}
        class="menu"
        style="left:{position.left}px; top:{position.top}px"
        role="menu"
        tabindex="-1"
        onclick={(event) => event.stopPropagation()}
        oncontextmenu={(event) => event.preventDefault()}
        onkeydown={() => {}}
    >
        {#each ordinary as item (item.label)}
            {@const Icon = item.icon}
            <button
                class="item"
                role="menuitem"
                disabled={item.disabled}
                onclick={() => {
                    item.action();
                    onclose();
                }}
            >
                {#if Icon}<Icon size={14} strokeWidth={1.8} />{/if}
                <span>{item.label}</span>
            </button>
        {/each}

        {#if destructive.length > 0 && ordinary.length > 0}
            <div class="divider"></div>
        {/if}

        {#each destructive as item (item.label)}
            {@const Icon = item.icon}
            <button
                class="item danger"
                role="menuitem"
                disabled={item.disabled}
                onclick={() => {
                    item.action();
                    onclose();
                }}
            >
                {#if Icon}<Icon size={14} strokeWidth={1.8} />{/if}
                <span>{item.label}</span>
            </button>
        {/each}
    </div>
{/if}

<style>
    .menu {
        position: fixed;
        z-index: 150;
        min-width: 176px;
        padding: 5px;
        background: var(--dialog);
        border: 1px solid var(--bd-strong);
        border-radius: var(--r-inset);
        box-shadow: var(--shadow-dialog);
        display: flex;
        flex-direction: column;
        gap: 1px;
        animation: rise 120ms ease-out;
    }

    .item {
        display: flex;
        align-items: center;
        gap: 10px;
        width: 100%;
        padding: 7px 10px;
        border-radius: var(--r-control);
        font-size: var(--fs-btn);
        color: var(--tx-2);
        text-align: left;
        transition: background var(--t-hover), color var(--t-hover);
    }

    .item :global(svg) {
        color: var(--tx-faint);
        flex: none;
    }

    .item:hover:not(:disabled) {
        background: var(--hover-nav);
        color: var(--tx-hi);
    }

    .item:disabled {
        opacity: 0.4;
        cursor: not-allowed;
    }

    .item.danger {
        color: var(--danger);
    }

    .item.danger :global(svg) {
        color: var(--danger);
    }

    .item.danger:hover:not(:disabled) {
        background: var(--danger-bg);
        color: var(--danger);
    }

    .divider {
        height: 1px;
        margin: 4px 2px;
        background: var(--bd-meta);
    }

    @keyframes rise {
        from { opacity: 0; transform: translateY(-2px); }
        to { opacity: 1; transform: translateY(0); }
    }
</style>
