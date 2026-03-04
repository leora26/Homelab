<script lang="ts">
    interface TileProps {
        url: string;
        icon: string;
        name: string;
        description: string;
        disabled?: boolean;
    }

    const {
        url,
        icon,
        name,
        description,
        disabled
    }: TileProps = $props()
</script>

<a href={url} class="tile" class:disabled>
    <div class="tile-icon">{icon}</div>
    <div class="tile-content">
        <h2>{name}</h2>
        <p>{description}</p>
    </div>
</a>

<style>
    .tile {
        background: white;
        border-radius: 12px;
        padding: 1.5rem;
        text-decoration: none;
        color: inherit;
        display: flex;
        flex-direction: column;
        gap: 1rem;
        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.05);
        border: 1px solid #e1e4e8;
        transition: transform 0.2s ease, box-shadow 0.2s ease;
    }

    .tile:hover:not(.disabled) {
        transform: translateY(-4px);
        box-shadow: 0 8px 15px rgba(0, 0, 0, 0.1);
        border-color: #007bff;
    }

    .tile.disabled {
        opacity: 0.6;
        cursor: not-allowed;
        background: #f8f9fa;
    }

    .tile-icon {
        font-size: 2.5rem;
        background: #f0f2f5;
        width: 60px;
        height: 60px;
        display: flex;
        align-items: center;
        justify-content: center;
        border-radius: 12px;
    }

    .tile-content h2 {
        margin: 0 0 0.5rem 0;
        font-size: 1.25rem;
        color: #1e1e2f;
    }

    .tile-content p {
        margin: 0;
        font-size: 0.9rem;
        color: #666;
        line-height: 1.4;
    }
</style>