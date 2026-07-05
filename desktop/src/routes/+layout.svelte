<script lang="ts">
    import { onMount } from "svelte";
    import { listen } from "@tauri-apps/api/event";
    import Sidebar from "$lib/components/Sidebar.svelte";
    import NotificationManager from "$lib/components/common/NotificationManager.svelte";
    import Login from "$lib/components/Login.svelte";
    import { safeInvoke } from "$lib/components/helpers/safeInvoke";

    let { children } = $props();

    let isAuthenticated = $state(false);
    let isChecking = $state(true);

    onMount(() => {
        let unlistenFn: () => void;

        const setupAuth = async () => {
            const res = await safeInvoke<boolean>("get_auth_status");
            if (res.ok) {
                isAuthenticated = res.data;
            }
            isChecking = false;

            unlistenFn = await listen<boolean>("auth_state_changed", (event) => {
                isAuthenticated = event.payload;
            });
        };

        setupAuth();

        return () => {
            if (unlistenFn) {
                unlistenFn();
            }
        };
    });
</script>

{#if isChecking}
    <div class="loader">Loading Pavuk NAS...</div>
{:else if !isAuthenticated}
    <Login />
{:else}
    <div class="app-layout">
        <Sidebar />
        <main class="content">
            {@render children()}
        </main>
    </div>
    <NotificationManager />
{/if}

<style>
    .app-layout {
        display: flex;
        height: 100vh;
        font-family: sans-serif;
    }

    .content {
        flex: 1;
        padding: 2rem;
        background: #f4f4f9;
        overflow-y: auto;
    }

    .loader {
        display: flex;
        height: 100vh;
        align-items: center;
        justify-content: center;
        font-size: 1.5rem;
        color: #555;
    }
</style>