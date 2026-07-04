<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    import { listen, type UnlistenFn } from "@tauri-apps/api/event";
    import { safeInvoke } from "$lib/components/helpers/safeInvoke";
    import {goto} from "$app/navigation";

    let isLoggingIn = false;
    let errorMessage = "";
    let unlistenAuthEvent: UnlistenFn;

    onMount(async () => {
        unlistenAuthEvent = await listen<boolean>("auth_state_changed", (event) => {
            const isSuccess = event.payload;
            if (isSuccess) {
                console.log("Auth state verified via deep link!");
                // TODO: Replace with your actual router choice (e.g., page.router, svelte-routing, etc.)
                goto("/profile");
            } else {
                errorMessage = "Authentication failed.";
                isLoggingIn = false;
            }
        });
    });

    onDestroy(() => {
        if (unlistenAuthEvent) unlistenAuthEvent();
    });

    const handleLogin = async () => {
        isLoggingIn = true;
        errorMessage = "";

        const result = await safeInvoke("trigger_login");

        if (!result.ok) {
            errorMessage = result.error;
            isLoggingIn = false;
        }
    };
</script>

<div class="login-container">
    <h1>Welcome to Pavuk NAS</h1>
    <p>Please authenticate to access your homelab.</p>

    {#if errorMessage}
        <div class="error">{errorMessage}</div>
    {/if}

    <button on:click={handleLogin} disabled={isLoggingIn}>
        {isLoggingIn ? "Authenticating..." : "Log In with Zitadel"}
    </button>
</div>

<style>
    .error { color: red; margin-bottom: 1rem; }
</style>