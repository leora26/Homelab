<script lang="ts">
    import ContextMenu, { type ContextMenuOption } from "$lib/components/common/ContextMenu.svelte";

    interface FolderContextMenuProps {
        x: number;
        y: number;
        targetId: string;
        rootFolderId: string;
        viewMode: string;
        onRename: () => void;
        onDelete: () => void;
        onRestore: () => void;
        onNewFolder: (parentFolderId: string) => void;
        onRemove: () => void;
    }

    let {
        x,
        y,
        targetId,
        rootFolderId,
        viewMode,
        onRename,
        onDelete,
        onRestore,
        onNewFolder,
        onRemove
    }: FolderContextMenuProps = $props();

    let menuOptions = $derived.by<ContextMenuOption[]>(() => {
        let options: ContextMenuOption[] = [];

        if (viewMode === 'active') {
            options.push({
                label: 'New Subfolder',
                icon: '📁',
                action: () => onNewFolder(targetId)
            });

            if (targetId !== rootFolderId) {
                options.push(
                    {
                        label: 'Rename',
                        icon: '✏️',
                        action: onRename
                    },
                    {
                        label: 'Move to Trash',
                        icon: '🗑️',
                        danger: true,
                        action: onDelete
                    }
                );
            }
        }
        else if (viewMode === 'trash') {
            options.push(
                {
                    label: 'Restore',
                    icon: '↩️',
                    action: onRestore
                },
                {
                    label: 'Permanently Delete',
                    icon: '❌',
                    danger: true,
                    action: onRemove
                }
            );
        }

        return options;
    });
</script>

<ContextMenu {x} {y} options={menuOptions}/>