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
  - [Shell Completions](#shell-completions)
  - [Non-Interactive Configuration](#non-interactive-configuration)
- [Configuration](#configuration)
- [Contributing](#contributing)
- [License](#license)

## Key Features

- **Git Installation Check**: Verifies if Git is installed on your system.
- **Cross-Platform Guidance**: If Git is not installed, `gitup` provides the correct installation command for your OS.
- **Interactive Setup**: A guided `gitup setup` command for new users that configures name, email, and signing key, then offers to save it all as a profile.
- **Cryptographic Signing**: Associate GPG/SSH signing keys with your profiles to ensure all commits for that identity are automatically and correctly signed.
- **Effortless Profile Management**: Save multiple Git configurations and switch between them seamlessly.
- **Interactive Switching**: Simply run `gitup use` to get an interactive list of profiles to choose from.
- **Shell Completions**: Generate completion scripts for Bash, Zsh, Fish, and other shells for a faster workflow.
- **Script-Friendly**: A global `--json` flag provides machine-readable output for all commands.
- **Secure by Design**: Does not require or execute commands with `sudo` itself; it empowers the user to run provided installation commands securely.

## Installation

You will need the Rust toolchain (version 1.70 or newer) installed.

1.  **Install from Crates.io (Recommended):**
    ```sh
    cargo install gitup
    ```

2.  **Build from Source:**
    ```sh
    # Clone the repository
    git clone https://github.com/gni/gitup.git
    cd gitup

    # Build the release binary
    cargo build --release

    # The executable will be at `target/release/gitup`
    # For global access, move it to a directory in your PATH
    mv target/release/gitup /usr/local/bin/
    ```

## Usage

### Initial Setup

For first-time use, the `setup` command is the best starting point. It will guide you through setting your name, email, and an optional GPG/SSH signing key. It will then ask if you want to save the result as your first profile.

```sh
gitup setup
````

### Checking Configuration

To see your current global `user.name`, `user.email`, `user.signingkey`, and the active `gitup` profile.

```sh
gitup check
```

*Alias: `gitup status`*

### Profile Management

This is the core feature for managing multiple Git identities.

#### Switch Profiles (Easy Switch)

Run `use` without a name for an interactive selector. This is the easiest way to switch contexts.

```sh
$ gitup use

? Select a profile to use ›
  personal
❯ work
```

Or switch directly if you know the name: `gitup use personal`

#### Save a New Profile

This command reads your **current global Git configuration** (including signing key) and saves it as a named profile.

```sh
# First, ensure your global config is what you want to save
gitup set -n "Work User" -e "work.user@example.com" -s "A1B2C3D4"

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

### Shell Completions

To enable shell completions, you need to generate the script for your shell and source it in your shell's configuration file (e.g., `.bashrc`, `.zshrc`).

#### Bash

Add the following to your `~/.bashrc`:

```sh
eval "$(gitup completions bash)"
```

#### Zsh

Add the following to your `~/.zshrc`:

```sh
eval "$(gitup completions zsh)"
```

#### Fish

Add the following to your `~/.config/fish/config.fish`:

```sh
gitup completions fish | source
```

### Non-Interactive Configuration

For use in scripts or CI/CD environments.

```sh
# Set config using long flags
gitup set --name "User" --email "user@example.com" --signing-key "A1B2C3D4"

# Or with short flags
gitup set -n "User" -e "user@example.com" -s "A1B2C3D4"

# Unset a signing key by passing an empty string
gitup set -s ""

# Flags can also be populated from environment variables
export GITUP_USER_NAME="User"
export GITUP_USER_EMAIL="user@example.com"
export GITUP_SIGNING_KEY="A1B2C3D4"
gitup set
```

## Configuration

`gitup` stores its profile data in a simple JSON file located at:

  - **Linux/macOS:** `$HOME/.config/gitup/config.json`
  - **Windows:** `{FOLDERID_RoamingAppData}\gitup\config.json`

It is recommended to manage this file through the CLI commands.

## Contributing

Contributions, issues, and feature requests are welcome. Please check the [issues page](https://github.com/gni/gitup/issues) for this project.

## License

This project is licensed under the MIT License.

## Author
Lucian BLETAN