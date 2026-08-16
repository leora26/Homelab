/*
 * Transient notifications.
 *
 * The design doesn't mock toasts, but several flows need to say something after the fact
 * — a completed download, a folder restored to a different place than it came from. They
 * follow the dialog's surface treatment so they read as part of the same system.
 */

export type ToastTone = "success" | "error" | "info";

export interface Toast {
    id: number;
    tone: ToastTone;
    title: string;
    message?: string;
}

const DISMISS_AFTER = 5000;

/** Monotonic, so two toasts raised in the same millisecond can't collide. */
let nextId = 0;

class ToastStore {
    items = $state<Toast[]>([]);

    private push(tone: ToastTone, title: string, message?: string): number {
        const id = ++nextId;
        this.items = [...this.items, { id, tone, title, message }];

        setTimeout(() => this.dismiss(id), DISMISS_AFTER);

        return id;
    }

    success = (title: string, message?: string) => this.push("success", title, message);
    error = (title: string, message?: string) => this.push("error", title, message);
    info = (title: string, message?: string) => this.push("info", title, message);

    dismiss = (id: number) => {
        this.items = this.items.filter((toast) => toast.id !== id);
    };
}

export const toasts = new ToastStore();
