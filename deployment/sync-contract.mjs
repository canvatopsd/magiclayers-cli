import { cp } from "node:fs/promises";
import { resolve } from "node:path";
import { unit } from "./paths.mjs";
const source = process.argv[2];
if (!source)
  throw new Error("Pass the API-owned guest-agent-commands.json explicitly");
await cp(resolve(source), resolve(unit, "contracts/guest-agent-commands.json"));
console.log("CLI public command contract selected");
