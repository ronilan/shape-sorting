# Shape Sorting

A shape sorting game built on the [Incredible](https://www.incredible.rs) framework.

Move shapes from the shape box into the drop boxes and sort them by **shape** or **color**. Play with the keyboard (🔑), the mouse (🐭), or both (🔑 & 🐭) — and find out which is faster!

## Overview

The game opens on a splash screen where you pick an input mode, runs on the board while you sort, and shows your time when the shape box is emptied. Your fastest game and total time wasted are saved between runs.

## Features

- **Keyboard, Mouse, or Both** — pick your input mode on the splash screen
- **Sort by shape or by color** — two ways to empty the box
- **Stats between runs** — fastest game and total time wasted are persisted
- **Cross-platform** — a terminal app, a native GUI app on macOS and Windows, and a browser version (WASM)

## Installation

Pre-built binaries are provided for each [release](https://github.com/ronilan/shape_sorting/releases).

### Quick install (script)

One-liners, in the style of rustup's installer. The scripts fetch the right binary for your platform from the latest release and place it on the PATH automatically. No clone or build required.

**macOS / Linux** — installs into `/usr/local/bin` (asks for your password for `sudo`):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/ronilan/shape_sorting/main/install.sh | bash
```

**Windows** — installs into `C:\Program Files\shape_sorting` and adds that folder to the system PATH (the script self-elevates with a UAC prompt):

```powershell
irm https://raw.githubusercontent.com/ronilan/shape_sorting/main/install.ps1 | iex
```

If you prefer to look before you run, download first, inspect, then execute:

**macOS / Linux:**

```bash
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/ronilan/shape_sorting/main/install.sh -o install.sh
bash install.sh
```

**Windows:**

```powershell
Invoke-WebRequest -Uri "https://raw.githubusercontent.com/ronilan/shape_sorting/main/install.ps1" -OutFile install.ps1
powershell -ExecutionPolicy Bypass -File install.ps1
```

### Manual install

**macOS / Linux** — download the binary for your platform from the [latest release](https://github.com/ronilan/shape_sorting/releases), then move it to `/usr/local/bin` (a centralized folder on the default PATH) and give it execution permissions:

```bash
sudo mv shape_sorting /usr/local/bin/
sudo chmod +x /usr/local/bin/shape_sorting
```

**Windows** — download `shape_sorting-terminal-windows.zip` from the [latest release](https://github.com/ronilan/shape_sorting/releases), create a dedicated folder (e.g. `C:\Program Files\shape_sorting\`), place `shape_sorting.exe` inside it, then search Windows for "Environment Variables", edit the system variables, and append that folder to the system PATH.

Verify it works by opening a new terminal anywhere and typing `shape_sorting`. If the program responds, it is correctly placed.

### Uninstall

One-liners, mirroring the quick install. The scripts remove the installed binary; the Windows script also removes the (now-empty) install folder and its PATH entry.

**macOS / Linux:**

```bash
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/ronilan/shape_sorting/main/uninstall.sh | bash
```

**Windows:**

```powershell
irm https://raw.githubusercontent.com/ronilan/shape_sorting/main/uninstall.ps1 | iex
```

Both scripts resolve the binary name from the latest release. If that lookup fails (e.g. the release is gone), pass the name explicitly: `bash uninstall.sh <binary-name>` / `uninstall.ps1 -BinName <name>`.

## Your stats

Your fastest game and total time wasted are saved between runs in a per-user file:

- **macOS:** `~/Library/Application Support/shape_sorting/.shape_sorting`
- **Linux:** `~/.local/share/shape_sorting/.shape_sorting`
- **Windows:** `%APPDATA%\shape_sorting\.shape_sorting`

The web version stores your stats in the browser's local storage. If the file can't be written (or the browser blocks storage), the game simply runs without saving — nothing crashes.

## Development

See [Development](./markdowns/DEVELOPMENT.md) and [Development Environment Prerequisites](./markdowns/DEVELOPMENT_PREREQUISITES.md)

---

*Fabriqué au Canada : Made in Canada 🇨🇦*