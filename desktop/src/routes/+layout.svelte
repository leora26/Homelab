<script lang="ts">
    import Sidebar from "$lib/components/Sidebar.svelte";
    let { children } = $props();
</script>

<div class="app-layout">
    <Sidebar />

    <main class="content">
        {@render children()}
    </main>
</div>

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
</style>