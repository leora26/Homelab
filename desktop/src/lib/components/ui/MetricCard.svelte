<script lang="ts">
    interface Props {
        label: string;
        value: string;
        caption: string;
        /** Green caption for a positive figure, muted otherwise. */
        positive?: boolean;
    }

    const { label, value, caption, positive = false }: Props = $props();
</script>

<div class="metric">
    <p class="label">{label}</p>
    <p class="value mono">{value}</p>
    <p class="caption" class:positive>{caption}</p>
</div>

<style>
    .metric {
        border: 1px solid var(--bd);
        border-radius: var(--r-card);
        background: var(--card);
        padding: 14px 15px;
        display: flex;
        flex-direction: column;
        gap: 6px;
        min-width: 0;
    }

    .label {
        font-size: var(--fs-label);
        text-transform: uppercase;
        letter-spacing: var(--track-label);
        color: var(--tx-faint);
    }

    .value {
        font-size: var(--fs-metric);
        font-weight: 600;
        color: var(--tx);
        line-height: 1.15;
    }

    .caption {
        font-size: var(--fs-caption);
        color: var(--tx-faint-2);
    }

    .caption.positive {
        color: var(--success);
    }
</style>
