<script lang="ts">
    export interface FormField {
        name: string;
        label: string;
        type: 'text' | 'number';
        placeholder?: string;
        required?: boolean;
        defaultValue?: string | number;
    }

    interface Props {
        isOpen: boolean;
        title: string;
        description?: string;
        fields: FormField[];
        submitText?: string;
        loadingText?: string;
        onClose: () => void;
        onSubmit: (data: Record<string, string | number>) => Promise<void>;
    }

    let {
        isOpen,
        title,
        description,
        fields,
        submitText = "Submit",
        loadingText = "Submitting...",
        onClose,
        onSubmit
    }: Props = $props();

    let formData = $state<Record<string, string | number>>({});
    let isSubmitting = $state(false);
    let error = $state<string | null>(null);

    $effect(() => {
        if (isOpen) {
            error = null;
            isSubmitting = false;
            let initialData: Record<string, string | number> = {};
            for (const field of fields) {
                initialData[field.name] = field.defaultValue ?? (field.type === 'number' ? 0 : '');
            }
            formData = initialData;
        }
    });

    async function handleSubmit() {
        error = null;

        for (const field of fields) {
            if (field.required) {
                const val = formData[field.name];
                if (val === undefined || val === null || String(val).trim() === '') {
                    error = `${field.label} is required.`;
                    return;
                }
            }
        }

        isSubmitting = true;
        try {
            await onSubmit(formData);
        } catch (err) {
            error = String(err);
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

                {#if description}
                    <p class="modal-description">{description}</p>
                {/if}

                {#each fields as field}
                    <div class="form-group">
                        <label for={field.name}>
                            {field.label}
                            {#if field.required}<span class="required">*</span>{/if}
                        </label>
                        <input
                                type={field.type}
                                id={field.name}
                                bind:value={formData[field.name]}
                                placeholder={field.placeholder}
                                disabled={isSubmitting}
                        />
                    </div>
                {/each}
            </div>

            <div class="modal-actions">
                <button class="btn secondary" onclick={onClose} disabled={isSubmitting}>
                    Cancel
                </button>
                <button class="btn primary" onclick={handleSubmit} disabled={isSubmitting}>
                    {#if isSubmitting}
                        {loadingText}
                    {:else}
                        {submitText}
                    {/if}
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

    .form-group {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    .form-group label {
        font-size: 0.9rem;
        font-weight: 500;
        color: #444;
    }

    .required {
        color: #c62828;
        margin-left: 0.2rem;
    }

    .form-group input {
        padding: 0.6rem 0.8rem;
        border: 1px solid #ccc;
        border-radius: 6px;
        font-size: 1rem;
        outline: none;
    }

    .form-group input:focus {
        border-color: #007bff;
        box-shadow: 0 0 0 3px rgba(0, 123, 255, 0.1);
    }

    .error-banner {
        background: #ffebee;
        color: #c62828;
        padding: 0.75rem;
        border-radius: 6px;
        font-size: 0.85rem;
        margin-bottom: 0.5rem;
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