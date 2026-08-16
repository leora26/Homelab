<script lang="ts">
    import { onDestroy } from "svelte";
    import { listen, type UnlistenFn } from "@tauri-apps/api/event";
    import { open as openFileDialog } from "@tauri-apps/plugin-dialog";
    import { UploadCloud } from "@lucide/svelte";

    import Button from "$lib/components/ui/Button.svelte";
    import Dialog from "$lib/components/ui/Dialog.svelte";
    import LabelChip from "$lib/components/ui/LabelChip.svelte";
    import ProgressBar from "$lib/components/ui/ProgressBar.svelte";
    import SelectField from "$lib/components/ui/SelectField.svelte";
    import { safeInvoke } from "$lib/utils/safeInvoke";
    import { formatBytes } from "$lib/utils/format";
    import type { FileView, LabelView, UploadProgress } from "$lib/types/models";

    interface Props {
        open: boolean;
        destinationId: string | null;
        destinationPath: string;
        allLabels: LabelView[];
        /** Bytes the user has left, so the dialog can state the real limit. */
        remainingQuota: number | null;
        oncomplete: () => void;
        onclose: () => void;
    }

    const {
        open,
        destinationId,
        destinationPath,
        allLabels,
        remainingQuota,
        oncomplete,
        onclose,
    }: Props = $props();

    type ItemStatus = "queued" | "uploading" | "done" | "error";

    interface Item {
        path: string;
        name: string;
        status: ItemStatus;
        sent: number;
        total: number;
        error?: string;
    }

    let items = $state<Item[]>([]);
    let bulkLabelId = $state("");
    let running = $state(false);
    let unlisten: UnlistenFn | undefined;

    /*
     * `upload_content` emits progress per file. The listener is attached for the whole
     * life of the dialog rather than per file, so events can't be missed in the gap
     * between starting a transfer and subscribing to it.
     */
    $effect(() => {
        if (!open) return;

        let active = true;

        listen<UploadProgress>("upload_progress", (event) => {
            const { file_id, bytes_sent, total_bytes } = event.payload;
            const index = uploadIds.get(file_id);
            if (index === undefined) return;

            items[index].sent = bytes_sent;
            items[index].total = total_bytes || items[index].total;
        }).then((fn) => {
            if (active) unlisten = fn;
            else fn();
        });

        return () => {
            active = false;
            unlisten?.();
            unlisten = undefined;
        };
    });

    onDestroy(() => unlisten?.());

    /** Maps the server-side file id back to its row while a transfer runs. */
    const uploadIds = new Map<string, number>();

    $effect(() => {
        if (open) {
            items = [];
            bulkLabelId = "";
            running = false;
            uploadIds.clear();
        }
    });

    const labelOptions = $derived([
        { value: "", label: "No label" },
        ...allLabels.map((label) => ({ value: label.id, label: label.name })),
    ]);

    const selectedLabel = $derived(allLabels.find((label) => label.id === bulkLabelId) ?? null);

    const completed = $derived(items.filter((item) => item.status === "done").length);
    const allSettled = $derived(
        items.length > 0 && items.every((item) => item.status === "done" || item.status === "error"),
    );

    async function browse() {
        const selection = await openFileDialog({ multiple: true, title: "Select files to upload" });
        if (!selection) return;

        const paths = Array.isArray(selection) ? selection : [selection];
        addPaths(paths);
    }

    function addPaths(paths: string[]) {
        const existing = new Set(items.map((item) => item.path));

        const added = paths
            .filter((path) => !existing.has(path))
            .map((path) => ({
                path,
                name: path.split(/[\\/]/).pop() || "file",
                status: "queued" as ItemStatus,
                sent: 0,
                total: 0,
            }));

        items = [...items, ...added];
    }

    async function start() {
        if (!destinationId || running) return;

        running = true;

        for (let index = 0; index < items.length; index++) {
            if (items[index].status !== "queued") continue;

            items[index].status = "uploading";

            const init = await safeInvoke<FileView>("init_file", {
                name: items[index].name,
                destination: destinationId,
                localPath: items[index].path,
            });

            if (!init.ok) {
                items[index].status = "error";
                items[index].error = init.error;
                continue;
            }

            const fileId = init.data.id;
            items[index].total = init.data.size;
            uploadIds.set(fileId, index);

            const upload = await safeInvoke("upload_content", {
                fileId,
                localPath: items[index].path,
            });

            if (!upload.ok) {
                items[index].status = "error";
                items[index].error = upload.error;
                continue;
            }

            items[index].status = "done";
            items[index].sent = items[index].total;

            if (bulkLabelId) {
                await safeInvoke("create_fl", { fileId, labelId: bulkLabelId });
            }
        }

        running = false;
        oncomplete();
    }

    function percentOfItem(item: Item): number {
        if (item.status === "done") return 100;
        if (!item.total) return 0;
        return (item.sent / item.total) * 100;
    }

    function toneOf(item: Item): "accent" | "success" | "danger" {
        if (item.status === "done") return "success";
        if (item.status === "error") return "danger";
        return "accent";
    }
</script>

<Dialog {open} title="Upload files" subtitle={destinationPath} {onclose} width={520}>
    {#snippet children()}
        <!--
          Tauri delivers OS drag-and-drop as a window event rather than DOM drag events,
          so the zone is a click target here and the window-level handler in the parent
          screen feeds `addPaths`.
        -->
        <button class="dropzone" onclick={browse} disabled={running}>
            <UploadCloud size={22} strokeWidth={1.8} />
            <span class="drop-title">Drop files here</span>
            <span class="drop-sub">
                or <span class="link">browse your computer</span>
                {#if remainingQuota !== null}
                    · {formatBytes(remainingQuota)} of quota left
                {/if}
            </span>
        </button>

        {#if items.length > 0}
            <div class="items">
                {#each items as item (item.path)}
                    <div class="item">
                        <div class="item-head">
                            <span class="item-name truncate" class:failed={item.status === "error"}>
                                {item.name}
                            </span>
                            <span class="item-status mono" class:failed={item.status === "error"}>
                                {#if item.status === "error"}
                                    Failed
                                {:else if item.status === "done"}
                                    {formatBytes(item.total)}
                                {:else if item.status === "uploading"}
                                    {formatBytes(item.sent)} / {formatBytes(item.total)}
                                {:else}
                                    Queued
                                {/if}
                            </span>
                        </div>
                        <ProgressBar percent={percentOfItem(item)} tone={toneOf(item)} />
                        {#if item.error}
                            <p class="item-error">{item.error}</p>
                        {/if}
                    </div>
                {/each}
            </div>

            <div class="bulk">
                <span>Label every file as</span>
                <SelectField bind:value={bulkLabelId} options={labelOptions} width={160} />
                {#if selectedLabel}
                    <LabelChip name={selectedLabel.name} color={selectedLabel.color} />
                {/if}
            </div>
        {/if}
    {/snippet}

    {#snippet footer()}
        {#if items.length > 0}
            <span class="summary mono">{completed} of {items.length} complete</span>
        {/if}
        <Button onclick={onclose} disabled={running}>{allSettled ? "Done" : "Cancel"}</Button>
        {#if !allSettled}
            <Button
                variant="primary"
                onclick={start}
                disabled={running || items.length === 0 || !destinationId}
            >
                {running ? "Uploading…" : "Upload"}
            </Button>
        {/if}
    {/snippet}
</Dialog>

<style>
    .dropzone {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 7px;
        padding: 26px;
        border: 1px dashed var(--accent);
        border-radius: var(--r-card);
        background: #101828;
        color: var(--link);
        cursor: pointer;
        transition: background var(--t-hover);
    }

    .dropzone:hover:not(:disabled) {
        background: #132038;
    }

    .dropzone:disabled {
        opacity: 0.6;
        cursor: not-allowed;
    }

    .drop-title {
        font-size: var(--fs-nav);
        font-weight: 500;
        color: var(--tx);
    }

    .drop-sub {
        font-size: var(--fs-caption);
        color: var(--tx-faint);
    }

    .link {
        color: var(--link);
        font-weight: 500;
    }

    .items {
        display: flex;
        flex-direction: column;
        gap: 10px;
        max-height: 220px;
        overflow-y: auto;
    }

    .item {
        display: flex;
        flex-direction: column;
        gap: 5px;
    }

    .item-head {
        display: flex;
        align-items: baseline;
        justify-content: space-between;
        gap: 12px;
    }

    .item-name {
        font-size: var(--fs-btn);
        color: var(--tx-2);
        min-width: 0;
    }

    .item-status {
        font-size: var(--fs-caption);
        color: var(--tx-faint);
        flex: none;
    }

    .item-name.failed,
    .item-status.failed {
        color: var(--danger);
    }

    .item-error {
        font-size: var(--fs-caption);
        color: var(--danger);
        overflow-wrap: anywhere;
    }

    .bulk {
        display: flex;
        align-items: center;
        gap: 10px;
        padding: 10px 12px;
        border-radius: var(--r-inset);
        background: var(--sunken-alt);
        border: 1px solid var(--bd-dialog);
        font-size: var(--fs-btn);
        color: var(--tx-mut);
    }

    .summary {
        margin-right: auto;
        font-size: var(--fs-caption);
        color: var(--tx-faint-2);
    }
</style>
