import { readFile } from "node:fs/promises";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";

const manifest = await readFile(
  resolve(import.meta.dirname, "../Cargo.toml"),
  "utf8",
);
const match = manifest.match(/^version = "(\d+\.\d+\.\d+)"$/m);
if (!match)
  throw new Error("CLI Cargo.toml must declare a major.minor.patch version");
export const version = match[1];
export const archive = `magiclayers-${version}-source.tar.gz`;
if (
  process.argv[1] &&
  import.meta.url === pathToFileURL(resolve(process.argv[1])).href
)
  console.log(version);
