<script lang="ts">
    import { CircleAlert, CircleCheck, Info, X } from "@lucide/svelte";
    import { toasts } from "$lib/stores/toasts.svelte";
    import { fly } from "svelte/transition";

    const ICONS = { success: CircleCheck, error: CircleAlert, info: Info };
</script>

<div class="host" aria-live="polite">
    {#each toasts.items as toast (toast.id)}
        {@const Icon = ICONS[toast.tone]}
        <div class="toast {toast.tone}" transition:fly={{ y: 6, duration: 150 }}>
            <span class="icon"><Icon size={16} strokeWidth={1.8} /></span>

            <div class="text">
                <p class="title">{toast.title}</p>
                {#if toast.message}<p class="message">{toast.message}</p>{/if}
            </div>

            <button class="close" onclick={() => toasts.dismiss(toast.id)} aria-label="Dismiss">
                <X size={13} strokeWidth={2} />
            </button>
        </div>
    {/each}
</div>

<style>
    .host {
        position: fixed;
        right: 18px;
        bottom: 18px;
        z-index: 200;
        display: flex;
        flex-direction: column;
        gap: 9px;
        pointer-events: none;
    }

    .toast {
        pointer-events: auto;
        display: flex;
        align-items: flex-start;
        gap: 11px;
        width: 320px;
        padding: 12px 14px;
        background: var(--dialog);
        border: 1px solid var(--bd-strong);
        border-radius: var(--r-inset);
        box-shadow: var(--shadow-dialog);
    }

    .icon {
        display: flex;
        flex: none;
        margin-top: 1px;
    }

    .success .icon { color: var(--success); }
    .error .icon { color: var(--danger); }
    .info .icon { color: var(--link); }

    .error {
        border-color: var(--danger-bd);
    }

    .text {
        flex: 1;
        min-width: 0;
        display: flex;
        flex-direction: column;
        gap: 3px;
    }

    .title {
        font-size: var(--fs-btn);
        font-weight: 500;
        color: var(--tx);
    }

    .message {
        font-size: var(--fs-caption);
        color: var(--tx-mut-2);
        line-height: 1.45;
        overflow-wrap: anywhere;
    }

    .close {
        display: flex;
        flex: none;
        color: var(--tx-faint-2);
        padding: 2px;
        border-radius: var(--r-badge);
    }

    .close:hover {
        color: var(--tx);
    }
</style>
