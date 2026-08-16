<script lang="ts">
    import Button from "$lib/components/ui/Button.svelte";
    import Card from "$lib/components/ui/Card.svelte";
    import Chip from "$lib/components/ui/Chip.svelte";
    import ProgressBar from "$lib/components/ui/ProgressBar.svelte";
    import ConfirmDialog from "$lib/components/dialogs/ConfirmDialog.svelte";
    import PromptDialog from "$lib/components/dialogs/PromptDialog.svelte";
    import { Pencil } from "@lucide/svelte";
    import type { UserProfileView } from "$lib/types/models";

    import { safeInvoke } from "$lib/utils/safeInvoke";
    import { toasts } from "$lib/stores/toasts.svelte";
    import { session } from "$lib/stores/session.svelte";
    import { formatBytes, formatCount, formatDate, formatUptime, percentOf, splitBytes } from "$lib/utils/format";

    let showEmptyConfirm = $state(false);
    let showRename = $state(false);
    let busy = $state(false);

    $effect(() => {
        session.loadUserCount();
    });

    const user = $derived(session.user);
    const profile = $derived(session.profile);
    const machine = $derived(session.machine);
    const stats = $derived(session.stats);

    const percent = $derived(
        profile ? percentOf(profile.taken_storage, profile.allowed_storage) : 0,
    );

    const used = $derived(splitBytes(profile?.taken_storage ?? 0));

    const initial = $derived((user?.name || user?.email || "?").charAt(0).toUpperCase());

    /** Files the user holds that aren't in the trash and aren't shared. */
    const privateBytes = $derived(
        profile && stats
            ? Math.max(0, profile.taken_storage - stats.shared_bytes - stats.trashed_bytes)
            : null,
    );

    async function renameUser(name: string) {
        const result = await safeInvoke<UserProfileView>("update_user_name", { fullName: name });
        if (!result.ok) throw new Error(result.error);

        session.user = result.data;
        showRename = false;
        toasts.success("Name updated", name);
    }

    async function emptyTrash() {
        busy = true;
        const result = await safeInvoke("cleanup_trash");

        if (!result.ok) {
            busy = false;
            toasts.error("Could not empty trash", result.error);
            return;
        }

        showEmptyConfirm = false;

        // The server only queues the work, so poll until the trash actually clears
        // rather than refetching once and showing an unchanged figure.
        const cleared = await session.awaitTrashCleared();
        busy = false;

        if (cleared) toasts.success("Trash emptied");
        else toasts.info("Emptying the trash", "This is still running in the background.");
    }

    async function logout() {
        const result = await safeInvoke("logout");
        if (!result.ok) toasts.error("Log out failed", result.error);
    }
</script>

<div class="page">
    <header class="heading">
        <p class="page-meta">
            {#if machine}Signed in on {machine.hostname}{/if}
        </p>
        <h1 class="page-title">Profile</h1>
    </header>

    <div class="columns">
        <Card>
            {#snippet children()}
                <div class="account">
                    <div class="avatar">{initial}</div>
                    <div class="identity">
                        <button
                            class="name"
                            onclick={() => (showRename = true)}
                            disabled={!user}
                            title="Change your name"
                        >
                            <span>{user?.name ?? "—"}</span>
                            <Pencil size={13} strokeWidth={1.8} />
                        </button>
                        <p class="email mono selectable">{user?.email ?? "—"}</p>
                    </div>
                    {#if profile}
                        <Chip tone={profile.is_blocked ? "danger" : "success"}>
                            {profile.is_blocked ? "Blocked" : "Active"}
                        </Chip>
                    {/if}
                </div>

                <dl class="meta-list">
                    <div class="meta-row">
                        <dt>Role</dt>
                        <dd>{user?.role ?? "—"}</dd>
                    </div>
                    <div class="meta-row">
                        <dt>Member since</dt>
                        <dd class="mono">{formatDate(user?.created_at)}</dd>
                    </div>
                    <div class="meta-row">
                        <dt>Account ID</dt>
                        <dd class="mono id selectable">{user?.id ?? "—"}</dd>
                    </div>
                </dl>

                <Button variant="destructive" block onclick={logout}>Log out</Button>
            {/snippet}
        </Card>

        <Card>
            {#snippet children()}
                <div class="quota-head">
                    <h2 class="card-title">Storage quota</h2>
                    <span class="mono muted">{Math.round(percent)}% used</span>
                </div>

                <div class="quota-figure">
                    <span class="quota-number mono">{used.value}</span>
                    <span class="quota-unit">
                        {used.unit} of {formatBytes(profile?.allowed_storage ?? 0)}
                    </span>
                </div>

                <ProgressBar {percent} height={8} label="Storage used" />

                <dl class="meta-list">
                    <div class="meta-row">
                        <dt>Your files</dt>
                        <dd class="mono">{formatBytes(privateBytes)}</dd>
                    </div>
                    <div class="meta-row">
                        <dt>Shared with everyone</dt>
                        <dd class="mono">{formatBytes(stats?.shared_bytes ?? null)}</dd>
                    </div>
                    <div class="meta-row">
                        <dt>In trash</dt>
                        <dd class="mono">{formatBytes(stats?.trashed_bytes ?? null)}</dd>
                    </div>
                </dl>

                <Button
                    block
                    onclick={() => (showEmptyConfirm = true)}
                    disabled={!stats || stats.trashed_item_count === 0}
                >
                    {#if stats && stats.trashed_bytes > 0}
                        Empty trash to reclaim {formatBytes(stats.trashed_bytes)}
                    {:else}
                        Trash is empty
                    {/if}
                </Button>
            {/snippet}
        </Card>
    </div>

    <Card title="This machine">
        {#snippet children()}
            <div class="machine">
                <div class="fact">
                    <span class="fact-label">Host</span>
                    <span class="mono fact-value">{machine?.hostname ?? "—"}</span>
                </div>
                <div class="fact">
                    <span class="fact-label">Address</span>
                    <span class="mono fact-value">{machine?.address ?? "—"}</span>
                </div>
                <div class="fact">
                    <span class="fact-label">Users</span>
                    <span class="mono fact-value">{formatCount(session.userCount)}</span>
                </div>
                <div class="fact">
                    <span class="fact-label">Pavuk version</span>
                    <span class="mono fact-value">
                        {machine?.app_version ?? "—"}
                        {#if machine}<span class="uptime">· {formatUptime(machine.uptime_seconds)}</span>{/if}
                    </span>
                </div>
            </div>
        {/snippet}
    </Card>
</div>

<PromptDialog
    open={showRename}
    title="Change your name"
    subtitle={user?.email}
    fieldLabel="Display name"
    hint="This is how you appear on files you share with everyone."
    initialValue={user?.name ?? ""}
    confirmLabel="Save"
    onsubmit={renameUser}
    onclose={() => (showRename = false)}
/>

<ConfirmDialog
    open={showEmptyConfirm}
    title="Empty trash"
    body={`All ${stats?.trashed_item_count ?? 0} items will be deleted permanently${
        stats ? ` and ${formatBytes(stats.trashed_bytes)} of disk space will be reclaimed` : ""
    }. This cannot be undone.`}
    confirmLabel="Empty trash"
    {busy}
    onconfirm={emptyTrash}
    onclose={() => (showEmptyConfirm = false)}
/>

<style>
    .page {
        flex: 1;
        min-height: 0;
        overflow-y: auto;
        padding: 26px 30px;
        display: flex;
        flex-direction: column;
        gap: 18px;
    }

    .heading {
        display: flex;
        flex-direction: column;
        gap: 5px;
        flex: none;
    }

    .columns {
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: 14px;
    }

    .account {
        display: flex;
        align-items: center;
        gap: 14px;
    }

    .avatar {
        width: 52px;
        height: 52px;
        border-radius: 50%;
        background: var(--accent);
        color: #ffffff;
        font-size: 20px;
        font-weight: 600;
        display: flex;
        align-items: center;
        justify-content: center;
        flex: none;
    }

    .identity {
        flex: 1;
        min-width: 0;
        display: flex;
        flex-direction: column;
        gap: 3px;
    }

    .name {
        display: flex;
        align-items: center;
        gap: 8px;
        font-size: var(--fs-name);
        font-weight: 600;
        color: var(--tx);
        text-align: left;
    }

    .name :global(svg) {
        color: var(--tx-ghost);
        opacity: 0;
        transition: opacity var(--t-hover);
    }

    .name:hover :global(svg),
    .name:focus-visible :global(svg) {
        opacity: 1;
    }

    .name:disabled {
        cursor: default;
    }

    .email {
        font-size: var(--fs-btn);
        color: var(--tx-mut-2);
        overflow-wrap: anywhere;
    }

    .meta-list {
        margin: 0;
        padding-top: 14px;
        border-top: 1px solid var(--bd-meta);
        display: flex;
        flex-direction: column;
        gap: 9px;
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
        overflow-wrap: anywhere;
    }

    .id {
        font-size: var(--fs-label);
        color: var(--tx-faint);
    }

    .quota-head {
        display: flex;
        align-items: center;
        justify-content: space-between;
        gap: 12px;
    }

    .card-title {
        font-size: var(--fs-card-title);
        font-weight: 600;
        color: var(--tx);
    }

    .muted {
        font-size: var(--fs-sm);
        color: var(--tx-faint);
    }

    .quota-figure {
        display: flex;
        align-items: baseline;
        gap: 8px;
    }

    .quota-number {
        font-size: var(--fs-quota);
        font-weight: 600;
        color: var(--tx);
        line-height: 1;
    }

    .quota-unit {
        font-size: var(--fs-card-title);
        color: var(--tx-mut-2);
    }

    .machine {
        display: grid;
        grid-template-columns: repeat(4, 1fr);
        gap: 14px;
    }

    .fact {
        display: flex;
        flex-direction: column;
        gap: 4px;
        min-width: 0;
    }

    .fact-label {
        font-size: var(--fs-btn);
        color: var(--tx-mut);
    }

    .fact-value {
        font-size: var(--fs-base);
        color: var(--tx);
        overflow-wrap: anywhere;
    }

    .uptime {
        color: var(--tx-faint-2);
    }
</style>
