<script lang="ts">
    import Tile from "$lib/components/Tile.svelte";
</script>

<main class="container">
    <header>
        <h1>Dashboard</h1>
    </header>
    <div class="tile-grid">
        <Tile
                url="/nas"
                icon="📁"
                name="NAS Storage"
                description="Browse, upload, and manage your homelab files."
        />

        <Tile
                url="/profile"
                icon="👤"
                name="User Profile"
                description="Manage your account, role, and application settings."
        />

        <Tile
                url="#"
                icon="⚙️"
                name="Settings"
                description="System configuration coming soon."
                disabled={true}
        />
    </div>
</main>

<style>
    :global(body) {
        margin: 0;
        font-family: 'Segoe UI', sans-serif;
        background-color: #f0f2f5;
    }

    .container {
        max-width: 900px;
        margin: 0 auto;
        padding: 2rem;
    }

    header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 2rem;
    }

    h1 {
        margin: 0;
        color: #1e1e2f;
    }

    .tile-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
        gap: 1.5rem;
    }
</style>