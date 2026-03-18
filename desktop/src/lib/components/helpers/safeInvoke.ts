import {invoke} from "@tauri-apps/api/core";

export type SafeResult<T> = { ok: true; data: T } | { ok: false; error: string };

export async function safeInvoke<T>(command: string, args?: Record<string, unknown>): Promise<SafeResult<T>> {
    try {
        const data = await invoke<T>(command, args);
        return {ok: true, data}
    } catch (err) {
        return {ok: false, error: String(err)}
    }
}