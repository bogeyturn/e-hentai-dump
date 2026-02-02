export function formatBytesIB(bytes: number): string {
    const units = ['KiB', 'MiB', 'GiB', 'TiB'];
    if (bytes < 1024) return bytes + ' B';
    let i = -1;
    let size = bytes;
    do {
        size /= 1024;
        i++;
    } while (size >= 1024 && i < units.length - 1);
    return size.toFixed(1) + ' ' + units[i];
}