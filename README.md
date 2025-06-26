# SysEcho 🚀

**SysEcho** is a terminal-based system summary and monitoring tool for **Linux (x86_64 only)**, written in **Rust**. It fetches key system metrics, displays Docker container stats, and provides a color-enhanced overview of your machine — all from a single binary.

---

## 🛠️ Features

- System Summary:
  - Hostname, user, kernel version
  - Uptime, CPU info, memory, disk
  - Battery status (if available)
  - Network interfaces
- Docker Container Stats:
  - Container ID, name, CPU usage, memory usage
- Colorful, clean terminal output
- Optional config support via `TOML`
- CLI argument parsing using `clap`

---

## 🔧 Crates & Technologies Used

| Crate        | Purpose                          |
|--------------|----------------------------------|
| `sysinfo`    | Core system info (CPU, memory, disk) |
| `whoami`     | Fetch current user info          |
| `battery`    | Battery status (if supported)    |
| `get_if_addrs` | Network interface details      |
| `duct`       | Run shell commands (e.g. Docker) |
| `colored`    | Terminal color formatting        |
| `console`    | Terminal cursor, styling         |
| `tui-logger` | Logging utilities for TUI/logs   |
| `serde` + `toml` | Config file parsing (TOML format) |
| `clap`       | CLI argument parsing             |

---

## 📁 Repository Structure

```
SysEcho/
├── src/                     # Rust source files
│   ├── main.rs
│   └── modules/
|    ├── battery.rs
|    ├── containers.rs
|    ├── hostinfo.rs
|    ├── mod.rs
|    ├── network.rs
|    └── systeminfo.rs
├── dist/                     # Precompiled binaries
│   └── sysecho-linux-x86_64
├── config.toml (optional)   # Sample config
├── Cargo.toml               # Project metadata
└── README.md                # You're here
```

---

##  Prerequisites

- You are on **Linux x86_64**
- You have downloaded the `SysEcho` repo or just the binary
- The binary is located at `dist/sysecho-linux-x86_64`

---

##  1. Make the Binary Executable

```bash
chmod +x dist/sysecho-linux-x86_64
```

---

##  2. Run It Directly

```bash
./dist/sysecho-linux-x86_64
```

You should now see system stats and Docker container information.

---

## 🛠️ 3. (Optional) Install the Binary

###  If you have root access:

```bash
sudo mv dist/sysecho-linux-x86_64 /usr/local/bin/sysecho
```

Now you can run it anywhere:

```bash
sysecho
```

---

### If you don’t have root access:

1. Move the binary:
   ```bash
   mkdir -p ~/.local/bin
   mv dist/sysecho-linux-x86_64 ~/.local/bin/sysecho
   ```

2. Add to your PATH (if not already):
   ```bash
   echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
   source ~/.bashrc
   ```

3. Run:
   ```bash
   sysecho
   ```

---


## 🐳 Docker Access

To use Docker container monitoring:

```bash
sudo usermod -aG docker $USER
newgrp docker
```

Docker must be installed and running. Otherwise, container stats will be skipped gracefully.

---

## 🏗️ Build from Source (Linux Only)

### 📦 Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 🔨 Build

```bash
cargo build --release
```

Optional: strip debug symbols and prepare for distribution:

```bash
strip target/release/sysecho
cp target/release/sysecho bin/sysecho-linux-x86_64
```

---

