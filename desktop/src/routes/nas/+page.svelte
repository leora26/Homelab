<script lang="ts">
    import FolderStructure from "$lib/components/FolderStructure.svelte";
    import ContentSection from "$lib/components/ContentSection.svelte";

    let activeFolderId = $state<string | null>(null);

    const handleActiveFolderChange = (folderId: string) => {
        activeFolderId = folderId
    }
</script>

<div class="app-layout">
    <header class="toolbar">
        <h2>NAS Storage</h2>
        <div class="actions">
            <button class="btn secondary">📁 New Folder</button>
            <button class="btn primary">⬆️ Upload File</button>
        </div>
    </header>

    <main class="split-view">


        <FolderStructure
                activeFolderId={activeFolderId}
                onActiveFolderChange={handleActiveFolderChange}
        />

        {#if activeFolderId}
            <ContentSection
                    activeFolderId={activeFolderId}
            />
        {/if}
    </main>
</div>

<style>
    .app-layout {
        display: flex;
        flex-direction: column;
        height: calc(100vh - 4rem);
        color: #1e1e2f;
    }

    .toolbar {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding-bottom: 1rem;
        margin-bottom: 1rem;
        border-bottom: 1px solid #e1e4e8;
        flex-shrink: 0;
    }

    .toolbar h2 {
        margin: 0;
        font-size: 1.5rem;
    }

    .actions {
        display: flex;
        gap: 1rem;
    }

    .btn {
        padding: 0.5rem 1rem;
        border-radius: 6px;
        font-weight: 500;
        cursor: pointer;
        border: none;
    }

    .btn.primary {
        background: #007bff;
        color: white;
    }

    .btn.secondary {
        background: #f0f2f5;
        border: 1px solid #d1d5db;
    }

    .split-view {
        display: grid;
        /* Left pane is 260px, right pane takes the rest */
        grid-template-columns: 260px 1fr;
        gap: 1.5rem;
        flex: 1;
        min-height: 0; /* Crucial for scrolling inside CSS Grid children */
    }

    @keyframes spin {
        0% {
            transform: rotate(0deg);
        }
        100% {
            transform: rotate(360deg);
        }
    }
</style>