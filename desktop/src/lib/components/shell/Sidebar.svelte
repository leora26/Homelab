<script lang="ts">
    import { page } from "$app/state";
    import { Folder, Globe, LayoutGrid, LogOut, Tag, Trash2, User } from "@lucide/svelte";
    import DiskWidget from "$lib/components/shell/DiskWidget.svelte";
    import { toasts } from "$lib/stores/toasts.svelte";
    import { safeInvoke } from "$lib/utils/safeInvoke";
    import type { StorageProfileView } from "$lib/types/models";

    interface Props {
        profile: StorageProfileView | null;
    }

    const { profile }: Props = $props();

    /*
     * Grouped so the sidebar can grow — the handoff leaves room under Storage for future
     * services without restructuring. Settings is absent by design; it lands later.
     */
    const GROUPS = [
        {
            heading: "Overview",
            items: [{ href: "/", label: "Dashboard", icon: LayoutGrid }],
        },
        {
            heading: "Storage",
            items: [
                { href: "/nas", label: "My Files", icon: Folder },
                { href: "/global", label: "Global Files", icon: Globe },
                { href: "/labels", label: "Labels", icon: Tag },
                { href: "/trash", label: "Trash", icon: Trash2 },
            ],
        },
        {
            heading: "Account",
            items: [{ href: "/profile", label: "Profile", icon: User }],
        },
    ];

    /** "/" only matches exactly; the rest match their subtree. */
    function isActive(href: string): boolean {
        const path = page.url.pathname;
        return href === "/" ? path === "/" : path.startsWith(href);
    }

    async function logout() {
        const result = await safeInvoke("logout");
        if (!result.ok) toasts.error("Log out failed", result.error);
    }
</script>

<nav class="sidebar">
    <div class="groups">
        {#each GROUPS as group (group.heading)}
            <div class="group">
                <p class="eyebrow heading">{group.heading}</p>
                {#each group.items as item (item.href)}
                    {@const Icon = item.icon}
                    <a href={item.href} class="item" class:active={isActive(item.href)}>
                        <Icon size={16} strokeWidth={1.8} />
                        <span>{item.label}</span>
                    </a>
                {/each}
            </div>
        {/each}
    </div>

    <DiskWidget {profile} />

    <div class="footer">
        <button class="logout" onclick={logout}>
            <LogOut size={15} strokeWidth={1.8} />
            <span>Log out</span>
        </button>
    </div>
</nav>

<style>
    .sidebar {
        width: var(--w-sidebar);
        flex: none;
        background: var(--rail);
        border-right: 1px solid var(--bd);
        padding: 14px 0;
        display: flex;
        flex-direction: column;
    }

    .groups {
        flex: 1;
        overflow-y: auto;
        display: flex;
        flex-direction: column;
    }

    .group {
        display: flex;
        flex-direction: column;
        gap: 3px;
    }

    .heading {
        padding: 6px 18px 8px;
    }

    .group:not(:first-child) .heading {
        padding-top: 16px;
    }

    .item {
        display: flex;
        align-items: center;
        gap: 11px;
        padding: 8px 18px;
        font-size: var(--fs-nav);
        color: var(--tx-2);
        border-left: 2px solid transparent;
        transition: background var(--t-hover), color var(--t-hover);
    }

    .item :global(svg) {
        color: var(--tx-mut-2);
        flex: none;
        transition: color var(--t-hover);
    }

    .item:hover {
        background: var(--hover-nav);
        color: var(--tx-hi);
    }

    .item.active {
        background: var(--nav-active);
        color: var(--tx-hi);
        border-left-color: var(--accent);
    }

    .item.active :global(svg) {
        color: var(--link);
    }

    .footer {
        margin: 0 14px;
        border-top: 1px solid var(--bd-meta);
        flex: none;
    }

    .logout {
        display: flex;
        align-items: center;
        gap: 11px;
        width: 100%;
        padding: 9px 12px;
        font-size: var(--fs-btn);
        color: var(--tx-mut-2);
        border-radius: var(--r-control);
        transition: background var(--t-hover), color var(--t-hover);
    }

    .logout:hover {
        background: var(--hover-nav);
        color: var(--tx-hi);
    }
</style>
