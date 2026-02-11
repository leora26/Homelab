<script lang="ts">
    import {invoke} from "@tauri-apps/api/core";
    import {onMount} from "svelte";

    interface FileView {
        id: string;
        name: string;
        size: number;
        file_type: string;
    }

    let files = $state<FileView[]>([]);
    let error = $state<string | null>(null);
    let isLoading = $state(false);

    async function loadFiles() {
        isLoading = true;
        error = null;

        try {
            files = await invoke("get_files");
        } catch (e) {
            error = String(e);
        } finally {
            isLoading = false;
        }
    }

    onMount(() => {
        loadFiles()
    })
</script>

<main class="container">
    <header>
        <h1>My NAS Files</h1>
        <button onclick={loadFiles} disabled={isLoading}>
            {isLoading ? "Refreshing..." : "Refresh List"}
        </button>
    </header>

    {#if error}
        <div class="error">
            <p>⚠️ {error}</p>
        </div>
    {/if}

    <div class="file-grid">
        {#each files as file}
            <div class="card">
                <div class="icon">
                    {#if file.file_type === "Video"} 🎬
                    {:else if file.file_type === "Text"} 📄
                    {:else} 📁
                    {/if}
                </div>
                <div class="details">
                    <h3>{file.name}</h3>
                    <p class="meta">
                        <span>{file.file_type}</span> •
                        <span>{(file.size / 1024 / 1024).toFixed(2)} MB</span>
                    </p>
                    <small class="id">ID: {file.id.substring(0, 8)}...</small>
                </div>
            </div>
        {:else}
            <p class="empty">No files found.</p>
        {/each}
    </div>
</main>

<style>
    :global(body) {
        margin: 0;
        font-family: 'Segoe UI', sans-serif;
        background-color: #f0f2f5;
    }

    .container {
        max-width: 800px;
        margin: 0 auto;
        padding: 2rem;
    }

    header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 2rem;
    }

    button {
        padding: 0.5rem 1rem;
        cursor: pointer;
        background: #007bff;
        color: white;
        border: none;
        border-radius: 4px;
    }

    button:disabled {
        background: #ccc;
    }

    .file-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
        gap: 1rem;
    }

    .card {
        background: white;
        padding: 1rem;
        border-radius: 8px;
        box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        display: flex;
        align-items: center;
        gap: 1rem;
        transition: transform 0.2s;
    }

    .card:hover {
        transform: translateY(-2px);
    }

    .icon {
        font-size: 2rem;
        background: #e9ecef;
        width: 50px;
        height: 50px;
        display: flex;
        align-items: center;
        justify-content: center;
        border-radius: 50%;
    }

    .details h3 {
        margin: 0 0 0.25rem 0;
        font-size: 1rem;
    }

    .meta {
        margin: 0;
        font-size: 0.85rem;
        color: #666;
    }

    .id {
        display: block;
        font-size: 0.7rem;
        color: #999;
        margin-top: 0.5rem;
    }

    .error {
        background: #fee;
        color: #c00;
        padding: 1rem;
        border-radius: 4px;
        margin-bottom: 1rem;
    }
</style>
