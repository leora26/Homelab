const isFileArchived = (fileName: string): boolean => {
    if (!fileName) {
        return false;
    }

    const parts = fileName.split('.');

    if (parts.length <= 1) {
        return false;
    }

    const ext = parts.pop()?.toLowerCase();
    return ['zip', 'gz', 'tar', 'rar', '7z', 'bz2', 'xz', 'iso'].includes(ext || '');
}

export default isFileArchived;