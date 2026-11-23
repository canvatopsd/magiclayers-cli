# MagicLayers Agent CLI

`magiclayers` submits an image, returns hosted checkout, reads task/payment status and downloads verified delivery. It uses private per-task access without registration or an account session. Check the installed version with `magiclayers --version`.

## Install

Install Node.js 22 or newer, then run `npm install -g @canvatopsd/magiclayers`. The npm package contains native executables and the Agent Skill; it does not compile or download executables during installation. Supported: macOS Apple Silicon and Linux x64 with glibc 2.31 or newer. Rust is not required. macOS Intel, Linux ARM64 and Windows are not currently included.

```sh
magiclayers --version
magiclayers doctor
magiclayers skill install agents
magiclayers skill status agents
```

The Skill also supports `codex`, `claude-code`, `cursor` and `copilot`; `--dir` selects a custom parent skills directory. Codex respects `CODEX_HOME`. Installation accepts identical content and refuses to overwrite different instructions or symlinks. Review local edits before removing/replacing an existing Skill. Start a new agent session after installation. `skill show` returns the embedded instructions offline.

The default service is `https://magiclayers.app` (prod). Select local explicitly with `--base-url http://127.0.0.1:3000` or `MAGICLAYERS_BASE_URL`. Doctor reads configuration; it does not prove payment or delivery. The npm package is `@canvatopsd/magiclayers`; its command and Skill name are both `magiclayers`. The selected Web release is published at `/agent/release.json`.

## Convert, pay, inspect, download

```sh
magiclayers convert ./image.png --email buyer@example.com --dry-run
magiclayers convert ./image.png --email buyer@example.com
# Use the task ID returned by convert:
magiclayers checkout TASK_ID
# After the buyer completes checkout:
magiclayers status TASK_ID
magiclayers download TASK_ID --output ./delivery.zip
```

Input is PNG/JPEG/WebP, up to 30 MiB. Add `--print` to both conversion commands for automatic spot separations; otherwise the result is editable design layers. `--locale` defaults to `en-US`. Downloads default to the delivery package; use `--kind psd` or `--kind preview` for a single file. Business options always use full names such as `--email` and `--output`.

Create freezes the server quote and saves private access before sending. It does not charge. Checkout defaults to Creem and returns a hosted link for the buyer. Read verified financial facts with `magiclayers payment TASK_ID`. A redirect or browser success message alone is not proof of payment.

For PayPal:

```sh
magiclayers checkout TASK_ID --provider paypal
# Only after buyer approval:
magiclayers verify-payment TASK_ID --provider paypal
```

PayPal confirmation can capture the saved order after approval. Creem verification uses `magiclayers verify-payment TASK_ID --checkout-id CHECKOUT_ID` and queues verification (HTTP 202). `--checkout-id` belongs only to Creem. Neither channel provides a refund command. Keep the original provider once checkout exists.

## Resume and protect access

Keep the returned UUIDv7 (compact or original form). After an uncertain submission, inspect that ID first. If resubmission is necessary, run `convert` with `--task-id TASK_ID` and the **same image, email, locale and workflow**. Never create a replacement task or payment merely because status is delayed. There is no automatic polling or retry.

Access files live under the OS local application-data directory in `magiclayers/tasks/<uuid>.json`, overridable by `MAGICLAYERS_ACCESS_DIRECTORY`. On macOS: `~/Library/Application Support/magiclayers/tasks/`. Unix directories/files use 0700/0600. Back them up privately; restore the original file on another machine and use the same service origin. Missing access is an explicit error. Do not share access keys, private task links or signed download URLs.

## Print output

New `--print` tasks automatically extract up to twelve inks at 300 DPI on white, preserving source pixel dimensions. Transparent input is composited onto white. The ZIP contains continuous-tone TIFF/PNG plates, an RGB layered PSD and proof. Halftoning, trapping, underbase and native Photoshop spot channels remain downstream. No manual palette is needed. Only retries of existing explicitly configured tasks need their saved `--print-settings` file together with `--print`.

## Output and discovery

Business commands return `{"ok":true,"data":...}` or `{"ok":false,"error":{"code":"...","message":"..."}}`. Argument errors exit 2; command failures exit 1. API failures retain HTTP status and code. Help/version are plain text. JSON is always used for business output; no `--json` flag is required or accepted.

`magiclayers discover` returns the offline API-owned [command inventory](../contracts/guest-agent-commands.json). Use the specific CLI commands for business actions; generic `request`, `read` aliases and the earlier `task` command layer are not exposed.

Downloads verify byte count and SHA-256 and never overwrite an existing output. The service checks current payment when download links are used; refunds/disputes revoke access. Transport errors do not print signed download URLs. Local payment uses test credentials; prod uses live payment. Local checks do not imply real payment acceptance.

## Source installation

The optional source installer at `https://magiclayers.app/agent/install.sh` requires Rust/Cargo with edition 2024 support and a native linker. Inspect the downloaded script before running it. It verifies the fixed-version source archive, builds locked dependencies and installs into `~/.local/share/magiclayers/cli/<version>/`. Add `~/.local/bin` to PATH. Source installation and npm installation are alternatives; do not install both into the same command directory.
