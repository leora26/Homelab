<script lang="ts">
    import type { Snippet } from "svelte";

    interface Props {
        /** Card title, rendered in the bordered header row. */
        title?: string;
        /** Right-hand side of the header — a link, count, or button. */
        action?: Snippet;
        /** Removes body padding, for cards whose content is a full-bleed table. */
        flush?: boolean;
        /** Lets the card own the remaining height in a flex column. */
        fill?: boolean;
        children: Snippet;
    }

    const { title, action, flush = false, fill = false, children }: Props = $props();
</script>

<section class="card" class:fill>
    {#if title || action}
        <header class="head">
            {#if title}<h2 class="title">{title}</h2>{/if}
            {#if action}
                <div class="action">{@render action()}</div>
            {/if}
        </header>
    {/if}

    <div class="body" class:flush>
        {@render children()}
    </div>
</section>

<style>
    .card {
        border: 1px solid var(--bd);
        border-radius: var(--r-card);
        background: var(--card);
        overflow: hidden;
        display: flex;
        flex-direction: column;
        min-height: 0;
    }

    .fill {
        flex: 1;
    }

    .head {
        display: flex;
        align-items: center;
        justify-content: space-between;
        gap: 16px;
        padding: 13px 18px;
        border-bottom: 1px solid #202632;
        flex: none;
    }

    .title {
        font-size: var(--fs-card-title);
        font-weight: 600;
        color: var(--tx);
    }

    .action {
        display: flex;
        align-items: center;
        gap: 10px;
        font-size: var(--fs-sm);
        color: var(--tx-faint-2);
    }

    .body {
        padding: 18px;
        display: flex;
        flex-direction: column;
        gap: 14px;
        min-height: 0;
        flex: 1;
    }

    .body.flush {
        padding: 0;
        gap: 0;
    }
</style>
