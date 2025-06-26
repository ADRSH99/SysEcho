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
- Single binary, installable via `install.sh`
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
│   └── ...
├── bin/                     # Precompiled binaries
│   └── sysecho-linux-x86_64
├── install.sh               # Install script
├── config.toml (optional)   # Sample config
├── Cargo.toml               # Project metadata
└── README.md                # You're here
```

---

## ⚙️ Installation

### 🔹 1. Clone the Repository

```bash
git clone https://github.com/ADRSH99/SysEcho.git
cd SysEcho
```

### 🔹 2. Install the Precompiled Binary

```bash
chmod +x install.sh
./install.sh
```

- Installs to `/usr/local/bin` (if root)
- Installs to `~/.local/bin` (if non-root)

Make sure `~/.local/bin` is in your `$PATH`.

---

## 💻 System Commands Used

SysEcho invokes these under the hood:

| Command | Description |
|---------|-------------|
| `uptime` | System uptime and load |
| `df -h` | Disk usage |
| `free -h` | Memory usage |
| `docker stats --no-stream` | Docker container stats |
| `uname`, `hostname`, `whoami` | System ID info |

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

## 📄 Configuration (Optional)

Create a `config.toml` in the working directory to override display preferences, logging, or sections. *(Feature in progress)*

---

## 🔚 Notes

- Currently only built and tested for Linux x86_64
- Binary size can be reduced with `strip`
- Future support: TUI mode, logging to file, sorting containers

---

Feel free to fork, improve, and open issues or PRs!
