<script lang="ts">
    import { page } from "$app/state";
    import {
        ArrowUp, Copy, Download, FileArchive, FolderOpen, FolderPlus, FolderInput,
        PackageOpen, Pencil, Tag, Trash2, Upload,
    } from "@lucide/svelte";

    import Button from "$lib/components/ui/Button.svelte";
    import Chip from "$lib/components/ui/Chip.svelte";
    import EmptyState from "$lib/components/ui/EmptyState.svelte";
    import ExtBadge from "$lib/components/ui/ExtBadge.svelte";
    import LabelChip from "$lib/components/ui/LabelChip.svelte";
    import LabelDot from "$lib/components/ui/LabelDot.svelte";
    import TextField from "$lib/components/ui/TextField.svelte";
    import FolderTreeNode from "$lib/components/folder/FolderTreeNode.svelte";
    import DetailsRail from "$lib/components/file/DetailsRail.svelte";
    import ConfirmDialog from "$lib/components/dialogs/ConfirmDialog.svelte";
    import PromptDialog from "$lib/components/dialogs/PromptDialog.svelte";
    import FolderPickerDialog from "$lib/components/dialogs/FolderPickerDialog.svelte";
    import LabelPickerDialog from "$lib/components/dialogs/LabelPickerDialog.svelte";
    import LabelFormDialog from "$lib/components/dialogs/LabelFormDialog.svelte";
    import UploadDialog from "$lib/components/dialogs/UploadDialog.svelte";
    import ContextMenu, { type MenuItem } from "$lib/components/ui/ContextMenu.svelte";

    import { safeInvoke } from "$lib/utils/safeInvoke";
    import { toasts } from "$lib/stores/toasts.svelte";
    import { session } from "$lib/stores/session.svelte";
    import { ancestorsOf, treeRevision } from "$lib/utils/folderPath.svelte";
    import { filePathString, pathString, ROOT_LABEL, type PathSegment } from "$lib/utils/paths";
    import { formatBytes, formatDate, pluralise } from "$lib/utils/format";
    import { isArchive } from "$lib/utils/files";
    import type { FileView, FolderView, LabelView } from "$lib/types/models";

    /** Label chips shown inline before the rest collapse into "+ N more". */
    const INLINE_LABEL_CHIPS = 2;

    type SortKey = "name" | "modified" | "size";

    let root = $state<FolderView | null>(null);
    let currentFolder = $state<FolderView | null>(null);
    let crumbs = $state<PathSegment[]>([]);

    let files = $state<FileView[]>([]);
    let subfolders = $state<FolderView[]>([]);
    let allLabels = $state<LabelView[]>([]);

    let loading = $state(false);
    let error = $state<string | null>(null);

    let selectedFile = $state<FileView | null>(null);
    let nameFilter = $state("");
    let activeLabelIds = $state<string[]>([]);
    let labelsExpanded = $state(false);
    let sortKey = $state<SortKey>("name");
    let sortAsc = $state(true);

    // Dialog state
    let showNewFolder = $state(false);
    let showUpload = $state(false);
    let showRename = $state(false);
    let showLabelPicker = $state(false);
    let showNewLabel = $state(false);
    let showTrashConfirm = $state(false);
    let picker = $state<{ mode: "move" | "copy" } | null>(null);
    let busy = $state(false);

    // Right-click menus. `menuFolder` is the folder the menu was opened on, which is not
    // necessarily the folder currently being browsed.
    let menu = $state<{ x: number; y: number; items: MenuItem[] } | null>(null);
    let menuFolder = $state<FolderView | null>(null);
    let showFolderRename = $state(false);
    let folderToDelete = $state<FolderView | null>(null);

    // ---- loading -------------------------------------------------------------

    async function loadRoot() {
        const result = await safeInvoke<FolderView>("get_root_folder");
        if (!result.ok) {
            error = result.error;
            return;
        }
        root = result.data;
        await openFolder(result.data);
    }

    async function openFolder(folder: FolderView) {
        currentFolder = folder;
        selectedFile = null;
        nameFilter = "";
        activeLabelIds = [];
        labelsExpanded = false;

        await Promise.all([loadContents(folder.id), loadCrumbs(folder.id)]);
    }

    async function loadContents(folderId: string) {
        loading = true;
        error = null;

        const [fileResult, folderResult] = await Promise.all([
            safeInvoke<FileView[]>("get_files_for_folder", { folderId }),
            safeInvoke<FolderView[]>("get_subfolders", { folderId }),
        ]);

        if (fileResult.ok) files = fileResult.data;
        else error = fileResult.error;

        if (folderResult.ok) subfolders = folderResult.data;

        loading = false;
    }

    async function loadCrumbs(folderId: string) {
        crumbs = await ancestorsOf(folderId);
    }

    async function loadLabels() {
        const result = await safeInvoke<LabelView[]>("get_labels");
        if (result.ok) allLabels = result.data;
    }

    $effect(() => {
        loadRoot();
        loadLabels();
    });

    $effect(() => {
        const action = page.url.searchParams.get("action");
        if (!action || !currentFolder) return;

        if (action === "folder") showNewFolder = true;
        if (action === "upload") showUpload = true;

        history.replaceState(null, "", "/nas");
    });

    const folderLabels = $derived.by(() => {
        const seen = new Map<string, LabelView>();
        for (const file of files) {
            for (const label of file.labels) {
                if (!seen.has(label.id)) seen.set(label.id, label);
            }
        }
        return [...seen.values()].sort((a, b) => a.name.localeCompare(b.name));
    });

    const visibleLabels = $derived(
        labelsExpanded ? folderLabels : folderLabels.slice(0, INLINE_LABEL_CHIPS),
    );
    const hiddenLabelCount = $derived(Math.max(0, folderLabels.length - INLINE_LABEL_CHIPS));

    const filteredFiles = $derived.by(() => {
        const needle = nameFilter.trim().toLowerCase();

        let result = files.filter((file) => {
            if (needle && !file.name.toLowerCase().includes(needle)) return false;

            // Multi-select is OR — a file matches if it carries any active label.
            if (activeLabelIds.length > 0) {
                const ids = new Set(file.labels.map((label) => label.id));
                if (!activeLabelIds.some((id) => ids.has(id))) return false;
            }

            return true;
        });

        const direction = sortAsc ? 1 : -1;
        result = [...result].sort((a, b) => {
            if (sortKey === "size") return (a.size - b.size) * direction;
            if (sortKey === "modified") return ((a.updated_at ?? 0) - (b.updated_at ?? 0)) * direction;
            return a.name.localeCompare(b.name) * direction;
        });

        return result;
    });

    const showFolders = $derived(nameFilter.trim() === "" && activeLabelIds.length === 0);

    const folderBytes = $derived(files.reduce((sum, file) => sum + file.size, 0));

    const currentPath = $derived(pathString(crumbs));

    const selectedPath = $derived(
        selectedFile ? filePathString(crumbs, selectedFile.name) : "",
    );

    const remainingQuota = $derived(
        session.profile
            ? Math.max(0, session.profile.allowed_storage - session.profile.taken_storage)
            : null,
    );

    function toggleLabel(id: string) {
        activeLabelIds = activeLabelIds.includes(id)
            ? activeLabelIds.filter((existing) => existing !== id)
            : [...activeLabelIds, id];
    }

    function sortBy(key: SortKey) {
        if (sortKey === key) sortAsc = !sortAsc;
        else {
            sortKey = key;
            sortAsc = true;
        }
    }

    async function refresh() {
        if (currentFolder) await loadContents(currentFolder.id);
        await session.refreshStorage();

        // Re-read the selected row from the refreshed list. Without this the rail keeps
        // the pre-action copy — after archiving, its name would still be the old one and
        // the button would stay on "Archive" instead of flipping to "Extract".
        if (selectedFile) {
            selectedFile = files.find((file) => file.id === selectedFile!.id) ?? null;
        }
    }

    async function createFolder(name: string) {
        if (!currentFolder) return;

        const result = await safeInvoke<FolderView>("create_folder", {
            parentFolderId: currentFolder.id,
            name,
        });

        if (!result.ok) throw new Error(result.error);

        showNewFolder = false;
        treeRevision.bump();
        await refresh();
        toasts.success("Folder created", name);
    }

    async function renameFile(newName: string) {
        if (!selectedFile) return;

        const result = await safeInvoke<FileView>("rename_file", {
            fileId: selectedFile.id,
            newName,
        });

        if (!result.ok) throw new Error(result.error);

        showRename = false;
        selectedFile = result.data;
        await refresh();
        toasts.success("File renamed", newName);
    }

    async function moveOrCopy(folderId: string) {
        if (!selectedFile || !picker) return;

        const isMove = picker.mode === "move";
        const result = isMove
            ? await safeInvoke("move_file", { fileId: selectedFile.id, folderId })
            : await safeInvoke("copy_file", { fileId: selectedFile.id, targetFolderId: folderId });

        if (!result.ok) {
            toasts.error(isMove ? "Move failed" : "Copy failed", result.error);
            return;
        }

        picker = null;
        if (isMove) selectedFile = null;
        await refresh();
        toasts.success(isMove ? "File moved" : "File copied");
    }

    async function trashFile() {
        if (!selectedFile) return;

        busy = true;
        const result = await safeInvoke("delete_file", { fileId: selectedFile.id });
        busy = false;

        if (!result.ok) {
            toasts.error("Could not move to trash", result.error);
            return;
        }

        showTrashConfirm = false;
        selectedFile = null;
        await refresh();
        toasts.success("Moved to trash", "Restore it any time from Trash.");
    }

    async function toggleArchive() {
        if (!selectedFile) return;

        const archived = isArchive(selectedFile.name);
        const fileId = selectedFile.id;
        const sizeBefore = selectedFile.size;

        const result = await safeInvoke(archived ? "unarchive_file" : "archive_file", { fileId });

        if (!result.ok) {
            toasts.error(archived ? "Extract failed" : "Archive failed", result.error);
            return;
        }

        await refresh();

        const sizeAfter = files.find((file) => file.id === fileId)?.size ?? null;

        if (archived) {
            toasts.success("Archive extracted", `Now ${formatBytes(sizeAfter)}.`);
            return;
        }

        /*
         * Report the actual saving. Gzip barely dents formats that are already
         * compressed — PNG, JPEG, MP4, ZIP — so saying nothing here reads as if
         * archiving didn't work.
         */
        if (sizeAfter === null) {
            toasts.success("File archived");
        } else if (sizeAfter < sizeBefore) {
            const saved = Math.round((1 - sizeAfter / sizeBefore) * 100);
            toasts.success(
                "File archived",
                `${formatBytes(sizeBefore)} → ${formatBytes(sizeAfter)}, ${saved}% smaller.`,
            );
        } else {
            toasts.info(
                "File archived",
                "No space saved — this format is already compressed.",
            );
        }
    }

    async function renameFolder(newName: string) {
        if (!menuFolder) return;

        const result = await safeInvoke<FolderView>("rename_folder", {
            folderId: menuFolder.id,
            newName,
        });

        if (!result.ok) throw new Error(result.error);

        showFolderRename = false;
        treeRevision.bump();

        // Renaming the folder we're inside changes the breadcrumb too.
        if (currentFolder?.id === menuFolder.id) {
            currentFolder = result.data;
            await loadCrumbs(result.data.id);
        }

        await refresh();
        toasts.success("Folder renamed", newName);
    }

    async function deleteFolder() {
        if (!folderToDelete) return;

        busy = true;
        const result = await safeInvoke("delete_selected_folder", {
            selectedFolderId: folderToDelete.id,
        });
        busy = false;

        if (!result.ok) {
            toasts.error("Could not move folder to trash", result.error);
            return;
        }

        const wasCurrent = currentFolder?.id === folderToDelete.id;
        const parentId = folderToDelete.parent_folder_id;
        folderToDelete = null;
        treeRevision.bump();

        // Deleting the folder we're browsing would leave the pane pointing at nothing.
        if (wasCurrent && parentId) {
            const parent = await safeInvoke<FolderView>("get_folder", { folderId: parentId });
            if (parent.ok) await openFolder(parent.data);
        } else {
            await refresh();
        }

        toasts.success("Folder moved to trash", "Restore it any time from Trash.");
    }

    function openFileMenu(event: MouseEvent, file: FileView) {
        event.preventDefault();
        selectedFile = file;

        menu = {
            x: event.clientX,
            y: event.clientY,
            items: [
                { label: "Labels", icon: Tag, action: () => (showLabelPicker = true) },
                { label: "Rename", icon: Pencil, action: () => (showRename = true) },
                { label: "Move to…", icon: FolderInput, action: () => (picker = { mode: "move" }) },
                { label: "Copy to…", icon: Copy, action: () => (picker = { mode: "copy" }) },
                {
                    label: isArchive(file.name) ? "Extract" : "Archive",
                    icon: isArchive(file.name) ? PackageOpen : FileArchive,
                    action: toggleArchive,
                },
                { label: "Download", icon: Download, action: () => downloadFile(file) },
                {
                    label: "Move to trash",
                    icon: Trash2,
                    destructive: true,
                    action: () => (showTrashConfirm = true),
                },
            ],
        };
    }

    function openFolderMenu(event: MouseEvent, folder: FolderView) {
        event.preventDefault();
        menuFolder = folder;

        const isRootFolder = folder.parent_folder_id === null;

        menu = {
            x: event.clientX,
            y: event.clientY,
            items: [
                {
                    label: "New subfolder",
                    icon: FolderPlus,
                    action: async () => {
                        await openFolder(folder);
                        showNewFolder = true;
                    },
                },
                {
                    label: "Rename",
                    icon: Pencil,
                    // The root folder's name is the account email and is shown as
                    // "My Files" everywhere, so renaming it would be meaningless.
                    disabled: isRootFolder,
                    action: () => (showFolderRename = true),
                },
                {
                    label: "Move to trash",
                    icon: Trash2,
                    destructive: true,
                    disabled: isRootFolder,
                    action: () => (folderToDelete = folder),
                },
            ],
        };
    }

    async function downloadFile(file: FileView) {
        const result = await safeInvoke<string>("download_file", {
            fileId: file.id,
            fileName: file.name,
        });

        if (result.ok) toasts.success("File downloaded", "Saved to your Downloads folder.");
        else toasts.error("Download failed", result.error);
    }

    async function afterLabelsChanged() {
        showLabelPicker = false;
        await Promise.all([refresh(), loadLabels()]);

        // Keep the rail in step with the row it is showing.
        if (selectedFile) {
            selectedFile = files.find((file) => file.id === selectedFile!.id) ?? selectedFile;
        }
    }

    async function createLabel(name: string, color: string) {
        const result = await safeInvoke<LabelView>("create_label", { name, color });
        if (!result.ok) throw new Error(result.error);

        showNewLabel = false;
        await loadLabels();
        toasts.success("Label created", name);
    }
</script>

<div class="screen">
    <div class="toolbar">
        <div class="row">
            <nav class="crumbs" aria-label="Breadcrumb">
                {#each crumbs as crumb, index (crumb.id)}
                    {#if index > 0}<span class="sep">/</span>{/if}
                    {#if index === crumbs.length - 1}
                        <span class="crumb current">{crumb.name}</span>
                    {:else}
                        <button
                            class="crumb link"
                            onclick={async () => {
                                const folder =
                                    crumb.id === root?.id
                                        ? root
                                        : subfolders.find((f) => f.id === crumb.id) ?? null;
                                if (folder) await openFolder(folder);
                                else {
                                    const result = await safeInvoke<FolderView>("get_folder", {
                                        folderId: crumb.id,
                                    });
                                    if (result.ok) await openFolder(result.data);
                                }
                            }}
                        >
                            {crumb.name}
                        </button>
                    {/if}
                {/each}
            </nav>

            <div class="actions">
                <Button onclick={() => (showNewFolder = true)} disabled={!currentFolder}>
                    <FolderPlus size={14} strokeWidth={1.8} />
                    New folder
                </Button>
                <Button variant="primary" onclick={() => (showUpload = true)} disabled={!currentFolder}>
                    <Upload size={14} strokeWidth={1.8} />
                    Upload files
                </Button>
            </div>
        </div>

        <div class="row filters">
            <div class="filter-field">
                <TextField bind:value={nameFilter} placeholder="Filter by name" />
            </div>

            {#each visibleLabels as label (label.id)}
                <LabelChip
                    name={label.name}
                    color={label.color}
                    interactive
                    selected={activeLabelIds.includes(label.id)}
                    onclick={() => toggleLabel(label.id)}
                />
            {/each}

            {#if hiddenLabelCount > 0 && !labelsExpanded}
                <Chip interactive onclick={() => (labelsExpanded = true)}>
                    + {hiddenLabelCount} more
                </Chip>
            {:else if labelsExpanded && folderLabels.length > INLINE_LABEL_CHIPS}
                <Chip interactive onclick={() => (labelsExpanded = false)}>Show fewer</Chip>
            {/if}

            {#if activeLabelIds.length > 0 || nameFilter}
                <button
                    class="clear"
                    onclick={() => {
                        activeLabelIds = [];
                        nameFilter = "";
                    }}
                >
                    Clear
                </button>
            {/if}
        </div>
    </div>

    <div class="panes">
        <nav class="tree" aria-label="Folders">
            <p class="eyebrow tree-heading">Folders</p>
            {#if root}
                <FolderTreeNode
                    folder={root}
                    depth={0}
                    selectedId={currentFolder?.id ?? null}
                    expandTo={crumbs.map((crumb) => crumb.id)}
                    onselect={openFolder}
                    oncontextmenu={openFolderMenu}
                />
            {/if}

            <a class="tree-footer" href="/trash">
                <Trash2 size={14} strokeWidth={1.8} />
                Trash
                {#if session.stats}
                    · {pluralise(session.stats.trashed_item_count, "item")}
                {/if}
            </a>
        </nav>

        <section class="table-pane">
            {#if error}
                <div class="state">
                    <EmptyState
                        icon={FolderOpen}
                        title="Couldn't load this folder"
                        body={error}
                    />
                </div>
            {:else if loading}
                <div class="skeletons">
                    {#each Array(6) as _, index (index)}
                        <div class="skeleton"></div>
                    {/each}
                </div>
            {:else if filteredFiles.length === 0 && (!showFolders || subfolders.length === 0)}
                <div class="state">
                    <EmptyState
                        icon={FolderOpen}
                        title={files.length === 0 ? "This folder is empty" : "No matching files"}
                        body={files.length === 0
                            ? "Use Upload files to add the first one."
                            : "Try a different name or clear the label filters."}
                    >
                        {#snippet action()}
                            {#if files.length === 0}
                                <Button variant="primary" onclick={() => (showUpload = true)}>
                                    Upload files
                                </Button>
                            {:else}
                                <Button
                                    onclick={() => {
                                        activeLabelIds = [];
                                        nameFilter = "";
                                    }}
                                >
                                    Clear filters
                                </Button>
                            {/if}
                        {/snippet}
                    </EmptyState>
                </div>
            {:else}
                <div class="thead">
                    <button class="th" onclick={() => sortBy("name")}>
                        Name
                        {#if sortKey === "name"}
                            <span class="caret" class:desc={!sortAsc}><ArrowUp size={11} strokeWidth={2.4} /></span>
                        {/if}
                    </button>
                    <span class="th static">Labels</span>
                    <button class="th" onclick={() => sortBy("modified")}>
                        Modified
                        {#if sortKey === "modified"}
                            <span class="caret" class:desc={!sortAsc}><ArrowUp size={11} strokeWidth={2.4} /></span>
                        {/if}
                    </button>
                    <button class="th right" onclick={() => sortBy("size")}>
                        Size
                        {#if sortKey === "size"}
                            <span class="caret" class:desc={!sortAsc}><ArrowUp size={11} strokeWidth={2.4} /></span>
                        {/if}
                    </button>
                </div>

                <div class="tbody">
                    {#if showFolders}
                        {#each subfolders as folder (folder.id)}
                            <button
                                class="trow"
                                onclick={() => openFolder(folder)}
                                oncontextmenu={(event) => openFolderMenu(event, folder)}
                            >
                                <span class="cell-name">
                                    <ExtBadge folder />
                                    <span class="truncate">{folder.name}</span>
                                </span>
                                <span class="cell-labels"><span class="empty-value">—</span></span>
                                <span class="mono cell-muted">{formatDate(folder.created_at)}</span>
                                <span class="mono cell-muted right"><span class="empty-value">—</span></span>
                            </button>
                        {/each}
                    {/if}

                    {#each filteredFiles as file (file.id)}
                        <button
                            class="trow"
                            class:selected={selectedFile?.id === file.id}
                            onclick={() => (selectedFile = file)}
                            oncontextmenu={(event) => openFileMenu(event, file)}
                        >
                            <span class="cell-name">
                                <ExtBadge name={file.name} />
                                <span class="truncate">{file.name}</span>
                            </span>
                            <span class="cell-labels">
                                {#if file.labels.length > 0}
                                    {#each file.labels as label (label.id)}
                                        <LabelDot name={label.name} color={label.color} />
                                    {/each}
                                {:else}
                                    <span class="empty-value">—</span>
                                {/if}
                            </span>
                            <span class="mono cell-muted">{formatDate(file.updated_at)}</span>
                            <span class="mono cell-muted right">{formatBytes(file.size)}</span>
                        </button>
                    {/each}
                </div>

                <div class="tfoot">
                    <span>{pluralise(filteredFiles.length + (showFolders ? subfolders.length : 0), "item")}</span>
                    <span class="mono">{formatBytes(folderBytes)} in this folder</span>
                </div>
            {/if}
        </section>

        {#if selectedFile}
            <DetailsRail
                file={selectedFile}
                labels={selectedFile.labels}
                path={selectedPath}
                onclose={() => (selectedFile = null)}
                onrename={() => (showRename = true)}
                onmove={() => (picker = { mode: "move" })}
                oncopy={() => (picker = { mode: "copy" })}
                onlabels={() => (showLabelPicker = true)}
                onarchive={toggleArchive}
                ontrash={() => (showTrashConfirm = true)}
                onsharechange={refresh}
            />
        {/if}
    </div>
</div>

<PromptDialog
    open={showNewFolder}
    title="Create folder"
    subtitle={currentPath}
    fieldLabel="Folder name"
    hint="Letters, numbers, dashes and underscores."
    confirmLabel="Create folder"
    placeholder="e.g. vacation-photos"
    onsubmit={createFolder}
    onclose={() => (showNewFolder = false)}
/>

<PromptDialog
    open={showRename}
    title="Rename file"
    subtitle={selectedFile?.name}
    fieldLabel="File name"
    hint="Changing the extension may make the file unopenable."
    initialValue={selectedFile?.name ?? ""}
    confirmLabel="Rename"
    onsubmit={renameFile}
    onclose={() => (showRename = false)}
/>

<FolderPickerDialog
    open={picker !== null}
    mode={picker?.mode ?? "move"}
    fileName={selectedFile?.name ?? ""}
    {root}
    currentFolderId={currentFolder?.id ?? null}
    onsubmit={moveOrCopy}
    onclose={() => (picker = null)}
/>

{#if selectedFile}
    <LabelPickerDialog
        open={showLabelPicker}
        fileId={selectedFile.id}
        fileName={selectedFile.name}
        current={selectedFile.labels}
        {allLabels}
        onsaved={afterLabelsChanged}
        onnewlabel={() => {
            showLabelPicker = false;
            showNewLabel = true;
        }}
        onclose={() => (showLabelPicker = false)}
    />
{/if}

<LabelFormDialog
    open={showNewLabel}
    onsubmit={createLabel}
    onclose={() => (showNewLabel = false)}
/>

<ConfirmDialog
    open={showTrashConfirm}
    title="Move to trash"
    body="{selectedFile?.name ?? 'This file'} will be moved to the trash. It keeps taking up space until you empty it, and you can restore it any time."
    confirmLabel="Move to trash"
    {busy}
    onconfirm={trashFile}
    onclose={() => (showTrashConfirm = false)}
/>

<ContextMenu
    open={menu !== null}
    x={menu?.x ?? 0}
    y={menu?.y ?? 0}
    items={menu?.items ?? []}
    onclose={() => (menu = null)}
/>

<PromptDialog
    open={showFolderRename}
    title="Rename folder"
    subtitle={menuFolder?.name}
    fieldLabel="Folder name"
    hint="Letters, numbers, dashes and underscores."
    initialValue={menuFolder?.name ?? ""}
    confirmLabel="Rename"
    onsubmit={renameFolder}
    onclose={() => (showFolderRename = false)}
/>

<ConfirmDialog
    open={folderToDelete !== null}
    title="Move folder to trash"
    body="{folderToDelete?.name ?? 'This folder'} and everything inside it will be moved to the trash. It keeps taking up space until you empty it, and you can restore it any time."
    confirmLabel="Move to trash"
    {busy}
    onconfirm={deleteFolder}
    onclose={() => (folderToDelete = null)}
/>

<UploadDialog
    open={showUpload}
    destinationId={currentFolder?.id ?? null}
    destinationPath={currentPath}
    {allLabels}
    {remainingQuota}
    oncomplete={refresh}
    onclose={() => (showUpload = false)}
/>

<style>
    .screen {
        flex: 1;
        display: flex;
        flex-direction: column;
        min-height: 0;
    }

    .toolbar {
        padding: 18px 24px 14px;
        display: flex;
        flex-direction: column;
        gap: 14px;
        border-bottom: 1px solid var(--bd-pane);
        flex: none;
    }

    .row {
        display: flex;
        align-items: center;
        justify-content: space-between;
        gap: 16px;
    }

    .row.filters {
        justify-content: flex-start;
        gap: 9px;
        flex-wrap: wrap;
    }

    .filter-field {
        flex: 1;
        min-width: 200px;
        max-width: 420px;
    }

    .crumbs {
        display: flex;
        align-items: center;
        gap: 7px;
        font-size: var(--fs-base);
        min-width: 0;
        overflow: hidden;
    }

    .crumb {
        white-space: nowrap;
    }

    .crumb.link {
        color: var(--link);
    }

    .crumb.link:hover {
        color: var(--link-hover);
    }

    .crumb.current {
        color: var(--tx);
        font-weight: 500;
    }

    .sep {
        color: var(--tx-ghost);
    }

    .actions {
        display: flex;
        gap: 9px;
        flex: none;
    }

    .clear {
        font-size: var(--fs-sm);
        color: var(--link);
    }

    .clear:hover {
        color: var(--link-hover);
    }

    .panes {
        flex: 1;
        display: flex;
        min-height: 0;
    }

    .tree {
        width: var(--w-tree);
        flex: none;
        border-right: 1px solid var(--bd-pane);
        padding: 14px 12px;
        overflow-y: auto;
        display: flex;
        flex-direction: column;
    }

    .tree-heading {
        padding: 0 8px 8px;
    }

    .tree-footer {
        margin-top: auto;
        display: flex;
        align-items: center;
        gap: 8px;
        padding: 8px;
        border-top: 1px solid var(--bd-meta);
        font-size: var(--fs-btn);
        color: var(--tx-mut);
    }

    .tree-footer:hover {
        color: var(--tx-hi);
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
        grid-template-columns: minmax(240px, 1fr) 84px 104px 84px;
        gap: 12px;
        align-items: center;
    }

    .thead {
        padding: 9px 16px;
        border-bottom: 1px solid var(--bd-pane);
        flex: none;
    }

    .th {
        display: flex;
        align-items: center;
        gap: 5px;
        font-size: var(--fs-label);
        text-transform: uppercase;
        letter-spacing: var(--track-th);
        color: var(--tx-faint-2);
        text-align: left;
    }

    .th.static {
        cursor: default;
    }

    .th.right {
        justify-content: flex-end;
    }

    .th:not(.static):hover {
        color: var(--tx-2);
    }

    .caret {
        display: flex;
        color: var(--link);
    }

    .caret.desc {
        transform: rotate(180deg);
    }

    .tbody {
        flex: 1;
        overflow-y: auto;
        min-height: 0;
    }

    .trow {
        width: 100%;
        padding: 9px 16px;
        font-size: var(--fs-base);
        color: var(--tx);
        border-bottom: 1px solid var(--bd-row-soft);
        border-left: 2px solid transparent;
        text-align: left;
        transition: background var(--t-hover);
    }

    .trow:hover {
        background: var(--hover-row);
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

    .cell-labels {
        display: flex;
        align-items: center;
        gap: 5px;
        padding-left: 10px;
    }

    .cell-muted {
        color: var(--tx-mut);
        font-size: var(--fs-sm);
    }

    .right {
        text-align: right;
    }

    .tfoot {
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 9px 16px;
        border-top: 1px solid var(--bd-pane);
        font-size: var(--fs-caption);
        color: var(--tx-faint-2);
        flex: none;
    }

    .state {
        flex: 1;
        display: flex;
    }

    .skeletons {
        padding: 12px 16px;
        display: flex;
        flex-direction: column;
        gap: 12px;
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
