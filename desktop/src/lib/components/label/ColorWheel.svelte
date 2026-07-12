<script lang="ts">
    import { onMount } from "svelte";

    interface Props {
        color: string;
        onChange: (hex: string) => void;
        size?: number;
    }

    let { color, onChange, size = 180 }: Props = $props();

    let el: HTMLDivElement;
    let picker = $state<any>(null);
    // The exact hex the picker last produced; lets the sync effect ignore our own
    // updates and only push genuinely external changes (preset clicks, modal reopen).
    let lastEmitted = "";

    onMount(() => {
        let destroyed = false;

        // Loaded lazily so the browser-only iro.js is never evaluated during SSR/prerender.
        import("@jaames/iro").then((mod) => {
            if (destroyed) return;

            // iro's bundled types don't expose a construct signature, so treat as `any`.
            const iro: any = mod.default;

            const instance = new iro.ColorPicker(el, {
                width: size,
                color: color || "#3B82F6",
                layout: [
                    { component: iro.ui.Wheel },
                    { component: iro.ui.Slider, options: { sliderType: "value" } },
                ],
            });

            instance.on("color:change", (c: any) => {
                const hex = c.hexString;
                if (hex.toLowerCase() === lastEmitted.toLowerCase()) return;
                lastEmitted = hex;
                onChange(hex);
            });

            picker = instance;
        });

        return () => {
            destroyed = true;
            picker = null;
        };
    });

    // Reflect external color changes into the wheel. Setting lastEmitted first makes the
    // resulting color:change a no-op, so this never loops back through onChange.
    $effect(() => {
        const incoming = color;
        if (picker && incoming && incoming.toLowerCase() !== lastEmitted.toLowerCase()) {
            lastEmitted = incoming;
            picker.color.set(incoming);
        }
    });
</script>

<div class="wheel-host" bind:this={el}></div>

<style>
    .wheel-host {
        display: flex;
        justify-content: center;
    }
</style>
