<script lang="ts">
    import { badgeKind, badgeLabel } from "$lib/utils/files";

    interface Props {
        /** Filename; the extension and colour are derived from it. */
        name?: string;
        /** Set for folder rows — renders the DIR badge regardless of name. */
        folder?: boolean;
        /** 22px in tables, 24px on Global Files. */
        size?: number;
    }

    const { name = "", folder = false, size = 22 }: Props = $props();

    const kind = $derived(folder ? "dir" : badgeKind(name));
    const text = $derived(folder ? "DIR" : badgeLabel(name));
</script>

<span class="badge {kind}" style="width:{size}px;height:{size}px" aria-hidden="true">
    {text}
</span>

<style>
    .badge {
        display: inline-flex;
        align-items: center;
        justify-content: center;
        flex: none;
        border-radius: var(--r-badge);
        font-family: var(--font-mono);
        font-size: 9px;
        font-weight: 500;
        letter-spacing: 0.02em;
    }

    .img { background: var(--ext-img-bg); color: var(--ext-img-tx); }
    .jpg { background: var(--ext-jpg-bg); color: var(--ext-jpg-tx); }
    .vid { background: var(--ext-vid-bg); color: var(--ext-vid-tx); }
    .zip { background: var(--ext-zip-bg); color: var(--ext-zip-tx); }
    .doc { background: var(--ext-doc-bg); color: var(--ext-doc-tx); }
    .dir { background: var(--ext-dir-bg); color: var(--ext-dir-tx); }
</style>
