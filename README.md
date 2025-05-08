# RMLM

Make launching [RamaLama](https://github.com/containers/ramalama) boring 🚀

RamaLama is a Python-based CLI tool that may require additional Python dependencies and setup, especially on Windows. Native support on Windows can be tricky due to path and environment assumptions.

To simplify this experience, **RMLM** acts as a thin wrapper that runs RamaLama using a containerized approach with **Podman** to ensure consistency across platforms.

## 📦 Download

Head to the [Releases Page](https://github.com/benoitf/rmlm/releases) and download the binary for your platform.

### macOS

oneliner (assuming you have the jq utility) that will fetch the latest macOS binary and rename it to `rmlm`
```bash
curl -L $(curl -s https://api.github.com/repos/benoitf/rmlm/releases/latest | jq -r '.assets[] | select(.name | test("rmlm-mac-universal")) | .browser_download_url') -o rmlm-mac-universal && chmod u+x rmlm-mac-universal && mv rmlm-mac-universal rmlm
```

Option: copy the binary to `/usr/local/bin`

> **Note:** The binary is not code-signed. You can either [build it yourself](./CONTRIBUTING.md) or remove the quarantine attribute manually:

```bash
xattr -d com.apple.quarantine rmlm-mac-arm64
```


## ⚙️ Prerequisites

- A working [Podman](https://podman.io/) installation on **Windows** or **macOS**.
  - Podman should be installed and configured to run containers (e.g., rootless or with a virtual machine on macOS/Windows).

## 🚀 Getting Started

Once installed, you can use the `rmlm` command just like you would use `ramalama`:

```bash
./rmlm <RamaLama command>
```

## 🧠 Example: Run a Model (Interactive CLI)

This command launches a small LLM and gives you the familiar interactive RamaLama prompt:

```bash
rmlm run smollm:135m
🦭 >
```

## 🌐 Example: Serve a Model (Web Interface)

Start a model server and access it from your browser:

```bash
rmlm serve tinyllama | grep http://
main: server is listening on http://0.0.0.0:8080 - starting the main loop
```

Then open the displayed URL (e.g., `http://0.0.0.0:8080`) to chat with the model interactively in your browser.

## 🤝 Contributing

If you’d like to help improve RMLM, check out [CONTRIBUTING.md](./CONTRIBUTING.md) for setup instructions, development tips, and contribution guidelines.

Whether it’s fixing bugs, improving documentation, or adding new features, all kinds of contributions are welcome! 💡


## 📄 License

This project is licensed under the [Apache-2 Liense](./LICENSE).

## 💬 Questions / Feedback?

Open an [issue](https://github.com/benoitf/rmlm/issues) or start a discussion
