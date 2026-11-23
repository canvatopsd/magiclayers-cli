import { execFileSync, spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { readFile, lstat, mkdir, rm } from "node:fs/promises";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { unit, build } from "./paths.mjs";
import { version } from "./release.mjs";

export const archivePath = resolve(
  build,
  `npm/canvatopsd-magiclayers-${version}.tgz`,
);
export async function checkNpmArchive(path = archivePath) {
  const entries = execFileSync("tar", ["-tzf", path], { encoding: "utf8" })
    .trim()
    .split("\n");
  for (const entry of entries) {
    if (
      entry.includes("..") ||
      !/^package\/(?:package\.json|README\.md|release\.json|bin\/magiclayers\.mjs|runtime\/(?:darwin-arm64|linux-x64)\/magiclayers|skills\/magiclayers\/SKILL\.md|contracts\/guest-agent-commands\.json|doc\/usage\.md)$/.test(
        entry,
      )
    )
      throw new Error(`Unexpected npm entry: ${entry}`);
  }
  const destination = resolve(build, "npm/inspection");
  await rm(destination, { recursive: true, force: true });
  await mkdir(destination, { recursive: true });
  execFileSync("tar", ["-xzf", path, "-C", destination]);
  const directory = resolve(destination, "package");
  const metadata = JSON.parse(
    await readFile(resolve(directory, "package.json"), "utf8"),
  );
  const release = JSON.parse(
    await readFile(resolve(directory, "release.json"), "utf8"),
  );
  if (
    metadata.name !== "@canvatopsd/magiclayers" ||
    metadata.version !== version ||
    release.version !== version ||
    release.environment !== "prod" ||
    metadata.gitHead !== release.revision ||
    metadata.bin?.magiclayers !== "bin/magiclayers.mjs" ||
    Object.keys(metadata.scripts || {}).length
  )
    throw new Error("npm release metadata mismatch");
  if (
    JSON.stringify(metadata.magiclayers.targets) !==
      JSON.stringify(["darwin-arm64", "linux-x64"]) ||
    JSON.stringify(release.targets) !==
      JSON.stringify(metadata.magiclayers.targets)
  )
    throw new Error("npm platform matrix mismatch");
  for (const target of release.targets) {
    const path = resolve(directory, "runtime", target, "magiclayers");
    const info = await lstat(path);
    const bytes = await readFile(path);
    if (!info.isFile() || !(info.mode & 0o111) || bytes.length < 100000)
      throw new Error(`Missing native executable: ${target}`);
    if (
      createHash("sha256").update(bytes).digest("hex") !==
      release.binaries[target].sha256
    )
      throw new Error(`Native checksum mismatch: ${target}`);
    if (
      target === "darwin-arm64" &&
      (bytes.readUInt32LE(0) !== 0xfeedfacf ||
        bytes.readUInt32LE(4) !== 0x0100000c)
    )
      throw new Error("Expected Apple Silicon Mach-O");
    if (
      target === "linux-x64" &&
      (bytes.subarray(0, 4).toString("hex") !== "7f454c46" ||
        bytes[4] !== 2 ||
        bytes.readUInt16LE(18) !== 62)
    )
      throw new Error("Expected Linux x86_64 ELF");
  }
  for (const name of [
    "skills/magiclayers/SKILL.md",
    "contracts/guest-agent-commands.json",
    "doc/usage.md",
    "bin/magiclayers.mjs",
  ]) {
    if (
      !(await readFile(resolve(directory, name))).equals(
        await readFile(resolve(unit, name)),
      )
    )
      throw new Error(`Outdated npm content: ${name}`);
  }
  const entry = resolve(directory, "bin/magiclayers.mjs");
  const run = (args) =>
    execFileSync(process.execPath, [entry, ...args], {
      encoding: "utf8",
    }).trim();
  if (run(["--version"]) !== `magiclayers ${version}`)
    throw new Error("npm executable version mismatch");
  if (
    !JSON.parse(run(["discover"])).ok ||
    !JSON.parse(run(["skill", "show"])).ok
  )
    throw new Error("npm offline commands failed");
  const failure = spawnSync(process.execPath, [entry, "--json", "doctor"], {
    encoding: "utf8",
  });
  if (
    failure.status !== 2 ||
    JSON.parse(failure.stdout).error.code !== "invalid_arguments"
  )
    throw new Error("npm exit code forwarding failed");
  const integrity =
    "sha512-" +
    createHash("sha512")
      .update(await readFile(path))
      .digest("base64");
  return {
    directory,
    archive: path,
    integrity,
    version,
    targets: release.targets,
  };
}
if (
  process.argv[1] &&
  fileURLToPath(import.meta.url) === resolve(process.argv[1])
)
  console.log(
    JSON.stringify(await checkNpmArchive(process.argv[2] || archivePath)),
  );
