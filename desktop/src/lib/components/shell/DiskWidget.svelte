<script lang="ts">
    import ProgressBar from "$lib/components/ui/ProgressBar.svelte";
    import { formatBytes, percentOf } from "$lib/utils/format";
    import type { StorageProfileView } from "$lib/types/models";

    interface Props {
        profile: StorageProfileView | null;
    }

    const { profile }: Props = $props();

    const percent = $derived(
        profile ? percentOf(profile.taken_storage, profile.allowed_storage) : 0,
    );
</script>

<div class="disk">
    <div class="row">
        <span class="label">Disk</span>
        <span class="percent mono">{Math.round(percent)}%</span>
    </div>

    <ProgressBar {percent} height={4} label="Disk usage" />

    <p class="detail mono">
        {#if profile}
            {formatBytes(profile.taken_storage)} / {formatBytes(profile.allowed_storage)}
        {:else}
            —
        {/if}
    </p>
</div>

<style>
    .disk {
        margin: 0 14px 12px;
        padding: 11px 12px;
        border: 1px solid var(--bd-card);
        border-radius: var(--r-inset);
        background: var(--sunken);
        display: flex;
        flex-direction: column;
        gap: 7px;
        flex: none;
    }

    .row {
        display: flex;
        align-items: center;
        justify-content: space-between;
    }

    .label {
        font-size: var(--fs-label);
        color: var(--tx-mut-2);
    }

    .percent {
        font-size: var(--fs-label);
        color: var(--tx);
    }

    .detail {
        font-size: var(--fs-label);
        color: var(--tx-faint-3);
    }
</style>
