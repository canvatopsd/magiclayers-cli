import { homedir } from "node:os";
import { execFileSync } from "node:child_process";
import { resolve } from "node:path";
import { project as root, unit } from "./paths.mjs";

export function buildLinuxBinary() {
  const toolchain =
    process.env.MAGICLAYERS_RUST_TOOLCHAIN ??
    resolve(homedir(), ".rustup/toolchains/1.97.1-aarch64-apple-darwin");
  const environment = {
    ...process.env,
    PATH: `${resolve(root, ".tmp/cache/cross-tools/bin")}:${resolve(toolchain, "bin")}:${process.env.PATH}`,
    CARGO_HOME: resolve(root, ".tmp/cache/cargo-linux"),
    CARGO_TARGET_DIR: resolve(root, ".tmp/build/cli/linux-target"),
    RUSTC: resolve(toolchain, "bin/rustc"),
    RUSTFLAGS: `--sysroot=${resolve(root, ".tmp/cache/linux-sysroot")}`,
    ZIG_GLOBAL_CACHE_DIR: resolve(root, ".tmp/cache/zig-global"),
    ZIG_LOCAL_CACHE_DIR: resolve(root, ".tmp/cache/zig-local/cli"),
  };
  execFileSync(
    resolve(root, ".tmp/cache/cross-tools/bin/cargo-zigbuild"),
    [
      "build",
      "--manifest-path",
      resolve(unit, "Cargo.toml"),
      "--release",
      "--locked",
      "--target",
      "x86_64-unknown-linux-gnu.2.31",
    ],
    { cwd: root, env: environment, stdio: "inherit" },
  );

  return resolve(
    environment.CARGO_TARGET_DIR,
    "x86_64-unknown-linux-gnu/release/magiclayers",
  );
}
