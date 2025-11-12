# Frequently Asked Questions

Common questions about `claude-vault`.

## General Questions

### What is claude-vault?

`claude-vault` is a command-line tool for securely managing multiple Claude API credentials, inspired by `aws-vault`. It stores API keys in your system's secure keychain and provides automatic profile detection per project.

### Why use claude-vault instead of environment variables?

**Benefits:**
- **Security**: Keys stored in macOS Keychain, not plain text files
- **Multi-profile**: Easily switch between personal, work, and client accounts
- **Auto-detection**: Profiles automatically selected per project directory
- **Git-safe**: No risk of committing API keys
- **Team-friendly**: Each developer uses their own credentials

### How is this different from aws-vault?

`claude-vault` is inspired by `aws-vault` but designed specifically for Claude API:
- Simpler profile management (no IAM roles/sessions)
- Project-based auto-detection via `.claude-profile` files
- Built-in Claude Code integration
- Lightweight and focused on API key management

## Installation and Setup

### What platforms are supported?

**Currently:**
- macOS 12+ (with Keychain integration)

**Planned:**
- Linux (Secret Service API)
- Windows (Credential Manager)

### Do I need Rust installed to use claude-vault?

**To install from source:** Yes, you need Rust toolchain.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**To use pre-built binaries:** No, once we release binaries.

**To install via Homebrew:** No, coming soon.

### How do I get my Claude API key?

1. Go to https://console.anthropic.com/
2. Sign in to your account
3. Navigate to Settings → API Keys
4. Create a new API key
5. Copy the key (starts with `sk-ant-`)

## Profile Management

### Can I have multiple profiles?

Yes! That's the main purpose of `claude-vault`:

```bash
claude-vault add personal
claude-vault add work
claude-vault add client-a
claude-vault add experiment
```

### How do I switch between profiles?

Three methods:

**1. Auto-detection** (recommended):
```bash
cd ~/work/project
claude-vault init work
# Profile automatically detected when in this directory
```

**2. Explicit selection**:
```bash
claude-vault exec --profile work your-command
eval $(claude-vault env --profile work)
```

**3. Default profile**:
```bash
claude-vault default personal
# Used as fallback when no profile detected
```

### Can I update an existing profile's API key?

Yes, remove and re-add:

```bash
claude-vault remove old-profile --yes
claude-vault add old-profile
# Enter new API key
```

### How do I see which profile is currently active?

```bash
# Check detected profile
claude-vault detect

# Check environment variable
echo $ANTHROPIC_API_KEY

# See all profiles
claude-vault list
```

## Security

### Where are my API keys stored?

- **Keys**: macOS Keychain (secure encrypted storage)
- **Config**: `~/.claude-vault/config.toml` (profile metadata only, no keys)

### Is it safe to commit .claude-profile files?

**Yes!** `.claude-profile` files only contain the profile *name*, not the API key:

```bash
cat .claude-profile
# Output: work
```

The actual API key is stored securely in your keychain.

### What happens if someone gets my config.toml file?

**No risk.** The config file only contains:
- Profile names
- Descriptions
- Timestamps
- Default profile setting

It does NOT contain API keys. Keys are only in the keychain.

### Can I share profiles with my team?

**No, by design.** Each person should have their own API key and profiles. This ensures:
- Individual accountability
- Separate rate limits
- Easy key rotation
- No shared secrets

### How do I remove all traces of claude-vault?

```bash
# Remove profiles (deletes keys from keychain)
claude-vault list | grep "• " | cut -d' ' -f3 | xargs -I {} claude-vault remove {} --yes

# Remove config directory
rm -rf ~/.claude-vault

# Remove cache
rm -rf ~/.cache/claude-vault

# Remove binary
rm /usr/local/bin/claude-vault
```

## Usage

### Can I use claude-vault with other Claude tools?

**Yes!** Any tool that uses `ANTHROPIC_API_KEY` works:

```bash
# With Claude CLI
eval $(claude-vault env)
claude "Hello world"

# With Claude Code
claude-vault exec claude-code analyze.js

# With Python anthropic library
claude-vault exec python my_script.py

# With Node.js SDK
claude-vault exec node app.js
```

### Do I need to run claude-vault commands every time?

**No.** Add to your shell config for automatic loading:

```bash
# ~/.bashrc or ~/.zshrc
eval $(claude-vault env 2>/dev/null) || true
```

Then profiles are automatically detected per directory.

### Can I use claude-vault in scripts?

**Yes:**

```bash
#!/bin/bash
# my-script.sh

# Ensure profile is available
if ! claude-vault detect &>/dev/null; then
  echo "Error: No profile detected"
  exit 1
fi

# Run commands with profile
claude-vault exec your-command
```

### Does claude-vault work in CI/CD?

**Yes,** but differently since keychain isn't available:

```yaml
# GitHub Actions
- name: Setup profile
  env:
    CLAUDE_API_KEY: ${{ secrets.CLAUDE_API_KEY }}
  run: |
    echo "$CLAUDE_API_KEY" | claude-vault add ci
    claude-vault default ci

- name: Run tests
  run: claude-vault exec npm test
```

## Project Integration

### Should every project have a .claude-profile?

**Recommended for:**
- Shared repositories
- Projects with specific profiles (client work)
- Team projects

**Optional for:**
- Personal scripts
- Quick experiments
- Projects using default profile

### Can I have different profiles for different parts of a monorepo?

**Yes:**

```bash
~/monorepo/
  ├── .claude-profile         # Default: work
  ├── packages/
  │   ├── core/
  │   │   └── .claude-profile # Uses: work
  │   └── experimental/
  │       └── .claude-profile # Overrides: personal
```

### What if I forget to add .claude-profile to .gitignore?

**`claude-vault init` automatically adds it!**

But if needed:
```bash
# Remove from git
git rm --cached .claude-profile

# Add to .gitignore
echo ".claude-profile" >> .gitignore

# Commit
git add .gitignore
git commit -m "chore: ignore .claude-profile"
```

### Can I have multiple .claude-profile files in nested directories?

**Yes.** Detection walks up the tree and uses the nearest one:

```bash
~/project/
  ├── .claude-profile        # work
  └── experimental/
      └── .claude-profile    # personal

# In ~/project/: Uses work
# In ~/project/experimental/: Uses personal
```

## Performance

### Does claude-vault slow down my commands?

**Minimal overhead:**
- First detection: ~1-5ms (filesystem check)
- Cached detection: <1ms (memory lookup)
- Keychain access: ~10-50ms (first time per session)
- Cache TTL: 1 hour

### How does the cache work?

- Detected profiles are cached for 1 hour
- Cache is per-directory path
- Automatically refreshed when expired
- Stored in `~/.cache/claude-vault/`

### Can I disable caching?

**No, but you can work around it:**

```bash
# Navigate away and back to refresh
cd .. && cd -

# Or explicitly specify profile
claude-vault exec --profile name your-command
```

## Troubleshooting

### "No profile detected" error - what do I do?

**Three solutions:**

1. **Initialize project:**
```bash
claude-vault init your-profile
```

2. **Set default:**
```bash
claude-vault default your-profile
```

3. **Specify explicitly:**
```bash
claude-vault exec --profile your-profile your-command
```

### Keychain keeps asking for password - how to fix?

**Solution:**
1. When prompted, select "Always Allow"
2. Or in Keychain Access.app:
   - Find "claude-vault-*" entries
   - Right-click → Get Info
   - Access Control → Add your terminal to "Always allow"

### Profile detection is wrong - what happened?

**Check for unexpected .claude-profile files:**

```bash
# Find all .claude-profile files
find . -name .claude-profile

# Check what's detected
claude-vault detect

# Override with correct profile
claude-vault init correct-profile
```

### How do I report a bug?

**GitHub Issues:** https://github.com/rtzr/claude-vault/issues

**Include:**
- OS version: `sw_vers` (macOS)
- claude-vault version: `claude-vault --version`
- Error message
- Steps to reproduce

## Advanced

### Can I use claude-vault programmatically?

**Current:** CLI only.

**Future:** Rust library API planned for v1.0.

### Can I extend claude-vault with plugins?

**Not currently.** The tool is focused and purposely simple.

Feature requests welcome at: https://github.com/rtzr/claude-vault/issues

### Does claude-vault support credential rotation?

**Manual rotation:**
```bash
claude-vault remove old --yes
claude-vault add old  # Enter new key
```

**Automatic rotation:** Planned for future release.

### Can I export/import profiles?

**Export config** (no keys):
```bash
cp ~/.claude-vault/config.toml backup.toml
```

**Import:** Copy to `~/.claude-vault/config.toml`

**Note:** Keys must be re-added manually (security by design).

### Does claude-vault work offline?

**Yes,** once profiles are set up:
- Config is local
- Keychain is local
- No network required for profile management

**No,** for:
- Using Claude API (obviously requires network)

## Comparison

### claude-vault vs environment variables

| Feature | claude-vault | Environment Variables |
|---------|--------------|---------------------|
| Security | ✅ Keychain | ❌ Plain text |
| Multiple profiles | ✅ Easy | ❌ Manual |
| Auto-detection | ✅ Yes | ❌ No |
| Git-safe | ✅ Yes | ⚠️ Risk |
| Team-friendly | ✅ Yes | ❌ No |

### claude-vault vs .env files

| Feature | claude-vault | .env files |
|---------|--------------|-----------|
| Security | ✅ Encrypted | ❌ Plain text |
| Profile switching | ✅ Automatic | ❌ Manual |
| Git risk | ✅ None | ⚠️ High |
| Team usage | ✅ Individual keys | ❌ Shared secrets |
| Setup complexity | ⚠️ Moderate | ✅ Simple |

## Getting Help

### Where can I get help?

1. **Documentation:**
   - [Usage Guide](USAGE_GUIDE.md)
   - [Troubleshooting](TROUBLESHOOTING.md)
   - [Best Practices](BEST_PRACTICES.md)
   - [Claude Code Integration](CLAUDE_CODE_INTEGRATION.md)

2. **GitHub Issues:** https://github.com/rtzr/claude-vault/issues

3. **Discussions:** https://github.com/rtzr/claude-vault/discussions

### How can I contribute?

See [CONTRIBUTING.md](../CONTRIBUTING.md) (coming soon)

**Quick ways to help:**
- Report bugs
- Suggest features
- Improve documentation
- Submit pull requests
- Share your experience

### What's the roadmap?

See [Roadmap](../README.md#roadmap) in README.

**Completed:**
- ✅ Core profile management
- ✅ macOS Keychain integration
- ✅ Auto profile detection
- ✅ Command execution
- ✅ Shell completion

**Planned:**
- Linux/Windows support
- Homebrew formula
- Automatic key rotation
- Usage statistics
- Team features
