<script lang="ts">
   interface UserCardProps {
       isBlocked: boolean;
       name: string;
       email: string;
       createdAt: string;
       id: string
   }

   const {
       isBlocked,
       name,
       email,
       createdAt,
       id,
   }: UserCardProps = $props();
</script>

<div class="card user-card">
    <div class="card-header">
        <h2>Account Details</h2>
        {#if isBlocked}
            <span class="badge blocked">Blocked</span>
        {:else}
            <span class="badge active">Active</span>
        {/if}
    </div>

    <div class="avatar-section">
        <div class="avatar">{name.charAt(0).toUpperCase()}</div>
        <div class="name-block">
            <h3>{name}</h3>
            <p>{email}</p>
        </div>
    </div>

    <div class="divider"></div>

    <div class="info-row">
        <span class="label">Member Since</span>
        <span class="value">{createdAt}</span>
    </div>
    <div class="info-row">
        <span class="label">Account ID</span>
        <span class="value id-text">{id}</span>
    </div>
</div>

<style>
    header { margin-bottom: 2rem; }
    h1 { margin: 0; font-size: 2rem; }

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

    .avatar-section { display: flex; align-items: center; gap: 1.5rem; margin-bottom: 1.5rem; }
    .avatar {
        width: 80px; height: 80px;
        background: linear-gradient(135deg, #007bff, #0056b3);
        color: white; font-size: 2.5rem; font-weight: bold;
        display: flex; align-items: center; justify-content: center;
        border-radius: 50%;
    }
    .name-block h3 { margin: 0 0 0.25rem 0; font-size: 1.4rem; }
    .name-block p { margin: 0; color: #666; }

    .divider { height: 1px; background: #e1e4e8; margin: 1.5rem 0; }
    .info-row { display: flex; justify-content: space-between; margin-bottom: 0.75rem; font-size: 0.95rem; }
    .label { color: #666; font-weight: 500; }
    .value { font-weight: 600; }
    .id-text { font-family: monospace; font-size: 0.85rem; color: #888; }

    .badge { padding: 0.25rem 0.75rem; border-radius: 999px; font-size: 0.85rem; font-weight: bold; }
    .badge.active { background: #e8f5e9; color: #2e7d32; }
    .badge.blocked { background: #ffebee; color: #c62828; }

    @keyframes spin { 0% { transform: rotate(0deg); } 100% { transform: rotate(360deg); } }
</style>