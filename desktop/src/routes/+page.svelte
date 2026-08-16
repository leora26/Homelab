<script lang="ts">
    import { goto } from "$app/navigation";
    import { FolderPlus, Upload } from "@lucide/svelte";

    import Button from "$lib/components/ui/Button.svelte";
    import Card from "$lib/components/ui/Card.svelte";
    import ExtBadge from "$lib/components/ui/ExtBadge.svelte";
    import LabelDot from "$lib/components/ui/LabelDot.svelte";
    import MetricCard from "$lib/components/ui/MetricCard.svelte";
    import { session } from "$lib/stores/session.svelte";
    import { safeInvoke } from "$lib/utils/safeInvoke";
    import { formatBytes, formatCount, formatDate, pluralise } from "$lib/utils/format";
    import type { GlobalFileView, LabelView } from "$lib/types/models";

    const SHARED_PREVIEW_ROWS = 5;

    let shared = $state<GlobalFileView[]>([]);
    let labels = $state<LabelView[]>([]);

    $effect(() => {
        safeInvoke<GlobalFileView[]>("get_global_files").then((result) => {
            if (result.ok) shared = result.data;
        });
        safeInvoke<LabelView[]>("get_labels").then((result) => {
            if (result.ok) labels = result.data;
        });
    });

    const stats = $derived(session.stats);
    const profile = $derived(session.profile);
    const machine = $derived(session.machine);

    const freeBytes = $derived(
        profile ? Math.max(0, profile.allowed_storage - profile.taken_storage) : 0,
    );

    const sharedRows = $derived(shared.slice(0, SHARED_PREVIEW_ROWS));

    const labelledTotal = $derived(stats?.labelled_file_count ?? null);

    function startAction(action: "folder" | "upload") {
        goto(`/nas?action=${action}`);
    }
</script>

<div class="page">
    <header class="head">
        <div class="heading">
            <p class="page-meta">
                {#if machine}
                    {machine.hostname} · Pavuk {machine.app_version}
                {/if}
            </p>
            <h1 class="page-title">Dashboard</h1>
        </div>

        <div class="actions">
            <Button onclick={() => startAction("folder")}>
                <FolderPlus size={14} strokeWidth={1.8} />
                New folder
            </Button>
            <Button variant="primary" onclick={() => startAction("upload")}>
                <Upload size={14} strokeWidth={1.8} />
                Upload files
            </Button>
        </div>
    </header>

    <div class="metrics">
        <MetricCard
            label="Used"
            value={formatBytes(profile?.taken_storage ?? null)}
            caption={profile ? `of ${formatBytes(profile.allowed_storage)} quota` : "—"}
        />
        <MetricCard
            label="Free"
            value={formatBytes(freeBytes)}
            caption={profile ? `on ${machine?.hostname ?? "this machine"}` : "—"}
        />
        <MetricCard
            label="Files"
            value={formatCount(stats?.file_count)}
            caption={stats ? `in ${pluralise(stats.folder_count, "folder")}` : "—"}
        />
        <MetricCard
            label="Shared"
            value={formatCount(stats?.shared_file_count)}
            caption="visible to everyone"
            positive
        />
    </div>

    <div class="columns">
        <Card flush fill>
            {#snippet children()}
                <div class="card-head">
                    <h2 class="card-title">Shared with everyone</h2>
                    <a href="/global" class="card-link">Open Global Files</a>
                </div>

                <div class="table shared">
                    <div class="thead">
                        <span>Name</span>
                        <span>Owner</span>
                        <span>Modified</span>
                        <span class="right">Size</span>
                    </div>

                    <div class="tbody">
                        {#each sharedRows as row (row.id)}
                            <a class="trow" href="/global">
                                <span class="name">
                                    <ExtBadge name={row.file.name} />
                                    <span class="truncate">{row.file.name}</span>
                                </span>
                                <span class="owner truncate">{row.owner_name}</span>
                                <span class="mono muted">{formatDate(row.file.updated_at)}</span>
                                <span class="mono muted right">{formatBytes(row.file.size)}</span>
                            </a>
                        {/each}
                    </div>

                    <div class="tfoot">
                        Showing {sharedRows.length} of {formatCount(stats?.shared_file_count ?? shared.length)}
                        shared files
                    </div>
                </div>
            {/snippet}
        </Card>

        <Card flush fill>
            {#snippet children()}
                <div class="card-head">
                    <h2 class="card-title">Labels</h2>
                    <a href="/labels" class="card-link">New label</a>
                </div>

                <div class="label-rows">
                    {#each labels as label (label.id)}
                        <a class="label-row" href="/labels">
                            <LabelDot name={label.name} color={label.color} />
                            <span class="truncate">{label.name}</span>
                            <span class="mono count">{formatCount(label.file_count)}</span>
                        </a>
                    {/each}
                </div>

                <div class="tfoot split">
                    <span>
                        {pluralise(labels.length, "label")}{labelledTotal !== null
                            ? ` · ${formatCount(labelledTotal)} labelled files`
                            : ""}
                    </span>
                    <a href="/labels" class="card-link">Manage</a>
                </div>
            {/snippet}
        </Card>
    </div>
</div>

<style>
    .page {
        flex: 1;
        min-height: 0;
        padding: 26px 30px;
        display: flex;
        flex-direction: column;
        gap: 20px;
        overflow-y: auto;
    }

    .head {
        display: flex;
        align-items: flex-end;
        justify-content: space-between;
        gap: 20px;
        flex: none;
    }

    .heading {
        display: flex;
        flex-direction: column;
        gap: 5px;
    }

    .actions {
        display: flex;
        gap: 9px;
        flex: none;
    }

    .metrics {
        display: grid;
        grid-template-columns: repeat(4, 1fr);
        gap: 12px;
        flex: none;
    }

    .columns {
        flex: 1;
        min-height: 260px;
        display: grid;
        grid-template-columns: 1.55fr 1fr;
        gap: 14px;
    }

    /* Card headers here sit inside a flush card, so they carry their own padding. */
    .card-head {
        display: flex;
        align-items: center;
        justify-content: space-between;
        gap: 16px;
        padding: 13px 18px;
        border-bottom: 1px solid #202632;
        flex: none;
    }

    .card-title {
        font-size: var(--fs-card-title);
        font-weight: 600;
        color: var(--tx);
    }

    .card-link {
        font-size: var(--fs-sm);
        color: var(--link);
    }

    .table {
        display: flex;
        flex-direction: column;
        min-height: 0;
        flex: 1;
    }

    .thead,
    .trow {
        display: grid;
        grid-template-columns: minmax(0, 1fr) 88px 104px 84px;
        gap: 12px;
        align-items: center;
    }

    .thead {
        padding: 8px 18px;
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
        padding: 10px 18px;
        font-size: var(--fs-base);
        color: var(--tx);
        border-bottom: 1px solid var(--bd-row);
        transition: background var(--t-hover);
    }

    .trow:hover {
        background: var(--hover-row);
        color: var(--tx);
    }

    .name {
        display: flex;
        align-items: center;
        gap: 10px;
        min-width: 0;
    }

    .owner {
        color: var(--tx-mut);
    }

    .muted {
        color: var(--tx-mut);
        font-size: var(--fs-sm);
    }

    .right {
        text-align: right;
    }

    .tfoot {
        padding: 9px 18px;
        border-top: 1px solid var(--bd-row);
        font-size: var(--fs-caption);
        color: var(--tx-faint-2);
        flex: none;
    }

    .tfoot.split {
        display: flex;
        align-items: center;
        justify-content: space-between;
        gap: 12px;
        font-size: var(--fs-sm);
    }

    .label-rows {
        flex: 1;
        overflow-y: auto;
        min-height: 0;
    }

    .label-row {
        display: flex;
        align-items: center;
        gap: 14px;
        padding: 12px 18px;
        border-bottom: 1px solid var(--bd-row);
        font-size: var(--fs-base);
        color: var(--tx);
        transition: background var(--t-hover);
    }

    .label-row:hover {
        background: var(--hover-row);
        color: var(--tx);
    }

    .label-row .count {
        margin-left: auto;
        font-size: var(--fs-sm);
        color: var(--tx-faint);
    }
</style>
