<script lang="ts">
    import Chip from "$lib/components/ui/Chip.svelte";
    import { paletteFor } from "$lib/utils/labels";

    interface Props {
        name: string;
        /** Raw colour from the database; mapped to the nearest preset for display. */
        color: string;
        interactive?: boolean;
        selected?: boolean;
        onclick?: () => void;
    }

    const { name, color, interactive = false, selected = false, onclick }: Props = $props();

    const palette = $derived(paletteFor(color));
</script>

<Chip
    tone="custom"
    bg={palette.bg}
    color={palette.text}
    border={palette.border}
    {interactive}
    {selected}
    {onclick}
    title={name}
>
    {name}
</Chip>
