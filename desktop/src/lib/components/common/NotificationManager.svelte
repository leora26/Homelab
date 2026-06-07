<script>
    import { notifications } from "$lib/stores/notificationStore";
    import StatusMessage from "./StatusMessage.svelte";
</script>

<div class="notification-wrapper">
    {#each $notifications as note (note.id)}
        <StatusMessage
                status={note.status}
                title={note.title}
                message={note.message}
        />
    {/each}
</div>

<style>
    .notification-wrapper {
        position: fixed;
        bottom: 20px;
        right: 30%;
        z-index: 9999;
        display: flex;
        flex-direction: column;
        gap: 10px;
    }
</style>