import { execFileSync } from "node:child_process";
import { resolve } from "node:path";

export const unit = resolve(import.meta.dirname, "..");
export const project = execFileSync(
  "git",
  ["-C", unit, "rev-parse", "--show-toplevel"],
  { encoding: "utf8" },
).trim();
export const build = resolve(project, ".tmp/build/cli");
