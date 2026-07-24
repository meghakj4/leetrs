
# 🚀 leetrs

[![Rust](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org)
[![Crates.io](https://img.shields.io/crates/v/leetrs.svg)](https://crates.io/crates/leetrs)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Status](https://img.shields.io/badge/status-beta-orange.svg)]()
[![Neovim](https://img.shields.io/badge/Neovim-0.9%2B-green.svg)](https://neovim.io/)

**leetrs** is a blazing-fast, Rust-powered CLI engine that makes solving LeetCode problems from the terminal a first-class developer experience.

Built specifically for developers who live in the terminal and rely on **Neovim**, `leetrs` strips away the distraction of the browser. It handles intelligent authentication, Markdown problem generation, native editor window splitting, and asynchronous code submission without ever leaving your workflow.

https://github.com/user-attachments/assets/86783e7e-afc6-449a-828b-c29e34fa9dbb

---

## ✨ Features

* **Interactive TUI Browser (`leetrs tui` / `leetrs`)**
  * Search problems instantly using fuzzy matching.
  * Filter list by difficulty level (Easy, Medium, Hard) or topic overlays.
  * Visual indications of solved status, subscription locks, and premium gates.
  * Easily toggle help popups or open selected problems directly in your system browser.
* **Intelligent Authentication (`leetrs auth`)**
  * Automatically extracts `LEETCODE_SESSION` and `csrftoken` cookies from your active Chrome or Firefox sessions.
  * Secure, hidden manual fallback for containerized browser profiles.
* **Frictionless Problem Fetching (`leetrs pick`)**
  * Fetch problems using their URL slug (e.g., `two-sum`) or standard numerical ID (e.g., `1`).
  * Automatically parses LeetCode's raw HTML into clean, readable **Markdown**.
  * Generates idiomatic `snake_case.rs` files containing the exact boilerplate required.
* **Native Neovim Integration**
  * Instantly hijacks the terminal process to launch Neovim.
  * Forces a clean vertical split (`vsplit`) to place your problem description and code side-by-side, bypassing layout quirks from custom dashboards.
* **Async Submission & Test Engine (`leetrs submit` / `leetrs test`)**
  * Submit your local file directly to LeetCode's execution servers.
  * Run code against sample test cases locally without submitting officially.
  * Color-coded terminal output for execution results, including Runtime/Memory statistics and detailed compiler error logs.

## 🔧 Prerequisites (all methods)

| Requirement | Version | Notes |
|---|---|---|
| [Neovim](https://neovim.io/) | 0.9+ | Must be available as `nvim` in `$PATH` |
| LeetCode account | — | Required for auth & submission |
| Chrome or Firefox | Any | Used for automatic cookie extraction |

---

## 🛠️ Installation

Choose the method that best fits your setup.

### Option 1 — `cargo install` (Recommended for Rust users)

Requires [Rust & Cargo](https://rustup.rs/) to be installed.

```bash
cargo install leetrs
```

The binary will be placed in `~/.cargo/bin/`. Make sure that directory is in your `$PATH`.

---

### Option 2 — Homebrew (macOS & Linux)

```bash
brew install shadowmkj/tap/leetrs
```

> **Note:** If the tap isn't published yet, use one of the other methods below while it is being set up.

---

### Option 3 — `curl` Installer (Quickstart, no Rust required)

The installer script downloads the appropriate pre-built binary for your platform and places it in `/usr/local/bin`.

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/shadowmkj/leetrs/releases/download/v1.0.18/leetrs-installer.sh | sh
```

---

### Option 4 — Build from Source

Use this if you want to contribute, run unreleased features, or modify the code yourself.

#### Prerequisites

* [Rust & Cargo](https://rustup.rs/) (stable toolchain)
* [Neovim](https://neovim.io/) accessible as `nvim` in your `$PATH`
* A valid [LeetCode](https://leetcode.com) account

#### Steps

```bash
# 1. Clone the repository
git clone https://github.com/shadowmkj/leetrs.git
cd leetrs

# 2. Build the optimised release binary
cargo build --release

# 3. Move the binary somewhere on your PATH
cp target/release/leetrs ~/.cargo/bin/
# — or —
sudo cp target/release/leetrs /usr/local/bin/
```

Verify the installation:

```bash
leetrs --version
```

---
## ⚙️ Configuration

The `leetrs` configuration file uses the **TOML** format and allows you to customize your workflow.

### Config File Location

Depending on your operating system, the configuration file (`config.toml`) is stored in the standard user configuration directory:

* **Linux / macOS:** `~/.config/leetrs/config.toml`
* **Windows:** `%APPDATA%\leetrs\config.toml` (e.g., `C:\Users\<Username>\AppData\Roaming\leetrs\config.toml`)

### Configuration Options

* **`editor`** (string, default: `"nvim"`): The editor command to launch when picking a problem. 
  * Use `"nvim"` or `"vim"` for a side-by-side terminal vertical split of description and code.
  * Use `"code"` to open the files in Visual Studio Code (ensure the `code` CLI tool is in your `$PATH`).
* **`language`** (string, default: `"python3"`): Default programming language (e.g., `"rust"`, `"python3"`, `"pythondata"`, `"mysql"`, `"postgresql"`).
* **`show_description`** (boolean, default: `true`): Whether to open the problem description alongside the code template.

### Example Configuration

```toml
editor = "code"
language = "rust"
show_description = true
```

## ⚡ Quick Start

```bash
# 1. Authenticate with your LeetCode session
leetrs auth

# 2. Launch the interactive TUI problem browser
leetrs tui
# or simply:
leetrs

# 3. (Optional) Directly pick a problem by slug or numeric ID
leetrs pick two-sum
leetrs pick 1

# 4. Open it in Neovim — the problem description opens in a vertical split automatically

# 5. Solve the problem, then test or submit it
leetrs test two_sum.rs
leetrs submit two_sum.rs
```



## 🤝 Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on reporting bugs, suggesting features, and submitting pull requests.

---

## 📄 License

MIT © [shadowmkj](https://github.com/shadowmkj)
