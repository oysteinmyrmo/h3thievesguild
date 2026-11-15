import { describe, expect, it } from "vitest";
import { fileName, printable } from "../../src/lib/export";
import { crc32, zipStore } from "../../src/lib/zip";

/** The entries of a ZIP file, read back through its central directory. */
function readZip(zip: Uint8Array): { name: string; data: Uint8Array; crc: number; flags: number }[] {
    const view = new DataView(zip.buffer, zip.byteOffset, zip.byteLength);
    const end = zip.length - 22;
    expect(view.getUint32(end, true)).toBe(0x06054b50);
    const count = view.getUint16(end + 10, true);
    let pos = view.getUint32(end + 16, true);
    const entries = [];
    for (let i = 0; i < count; i++) {
        expect(view.getUint32(pos, true)).toBe(0x02014b50);
        const flags = view.getUint16(pos + 8, true);
        const method = view.getUint16(pos + 10, true);
        const crc = view.getUint32(pos + 16, true);
        const size = view.getUint32(pos + 20, true);
        const nameLength = view.getUint16(pos + 28, true);
        const local = view.getUint32(pos + 42, true);
        const name = new TextDecoder().decode(zip.subarray(pos + 46, pos + 46 + nameLength));
        expect(method).toBe(0);
        // The local header agrees and the data follows it.
        expect(view.getUint32(local, true)).toBe(0x04034b50);
        expect(view.getUint32(local + 14, true)).toBe(crc);
        const localNameLength = view.getUint16(local + 26, true);
        const start = local + 30 + localNameLength + view.getUint16(local + 28, true);
        entries.push({ name, data: zip.subarray(start, start + size), crc, flags });
        pos += 46 + nameLength + view.getUint16(pos + 30, true) + view.getUint16(pos + 32, true);
    }
    return entries;
}

describe("zip", () => {
    it("computes CRC-32", () => {
        expect(crc32(new TextEncoder().encode("123456789"))).toBe(0xcbf43926);
        expect(crc32(new Uint8Array())).toBe(0);
    });

    it("stores files that read back unchanged, with UTF-8 names", () => {
        const files = [
            { name: "Ålesund - Gold.png", data: new Uint8Array([137, 80, 78, 71, 0, 255]) },
            { name: "empty.png", data: new Uint8Array() },
            { name: "big.bin", data: Uint8Array.from({ length: 70_000 }, (_, i) => i % 251) },
        ];
        const entries = readZip(zipStore(files, new Date(2026, 10, 15, 21, 40, 0)));
        expect(entries.map((e) => e.name)).toEqual(files.map((f) => f.name));
        entries.forEach((entry, i) => {
            expect(entry.data).toEqual(files[i]!.data);
            expect(entry.crc).toBe(crc32(files[i]!.data));
            expect(entry.flags & (1 << 11)).not.toBe(0);
        });
    });

    it("writes an empty archive", () => {
        expect(readZip(zipStore([]))).toEqual([]);
    });
});

describe("file names", () => {
    it("removes control characters, which some maps have in their names", () => {
        expect(printable("\u0012\u0001Tags\u0001Simon\u00022025.11.13\u0002FFA_7M")).toBe("Tags Simon 2025.11.13 FFA_7M");
    });

    it("are safe on every file system and never empty", () => {
        expect(fileName("My map - Gold", "png")).toBe("My map - Gold.png");
        expect(fileName('a/b\\c:d*e?f"g<h>i|j', "png")).toBe("a_b_c_d_e_f_g_h_i_j.png");
        expect(fileName("  ..hidden.  ", "zip")).toBe("hidden.zip");
        expect(fileName("\u0001\u0002", "png")).toBe("chart.png");
        expect(fileName("x".repeat(300), "png")).toBe(`${"x".repeat(120)}.png`);
        expect(fileName("2025.11.13 12;13 Øystein", "png")).toBe("2025.11.13 12;13 Øystein.png");
    });
});
