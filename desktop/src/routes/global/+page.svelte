<script lang="ts">
    import { Globe } from "@lucide/svelte";

    import EmptyState from "$lib/components/ui/EmptyState.svelte";
    import ExtBadge from "$lib/components/ui/ExtBadge.svelte";
    import SelectField from "$lib/components/ui/SelectField.svelte";
    import TextField from "$lib/components/ui/TextField.svelte";
    import DetailsRail from "$lib/components/file/DetailsRail.svelte";

    import { safeInvoke } from "$lib/utils/safeInvoke";
    import { session } from "$lib/stores/session.svelte";
    import { ancestorsOf } from "$lib/utils/folderPath.svelte";
    import { filePathString } from "$lib/utils/paths";
    import { formatBytes, formatDate, pluralise } from "$lib/utils/format";
    import type { GlobalFileView } from "$lib/types/models";

    let items = $state<GlobalFileView[]>([]);
    let loading = $state(true);
    let error = $state<string | null>(null);

    let selected = $state<GlobalFileView | null>(null);
    let selectedPath = $state("");
    let nameFilter = $state("");
    let ownerFilter = $state("");

    async function load() {
        loading = true;
        error = null;

        const result = await safeInvoke<GlobalFileView[]>("get_global_files");

        if (result.ok) {
            items = result.data;
            if (selected && !items.some((item) => item.id === selected!.id)) selected = null;
        } else {
            error = result.error;
        }

        loading = false;
    }

    $effect(() => {
        load();
    });

    $effect(() => {
        const current = selected;
        selectedPath = "";
        if (!current || current.file.owner_id !== session.user?.id) return;

        ancestorsOf(current.file.parent_folder_id).then((segments) => {
            if (selected?.id === current.id) {
                selectedPath = filePathString(segments, current.file.name);
            }
        });
    });

    const owners = $derived([
        { value: "", label: "All owners" },
        ...[...new Set(items.map((item) => item.owner_name))]
            .sort()
            .map((name) => ({ value: name, label: name })),
    ]);

    const visible = $derived.by(() => {
        const needle = nameFilter.trim().toLowerCase();
        return items.filter((item) => {
            if (needle && !item.file.name.toLowerCase().includes(needle)) return false;
            if (ownerFilter && item.owner_name !== ownerFilter) return false;
            return true;
        });
    });

    const totalBytes = $derived(items.reduce((sum, item) => sum + item.file.size, 0));

    const canManageSelected = $derived(
        selected !== null && selected.file.owner_id === session.user?.id,
    );
</script>

<div class="page">
    <header class="head">
        <div class="heading">
            <h1 class="page-title">Global Files</h1>
            <p class="page-subtitle">
                Shared with every user on this machine. Anyone can view and download; only the
                owner can un-share.
            </p>
        </div>

        <p class="mono totals">
            {pluralise(items.length, "file")} · {formatBytes(totalBytes)}
        </p>
    </header>

    <div class="filters">
        <div class="filter-field">
            <TextField bind:value={nameFilter} placeholder="Filter by name" />
        </div>
        <SelectField bind:value={ownerFilter} options={owners} width={160} />
    </div>

    <div class="panes">
        <section class="table-pane">
            {#if loading}
                <div class="skeletons">
                    {#each Array(5) as _, index (index)}
                        <div class="skeleton"></div>
                    {/each}
                </div>
            {:else if error}
                <EmptyState icon={Globe} title="Couldn't load shared files" body={error} />
            {:else if items.length === 0}
                <EmptyState
                    icon={Globe}
                    title="Nothing shared yet"
                    body="Files shared with everyone on this machine will appear here."
                />
            {:else if visible.length === 0}
                <EmptyState
                    icon={Globe}
                    title="No matching files"
                    body="Try a different name, or switch back to all owners."
                />
            {:else}
                <div class="thead">
                    <span>Name</span>
                    <span>Shared by</span>
                    <span>Shared since</span>
                    <span class="right">Size</span>
                </div>

                <div class="tbody">
                    {#each visible as item (item.id)}
                        <button
                            class="trow"
                            class:selected={selected?.id === item.id}
                            onclick={() => (selected = item)}
                        >
                            <span class="cell-name">
                                <ExtBadge name={item.file.name} size={24} />
                                <span class="truncate">{item.file.name}</span>
                            </span>
                            <span class="muted truncate">{item.owner_name}</span>
                            <span class="mono muted">{formatDate(item.shared_at)}</span>
                            <span class="mono muted right">{formatBytes(item.file.size)}</span>
                        </button>
                    {/each}
                </div>

                <div class="tfoot">
                    Showing {visible.length} of {items.length}
                    {#if selected}· 1 selected{/if}
                </div>
            {/if}
        </section>

        {#if selected}
            <DetailsRail
                file={selected.file}
                labels={selected.file.labels}
                path={selectedPath || "—"}
                ownerName={selected.owner_name}
                canManage={canManageSelected}
                onclose={() => (selected = null)}
                onsharechange={load}
            />
        {/if}
    </div>
</div>

<style>
    .page {
        flex: 1;
        min-height: 0;
        display: flex;
        flex-direction: column;
        overflow: hidden;
    }

    .head {
        display: flex;
        align-items: flex-end;
        justify-content: space-between;
        gap: 20px;
        padding: 24px 28px 16px;
        flex: none;
    }

    .heading {
        display: flex;
        flex-direction: column;
        gap: 6px;
        max-width: 60ch;
    }

    .totals {
        font-size: var(--fs-sm);
        color: var(--tx-faint-2);
        flex: none;
    }

    .filters {
        display: flex;
        align-items: center;
        gap: 9px;
        padding: 0 28px 14px;
        border-bottom: 1px solid var(--bd-pane);
        flex: none;
    }

    .filter-field {
        flex: 1;
        max-width: 420px;
    }

    .panes {
        flex: 1;
        display: flex;
        min-height: 0;
    }

    .table-pane {
        flex: 1;
        min-width: 0;
        display: flex;
        flex-direction: column;
    }

    .thead,
    .trow {
        display: grid;
        grid-template-columns: minmax(240px, 1fr) 104px 104px 84px;
        gap: 12px;
        align-items: center;
    }

    .thead {
        padding: 9px 22px;
        border-bottom: 1px solid var(--bd-pane);
        font-size: var(--fs-label);
        text-transform: uppercase;
        letter-spacing: var(--track-th);
        color: var(--tx-faint-2);
        flex: none;
    }

    .tbody {
        flex: 1;
        overflow-y: auto;
        min-height: 0;
    }

    .trow {
        width: 100%;
        padding: 11px 22px;
        font-size: var(--fs-base);
        color: var(--tx);
        border-bottom: 1px solid var(--bd-row-soft);
        border-left: 2px solid transparent;
        text-align: left;
        transition: background var(--t-hover);
    }

    .trow:hover {
        background: var(--hover-row-light);
    }

    .trow.selected {
        background: var(--row-selected);
        border-left-color: var(--accent);
    }

    .cell-name {
        display: flex;
        align-items: center;
        gap: 10px;
        min-width: 0;
    }

    .muted {
        color: var(--tx-mut);
        font-size: var(--fs-sm);
    }

    .right {
        text-align: right;
    }

    .tfoot {
        padding: 9px 22px;
        border-top: 1px solid var(--bd-pane);
        font-size: var(--fs-caption);
        color: var(--tx-faint-2);
        flex: none;
    }

    .skeletons {
        padding: 14px 22px;
        display: flex;
        flex-direction: column;
        gap: 14px;
    }

    .skeleton {
        height: 20px;
        border-radius: var(--r-badge);
        background: var(--bd-row);
        animation: shimmer 1.3s ease-in-out infinite;
    }

    @keyframes shimmer {
        0%, 100% { opacity: 1; }
        50% { opacity: 0.5; }
    }
</style>
