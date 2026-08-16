<script lang="ts">
    import type { Component, Snippet } from "svelte";

    interface Props {
        /** Lucide icon component, drawn at 24px inside the dashed tile. */
        icon: Component;
        title: string;
        body?: string;
        /** One action, per the handoff's empty-state composition. */
        action?: Snippet;
    }

    const { icon: Icon, title, body, action }: Props = $props();
</script>

<div class="empty">
    <div class="tile">
        <Icon size={24} strokeWidth={1.6} />
    </div>

    <div class="text">
        <p class="title">{title}</p>
        {#if body}<p class="body">{body}</p>{/if}
    </div>

    {#if action}
        {@render action()}
    {/if}
</div>

<style>
    .empty {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        gap: 16px;
        padding: 40px;
        flex: 1;
        text-align: center;
    }

    .tile {
        width: 56px;
        height: 56px;
        border-radius: var(--r-window);
        border: 1px dashed #2f3849;
        display: flex;
        align-items: center;
        justify-content: center;
        color: var(--tx-ghost);
        flex: none;
    }

    .text {
        display: flex;
        flex-direction: column;
        gap: 6px;
        align-items: center;
    }

    .title {
        font-size: var(--fs-empty-title);
        font-weight: 600;
        color: var(--tx);
    }

    .body {
        font-size: var(--fs-btn);
        color: var(--tx-mut-2);
        max-width: 280px;
        line-height: 1.5;
        text-wrap: pretty;
    }
</style>
