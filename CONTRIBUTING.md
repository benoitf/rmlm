# Contributing 😄

Thanks for considering contributing to **rmlm**! Your help makes the project better. Below are the steps to get you set up and contributing smoothly. 🚀


## 🛠️ Prerequisites

Make sure Rust is installed on your system. You can get it here:

👉 [Rust Installation Guide](https://www.rust-lang.org/fr/tools/install)

## 📥 Clone the Project

```bash
git clone https://github.com/benoitf/rmlm
```

Then move into the project directory:

```bash
cd rmlm
```

## ▶️ Run the Tool in Development Mode

Use this command to run the tool:

```bash
cargo run <RamaLama command>
```

## ✨ Add Linter and Formatter Tools

To ensure your code is clean and tidy, install these Rust components:

```bash
rustup component add clippy
rustup component add rustfmt
```

## ✅ Check Formatting, Linting, and Tests

Check code formatting:

```bash
make fmt-check
```

Check code linting:

```bash
make lint
```

Run the test suite:

```bash
make test
```

✅ Please make sure all checks pass before opening a pull request!

## 🍏 macOS Configuration for Cross-Compilation

To build for different targets on macOS:

```bash
brew install filosottile/musl-cross/musl-cross
brew install mingw-w64
rustup target add x86_64-apple-darwin
rustup target add aarch64-unknown-linux-musl
rustup target add x86_64-pc-windows-gnu
rustup target add x86_64-unknown-linux-musl
```

## 🙌 You're All Set!

Feel free to open issues, propose features, or submit pull requests. Every bit of help counts 🎉
