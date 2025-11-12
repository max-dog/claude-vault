# Troubleshooting Guide

Common issues and solutions for `claude-vault`.

## Table of Contents

- [Installation Issues](#installation-issues)
- [Profile Management Issues](#profile-management-issues)
- [Detection Issues](#detection-issues)
- [Keychain Issues](#keychain-issues)
- [Execution Issues](#execution-issues)
- [Environment Issues](#environment-issues)

## Installation Issues

### Issue: `cargo: command not found`

**Problem:** Rust toolchain is not installed.

**Solution:**
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Restart your shell or source cargo
source $HOME/.cargo/env

# Verify installation
cargo --version
```

### Issue: Build fails with compilation errors

**Problem:** Incompatible Rust version or missing dependencies.

**Solution:**
```bash
# Update Rust to latest stable
rustup update stable

# Clean and rebuild
cd claude-vault
cargo clean
cargo build --release
```

### Issue: Permission denied when moving to `/usr/local/bin/`

**Problem:** Insufficient permissions to write to system directory.

**Solution:**
```bash
# Option 1: Use sudo
sudo mv target/release/claude-vault /usr/local/bin/

# Option 2: Use user-local directory
mkdir -p ~/.local/bin
mv target/release/claude-vault ~/.local/bin/
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

## Profile Management Issues

### Issue: `Profile 'name' not found`

**Problem:** Trying to use a profile that doesn't exist.

**Solution:**
```bash
# List all profiles to see what exists
claude-vault list

# Add the missing profile
claude-vault add name

# Verify it was created
claude-vault show name
```

### Issue: `Profile 'name' already exists`

**Problem:** Trying to add a profile that already exists.

**Solution:**
```bash
# Remove the existing profile first
claude-vault remove name

# Then add it again
claude-vault add name

# Or just use the existing profile
claude-vault show name
```

### Issue: Cannot remove profile

**Problem:** Profile removal confirmation not working.

**Solution:**
```bash
# Skip confirmation with --yes flag
claude-vault remove name --yes

# Or ensure your terminal supports interactive input
# Try in a different terminal or shell
```

### Issue: Lost API key after adding profile

**Problem:** API key not stored in keychain.

**Solution:**
```bash
# Check if profile exists in config
claude-vault show name

# If it exists, keychain storage might have failed
# Re-add the profile
claude-vault remove name --yes
claude-vault add name
# Make sure to enter the API key correctly
```

## Detection Issues

### Issue: `No profile detected and no default profile set`

**Problem:** No `.claude-profile` file found and no default profile configured.

**Solution:**
```bash
# Option 1: Set a default profile
claude-vault default personal

# Option 2: Initialize the current directory
claude-vault init personal

# Option 3: Specify profile explicitly
claude-vault exec --profile personal your-command
```

### Issue: Wrong profile detected

**Problem:** Directory has unexpected `.claude-profile` file in parent directory.

**Solution:**
```bash
# Check what's being detected
claude-vault detect

# Find all .claude-profile files in tree
find . -name .claude-profile

# Create a new .claude-profile in current directory to override
claude-vault init correct-profile

# Or remove unwanted .claude-profile files
rm ../unwanted/.claude-profile
```

### Issue: Profile not detected after `init`

**Problem:** Cache might be stale or file not created.

**Solution:**
```bash
# Verify .claude-profile exists
cat .claude-profile

# If it doesn't exist, run init again
claude-vault init profile-name

# Navigate out and back to refresh cache
cd .. && cd -

# Try detection again
claude-vault detect
```

### Issue: `Profile 'name' in .claude-profile does not exist`

**Problem:** `.claude-profile` references a profile that hasn't been added to vault.

**Solution:**
```bash
# Check what profile is referenced
cat .claude-profile

# Add that profile
claude-vault add name

# Or update .claude-profile to existing profile
claude-vault list  # See available profiles
echo "existing-profile" > .claude-profile
```

## Keychain Issues

### Issue: `Keychain error: ...`

**Problem:** macOS Keychain access denied or corrupted.

**Solution:**
```bash
# Option 1: Open Keychain Access.app
# Search for "claude-vault"
# Delete old entries and try again

# Option 2: Reset keychain for claude-vault
# Open Keychain Access.app
# Find all "claude-vault-*" items
# Delete them
# Re-add your profiles
claude-vault add profile-name

# Option 3: Grant keychain access
# System Preferences → Security & Privacy → Privacy → Automation
# Allow Terminal/your app to control Keychain Access
```

### Issue: Keychain password prompt every time

**Problem:** Keychain access not saved.

**Solution:**
```bash
# When prompted, select "Always Allow"
# This saves the permission permanently

# If using Terminal, go to:
# Keychain Access.app → Right-click Terminal in "Always Allow" apps
# Select "Allow Always"
```

### Issue: Cannot access keychain in automation/CI

**Problem:** Keychain not available in headless environments.

**Solution:**
```bash
# CI environments don't have macOS Keychain
# Use environment variables instead:

# GitHub Actions
env:
  ANTHROPIC_API_KEY: ${{ secrets.CLAUDE_API_KEY }}
run: your-command

# Or create profiles from env vars
echo "$CLAUDE_API_KEY" | claude-vault add ci
```

## Execution Issues

### Issue: `exec` command not passing through exit codes

**Problem:** Parent process not seeing child exit code.

**Solution:**
The `exec` command properly forwards exit codes. If you're seeing issues:

```bash
# Test exit code forwarding
claude-vault exec false
echo $?  # Should print 1

claude-vault exec true
echo $?  # Should print 0

# If not working, check your shell configuration
# Some shells or wrappers might be interfering
```

### Issue: Command not found when using `exec`

**Problem:** Command not in PATH or incorrectly specified.

**Solution:**
```bash
# Use full path
claude-vault exec /usr/local/bin/claude --version

# Or ensure command is in PATH
which claude

# Check PATH in exec environment
claude-vault exec bash -c 'echo $PATH'
```

### Issue: Arguments with spaces not working

**Problem:** Shell parsing arguments incorrectly.

**Solution:**
```bash
# Use quotes around the entire command
claude-vault exec claude "say hello world"

# Or use -- separator for complex arguments
claude-vault exec -- claude --option "value with spaces"
```

## Environment Issues

### Issue: `ANTHROPIC_API_KEY` not set after `env` command

**Problem:** Not using `eval` to execute the output.

**Solution:**
```bash
# Wrong way
claude-vault env
# This just prints the export statement

# Right way
eval $(claude-vault env)
# This executes the export statement

# Verify it's set
echo $ANTHROPIC_API_KEY
```

### Issue: Environment not persisting across shells

**Problem:** Environment variables only last for current shell session.

**Solution:**
```bash
# Add to shell startup file for persistence

# Bash (~/.bashrc)
echo 'eval $(claude-vault env 2>/dev/null)' >> ~/.bashrc

# Zsh (~/.zshrc)
echo 'eval $(claude-vault env 2>/dev/null)' >> ~/.zshrc

# Fish (~/.config/fish/config.fish)
echo 'claude-vault env 2>/dev/null | source' >> ~/.config/fish/config.fish

# Reload shell
source ~/.bashrc  # or restart terminal
```

### Issue: Wrong profile environment loaded

**Problem:** Environment cached from previous directory.

**Solution:**
```bash
# Environment is loaded once per shell session
# To refresh, either:

# Option 1: Start a new shell
exec $SHELL

# Option 2: Re-run eval
eval $(claude-vault env)

# Option 3: Use exec for one-off commands
claude-vault exec --profile correct-profile your-command
```

## Cache Issues

### Issue: Stale profile detection

**Problem:** Cache not updating after profile changes.

**Solution:**
```bash
# Cache automatically expires after 1 hour
# To force refresh:

# Navigate out and back
cd .. && cd -

# Or recreate .claude-profile
claude-vault init profile-name

# Cache is stored in ~/.cache/claude-vault/
# You can manually clear it if needed
rm -rf ~/.cache/claude-vault/
```

## Platform-Specific Issues

### macOS

**Issue:** `xcrun: error: invalid active developer path`

**Problem:** Command Line Tools not installed.

**Solution:**
```bash
xcode-select --install
```

**Issue:** Gatekeeper blocking execution

**Problem:** macOS security blocking unsigned binary.

**Solution:**
```bash
# Remove quarantine attribute
xattr -d com.apple.quarantine /path/to/claude-vault

# Or allow in System Preferences:
# System Preferences → Security & Privacy → General
# Click "Allow Anyway" next to the blocked message
```

### Linux (Future Support)

Currently Linux is not supported. Support is planned for:
- Secret Service API (GNOME Keyring, KWallet)
- GPG-encrypted storage fallback

### Windows (Future Support)

Currently Windows is not supported. Support is planned for:
- Windows Credential Manager
- DPAPI encryption

## Getting Help

If none of these solutions work:

1. **Check version:**
```bash
claude-vault --version
```

2. **Enable verbose output:**
```bash
# Set Rust log level
RUST_LOG=debug claude-vault detect
```

3. **Verify setup:**
```bash
# Check config file
cat ~/.claude-vault/config.toml

# Check profiles
claude-vault list

# Check detection
claude-vault detect
```

4. **File an issue:**
- GitHub: https://github.com/rtzr/claude-vault/issues
- Include:
  - OS version
  - `claude-vault --version` output
  - Error message
  - Steps to reproduce

## Next Steps

- [Best Practices](BEST_PRACTICES.md)
- [FAQ](FAQ.md)
- [Usage Guide](USAGE_GUIDE.md)
- [Claude Code Integration](CLAUDE_CODE_INTEGRATION.md)
