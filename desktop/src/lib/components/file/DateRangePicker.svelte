<script lang="ts">
    import { onMount } from "svelte";
    import "flatpickr/dist/flatpickr.css";

    interface Props {
        // Emits the selected range; either side is null when not set / cleared.
        onChange: (from: Date | null, to: Date | null) => void;
        placeholder?: string;
    }

    let { onChange, placeholder = "Any date" }: Props = $props();

    let inputEl: HTMLInputElement;
    let fp: any = null;
    let hasValue = $state(false);

    onMount(() => {
        let destroyed = false;

        // Loaded lazily so the browser-only flatpickr is never evaluated during prerender.
        import("flatpickr").then(({ default: flatpickr }) => {
            if (destroyed) return;

            fp = flatpickr(inputEl, {
                mode: "range",
                dateFormat: "M j, Y",
                onChange: (dates: Date[]) => {
                    hasValue = dates.length > 0;
                    onChange(dates[0] ?? null, dates[1] ?? null);
                },
            });
        });

        return () => {
            destroyed = true;
            if (fp) {
                fp.destroy();
                fp = null;
            }
        };
    });

    // Exposed on the instance so the parent's "Clear filters" can reset the calendar.
    export function clear() {
        fp?.clear();
        hasValue = false;
    }

    function clearRange(e: MouseEvent) {
        e.stopPropagation();
        clear();
        onChange(null, null);
    }
</script>

<div class="date-field">
    <span class="cal-icon" aria-hidden="true">📅</span>
    <input bind:this={inputEl} type="text" placeholder={placeholder} readonly />
    {#if hasValue}
        <button class="clear-btn" title="Clear dates" onclick={clearRange}>✕</button>
    {/if}
</div>

<style>
    .date-field {
        display: flex;
        align-items: center;
        gap: 0.4rem;
        border: 1px solid #ccc;
        border-radius: 6px;
        padding: 0 0.6rem;
        background: white;
        min-width: 210px;
    }

    .date-field:focus-within {
        border-color: #007bff;
        box-shadow: 0 0 0 3px rgba(0, 123, 255, 0.1);
    }

    .cal-icon {
        font-size: 0.9rem;
    }

    .date-field input {
        border: none;
        outline: none;
        padding: 0.55rem 0;
        font-size: 0.9rem;
        background: transparent;
        flex: 1;
        cursor: pointer;
    }

    .clear-btn {
        border: none;
        background: none;
        color: #999;
        cursor: pointer;
        font-size: 0.85rem;
        padding: 0.2rem;
        line-height: 1;
    }

    .clear-btn:hover {
        color: #333;
    }
</style>
