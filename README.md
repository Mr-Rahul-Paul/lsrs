## lsrs — A modern, Rust-powered directory lister

`lsrs` is a fast and extensible CLI that lists files and folders with a clean, colorized table output. It aims to be a alternative to `ls`, with a focus on readability and developer-friendly extensibility.

### What is this? What’s improved
- **Clear, readable output**: Neat table layout powered by `tabled` with column colors for quick scanning.
- **Consistent cross-platform behavior**: Built with Rust; no GNU/BSD quirks to memorize.
- **Simple, explicit usage**: Call it as `lsrs` (does not override your system `ls` by default).
- **Extensible foundation**: The codebase uses `clap` for arguments and is structured for adding flags and behaviors.

### Core functionality (current)
- Lists entries for a given path (default: current directory).
- Displays:
  - **Name**
  - **Type** (File/Dir)
  - **Size (bytes)** for files; directories show `0` for now

> Note: "modified" time is currently a placeholder. Functionality will be added sooner or later

---

## Installation and setup

### 1) Install from source
```bash
cargo install --path <LOCATION OF FILE> --force
```

### 2) Ensure Cargo bin directory is on your PATH (one-time)
```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### 3) Verify 
```bash
which lsrs
lsrs .
```

> Prefer to make it act as `ls`? The safest way is an alias:
> ```bash
> echo "alias ls='lsrs'" >> ~/.bashrc && source ~/.bashrc
> ```
<!-- > # Use \ls or /bin/ls to call the original when needed -->

---

## Usage

Basic usage:
```bash
lsrs <path>     # list a specific path
```

Exit codes follow standard conventions; errors are printed in red with helpful messages.

---

## Development

Build and run locally:
```bash
cargo build
cargo run -- .
```



