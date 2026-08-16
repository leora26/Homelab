/*
 * Session-wide data the shell needs on every screen: who is signed in, their quota, the
 * machine the app is talking to, and the aggregate counts several screens display.
 *
 * Loaded once after authentication and refreshable by anything that changes storage —
 * an upload, a delete, emptying the trash.
 */

import { safeInvoke } from "$lib/utils/safeInvoke";
import type {
    MachineInfoView,
    StorageProfileView,
    StorageStatsView,
    UserProfileView,
} from "$lib/types/models";

class SessionStore {
    user = $state<UserProfileView | null>(null);
    profile = $state<StorageProfileView | null>(null);
    machine = $state<MachineInfoView | null>(null);
    stats = $state<StorageStatsView | null>(null);
    userCount = $state<number | null>(null);
    loading = $state(false);
    error = $state<string | null>(null);

    load = async () => {
        this.loading = true;
        this.error = null;

        const [user, profile, machine, stats] = await Promise.all([
            safeInvoke<UserProfileView>("get_user_profile"),
            safeInvoke<StorageProfileView>("get_storage_profile"),
            safeInvoke<MachineInfoView>("get_machine_info"),
            safeInvoke<StorageStatsView>("get_storage_stats"),
        ]);

        if (user.ok) this.user = user.data;
        if (profile.ok) this.profile = profile.data;
        if (machine.ok) this.machine = machine.data;
        if (stats.ok) this.stats = stats.data;

        // The identity call is the one the rest of the app can't work without.
        if (!user.ok) this.error = user.error;

        this.loading = false;
    };

    /** Only Profile shows this, so it isn't part of the initial load. */
    loadUserCount = async () => {
        const result = await safeInvoke<number>("get_user_count");
        if (result.ok) this.userCount = result.data;
    };

    /** Re-reads quota and counts after an operation that changes stored bytes. */
    refreshStorage = async () => {
        const [profile, stats] = await Promise.all([
            safeInvoke<StorageProfileView>("get_storage_profile"),
            safeInvoke<StorageStatsView>("get_storage_stats"),
        ]);

        if (profile.ok) this.profile = profile.data;
        if (stats.ok) this.stats = stats.data;
    };

    /**
     * Waits for an emptied trash to actually be gone.
     *
     * `cleanup_trash` only publishes a message — the deletion happens in a RabbitMQ
     * consumer, so refetching immediately returns the trash still full and the screen
     * looks broken. This polls the stats until the count drops or we give up.
     *
     * Returns true if the trash cleared within the window.
     */
    awaitTrashCleared = async (timeoutMs = 8000, intervalMs = 400): Promise<boolean> => {
        const deadline = Date.now() + timeoutMs;

        while (Date.now() < deadline) {
            await new Promise((resolve) => setTimeout(resolve, intervalMs));
            await this.refreshStorage();

            if ((this.stats?.trashed_item_count ?? 0) === 0) return true;
        }

        return false;
    };

    clear = () => {
        this.user = null;
        this.profile = null;
        this.machine = null;
        this.stats = null;
        this.userCount = null;
        this.error = null;
    };
}

export const session = new SessionStore();
