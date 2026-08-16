<script lang="ts">
    import type { Snippet } from "svelte";

    interface Props {
        open: boolean;
        title: string;
        /** Mono line under the title — a destination path or filename. */
        subtitle?: string;
        /** Panel width; the handoff specifies 420–520 depending on the dialog. */
        width?: number;
        /** Switches the panel border to the danger tone. */
        destructive?: boolean;
        /** Right side of the header — the live chip preview in the label dialog. */
        headerAction?: Snippet;
        footer?: Snippet;
        onclose: () => void;
        children: Snippet;
    }

    const {
        open,
        title,
        subtitle,
        width = 420,
        destructive = false,
        headerAction,
        footer,
        onclose,
        children,
    }: Props = $props();

    let panel = $state<HTMLDivElement | null>(null);

    const FOCUSABLE =
        'input:not([disabled]), textarea:not([disabled]), select:not([disabled]), button:not([disabled]), [href], [tabindex]:not([tabindex="-1"])';

    // Move focus into the panel on open so Esc and Tab act on the dialog, not the page
    // behind it. Prefers a field over a button, since most dialogs open onto an input.
    $effect(() => {
        if (!open || !panel) return;

        const fields = panel.querySelectorAll<HTMLElement>("input, textarea, select");
        const target = fields[0] ?? panel.querySelector<HTMLElement>(FOCUSABLE);
        target?.focus();
    });

    function onKeydown(event: KeyboardEvent) {
        if (!open) return;

        if (event.key === "Escape") {
            event.preventDefault();
            onclose();
            return;
        }

        // Keep Tab inside the panel while the dialog owns the screen.
        if (event.key === "Tab" && panel) {
            const focusable = Array.from(panel.querySelectorAll<HTMLElement>(FOCUSABLE)).filter(
                (element) => element.offsetParent !== null,
            );
            if (focusable.length === 0) return;

            const first = focusable[0];
            const last = focusable[focusable.length - 1];

            if (event.shiftKey && document.activeElement === first) {
                event.preventDefault();
                last.focus();
            } else if (!event.shiftKey && document.activeElement === last) {
                event.preventDefault();
                first.focus();
            }
        }
    }
</script>

<svelte:window onkeydown={onKeydown} />

{#if open}
    <!--
      The overlay closes on click, but only when the click starts and ends on the overlay
      itself — otherwise a text selection that drags out of the panel would dismiss it.
    -->
    <div
        class="overlay"
        role="presentation"
        onmousedown={(event) => {
            if (event.target === event.currentTarget) onclose();
        }}
    >
        <div
            bind:this={panel}
            class="panel"
            class:destructive
            style="width:{width}px"
            role="dialog"
            aria-modal="true"
            aria-label={title}
            tabindex="-1"
        >
            <header class="head">
                <div class="heading">
                    <h2 class="title">{title}</h2>
                    {#if subtitle}<p class="subtitle mono">{subtitle}</p>{/if}
                </div>
                {#if headerAction}
                    <div class="head-action">{@render headerAction()}</div>
                {/if}
            </header>

            <div class="body">
                {@render children()}
            </div>

            {#if footer}
                <footer class="foot">{@render footer()}</footer>
            {/if}
        </div>
    </div>
{/if}

<style>
    .overlay {
        position: fixed;
        inset: 0;
        background: var(--overlay);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 100;
        animation: fade var(--t-dialog);
    }

    .panel {
        background: var(--dialog);
        border: 1px solid var(--bd-strong);
        border-radius: var(--r-dialog);
        box-shadow: var(--shadow-dialog);
        overflow: hidden;
        max-width: calc(100vw - 48px);
        max-height: calc(100vh - 80px);
        display: flex;
        flex-direction: column;
        animation: rise var(--t-dialog);
    }

    .panel.destructive {
        border-color: var(--danger-bd);
    }

    .head {
        display: flex;
        align-items: flex-start;
        justify-content: space-between;
        gap: 16px;
        padding: 18px 20px 14px;
        flex: none;
    }

    .heading {
        display: flex;
        flex-direction: column;
        gap: 5px;
        min-width: 0;
    }

    .title {
        font-size: var(--fs-dialog-title);
        font-weight: 600;
        color: var(--tx);
    }

    .subtitle {
        font-size: var(--fs-btn);
        color: var(--tx-mut-2);
        overflow-wrap: anywhere;
    }

    .head-action {
        flex: none;
    }

    .body {
        padding: 0 20px 18px;
        display: flex;
        flex-direction: column;
        gap: 14px;
        overflow-y: auto;
        min-height: 0;
    }

    .foot {
        display: flex;
        align-items: center;
        justify-content: flex-end;
        gap: 9px;
        padding: 14px 20px;
        border-top: 1px solid var(--bd-dialog);
        background: var(--dialog-footer);
        flex: none;
    }

    @keyframes fade {
        from { opacity: 0; }
        to { opacity: 1; }
    }

    @keyframes rise {
        from { opacity: 0; transform: translateY(2px); }
        to { opacity: 1; transform: translateY(0); }
    }
</style>
