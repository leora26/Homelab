<script lang="ts">
    interface StatusMessageProps {
        status: "FAILURE" | "SUCCESS" | "INFO",
        title: string;
        message?: string;
    }

    const {
        status,
        title,
        message
    }: StatusMessageProps = $props();

    let statusClass = $derived(status.toLowerCase());

</script>

<div class="status-message-container {statusClass}">
    <p class="status-header">{title}:</p>
    {#if message}
        <p class="status-body">{message}</p>
    {/if}
</div>

<style>
    .status-message-container {
        padding: 0.5rem 1rem;
        border-radius: 6px;
        border-left: 5px solid transparent;
        font-family: sans-serif;
        display: flex;
        flex-direction: row;
        justify-content: flex-start;
        opacity: 0.7;
        align-items: center;
    }

    .success {
        background-color: #d4edda;
        border-color: #155724;
        color: #28a745;
    }

    .failure {
        background-color: #f8d7da;
        border-color: #721c24;
        color: #dc3545;
    }

    .info {
        background-color: #d1ecf1;
        border-color: #0c5460;
        color: #17a2b8;
    }

    .status-header {
        font-weight: bold;
        font-size: 0.8rem;
        margin-right: 5px;
    }

    .status-body {
        font-size: 0.8rem;
    }

</style>