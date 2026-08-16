<script lang="ts">
    interface Props {
        /** 0–100. Clamped, so an over-quota value can't overflow the track. */
        percent: number;
        /** Fill colour: accent while running, green on success, red on failure. */
        tone?: "accent" | "success" | "danger";
        /** 4px in tables and the upload dialog, 8px on the profile quota card. */
        height?: number;
        label?: string;
    }

    const { percent, tone = "accent", height = 4, label }: Props = $props();

    const clamped = $derived(Math.min(100, Math.max(0, percent)));
</script>

<div
    class="track"
    style="height:{height}px;border-radius:{height / 2}px"
    role="progressbar"
    aria-valuenow={Math.round(clamped)}
    aria-valuemin={0}
    aria-valuemax={100}
    aria-label={label}
>
    <div
        class="fill {tone}"
        style="width:{clamped}%;border-radius:{height / 2}px"
    ></div>
</div>

<style>
    .track {
        width: 100%;
        background: #1c2331;
        overflow: hidden;
    }

    .fill {
        height: 100%;
        transition: width 200ms ease-out;
    }

    .accent { background: var(--accent); }
    .success { background: var(--success); }
    .danger { background: var(--danger); }
</style>
