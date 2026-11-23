import { execFileSync } from "node:child_process";
import { resolve } from "node:path";
import { checkNpmArchive } from "./check-npm.mjs";
import { unit, project } from "./paths.mjs";
import { version } from "./release.mjs";

if (process.argv.slice(2).some((arg) => arg !== "--dry-run"))
  throw new Error("Only --dry-run is supported");
execFileSync(process.execPath, [resolve(unit, "deployment/build-npm.mjs")], {
  cwd: unit,
  stdio: "inherit",
});
const checked = await checkNpmArchive();
// Publish the exact final archive; never repack source during publication.
execFileSync(
  "npm",
  [
    "publish",
    checked.archive,
    "--access",
    "public",
    "--registry=https://registry.npmjs.org/",
    "--cache",
    resolve(project, ".tmp/cache/npm"),
    ...process.argv.slice(2),
  ],
  { cwd: unit, stdio: "inherit" },
);
console.log(
  JSON.stringify({
    name: "@canvatopsd/magiclayers",
    version,
    integrity: checked.integrity,
  }),
);
