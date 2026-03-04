<script lang="ts">
   interface StorageCardProps {
       percentage: number;
       activeDisks: number;
       takenStorage: number;
       allowedStorage: number;
   }

   function formatBytes(bytes: number) {
       if (bytes === 0) return '0 B';
       const k = 1024;
       const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
       const i = Math.floor(Math.log(bytes) / Math.log(k));
       return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
   }

   const {
       percentage,
       allowedStorage,
       takenStorage,
       activeDisks
   }: StorageCardProps = $props();
</script>

<div class="card storage-card">
    <div class="card-header">
        <h2>Storage Overview</h2>
        <span class="percentage-text">{percentage.toFixed(1)}% Used</span>
    </div>

    <div class="storage-visualization">
        <div class="db-stack">
            {#each Array(20) as _, i}
                <div class="disk" class:taken={i < activeDisks}>
                    <div class="disk-top"></div>
                </div>
            {/each}
        </div>

        <div class="storage-stats">
            <div class="stat-box">
                <span class="stat-label">Used Space</span>
                <span class="stat-value taken-color">{formatBytes(takenStorage)}</span>
            </div>
            <div class="stat-box">
                <span class="stat-label">Total Capacity</span>
                <span class="stat-value">{formatBytes(allowedStorage)}</span>
            </div>
            <div class="stat-box">
                <span class="stat-label">Free Space</span>
                <span class="stat-value free-color">{formatBytes(allowedStorage - takenStorage)}</span>
            </div>
        </div>
    </div>
</div>

<style>

    .card {
        background: white;
        border-radius: 12px;
        padding: 2rem;
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.05);
        border: 1px solid #e1e4e8;
        display: flex;
        flex-direction: column;
    }

    .card-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 1.5rem;
    }
    .card-header h2 { margin: 0; font-size: 1.25rem; }

    .percentage-text { font-size: 1.2rem; font-weight: bold; color: #007bff; }

    .storage-visualization {
        display: flex;
        align-items: flex-start;
        gap: 2rem;
        flex: 1;
        flex-wrap: wrap;
        justify-content: center;
    }

    .db-stack {
        display: flex;
        flex-direction: column-reverse;
        gap: 1px;
        width: 70px;
        margin-top: 5px;
    }

    .disk {
        height: 6px;
        background-color: #d1d5db;
        border-radius: 100% / 50%;
        position: relative;
        box-shadow: 0 1px 0 #9ca3af;
        transition: all 0.3s ease;
    }

    .disk-top {
        position: absolute;
        top: 0; left: 0; right: 0; bottom: 0;
        border-radius: inherit;
        background-color: #e5e7eb;
        border: 1px solid #d1d5db;
    }

    .disk.taken { background-color: #2e7d32; box-shadow: 0 1px 0 #1b5e20; }
    .disk.taken .disk-top { background-color: #4caf50; border-color: #388e3c; }

    .storage-stats { display: flex; flex-direction: column; gap: 1.5rem; flex: 1; min-width: 150px; }
    .stat-box { display: flex; flex-direction: column; }
    .stat-label { font-size: 0.85rem; color: #666; margin-bottom: 0.25rem; text-transform: uppercase; letter-spacing: 0.5px; }
    .stat-value { font-size: 1.25rem; font-weight: bold; }
    .taken-color { color: #2e7d32; }
    .free-color { color: #666; }

    @keyframes spin { 0% { transform: rotate(0deg); } 100% { transform: rotate(360deg); } }
</style>