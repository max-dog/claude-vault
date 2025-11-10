# Claude Code Integration Guide

Guide for integrating `claude-vault` with Claude Code for seamless profile management.

## Table of Contents

- [Overview](#overview)
- [Setup](#setup)
- [Integration Methods](#integration-methods)
- [Workflows](#workflows)
- [Troubleshooting](#troubleshooting)

## Overview

Claude Code reads the `ANTHROPIC_API_KEY` environment variable to authenticate with Claude's API. `claude-vault` provides multiple ways to manage and inject these credentials:

1. **Shell Integration**: Export environment variables in your shell
2. **Exec Wrapper**: Run Claude Code through `claude-vault exec`
3. **Project Auto-Detection**: Automatic profile selection per project

## Setup

### Prerequisites

1. Install `claude-vault`:
```bash
cargo install --path .
# or
brew install claude-vault  # Coming soon
```

2. Install Claude Code:
```bash
npm install -g @anthropic-ai/claude-code
```

3. Add at least one profile:
```bash
claude-vault add personal
# Enter your API key when prompted
```

## Integration Methods

### Method 1: Shell Environment (Recommended)

Export the profile environment variables in your shell startup file.

**For Bash** (`~/.bashrc`):
```bash
# Auto-detect and load claude-vault profile
if command -v claude-vault &> /dev/null; then
  eval $(claude-vault env 2>/dev/null) || true
fi
```

**For Zsh** (`~/.zshrc`):
```bash
# Auto-detect and load claude-vault profile
if command -v claude-vault &> /dev/null; then
  eval $(claude-vault env 2>/dev/null) || true
fi
```

**For Fish** (`~/.config/fish/config.fish`):
```fish
# Auto-detect and load claude-vault profile
if command -v claude-vault > /dev/null
  claude-vault env 2>/dev/null | source
end
```

**Benefits:**
- Automatic profile detection per directory
- No need to prefix commands
- Works with all tools that use `ANTHROPIC_API_KEY`

**Usage:**
```bash
# Just navigate to your project
cd ~/projects/work-project

# Claude Code automatically uses the detected profile
claude "Write a hello world function"
```

### Method 2: Exec Wrapper

Wrap Claude Code commands with `claude-vault exec`.

**Benefits:**
- Explicit profile control
- No shell configuration needed
- Useful for scripts and automation

**Usage:**
```bash
# Use detected profile
claude-vault exec claude "Write a hello world function"

# Use specific profile
claude-vault exec --profile work claude "Write a hello world function"

# Create an alias for convenience
alias cv-claude='claude-vault exec claude'
cv-claude "Write a hello world function"
```

### Method 3: Project Auto-Detection

Initialize projects with specific profiles using `.claude-profile` files.

**Setup:**
```bash
# Work project
cd ~/projects/work-project
claude-vault init work

# Personal project
cd ~/projects/personal-project
claude-vault init personal
```

**Benefits:**
- Automatic profile switching per project
- No manual profile selection needed
- Works with both Method 1 and Method 2

**Usage:**
```bash
# Navigate to work project
cd ~/projects/work-project
claude-vault detect
# Output: Detected profile: work

# Claude Code uses work profile
claude "Analyze this codebase"

# Navigate to personal project
cd ~/projects/personal-project
claude-vault detect
# Output: Detected profile: personal

# Claude Code uses personal profile
claude "Help me with this script"
```

## Workflows

### Workflow 1: Personal vs Work Separation

**Scenario:** You have separate API accounts for personal and work projects.

**Setup:**
```bash
# Add profiles
claude-vault add personal
claude-vault add work

# Set personal as default
claude-vault default personal

# Initialize work projects
cd ~/work/company-project
claude-vault init work

# Initialize personal projects
cd ~/personal/side-project
claude-vault init personal
```

**Usage:**
```bash
# Work on company project
cd ~/work/company-project
claude "Review this PR"  # Uses work profile

# Work on personal project
cd ~/personal/side-project
claude "Help me with this feature"  # Uses personal profile
```

### Workflow 2: Client-Specific Projects

**Scenario:** You work with multiple clients, each with their own API keys.

**Setup:**
```bash
# Add client profiles
claude-vault add client-a --description "Client A projects"
claude-vault add client-b --description "Client B projects"
claude-vault add client-c --description "Client C projects"

# Initialize projects
cd ~/clients/client-a/project1
claude-vault init client-a

cd ~/clients/client-b/project1
claude-vault init client-b
```

**Usage:**
```bash
# Work on Client A project
cd ~/clients/client-a/project1
claude "Implement the feature"  # Uses client-a profile

# Switch to Client B project
cd ~/clients/client-b/project1
claude "Fix the bug"  # Uses client-b profile
```

### Workflow 3: Team Development

**Scenario:** Team members share a repository but use individual API keys.

**Setup:**
```bash
# Clone shared repository
git clone https://github.com/company/shared-project.git
cd shared-project

# Each team member initializes with their profile
claude-vault init alice  # Alice's setup
# or
claude-vault init bob    # Bob's setup

# .claude-profile is in .gitignore
cat .gitignore | grep .claude-profile
# Output: .claude-profile
```

**Usage:**
```bash
# Alice works on the project
cd ~/work/shared-project
claude "Implement feature X"  # Uses alice profile

# Bob works on the same project (on his machine)
cd ~/work/shared-project
claude "Review Alice's code"  # Uses bob profile
```

### Workflow 4: Testing with Different Accounts

**Scenario:** You want to test with different API tiers or accounts.

**Setup:**
```bash
# Add profiles for different tiers
claude-vault add prod --description "Production API key"
claude-vault add dev --description "Development API key"
claude-vault add test --description "Testing API key"
```

**Usage:**
```bash
# Test with development key
claude-vault exec --profile dev claude "Test this feature"

# Deploy with production key
claude-vault exec --profile prod claude "Deploy to production"

# Run tests with test key
claude-vault exec --profile test npm test
```

## Advanced Integration

### Shell Functions

Add to your shell config for enhanced functionality:

```bash
# Quick profile switcher
cv() {
  case "$1" in
    list)
      claude-vault list
      ;;
    use)
      eval $(claude-vault env --profile "$2")
      echo "Switched to profile: $2"
      ;;
    detect)
      claude-vault detect
      ;;
    *)
      echo "Usage: cv {list|use <profile>|detect}"
      ;;
  esac
}

# Claude Code with automatic profile
claude-profile() {
  claude-vault exec claude "$@"
}

# Alias for convenience
alias cvp='claude-profile'
```

**Usage:**
```bash
cv list              # List all profiles
cv use work          # Switch to work profile
cv detect            # Show current profile
cvp "Hello world"    # Run Claude Code with auto-detected profile
```

### VS Code Integration

If using Claude Code in VS Code, you can set up environment variables:

**Option 1: VS Code Settings**

Add to `.vscode/settings.json`:
```json
{
  "terminal.integrated.env.osx": {
    "ANTHROPIC_API_KEY": "sk-ant-..."
  },
  "terminal.integrated.env.linux": {
    "ANTHROPIC_API_KEY": "sk-ant-..."
  }
}
```

**Option 2: Task Runner**

Create `.vscode/tasks.json`:
```json
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "claude-code-work",
      "type": "shell",
      "command": "claude-vault exec --profile work claude",
      "problemMatcher": []
    },
    {
      "label": "claude-code-personal",
      "type": "shell",
      "command": "claude-vault exec --profile personal claude",
      "problemMatcher": []
    }
  ]
}
```

### JetBrains IDEs Integration

**Option 1: External Tool**

1. Go to Settings → Tools → External Tools
2. Add New Tool:
   - Name: `Claude Code (Work)`
   - Program: `claude-vault`
   - Arguments: `exec --profile work claude "$PROMPT$"`
   - Working Directory: `$ProjectFileDir$`

**Option 2: Run Configuration**

1. Run → Edit Configurations → Add New → Shell Script
2. Configure:
   - Script: `claude-vault exec --profile work claude "$@"`
   - Script Options: Your prompt here

## CI/CD Integration with Claude Code

### GitHub Actions

```yaml
name: Claude Code Analysis

on: [push, pull_request]

jobs:
  analyze:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Setup Node.js
        uses: actions/setup-node@v3
        with:
          node-version: '18'

      - name: Install Claude Code
        run: npm install -g @anthropic-ai/claude-code

      - name: Install claude-vault
        run: |
          curl -L https://github.com/max-dog/claude-vault/releases/latest/download/claude-vault-x86_64-unknown-linux-gnu -o /usr/local/bin/claude-vault
          chmod +x /usr/local/bin/claude-vault

      - name: Setup profile
        env:
          CLAUDE_API_KEY: ${{ secrets.CLAUDE_API_KEY }}
        run: |
          echo "$CLAUDE_API_KEY" | claude-vault add ci
          claude-vault default ci

      - name: Run Claude Code analysis
        run: |
          claude-vault exec claude "Analyze this codebase for potential issues"
```

## Environment Variables

`claude-vault` exports the following environment variables:

- `ANTHROPIC_API_KEY`: The API key for the selected profile

Claude Code also respects these optional variables:

- `CLAUDE_MODEL`: Default model to use
- `CLAUDE_MAX_TOKENS`: Maximum tokens per request
- `CLAUDE_TEMPERATURE`: Temperature setting

Example with additional variables:

```bash
# Export profile and configure Claude Code
eval $(claude-vault env --profile work)
export CLAUDE_MODEL="claude-3-opus-20240229"
export CLAUDE_MAX_TOKENS=4096

# Now run Claude Code
claude "Write a complex function"
```

## Verification

Verify your integration is working:

```bash
# Check profile detection
claude-vault detect

# Check environment variable
echo $ANTHROPIC_API_KEY
# Should output: sk-ant-...

# Test Claude Code
claude --version

# Run a simple test
claude "Hello, can you confirm you're working?"
```

## Next Steps

- [Troubleshooting Guide](TROUBLESHOOTING.md)
- [Best Practices](BEST_PRACTICES.md)
- [Usage Guide](USAGE_GUIDE.md)
- [FAQ](FAQ.md)
