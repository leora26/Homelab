<script lang="ts">
    import LabelChip from "./LabelChip.svelte";
    import ColorWheel from "./ColorWheel.svelte";

    interface Props {
        isOpen: boolean;
        title: string;
        submitText?: string;
        loadingText?: string;
        initialName?: string;
        initialColor?: string;
        onClose: () => void;
        onSubmit: (name: string, color: string) => Promise<void>;
    }

    const PRESETS = [
        "#EF4444", "#F97316", "#EAB308", "#22C55E", "#14B8A6",
        "#3B82F6", "#6366F1", "#A855F7", "#EC4899", "#6B7280"
    ];

    let {
        isOpen,
        title,
        submitText = "Save",
        loadingText = "Saving...",
        initialName = "",
        initialColor = "#3B82F6",
        onClose,
        onSubmit
    }: Props = $props();

    let name = $state("");
    let color = $state("#3B82F6");
    let isSubmitting = $state(false);
    let error = $state<string | null>(null);

    // Reset the form to the provided initial values each time the modal opens, so a
    // create-form starts blank and an edit-form starts prefilled.
    $effect(() => {
        if (isOpen) {
            name = initialName;
            color = initialColor;
            error = null;
            isSubmitting = false;
        }
    });

    async function handleSubmit() {
        if (name.trim() === "") {
            error = "Name is required.";
            return;
        }

        error = null;
        isSubmitting = true;
        try {
            await onSubmit(name.trim(), color);
        } catch (e) {
            error = String(e);
        } finally {
            isSubmitting = false;
        }
    }
</script>

{#if isOpen}
    <div class="modal-backdrop" onclick={onClose}>
        <div class="modal-content" onclick={(e) => e.stopPropagation()}>
            <div class="modal-header">
                <h3>{title}</h3>
            </div>

            <div class="modal-body">
                {#if error}
                    <div class="error-banner">{error}</div>
                {/if}

                <div class="preview">
                    <LabelChip name={name.trim() || "Label preview"} color={color} />
                </div>

                <div class="form-group">
                    <label for="label-name">Name <span class="required">*</span></label>
                    <input
                            id="label-name"
                            type="text"
                            bind:value={name}
                            placeholder="e.g. Important"
                            disabled={isSubmitting}
                    />
                </div>

                <div class="form-group">
                    <span class="field-label">Color</span>

                    <ColorWheel color={color} onChange={(c) => (color = c)} />

                    <div class="swatches">
                        {#each PRESETS as preset}
                            <button
                                    type="button"
                                    class="swatch"
                                    class:selected={color.toLowerCase() === preset.toLowerCase()}
                                    style="background:{preset};"
                                    aria-label={preset}
                                    onclick={() => (color = preset)}
                                    disabled={isSubmitting}
                            ></button>
                        {/each}
                    </div>
                </div>
            </div>

            <div class="modal-actions">
                <button class="btn secondary" onclick={onClose} disabled={isSubmitting}>
                    Cancel
                </button>
                <button class="btn primary" onclick={handleSubmit} disabled={isSubmitting}>
                    {isSubmitting ? loadingText : submitText}
                </button>
            </div>
        </div>
    </div>
{/if}

<style>
    .modal-backdrop {
        position: fixed;
        top: 0;
        left: 0;
        width: 100vw;
        height: 100vh;
        background: rgba(0, 0, 0, 0.4);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 1000;
        backdrop-filter: blur(2px);
    }

    .modal-content {
        background: white;
        border-radius: 10px;
        width: 100%;
        max-width: 400px;
        box-shadow: 0 10px 25px rgba(0, 0, 0, 0.15);
        display: flex;
        flex-direction: column;
        overflow: hidden;
    }

    .modal-header {
        padding: 1.25rem 1.5rem;
        border-bottom: 1px solid #e1e4e8;
    }

    .modal-header h3 {
        margin: 0;
        font-size: 1.15rem;
        color: #1e1e2f;
    }

    .modal-body {
        padding: 1.5rem;
        display: flex;
        flex-direction: column;
        gap: 1rem;
    }

    .preview {
        display: flex;
        justify-content: center;
        padding: 0.5rem 0;
    }

    .form-group {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    .form-group label,
    .field-label {
        font-size: 0.9rem;
        font-weight: 500;
        color: #444;
    }

    .required {
        color: #c62828;
        margin-left: 0.2rem;
    }

    .form-group input[type="text"] {
        padding: 0.6rem 0.8rem;
        border: 1px solid #ccc;
        border-radius: 6px;
        font-size: 1rem;
        outline: none;
    }

    .form-group input[type="text"]:focus {
        border-color: #007bff;
        box-shadow: 0 0 0 3px rgba(0, 123, 255, 0.1);
    }

    .swatches {
        display: flex;
        flex-wrap: wrap;
        gap: 0.5rem;
        align-items: center;
    }

    .swatch {
        width: 26px;
        height: 26px;
        border-radius: 50%;
        border: 2px solid transparent;
        cursor: pointer;
        padding: 0;
        transition: transform 0.1s ease;
    }

    .swatch:hover {
        transform: scale(1.1);
    }

    .swatch.selected {
        border-color: #1e1e2f;
        box-shadow: 0 0 0 2px white inset;
    }

    .error-banner {
        background: #ffebee;
        color: #c62828;
        padding: 0.75rem;
        border-radius: 6px;
        font-size: 0.85rem;
    }

    .modal-actions {
        padding: 1.25rem 1.5rem;
        border-top: 1px solid #e1e4e8;
        background: #f8f9fa;
        display: flex;
        justify-content: flex-end;
        gap: 1rem;
    }

    .btn {
        padding: 0.5rem 1rem;
        border-radius: 6px;
        font-weight: 500;
        cursor: pointer;
        border: none;
        transition: opacity 0.2s;
    }

    .btn:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }

    .btn.primary {
        background: #007bff;
        color: white;
    }

    .btn.secondary {
        background: #f0f2f5;
        border: 1px solid #d1d5db;
    }
</style>
