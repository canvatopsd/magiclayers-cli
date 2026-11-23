import { cp, mkdir, readFile, rm, writeFile, chmod } from "node:fs/promises";
import { resolve } from "node:path";
import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import { version } from "./release.mjs";
import { unit, project, build } from "./paths.mjs";
import { buildLinuxBinary } from "./build-linux-binary.mjs";

if (process.platform !== "darwin" || process.arch !== "arm64")
  throw new Error(
    "The release matrix currently requires an Apple Silicon macOS build host",
  );
const metadata = JSON.parse(
  await readFile(resolve(unit, "package.json"), "utf8"),
);
if (metadata.version !== version) throw new Error("Cargo/npm versions differ");
execFileSync("bash", [resolve(unit, "deployment/build.sh")], {
  stdio: "inherit",
});
execFileSync("bash", [resolve(unit, "deployment/check-artifact.sh")], {
  stdio: "inherit",
});
const linux = buildLinuxBinary();
const directory = resolve(build, "npm/package");
await rm(directory, { recursive: true, force: true });
await mkdir(directory, { recursive: true });
const binaries = {};
for (const [target, binary] of [
  ["darwin-arm64", resolve(build, "release/magiclayers")],
  ["linux-x64", linux],
]) {
  const path = resolve(directory, "runtime", target, "magiclayers");
  await mkdir(resolve(path, ".."), { recursive: true });
  await cp(binary, path);
  await chmod(path, 0o755);
  binaries[target] = {
    sha256: createHash("sha256")
      .update(await readFile(path))
      .digest("hex"),
  };
}
for (const name of ["bin", "skills", "contracts"])
  await cp(resolve(unit, name), resolve(directory, name), { recursive: true });
await mkdir(resolve(directory, "doc"));
await cp(resolve(unit, "doc/usage.md"), resolve(directory, "doc/usage.md"));
await cp(resolve(unit, "README.md"), resolve(directory, "README.md"));
metadata.scripts = {};
const revision =
  process.env.MAGICLAYERS_PUBLIC_REVISION ||
  execFileSync("git", ["rev-parse", "HEAD"], {
    cwd: unit,
    encoding: "utf8",
  }).trim();
if (!/^[0-9a-f]{40}$/.test(revision))
  throw new Error("Expected a full public source commit SHA");
metadata.gitHead = revision;
await writeFile(
  resolve(directory, "package.json"),
  JSON.stringify(metadata, null, 2) + "\n",
);
await writeFile(
  resolve(directory, "release.json"),
  JSON.stringify(
    {
      version,
      revision,
      environment: "prod",
      targets: metadata.magiclayers.targets,
      binaries,
    },
    null,
    2,
  ) + "\n",
);
execFileSync(
  "npm",
  [
    "pack",
    directory,
    "--ignore-scripts",
    "--json",
    "--pack-destination",
    resolve(build, "npm"),
    "--cache",
    resolve(project, ".tmp/cache/npm"),
    "--registry=https://registry.npmjs.org/",
  ],
  { cwd: unit, stdio: "inherit" },
);
