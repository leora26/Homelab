<script lang="ts">
    export interface ContextMenuOption {
        label: string;
        icon?: string;
        danger?: boolean;
        action: () => void;
    }

    interface Props {
        x: number;
        y: number;
        options: ContextMenuOption[];
    }

    let {
        x,
        y,
        options
    }: Props = $props();
</script>

<div
        class="context-menu"
        style="top: {y}px; left: {x}px;"
>
    {#each options as option}
        <button
                class:danger={option.danger}
                onclick={(e) => {
                e.stopPropagation();
                option.action();
            }}
        >
            {#if option.icon}
                <span class="icon">{option.icon}</span>
            {/if}
            {option.label}
        </button>
    {/each}
</div>

<style>
    .context-menu {
        position: fixed;
        background: white;
        border-radius: 6px;
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
        border: 1px solid #e1e4e8;
        padding: 0.5rem 0;
        min-width: 160px;
        z-index: 9999;
        display: flex;
        flex-direction: column;
    }

    .context-menu button {
        background: none;
        border: none;
        padding: 0.5rem 1rem;
        text-align: left;
        font-size: 0.9rem;
        cursor: pointer;
        display: flex;
        align-items: center;
        gap: 0.5rem;
        color: #1e1e2f;
        width: 100%;
    }

    .context-menu button:hover {
        background: #f0f2f5;
    }

    .context-menu button.danger {
        color: #d32f2f;
    }

    .context-menu button.danger:hover {
        background: #ffebee;
    }

    .icon {
        font-size: 1.1rem;
    }
</style>