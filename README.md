# MagicLayers CLI

Convert an image into editable PSD layers or automatic spot-color print separations from your AI agent or terminal. Submit an image, share hosted checkout, inspect progress and download verified delivery.

```sh
npm install -g @canvatopsd/magiclayers
magiclayers --version
magiclayers skill install agents
```

Node.js 22 or newer. Native executables are included for macOS Apple Silicon and Linux x64 (glibc 2.31+); no Rust, Photoshop, account registration or API key is required. Other platforms are not included in this npm release.

```sh
magiclayers convert image.png --email buyer@example.com
magiclayers checkout TASK_ID
# After payment is verified:
magiclayers status TASK_ID
magiclayers download TASK_ID --output delivery.zip
```

Add `--print` to `convert` for automatic spot-color separations. Business output is JSON. Use full option names, keep the same task ID when resuming, and back up the private access file. The CLI never automatically polls, retries, creates a replacement payment or overwrites a downloaded file.

Install the bundled Skill for `codex`, `claude-code`, `cursor`, `copilot` or generic `agents`. Use `--dir` for a custom parent directory and start a new Agent session after installation.

[Usage and recovery](doc/usage.md) · [Agent Skill](skills/magiclayers/SKILL.md) · [Release guide](doc/npm-release.md) · [Website](https://magiclayers.app/ai-agent/)

The hosted service defaults to `https://magiclayers.app`. Local development selects `--base-url http://127.0.0.1:3000` explicitly. Buyers complete hosted checkout before processing starts. PayPal confirmation is explicit after buyer approval. Delivery size and SHA-256 are verified.
