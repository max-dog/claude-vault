# claude-vault

> Secure credential management for Claude API - inspired by aws-vault

`claude-vault` is a CLI tool for managing multiple Claude API credentials with automatic profile detection and secure keychain storage.

## Features

- 🔐 **Secure Storage**: API keys stored in macOS Keychain (Linux/Windows support coming soon)
- 🔄 **Multiple Profiles**: Manage personal, work, and project-specific Claude accounts
- 🎯 **Auto-Detection**: Automatically detect profiles from `.claude-profile` files
- ⚡ **Command Execution**: Run commands with profile credentials via `exec` and `env`
- 🐚 **Shell Completion**: Tab completion for Bash, Zsh, and Fish (✨ NEW!)
- ✨ **Simple CLI**: Intuitive commands for profile management
- 📦 **Smart Caching**: Performance-optimized profile detection with intelligent caching
- 🧪 **Well-Tested**: Comprehensive test coverage (26 passing tests)

## Installation

### From Source

```bash
git clone https://github.com/max-dog/claude-vault.git
cd claude-vault
cargo build --release
sudo mv target/release/claude-vault /usr/local/bin/
```

### Homebrew (Coming Soon)

```bash
brew install claude-vault
```

### Shell Completion

Enable tab completion for your shell:

**Bash**
```bash
# Generate completion script
claude-vault completion bash > ~/.local/share/bash-completion/completions/claude-vault

# Or add to your ~/.bashrc
echo 'eval "$(claude-vault completion bash)"' >> ~/.bashrc
```

**Zsh**
```bash
# Generate completion script
claude-vault completion zsh > ~/.zsh/completions/_claude-vault

# Or add to your ~/.zshrc
echo 'eval "$(claude-vault completion zsh)"' >> ~/.zshrc

# Make sure completion system is initialized
echo 'autoload -Uz compinit && compinit' >> ~/.zshrc
```

**Fish**
```bash
# Generate completion script
claude-vault completion fish > ~/.config/fish/completions/claude-vault.fish
```

## Quick Start

### 1. Add Your First Profile

```bash
claude-vault add personal
# Enter your Claude API key when prompted
```

### 2. List Profiles

```bash
claude-vault list
```

### 3. Set Default Profile

```bash
claude-vault default personal
```

## Usage

### Add a New Profile

```bash
# Add with name only
claude-vault add work

# Add with description
claude-vault add work --description "Company account"
```

### List All Profiles

```bash
claude-vault list
```

Output:
```
Profiles:
  • personal - Personal projects
    Last used: 2025-11-10T12:30:00Z
  • work - Company account
```

### Show Profile Details

```bash
claude-vault show personal
```

### Remove a Profile

```bash
# With confirmation
claude-vault remove work

# Skip confirmation
claude-vault remove work --yes
```

### Set Default Profile

```bash
claude-vault default personal
```

### Execute Commands with Profile

```bash
# Execute command with detected/default profile
claude-vault exec claude --version

# Execute command with specific profile
claude-vault exec --profile work claude --model sonnet "Hello world"

# Use in scripts
claude-vault exec npm run test
```

### Export Environment Variables

```bash
# Export for shell integration
eval $(claude-vault env)

# Export specific profile
eval $(claude-vault env --profile work)
```

## Project-Specific Profiles

Create a `.claude-profile` file in your project root:

```bash
# In your work project
cd ~/work/my-project
echo "work" > .claude-profile

# In your personal project
cd ~/personal/side-project
echo "personal" > .claude-profile
```

Now when you run Claude Code or other tools, `claude-vault` can automatically detect and use the appropriate profile.

## Configuration

Configuration is stored in `~/.claude-vault/config.toml`:

```toml
version = "1.0"
default_profile = "personal"

[[profiles]]
name = "personal"
description = "Personal projects"
created_at = "2025-11-10T12:00:00Z"
```

API keys are securely stored in your system keychain:
- **macOS**: Keychain Access
- **Linux**: Secret Service API (coming soon)
- **Windows**: Windows Credential Manager (coming soon)

## Development

### Prerequisites

- Rust 1.70+
- macOS 12+ (for MVP)

### Build

```bash
cargo build
```

### Run Tests

```bash
cargo test
```

### Run Locally

```bash
cargo run -- add test
cargo run -- list
```

## Roadmap

- [x] MVP: Core profile management
- [x] macOS Keychain integration
- [x] Basic CLI commands
- [x] Automatic profile detection (`.claude-profile`)
- [x] `exec` command for command execution
- [x] `env` command for shell integration
- [x] Shell completion scripts (Bash, Zsh, Fish)
- [ ] Linux support
- [ ] Windows support
- [ ] Homebrew formula
- [ ] Usage statistics
- [ ] Team features

## Documentation

Comprehensive guides and best practices:

- 📖 [Usage Guide](docs/USAGE_GUIDE.md) - Advanced usage scenarios and workflows
- 🔌 [Claude Code Integration](docs/CLAUDE_CODE_INTEGRATION.md) - Integrate with Claude Code seamlessly
- 🔧 [Troubleshooting](docs/TROUBLESHOOTING.md) - Common issues and solutions
- ✨ [Best Practices](docs/BEST_PRACTICES.md) - Security and operational best practices
- ❓ [FAQ](docs/FAQ.md) - Frequently asked questions

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT License - see LICENSE file for details

## Acknowledgments

Inspired by [aws-vault](https://github.com/99designs/aws-vault) - the excellent credential management tool for AWS.

---

**Status**: ✨ MVP Complete - Ready for Use

Built with Rust 🦀
