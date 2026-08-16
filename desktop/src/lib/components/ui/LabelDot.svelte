<script lang="ts">
    import { paletteFor } from "$lib/utils/labels";

    interface Props {
        name: string;
        color: string;
    }

    const { name, color }: Props = $props();

    const palette = $derived(paletteFor(color));
</script>

<!--
  The 9px dot in the file table's Labels column. Carries the label name as a tooltip,
  since at this size the colour alone can't identify it.
-->
<span class="dot" style="background:{palette.text}" title={name} aria-label={name}></span>

<style>
    .dot {
        width: 9px;
        height: 9px;
        border-radius: var(--r-dot);
        flex: none;
    }
</style>
