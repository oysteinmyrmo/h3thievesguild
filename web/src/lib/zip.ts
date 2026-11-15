// A minimal ZIP writer. The files are stored, not compressed: the charts are PNG images, which
// are compressed already, so deflating them again would gain almost nothing.

export interface ZipEntry {
    name: string;
    data: Uint8Array;
}

const CRC_TABLE = (() => {
    const table = new Uint32Array(256);
    for (let n = 0; n < 256; n++) {
        let c = n;
        for (let k = 0; k < 8; k++) {
            c = c & 1 ? 0xedb88320 ^ (c >>> 1) : c >>> 1;
        }
        table[n] = c >>> 0;
    }
    return table;
})();

/** CRC-32 as ZIP uses it. */
export function crc32(data: Uint8Array): number {
    let crc = 0xffffffff;
    for (const byte of data) {
        crc = CRC_TABLE[(crc ^ byte) & 0xff]! ^ (crc >>> 8);
    }
    return (crc ^ 0xffffffff) >>> 0;
}

/** File time and date in MS-DOS format, as ZIP stores them. */
function dosDateTime(date: Date): { time: number; day: number } {
    const year = Math.max(date.getFullYear(), 1980);
    return {
        time: (date.getHours() << 11) | (date.getMinutes() << 5) | (date.getSeconds() >> 1),
        day: ((year - 1980) << 9) | ((date.getMonth() + 1) << 5) | date.getDate(),
    };
}

/** A ZIP archive of the entries, stored uncompressed, with UTF-8 names. */
export function zipStore(entries: ZipEntry[], modified = new Date()): Uint8Array<ArrayBuffer> {
    const encoder = new TextEncoder();
    const { time, day } = dosDateTime(modified);
    const UTF8_NAMES = 1 << 11;
    const locals: Uint8Array[] = [];
    const centrals: Uint8Array[] = [];
    let offset = 0;
    for (const entry of entries) {
        const name = encoder.encode(entry.name);
        const crc = crc32(entry.data);
        const size = entry.data.length;

        const local = new Uint8Array(30 + name.length);
        const l = new DataView(local.buffer);
        l.setUint32(0, 0x04034b50, true);
        l.setUint16(4, 20, true); // Version needed: 2.0.
        l.setUint16(6, UTF8_NAMES, true);
        l.setUint16(8, 0, true); // Stored.
        l.setUint16(10, time, true);
        l.setUint16(12, day, true);
        l.setUint32(14, crc, true);
        l.setUint32(18, size, true);
        l.setUint32(22, size, true);
        l.setUint16(26, name.length, true);
        local.set(name, 30);

        const central = new Uint8Array(46 + name.length);
        const c = new DataView(central.buffer);
        c.setUint32(0, 0x02014b50, true);
        c.setUint16(4, 20, true); // Made by: 2.0.
        c.setUint16(6, 20, true);
        c.setUint16(8, UTF8_NAMES, true);
        c.setUint16(10, 0, true);
        c.setUint16(12, time, true);
        c.setUint16(14, day, true);
        c.setUint32(16, crc, true);
        c.setUint32(20, size, true);
        c.setUint32(24, size, true);
        c.setUint16(28, name.length, true);
        c.setUint32(42, offset, true);
        central.set(name, 46);

        locals.push(local, entry.data);
        centrals.push(central);
        offset += local.length + size;
    }
    const centralSize = centrals.reduce((sum, c) => sum + c.length, 0);
    const end = new Uint8Array(22);
    const e = new DataView(end.buffer);
    e.setUint32(0, 0x06054b50, true);
    e.setUint16(8, entries.length, true);
    e.setUint16(10, entries.length, true);
    e.setUint32(12, centralSize, true);
    e.setUint32(16, offset, true);

    const out = new Uint8Array(offset + centralSize + end.length);
    let pos = 0;
    for (const part of [...locals, ...centrals, end]) {
        out.set(part, pos);
        pos += part.length;
    }
    return out;
}
