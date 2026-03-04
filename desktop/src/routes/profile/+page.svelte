<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import UserCard from "$lib/components/UserCard.svelte";
    import StorageCard from "$lib/components/StorageCard.svelte";

    let userId = "b692e956-4c37-4b3a-8080-ec4aa2af2e6b";

    interface UserProfile {
        id: string;
        email: string;
        name: string;
        created_at: string;
    }

    interface StorageProfile {
        user_id: string;
        allowed_storage: number;
        taken_storage: number;
        is_blocked: boolean;
    }

    let error = $state<string | null>(null);
    let isLoading = $state(true);

    let user = $state<UserProfile | null>(null);
    let sp = $state<StorageProfile | null>(null);

    let percentage = $derived(
        sp ? (Number(sp.taken_storage) / Number(sp.allowed_storage)) * 100 : 0
    );

    let activeDisks = $derived(Math.round(percentage / 5));

    onMount(async () => {
        try {
            const [fetchedUser, fetchedSp] = await Promise.all([
                invoke<UserProfile>('get_user_profile', { userId }),
                invoke<StorageProfile>('get_storage_profile', { userId })
            ]);

            user = fetchedUser;
            sp = fetchedSp;
        } catch (err) {
            error = String(err);
            console.error("Failed to fetch user: ", error);
        } finally {
            isLoading = false;
        }
    });
</script>

<div class="page-container">
    <header>
        <h1>User Profile</h1>
        <p class="subtitle">Manage your account and view storage quotas.</p>
    </header>

    <main>
        {#if isLoading}
            <div class="status-card loading">
                <div class="spinner"></div>
                <p>Loading profile data...</p>
            </div>
        {:else if error}
            <div class="status-card error">
                <p>⚠️ {error}</p>
            </div>
        {:else if user && sp}
            <div class="grid-layout">

                <UserCard
                    isBlocked={sp.is_blocked}
                    name={user.name}
                    email={user.email}
                    createdAt={user.created_at}
                    id={user.id}
                />

                <StorageCard
                activeDisks={activeDisks}
                allowedStorage={sp.allowed_storage}
                takenStorage={sp.taken_storage}
                percentage={percentage}
                />

            </div>
        {/if}
    </main>
</div>

<style>
    .page-container {
        max-width: 1000px;
        margin: 0 auto;
        color: #1e1e2f;
    }

    header { margin-bottom: 2rem; }
    h1 { margin: 0; font-size: 2rem; }
    .subtitle { color: #666; margin-top: 0.5rem; }

    .grid-layout {
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: 2rem;
    }

    @media (max-width: 900px) {
        .grid-layout { grid-template-columns: 1fr; }
    }

    .status-card { text-align: center; padding: 3rem; background: white; border-radius: 12px; }
    .spinner {
        width: 30px; height: 30px;
        border: 3px solid #f3f3f3; border-top: 3px solid #007bff;
        border-radius: 50%; animation: spin 1s linear infinite; margin: 0 auto 1rem auto;
    }
    @keyframes spin { 0% { transform: rotate(0deg); } 100% { transform: rotate(360deg); } }
</style>