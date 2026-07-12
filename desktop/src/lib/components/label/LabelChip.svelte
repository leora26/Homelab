<script lang="ts">
    interface Props {
        name: string;
        color: string;
    }

    const { name, color }: Props = $props();

    // Choose black or white text based on the background's perceived brightness (YIQ),
    // so the label stays readable on any color. Falls back to dark text if the color
    // isn't a parseable hex.
    function textColor(hex: string): string {
        const m = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec((hex ?? "").trim());
        if (!m) return "#1e1e2f";
        const r = parseInt(m[1], 16);
        const g = parseInt(m[2], 16);
        const b = parseInt(m[3], 16);
        const yiq = (r * 299 + g * 587 + b * 114) / 1000;
        return yiq >= 150 ? "#1e1e2f" : "#ffffff";
    }
</script>

<span class="label-chip" style="background:{color}; color:{textColor(color)};">
    {name}
</span>

<style>
    .label-chip {
        display: inline-flex;
        align-items: center;
        padding: 0.2rem 0.65rem;
        border-radius: 999px;
        font-size: 0.8rem;
        font-weight: 600;
        line-height: 1.4;
        max-width: 100%;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
    }
</style>
