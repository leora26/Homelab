<script lang="ts">
    import { X } from "@lucide/svelte";
    import Button from "$lib/components/ui/Button.svelte";
    import Chip from "$lib/components/ui/Chip.svelte";
    import LabelChip from "$lib/components/ui/LabelChip.svelte";
    import { safeInvoke } from "$lib/utils/safeInvoke";
    import { toasts } from "$lib/stores/toasts.svelte";
    import { formatBytes, formatDateTime } from "$lib/utils/format";
    import { extensionOf, isArchive, previewCaption } from "$lib/utils/files";
    import type { FileView, LabelView } from "$lib/types/models";

    interface Props {
        file: FileView;
        labels: LabelView[];
        /** Resolved path of the containing folder, e.g. "/photos/exports". */
        path: string;
        /** Shown on Global Files, where the file may belong to someone else. */
        ownerName?: string;
        /** Hides the file-management actions when the file isn't the viewer's. */
        canManage?: boolean;
        onclose: () => void;
        onrename?: () => void;
        onmove?: () => void;
        oncopy?: () => void;
        onlabels?: () => void;
        ontrash?: () => void;
        onarchive?: () => void;
        onsharechange?: (isGlobal: boolean) => void;
    }

    const {
        file,
        labels,
        path,
        ownerName,
        canManage = true,
        onclose,
        onrename,
        onmove,
        oncopy,
        onlabels,
        ontrash,
        onarchive,
        onsharechange,
    }: Props = $props();

    let previewSrc = $state<string | null>(null);
    let previewFailed = $state(false);
    let dimensions = $state<string | null>(null);

    let isGlobal = $state<boolean | null>(null);
    let togglingShare = $state(false);
    let downloading = $state(false);

    const archived = $derived(isArchive(file.name));

    $effect(() => {
        const id = file.id;
        previewSrc = null;
        previewFailed = false;
        dimensions = null;

        safeInvoke<string>("get_file_preview", { fileId: id }).then((result) => {
            // Ignore a response for a file the user has already navigated away from.
            if (file.id !== id) return;
            if (result.ok) previewSrc = result.data;
            else previewFailed = true;
        });
    });

    $effect(() => {
        const id = file.id;
        isGlobal = null;

        safeInvoke<boolean>("is_file_global", { fileId: id }).then((result) => {
            if (file.id === id && result.ok) isGlobal = result.data;
        });
    });

    /*
     * Image dimensions aren't stored anywhere, but the preview is already decoded in the
     * webview — so the loaded element can report them for free.
     */
    function onPreviewLoad(event: Event) {
        const image = event.currentTarget as HTMLImageElement;
        if (image.naturalWidth) dimensions = `${image.naturalWidth}×${image.naturalHeight}`;
    }

    const metaLine = $derived(
        [formatBytes(file.size), extensionOf(file.name).toUpperCase() || file.file_type, dimensions]
            .filter(Boolean)
            .join(" · "),
    );

    async function download() {
        downloading = true;
        const result = await safeInvoke<string>("download_file", {
            fileId: file.id,
            fileName: file.name,
        });
        downloading = false;

        if (result.ok) toasts.success("File downloaded", "Saved to your Downloads folder.");
        else toasts.error("Download failed", result.error);
    }

    async function toggleShare() {
        if (isGlobal === null) return;

        togglingShare = true;
        const makePrivate = isGlobal;

        const result = await safeInvoke(makePrivate ? "make_file_private" : "make_file_global", {
            fileId: file.id,
        });

        togglingShare = false;

        if (!result.ok) {
            toasts.error(makePrivate ? "Could not make private" : "Could not share", result.error);
            return;
        }

        isGlobal = !makePrivate;
        toasts.success(
            isGlobal ? "Shared with everyone" : "File is now private",
            isGlobal
                ? "Anyone on this machine can view and download it."
                : "It is no longer visible to other users.",
        );
        onsharechange?.(isGlobal);
    }
</script>

<aside class="rail">
    <header class="head">
        <span class="title">Details</span>
        <button class="close" onclick={onclose} aria-label="Close details">
            <X size={14} strokeWidth={2} />
        </button>
    </header>

    <div class="body">
        <div class="preview">
            {#if previewSrc}
                <img src={previewSrc} alt={file.name} onload={onPreviewLoad} />
            {:else}
                <span class="preview-caption">
                    {previewFailed ? "no preview available" : previewCaption(file.file_type)}
                </span>
            {/if}
        </div>

        <div class="identity">
            <p class="name selectable">{file.name}</p>
            <p class="meta mono">{metaLine}</p>
        </div>

        <div class="chips">
            {#if isGlobal === true}
                <Chip tone="shared">Shared with everyone</Chip>
            {:else if isGlobal === false}
                <Chip tone="private">Private</Chip>
            {/if}

            {#each labels as label (label.id)}
                <LabelChip name={label.name} color={label.color} />
            {/each}
        </div>

        <dl class="meta-list">
            {#if ownerName}
                <div class="meta-row">
                    <dt>Owner</dt>
                    <dd>{ownerName}</dd>
                </div>
            {/if}
            <div class="meta-row">
                <dt>Modified</dt>
                <dd class="mono">{formatDateTime(file.updated_at)}</dd>
            </div>
            <div class="meta-row">
                <dt>Path</dt>
                <dd class="mono path selectable" title={path}>{path}</dd>
            </div>
        </dl>

        <div class="actions">
            <Button variant="primary" size="lg" block onclick={download} disabled={downloading}>
                {downloading ? "Downloading…" : "Download"}
            </Button>

            <Button
                block
                onclick={toggleShare}
                disabled={isGlobal === null || togglingShare || !canManage}
            >
                {#if togglingShare}
                    Working…
                {:else if isGlobal}
                    Make private
                {:else}
                    Share with everyone
                {/if}
            </Button>

            {#if canManage}
                <div class="grid">
                    <Button size="sm" onclick={onlabels}>Labels</Button>
                    <Button size="sm" onclick={onrename}>Rename</Button>
                    <Button size="sm" onclick={onmove}>Move</Button>
                    <Button size="sm" onclick={oncopy}>Copy</Button>
                </div>

                <Button block onclick={onarchive}>
                    {archived ? "Extract" : "Archive"}
                </Button>

                <Button variant="destructive" block onclick={ontrash}>Move to trash</Button>
            {/if}
        </div>
    </div>
</aside>

<style>
    .rail {
        width: var(--w-rail);
        flex: none;
        border-left: 1px solid var(--bd-pane);
        background: var(--rail);
        display: flex;
        flex-direction: column;
        min-height: 0;
    }

    .head {
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 13px 16px;
        border-bottom: 1px solid var(--bd-pane);
        flex: none;
    }

    .title {
        font-size: var(--fs-sm);
        text-transform: uppercase;
        letter-spacing: var(--track-label);
        color: var(--tx-faint);
    }

    .close {
        display: flex;
        color: var(--tx-faint-2);
        border-radius: var(--r-badge);
    }

    .close:hover {
        color: var(--tx);
    }

    .body {
        flex: 1;
        overflow-y: auto;
        padding: 16px;
        display: flex;
        flex-direction: column;
        gap: 14px;
        min-height: 0;
    }

    .preview {
        /* Roughly a fifth of the window, with sane stops so it stays usable on a
           short screen and doesn't dominate a tall one. */
        height: 20vh;
        min-height: 150px;
        max-height: 320px;
        border-radius: var(--r-inset);
        background: linear-gradient(135deg, #1b2331, #131822);
        border: 1px solid var(--bd-card);
        display: flex;
        align-items: center;
        justify-content: center;
        overflow: hidden;
        flex: none;
    }

    .preview img {
        /* Fill the box rather than sitting small in the middle of it; `contain` keeps
           the aspect ratio so nothing is cropped. */
        width: 100%;
        height: 100%;
        object-fit: contain;
    }

    .preview-caption {
        font-size: var(--fs-caption);
        color: var(--tx-ghost);
    }

    .identity {
        display: flex;
        flex-direction: column;
        gap: 5px;
    }

    .name {
        font-size: var(--fs-card-title);
        font-weight: 600;
        line-height: 1.35;
        word-break: break-all;
        color: var(--tx);
    }

    .meta {
        font-size: var(--fs-caption);
        color: var(--tx-faint);
    }

    .chips {
        display: flex;
        flex-wrap: wrap;
        gap: 6px;
    }

    .meta-list {
        padding-top: 10px;
        border-top: 1px solid var(--bd-meta);
        display: flex;
        flex-direction: column;
        gap: 8px;
        margin: 0;
    }

    .meta-row {
        display: flex;
        align-items: baseline;
        justify-content: space-between;
        gap: 12px;
        font-size: var(--fs-sm);
    }

    dt {
        color: var(--tx-mut);
        flex: none;
    }

    dd {
        margin: 0;
        color: var(--tx);
        text-align: right;
        min-width: 0;
    }

    .path {
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
        font-size: var(--fs-caption);
    }

    .actions {
        display: flex;
        flex-direction: column;
        gap: 8px;
    }

    .grid {
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: 8px;
    }
</style>
