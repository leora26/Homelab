<script lang="ts">
    import "@fontsource/ibm-plex-sans/400.css";
    import "@fontsource/ibm-plex-sans/500.css";
    import "@fontsource/ibm-plex-sans/600.css";
    import "@fontsource/ibm-plex-mono/400.css";
    import "@fontsource/ibm-plex-mono/500.css";
    import "$lib/styles/tokens.css";
    import "$lib/styles/base.css";

    import { onMount } from "svelte";
    import { goto } from "$app/navigation";
    import { listen } from "@tauri-apps/api/event";

    import Titlebar from "$lib/components/shell/Titlebar.svelte";
    import Sidebar from "$lib/components/shell/Sidebar.svelte";
    import ToastHost from "$lib/components/ui/ToastHost.svelte";
    import Login from "$lib/components/Login.svelte";
    import { session } from "$lib/stores/session.svelte";
    import { safeInvoke } from "$lib/utils/safeInvoke";

    let { children } = $props();

    let isAuthenticated = $state(false);
    let isChecking = $state(true);

    onMount(() => {
        let unlisten: (() => void) | undefined;

        (async () => {
            const status = await safeInvoke<boolean>("get_auth_status");
            if (status.ok) isAuthenticated = status.data;
            isChecking = false;

            if (isAuthenticated) await session.load();

            unlisten = await listen<boolean>("auth_state_changed", async (event) => {
                isAuthenticated = event.payload;

                if (event.payload) {
                    await session.load();
                    // The handoff lands a signed-in user on the Dashboard.
                    goto("/");
                } else {
                    session.clear();
                }
            });
        })();

        return () => unlisten?.();
    });
</script>

<!--
  The window chrome is ours: `decorations: false` in tauri.conf.json means this element
  is the entire window, including its border and corner radius.
-->
<div class="window">
    {#if isChecking}
        <div class="boot">
            <span class="mark"></span>
            <p>Starting Pavuk…</p>
        </div>
    {:else if !isAuthenticated}
        <Login />
    {:else}
        <Titlebar machine={session.machine} />

        <div class="body">
            <Sidebar profile={session.profile} />
            <main class="content">
                {@render children()}
            </main>
        </div>
    {/if}
</div>

<ToastHost />

<style>
    .window {
        height: 100vh;
        display: flex;
        flex-direction: column;
        background: var(--canvas);
        border: 1px solid var(--bd-window);
        border-radius: var(--r-window);
        overflow: hidden;
    }

    .body {
        flex: 1;
        display: flex;
        min-height: 0;
    }

    /*
     * Routes own their own padding — the handoff gives each screen a different value —
     * so the pane only establishes the scroll container and the flex context.
     */
    .content {
        flex: 1;
        min-width: 0;
        display: flex;
        flex-direction: column;
        overflow: hidden;
    }

    .boot {
        flex: 1;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        gap: 14px;
        color: var(--tx-mut-2);
        font-size: var(--fs-nav);
    }

    .boot .mark {
        width: 26px;
        height: 26px;
        border-radius: 7px;
        background: var(--accent);
        animation: pulse 1.4s ease-in-out infinite;
    }

    @keyframes pulse {
        0%, 100% { opacity: 1; }
        50% { opacity: 0.45; }
    }
</style>
