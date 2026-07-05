<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    import { listen, type UnlistenFn } from "@tauri-apps/api/event";
    import { safeInvoke } from "$lib/components/helpers/safeInvoke";
    import { goto } from "$app/navigation";

    let isLoggingIn = $state(false);
    let errorMessage = $state("");
    let unlistenAuthEvent: UnlistenFn;

    onMount(async () => {
        unlistenAuthEvent = await listen<boolean>("auth_state_changed", (event) => {
            const isSuccess = event.payload;
            if (isSuccess) {
                console.log("Auth state verified via deep link!");
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

<div class="login-screen">
    <div class="login-card">
        <div class="brand">
            <div class="brand-mark">
                <svg viewBox="0 0 64 64" aria-hidden="true">
                    <g fill="none" stroke="currentColor" stroke-width="2.4" stroke-linecap="round" stroke-linejoin="round">
                        <line x1="32" y1="32" x2="32" y2="4" />
                        <line x1="32" y1="32" x2="56" y2="18" />
                        <line x1="32" y1="32" x2="56" y2="46" />
                        <line x1="32" y1="32" x2="32" y2="60" />
                        <line x1="32" y1="32" x2="8" y2="46" />
                        <line x1="32" y1="32" x2="8" y2="18" />
                        <polygon points="32,13 46,21 46,43 32,51 18,43 18,21" />
                        <polygon points="32,22 39,26 39,38 32,42 25,38 25,26" />
                    </g>
                </svg>
            </div>
            <span class="brand-name">Pavuk<span class="brand-suffix">NAS</span></span>
        </div>

        <h1>Welcome back</h1>
        <p class="subtitle">Sign in to access your homelab storage.</p>

        {#if errorMessage}
            <div class="error" role="alert">
                <span class="error-icon">⚠</span>
                <span>{errorMessage}</span>
            </div>
        {/if}

        <button class="login-btn" onclick={handleLogin} disabled={isLoggingIn}>
            {#if isLoggingIn}
                <span class="spinner" aria-hidden="true"></span>
                Authenticating…
            {:else}
                Log in with Zitadel
            {/if}
        </button>

        <p class="footnote">
            <span class="lock">🔒</span> Secured by Zitadel single sign-on
        </p>
    </div>
</div>

<style>
    .login-screen {
        min-height: 100vh;
        width: 100%;
        display: flex;
        align-items: center;
        justify-content: center;
        padding: 2rem;
        box-sizing: border-box;
        font-family: sans-serif;
        background:
            radial-gradient(1000px 600px at 15% 10%, rgba(0, 123, 255, 0.35), transparent 60%),
            radial-gradient(900px 700px at 90% 90%, rgba(88, 86, 214, 0.35), transparent 55%),
            linear-gradient(135deg, #131a2e 0%, #1e2a4a 55%, #223a6b 100%);
    }

    .login-card {
        width: 100%;
        max-width: 400px;
        background: #ffffff;
        border-radius: 18px;
        padding: 2.75rem 2.5rem 2rem;
        box-sizing: border-box;
        text-align: center;
        box-shadow:
            0 20px 60px rgba(10, 20, 45, 0.45),
            0 2px 6px rgba(10, 20, 45, 0.2);
        animation: rise 0.45s cubic-bezier(0.16, 1, 0.3, 1);
    }

    .brand {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 0.75rem;
        margin-bottom: 1.75rem;
    }

    .brand-mark {
        width: 48px;
        height: 48px;
        border-radius: 14px;
        display: grid;
        place-items: center;
        color: #ffffff;
        background: linear-gradient(135deg, #007bff, #5856d6);
        box-shadow: 0 6px 16px rgba(0, 123, 255, 0.4);
    }

    .brand-mark svg {
        width: 30px;
        height: 30px;
    }

    .brand-name {
        font-size: 1.4rem;
        font-weight: 700;
        letter-spacing: -0.01em;
        color: #1e1e2f;
    }

    .brand-suffix {
        color: #007bff;
        margin-left: 0.15rem;
    }

    h1 {
        margin: 0 0 0.4rem;
        font-size: 1.5rem;
        font-weight: 700;
        color: #1e1e2f;
    }

    .subtitle {
        margin: 0 0 1.75rem;
        font-size: 0.95rem;
        color: #667085;
        line-height: 1.4;
    }

    .error {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        text-align: left;
        background: #fdecec;
        border: 1px solid #f6cccc;
        color: #c0392b;
        border-radius: 10px;
        padding: 0.7rem 0.9rem;
        font-size: 0.85rem;
        margin-bottom: 1.25rem;
    }

    .error-icon {
        font-size: 0.95rem;
        flex-shrink: 0;
    }

    .login-btn {
        width: 100%;
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 0.6rem;
        padding: 0.85rem 1rem;
        font-size: 0.98rem;
        font-weight: 600;
        color: #ffffff;
        border: none;
        border-radius: 11px;
        cursor: pointer;
        background: linear-gradient(135deg, #007bff, #0069d9);
        box-shadow: 0 8px 20px rgba(0, 123, 255, 0.35);
        transition: transform 0.15s ease, box-shadow 0.15s ease, filter 0.15s ease;
    }

    .login-btn:hover:not(:disabled) {
        transform: translateY(-1px);
        box-shadow: 0 10px 26px rgba(0, 123, 255, 0.45);
        filter: brightness(1.05);
    }

    .login-btn:active:not(:disabled) {
        transform: translateY(0);
        box-shadow: 0 5px 14px rgba(0, 123, 255, 0.35);
    }

    .login-btn:disabled {
        cursor: default;
        opacity: 0.75;
        box-shadow: none;
    }

    .spinner {
        width: 16px;
        height: 16px;
        border: 2px solid rgba(255, 255, 255, 0.45);
        border-top-color: #ffffff;
        border-radius: 50%;
        animation: spin 0.7s linear infinite;
    }

    .footnote {
        margin: 1.5rem 0 0;
        font-size: 0.78rem;
        color: #98a2b3;
    }

    .lock {
        font-size: 0.72rem;
    }

    @keyframes spin {
        to { transform: rotate(360deg); }
    }

    @keyframes rise {
        from { opacity: 0; transform: translateY(12px); }
        to { opacity: 1; transform: translateY(0); }
    }

    @media (prefers-reduced-motion: reduce) {
        .login-card { animation: none; }
        .spinner { animation-duration: 1.4s; }
        .login-btn { transition: none; }
    }
</style>
