// Build the h3wasm crate for WebAssembly and generate its JavaScript bindings into
// src/lib/wasm/pkg. Needs the wasm32-unknown-unknown Rust target and the wasm-bindgen
// command-line tool at the version of the wasm-bindgen crate in Cargo.lock.

import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const web = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const root = resolve(web, "..");
const targetDir = process.env.CARGO_TARGET_DIR ? resolve(process.env.CARGO_TARGET_DIR) : join(root, "target");
const outDir = join(web, "src", "lib", "wasm", "pkg");

function run(command, args) {
    try {
        execFileSync(command, args, { cwd: root, stdio: "inherit" });
    } catch (error) {
        if (error.code === "ENOENT") {
            console.error(`\n${command} was not found. See "Web frontend" in README.md for setup.\n`);
        }
        process.exit(1);
    }
}

function wantedWasmBindgen() {
    const lock = readFileSync(join(root, "Cargo.lock"), "utf8");
    const match = lock.match(/name = "wasm-bindgen"\nversion = "([^"]+)"/);
    return match ? match[1] : undefined;
}

function installedWasmBindgen() {
    try {
        return execFileSync("wasm-bindgen", ["--version"], { encoding: "utf8" }).trim().split(" ")[1];
    } catch {
        return undefined;
    }
}

const wanted = wantedWasmBindgen();
const installed = installedWasmBindgen();
if (installed === undefined) {
    console.error(`\nwasm-bindgen was not found. Install it with:\n  cargo install wasm-bindgen-cli --version ${wanted}\n`);
    process.exit(1);
}
if (wanted && installed !== wanted) {
    console.error(
        `\nwasm-bindgen ${installed} is installed, but Cargo.lock uses ${wanted}. Install the matching version:\n` +
            `  cargo install wasm-bindgen-cli --version ${wanted}\n`,
    );
    process.exit(1);
}

run("cargo", ["build", "--release", "-p", "h3wasm", "--target", "wasm32-unknown-unknown"]);
run("wasm-bindgen", [
    "--target",
    "web",
    "--out-dir",
    outDir,
    join(targetDir, "wasm32-unknown-unknown", "release", "h3wasm.wasm"),
]);
console.log(`WebAssembly bindings written to ${outDir}`);
