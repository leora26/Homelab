export const getFileIcon = (type: string) => {
    switch (type.toLowerCase()) {
        case 'image': return '🖼️';
        case 'video': return '🎞️';
        case 'audio': return '🎵';
        case 'text': return '📄';
        case 'pdf': return '📕';
        case 'zip':
        case 'archive': return '📦';
        default: return '📎';
    }
}