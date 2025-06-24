# Gitup 🚀

**Manage your Git identity across multiple accounts and projects.**

[![Crates.io](https://img.shields.io/crates/v/gitup.svg)](https://crates.io/crates/gitup)

`gitup` is cross-platform CLI tool designed to check, install, and manage your Git configurations with ease. The **profile management**, allows to switch between work, personal, and other Git accounts with a single command.


## Table of Contents

- [Key Features](#key-features)
- [Installation](#installation)
- [Usage](#usage)
  - [Initial Setup](#initial-setup)
  - [Checking Configuration](#checking-configuration)
  - [Profile Management](#profile-management)
  - [Non-Interactive Configuration](#non-interactive-configuration)
- [Configuration](#configuration)
- [Contributing](#contributing)
- [License](#license)

## Key Features

- **Git Installation Check**: Verifies if Git is installed on your system.
- **Cross-Platform Guidance**: If Git is not installed, `gitup` provides the correct installation command for your OS (supports Debian/Ubuntu, Fedora/RHEL, Arch Linux, macOS, and Windows).
- **Interactive Setup**: A guided `gitup setup` command for new users that also offers to save the configuration as a profile.
- **Effortless Profile Management**: Save multiple Git configurations (e.g., for 'work' and 'personal' use) and switch between them seamlessly.
- **Interactive Switching**: Simply run `gitup use` to get an interactive list of profiles to choose from.
- **Script-Friendly**: A global `--json` flag provides machine-readable output for all commands, and configuration can be set via environment variables.
- **Secure by Design**: Does not require or execute commands with `sudo` itself; it empowers the user to run provided installation commands securely.

## Installation

You will need the Rust toolchain (version 1.65 or newer) installed.

1.  **Install from Crates.io (Recommended):**
    ```sh
    cargo install gitup
    ```

2.  **Build from Source:**
    ```sh
    # Clone the repository
    git clone [https://github.com/gni/gitup.git](https://github.com/gni/gitup.git)
    cd gitup

    # Build the release binary
    cargo build --release

    # The executable will be at `target/release/gitup`
    # Consider moving it to a directory in your PATH
    mv target/release/gitup /usr/local/bin/
    ```

## Usage

### Initial Setup

For first-time use, the `setup` command is the best starting point. It will guide you through setting your name and email and then ask if you want to save it as your first profile.

```sh
gitup setup
````

### Checking Configuration

To see your current global `user.name`/`user.email` and the active `gitup` profile.

```sh
gitup check
```

*Alias: `gitup status`*

### Profile Management

This is the core feature for managing multiple Git identities.

#### Switch Profiles (Easy Switch)

Run `use` without a name to get an interactive selector. This is the easiest way to switch contexts.

```sh
$ gitup use

? Select a profile to use ›
  personal
❯ work
```

Or switch directly if you know the name:

```sh
gitup use personal
```

#### Save a New Profile

This command reads your **current global Git configuration** and saves it as a named profile.

```sh
# First, ensure your global config is what you want to save
git config --global user.name "Work User"
git config --global user.email "work.user@example.com"

# Then, save it
gitup save work
```

#### List All Saved Profiles

```sh
gitup list
```

*Alias: `gitup ls`*

You will see a list of your profiles, with the active one highlighted.

```
Saved Profiles
  - personal
  - work (active)
```

#### Show the Active Profile

```sh
gitup current
```

*Alias: `gitup active`*

#### Delete a Profile

```sh
gitup delete work
```

*Alias: `gitup rm`*

### Non-Interactive Configuration

For use in scripts or CI/CD environments.

```sh
# Set config using long flags
gitup set --name "Your Name" --email "your.email@example.com"

# Or with short flags
gitup set -n "Your Name" -e "your.email@example.com"

# Flags can also be populated from environment variables
export GITUP_USER_NAME="Your Name"
export GITUP_USER_EMAIL="your.email@example.com"
gitup set
```

## Configuration

`gitup` stores its profile data in a simple JSON file located at:

  - **Linux/macOS:** `$HOME/.config/gitup/config.json`
  - **Windows:** `{FOLDERID_RoamingAppData}\gitup\config.json`

You can view this file to see all your saved profiles, but it is recommended to manage it through the CLI commands.

## Author
Lucian BLETAN