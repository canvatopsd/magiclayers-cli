---
name: magiclayers
description: Convert images into editable PSD layers or automatic spot-color print separations with the MagicLayers CLI; submit tasks, share hosted checkout, inspect progress and download verified delivery.
---

## Setup

Use the installed `magiclayers` command. Check `--version`, `discover` and `doctor`. If missing, install Node.js 22 or newer and run `npm install -g @canvatopsd/magiclayers`. The package includes native executables for macOS Apple Silicon and Linux x64 (glibc 2.31 or newer); Rust is not required. Other platforms are not supported by this npm release. Install this bundled Skill with `magiclayers skill install agents` or a supported agent name; `--dir` selects a parent skills directory. Do not overwrite different local Skill instructions. Start a new agent session after installation.

The default service is prod, `https://magiclayers.app`. Select local explicitly with `--base-url http://127.0.0.1:3000`. No account login or API key is required. Business output is always JSON (`ok/data` or `ok/error`); errors exit nonzero. Use full option names. There is no `--json`, generic `request`, `read` alias or `task` command prefix.

## Image workflow

Get the user's source image, delivery email and choice of editable design layers or print separations. Email is an auxiliary delivery/recovery contact; payment, task status and downloads are available directly through CLI commands without an email confirmation step.

```sh
magiclayers convert ./image.png --email buyer@example.com --dry-run
magiclayers convert ./image.png --email buyer@example.com
magiclayers checkout TASK_ID
# After buyer payment:
magiclayers status TASK_ID
magiclayers download TASK_ID --output ./delivery.zip
```

Dry-run validates local arguments only and does not submit. Conversion freezes a quote and saves private access before sending; it does not charge. Share the hosted checkout link with the buyer. `payment TASK_ID` reads verified financial facts. For Creem, `verify-payment TASK_ID --checkout-id CHECKOUT_ID` queues provider verification. For PayPal, use `checkout TASK_ID --provider paypal`; only after buyer approval, run `verify-payment TASK_ID --provider paypal`, which can capture the saved order. Do not treat a redirect or HTTP 200 alone as proof of paid delivery. No refund command exists.

Add `--print` to both convert commands for print separations. New tasks automatically extract up to twelve inks at 300 DPI on white, retaining pixel dimensions. They need no palette/settings file. The ZIP includes continuous-tone TIFF/PNG plates, RGB layered PSD and proof; halftoning, trapping, underbase and native spot channels remain downstream. Use `--print-settings` only with an existing task's original explicit setup. Do not route print work through Canva.

## Resume and download

Preserve the returned task ID. On uncertain submission, inspect that same ID, then retry only if needed with `convert --task-id TASK_ID` and identical image/email/locale/workflow. Never create duplicate tasks or payments for recovery. Status checks are explicit; do not start automatic polling.

Private access is saved in the OS local application-data directory under `magiclayers/tasks/`, overridable with `MAGICLAYERS_ACCESS_DIRECTORY`. Missing access requires restoring the original private file, not a new payment. Do not print access keys or share private task/signed download links. Downloads verify size/SHA-256 and never overwrite output. Use `--kind psd` or `--kind preview` when needed.

Detailed installation, payment and recovery reference: https://magiclayers.app/agent/doc/usage.md.
