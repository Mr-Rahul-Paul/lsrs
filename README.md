## lsrs — A modern, Rust-powered directory lister

`lsrs` is a fast, ergonomic, and extensible CLI that lists files and folders with a clean, colorized table output. It aims to be a pleasant alternative to `ls`, with a focus on readability and developer-friendly extensibility.

### What’s improved
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
- Colorized, tabular layout.

> Note: "modified" time is currently a placeholder. Roadmap below includes adding real timestamps and sorting.

---

## Installation and setup

The following steps install `lsrs` and make it available from anywhere on your machine. These match the steps captured in `changelog.md`.

### 1) Install from source
```bash
cargo install --path /home/rahul/Desktop/Rust/lsrs --force
```

### 2) Ensure Cargo bin directory is on your PATH (one-time)
```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### 3) Verify installation
```bash
which lsrs
lsrs .
```

That’s it—open a new terminal anytime and use `lsrs` globally.

> Prefer to make it act as `ls`? The safest way is an alias:
> ```bash
> echo "alias ls='lsrs'" >> ~/.bashrc && source ~/.bashrc
> # Use \ls or /bin/ls to call the original when needed
> ```

---

## Usage

Basic usage:
```bash
lsrs            # list current directory
lsrs <path>     # list a specific path
```

Examples:
```bash
lsrs
lsrs src/
lsrs /var/log
```

Exit codes follow standard conventions; errors are printed in red with helpful messages.

---

## Development

Build and run locally:
```bash
cargo build
cargo run -- .
```

Recommended tooling: `rustup`, `cargo`, and `rust-analyzer`.

---

## Roadmap / planned improvements
- Show actual last modified timestamps.
- Flags similar to `ls` (e.g., `-a` for hidden, `-l` for long).
- Sorting (by name, size, time) and reverse order.
- Human-readable sizes (e.g., KiB/MiB) and alignment tweaks.
- Optional recursion (`-R`) and depth limits.
- Error handling/diagnostics polish and richer exit codes.

If you have ideas, feel free to open an issue or submit a PR.

---

## Notes
- `lsrs` intentionally does not override your system `ls`. You can opt-in with an alias or a PATH-preferred symlink if you want it to act as `ls`.
- The table style/colors are tuned for readability; feel free to adjust to your preferences.


