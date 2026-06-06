import {writable} from "svelte/store";

export interface Notification {
    id: number;
    status: "FAILURE" | "SUCCESS" | "INFO",
    title: string;
    message?: string;
}

const createNotificationStore = () => {
    const {subscribe, update} = writable<Notification[]>([]);

    return {
        subscribe,
        notify: (status: "FAILURE" | "SUCCESS" | "INFO", title: string, message?: string) => {
            const id = Date.now();
            update(n => [...n, {id, status, title, message}]);
            setTimeout(() => {
                update(n => n.filter(item => item.id !== id));
            }, 5000);
        },
        remove: (id: number) => update(n => n.filter(i => i.id !== id))
    };
}

export const notifications = createNotificationStore();