#!/usr/bin/env node
import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";
import { existsSync } from "node:fs";
import metadata from "../package.json" with { type: "json" };

function fail(code, message) {
  console.log(JSON.stringify({ ok: false, error: { code, message } }));
  process.exit(1);
}
const target = `${process.platform}-${process.arch}`;
if (!metadata.magiclayers.targets.includes(target))
  fail(
    "unsupported_platform",
    `Supported platforms: ${metadata.magiclayers.targets.join(", ")}; received ${target}`,
  );
const executable = fileURLToPath(
  new URL(`../runtime/${target}/magiclayers`, import.meta.url),
);
if (!existsSync(executable))
  fail(
    "native_binary_missing",
    `The package is missing its ${target} executable; reinstall the official release`,
  );
const result = spawnSync(executable, process.argv.slice(2), {
  stdio: "inherit",
});
if (result.error) fail("native_launch_failed", result.error.message);
if (result.signal) process.kill(process.pid, result.signal);
else process.exit(result.status ?? 1);
