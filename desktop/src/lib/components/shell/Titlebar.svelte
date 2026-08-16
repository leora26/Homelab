<script lang="ts">
    import { ChevronDown, Minus, Square, X } from "@lucide/svelte";
    import type { MachineInfoView } from "$lib/types/models";

    interface Props {
        machine: MachineInfoView | null;
        /** Whether the NAS server answered on startup — drives the status dot. */
        reachable?: boolean;
    }

    const { machine, reachable = true }: Props = $props();

    // The window API only exists inside the Tauri webview; in `vite dev` in a browser
    // these are no-ops rather than a crash on every click.
    async function windowAction(action: "minimize" | "toggleMaximize" | "close") {
        try {
            const { getCurrentWindow } = await import("@tauri-apps/api/window");
            await getCurrentWindow()[action]();
        } catch (error) {
            console.warn(`Window control "${action}" unavailable outside Tauri`, error);
        }
    }
</script>

<!-- The bar itself is the drag handle; every control below stops the drag. -->
<header class="titlebar" data-tauri-drag-region>
    <div class="left" data-tauri-drag-region>
        <span class="mark"></span>
        <span class="wordmark">Pavuk</span>

        <button class="host" title="Machine switcher — coming with multi-machine support">
            <span class="dot" class:offline={!reachable}></span>
            <span class="host-name mono">{machine?.hostname ?? "…"}</span>
            <ChevronDown size={12} strokeWidth={2} />
        </button>
    </div>

    <div class="right">
        <button onclick={() => windowAction("minimize")} aria-label="Minimise">
            <Minus size={13} strokeWidth={2} />
        </button>
        <button onclick={() => windowAction("toggleMaximize")} aria-label="Maximise">
            <Square size={11} strokeWidth={2} />
        </button>
        <button class="close" onclick={() => windowAction("close")} aria-label="Close">
            <X size={13} strokeWidth={2} />
        </button>
    </div>
</header>

<style>
    .titlebar {
        height: var(--h-titlebar);
        flex: none;
        display: flex;
        align-items: center;
        justify-content: space-between;
        gap: 14px;
        padding: 0 12px;
        background: var(--titlebar);
        border-bottom: 1px solid var(--bd-alt);
    }

    .left {
        display: flex;
        align-items: center;
        gap: 10px;
        flex: 1;
        min-width: 0;
    }

    .mark {
        width: 18px;
        height: 18px;
        border-radius: 5px;
        background: var(--accent);
        flex: none;
    }

    .wordmark {
        font-size: var(--fs-base);
        font-weight: 600;
        color: var(--tx);
    }

    .host {
        display: flex;
        align-items: center;
        gap: 7px;
        padding: 4px 10px;
        border: 1px solid var(--bd-strong);
        border-radius: var(--r-control);
        font-size: var(--fs-sm);
        color: var(--tx-2);
        transition: background var(--t-hover);
    }

    .host:hover {
        background: var(--hover-nav);
    }

    .host :global(svg) {
        color: var(--tx-faint-2);
    }

    .dot {
        width: 6px;
        height: 6px;
        border-radius: 50%;
        background: var(--success);
        flex: none;
    }

    .dot.offline {
        background: var(--danger);
    }

    .host-name {
        max-width: 240px;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
    }

    .right {
        display: flex;
        align-items: center;
        justify-content: flex-end;
        gap: 16px;
        flex: none;
    }

    .right button {
        display: flex;
        color: var(--tx-faint);
        padding: 3px;
        border-radius: var(--r-badge);
        transition: color var(--t-hover), background var(--t-hover);
    }

    .right button:hover {
        color: var(--tx);
        background: var(--hover-nav);
    }

    .right .close:hover {
        color: #ffffff;
        background: var(--danger-solid);
    }
</style>
