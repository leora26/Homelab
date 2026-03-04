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
    #dashboard {
        font-size: 1.2rem;
        opacity: 1;
    }
    .app-layout {
        display: flex;
        height: 100vh;
        font-family: sans-serif;
    }
    .sidebar {
        width: 200px;
        background: #1e1e2f;
        color: white;
        padding: 1rem;
    }
    .sidebar ul {
        list-style: none;
        padding: 0;
    }
    .sidebar a {
        color: #cdcdcd;
        text-decoration: none;
        display: block;
        padding: 0.5rem 0;
        opacity: 0.7;
    }
    .sidebar a:hover {
        color: white;
    }
    .content {
        flex: 1;
        padding: 2rem;
        background: #f4f4f9;
        overflow-y: auto;
    }
</style>