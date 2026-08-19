---
title: Getting started
description: A guide to get started by setting up Ardent.
---

## Installation

Ardent can be installed from several sources. Pick the one you're most familiar with.

### Winget

Microsoft ships [Winget](https://learn.microsoft.com/windows/package-manager/winget/) with Windows 11, Windows 10 (version 1809 and later), and Windows Server 2025. That makes it the ideal candidate to install Ardent on Windows.

```powershell
winget install --id idleberg.ardent
```

### Scoop

[Scoop](https://scoop.sh/) is a third-party package manager for Windows. Ardent can be installed after adding the official NSIS bucket.

```powershell
# One-time operation: add bucket
scoop bucket add nsis https://github.com/NSIS-Dev/scoop-nsis

# Install
scoop install nsis/ardent
```

### Homebrew

The popular package manager [Homebrew](https://brew.sh/) for macOS and Linux provides Ardent through the developer's [tap](https://docs.brew.sh/Taps).

```shell
brew install idleberg/asahi/ardent
```

### Cargo

[Cargo](https://doc.rust-lang.org/cargo/) is the package manager and build system for the Rust programming language. If you're at home in that ecosystem, you might want to use it to install Ardent.

```shell
cargo install ardent
```

### Nix

There is no official Nix package for Ardent yet, but you may install Ardent from the repository's flake.

```shell
nix profile install github:idleberg/ardent
```

### Docker

Use [Docker](https://www.docker.com/) to install or run Ardent in an isolated container.

```shell
docker run idleberg/ardent
```

### Pkgx

[Pkgx](https://pkgx.sh/) offers an elegant way to try Ardent without installing it. If you want to keep it, you can install it using [pkgm](https://github.com/pkgxdev/pkgm).

```shell
# Run once
pkgx ardent

# Install
pkgm install ardent
```

### Mise

[Mise-en-place](https://mise.jdx.dev/) is a tool that manages dev tools, env vars, and tasks per project. With [mise-ardent](https://github.com/idleberg/mise-ardent) installed, you can add the formatter to your local project.

```shell
# One-time operation: install plugin
mise plugin install ardent https://github.com/idleberg/mise-ardent

# Install ardent in your project
mise use ardent
```

## Build from source

```shell
# Clone the repository
git clone https://github.com/idleberg/ardent.git
cd ardent

# Setup tooling
mise install

# Build
mise run build
```

## CI/CD

To run formatting checks in a GitHub workflow, install the [ardent-check](https://github.com/idleberg/ardent-check) action. For more complex operations in your pipeline, you may use [setup-ardent](https://github.com/idleberg/setup-ardent) instead.
