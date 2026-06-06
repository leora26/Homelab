<script lang="ts">
    import ContextMenu, {type ContextMenuOption} from "$lib/components/common/ContextMenu.svelte";
    import isFileArchived from "$lib/components/helpers/file/isFileArchived";
    import type {IFileContextMenu} from "$lib/components/file/ContentSection.svelte";

    interface FileContextMenuProps {
        contextMenu: IFileContextMenu;
        triggerRename: () => void;
        triggerCopy: () => void;
        triggerDelete: (id: string) => void;
        triggerMove: () => void;
        triggerArchive: () => void;
        triggerUnarchive: () => void;
    }

    let {
        contextMenu,
        triggerRename,
        triggerCopy,
        triggerDelete,
        triggerUnarchive,
        triggerArchive,
        triggerMove
    }: FileContextMenuProps = $props();


    let menuOptions = $derived.by<ContextMenuOption[]>(() => {

        let targetIsArchived = isFileArchived(contextMenu.targetName);
        console.log("File archived: ", targetIsArchived)

        const options: ContextMenuOption[] = [
            {
                label: 'Rename',
                icon: '✏️',
                action: () => {
                    triggerRename()
                }
            },
            {
                label: 'Copy',
                icon: '📋',
                danger: false,
                action: () => {
                    triggerCopy();
                }
            },
            {
                label: 'Delete',
                icon: '🗑️',
                danger: true,
                action: () => {
                    triggerDelete(contextMenu.targetId);
                }
            },
            {
                label: 'Move',
                icon: '➡️',
                danger: false,
                action: () => {
                    triggerMove();
                }
            },
            {
                label: 'Archive',
                icon: '📦',
                disabled: targetIsArchived,
                action: () => {
                    if (targetIsArchived) return;
                    triggerArchive();
                }
            },
            {
                label: 'Extract',
                icon: '🗜️',
                disabled: !targetIsArchived,
                action: () => {
                    if (!targetIsArchived) return;
                    triggerUnarchive();
                }
            },
        ];

        return options;
    });
</script>

<ContextMenu
        x={contextMenu.x}
        y={contextMenu.y}
        options={menuOptions}
/>