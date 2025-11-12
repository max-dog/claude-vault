# Usage Guide

Comprehensive guide for using `claude-vault` in various scenarios.

## Table of Contents

- [Basic Workflows](#basic-workflows)
- [Advanced Scenarios](#advanced-scenarios)
- [Profile Management](#profile-management)
- [Project Integration](#project-integration)
- [CI/CD Integration](#cicd-integration)

## Basic Workflows

### Setting Up Your First Profile

**Option 1: Using an API Key (Pay-as-you-go)**

```bash
# Add your personal profile
claude-vault add personal
# Enter your API key when prompted: sk-ant-...

# Set it as default
claude-vault default personal

# Verify it works
claude-vault detect
# Output: Detected profile: personal
```

**Option 2: Importing OAuth Token (Subscription Accounts)**

If you have a Claude Pro/Max subscription:

```bash
# First, login to Claude Code to get a fresh token
claude /login

# Import the OAuth token
claude-vault import oauth --profile subscription

# Set it as default
claude-vault default subscription

# Verify it works
claude-vault detect
# Output: Detected profile: subscription

# Check token status
claude-vault show subscription
# Output includes:
#   Type: OAuth Token
#   Status: ✓ Valid
#   Expires: 2025-11-17T14:30:00Z
```

**OAuth Token Management**

OAuth tokens expire periodically (typically every 7-30 days). When your token expires:

```bash
# Re-login to Claude Code
claude /login

# Re-import the token (overwrites existing)
claude-vault import oauth --profile subscription

# Verify the new token
claude-vault show subscription
```

### Working with Multiple Profiles

```bash
# Add work profile
claude-vault add work --description "Company account"

# Add project-specific profile
claude-vault add project-x --description "Client X project"

# List all profiles
claude-vault list
```

### Using Profiles in Projects

```bash
# Initialize a project with a profile
cd ~/projects/my-app
claude-vault init work

# The profile is now auto-detected
claude-vault detect
# Output: Detected profile: work

# Run commands with the detected profile
claude-vault exec npm run dev
```

## Advanced Scenarios

### Dynamic Profile Switching

```bash
# Use different profiles for different parts of your workflow
cd ~/projects/work-project
claude-vault exec --profile work npm test

cd ~/projects/personal-project
claude-vault exec --profile personal npm test
```

### Shell Integration

```bash
# Export environment variables for your current shell session
eval $(claude-vault env)

# Verify the API key is set
echo $ANTHROPIC_API_KEY
# Output: sk-ant-...

# Now all Claude CLI commands use this profile
claude --model sonnet "Hello world"
```

### Profile-Specific Environment

```bash
# Create isolated shell sessions with different profiles
# Work session
claude-vault exec --profile work bash
# All commands in this session use work profile

# Personal session (in another terminal)
claude-vault exec --profile personal zsh
# All commands in this session use personal profile
```

### Claude Code Integration (OAuth Profiles)

For OAuth subscription accounts, `claude-vault` automatically manages Claude Code's keychain:

```bash
# Scenario: You have multiple Claude subscriptions
# - rtzr (max subscription)
# - personal (pro subscription)

# Use rtzr account with Claude Code
claude-vault exec --profile rtzr claude "Analyze this code"
# ✅ Automatically switches Claude Code keychain to rtzr
# ✅ Executes command with rtzr (max) subscription
# ✅ Restores original keychain after execution

# Use personal account with Claude Code
claude-vault exec --profile personal claude "Generate unit tests"
# ✅ Automatically switches Claude Code keychain to personal
# ✅ Executes command with personal (pro) subscription
# ✅ Restores original keychain after execution

# No manual /logout and /login required!
```

**How it works:**

1. **Before execution**: Backs up current Claude Code keychain
2. **During execution**: Switches to selected profile's OAuth token
3. **After execution**: Restores original keychain (even on error)

**Benefits:**

- Seamlessly switch between multiple Claude subscription accounts
- No need to manually run `claude /logout` and `claude /login`
- Original Claude Code login is preserved
- Works with any command that uses Claude Code

### Temporary Profile Override

```bash
# Your project uses 'work' profile by default
cd ~/projects/work-project
cat .claude-profile
# Output: work

# But you want to test with personal profile temporarily
claude-vault exec --profile personal npm test
# Uses personal profile, doesn't change .claude-profile
```

## Profile Management

### Updating API Keys

```bash
# Remove old profile
claude-vault remove old-profile

# Add with same name but new key
claude-vault add old-profile
# Enter new API key
```

### Profile Metadata

```bash
# View profile details
claude-vault show personal
# Output:
# Profile: personal
# Description: Personal projects
# Created: 2025-11-10T12:00:00Z
# Last used: 2025-11-10T18:30:00Z

# Update description by removing and re-adding
claude-vault remove personal
claude-vault add personal --description "Updated description"
```

### Batch Profile Setup

```bash
# Script to set up multiple profiles
#!/bin/bash

profiles=(
  "personal"
  "work"
  "client-a"
  "client-b"
)

for profile in "${profiles[@]}"; do
  echo "Setting up $profile"
  claude-vault add "$profile" --description "Profile for $profile"
done

# Set default
claude-vault default personal
```

## Project Integration

### Monorepo Setup

```bash
# Root uses default profile
cd ~/monorepo

# Backend uses work profile
cd packages/backend
echo "work" > .claude-profile

# Frontend uses personal profile
cd ../frontend
echo "personal" > .claude-profile

# Admin uses admin profile
cd ../admin
echo "admin" > .claude-profile

# Each package auto-detects its profile
cd packages/backend
claude-vault detect  # Output: work

cd ../frontend
claude-vault detect  # Output: personal
```

### Nested Projects

```bash
# Parent project
cd ~/projects/platform
echo "work" > .claude-profile

# Child project overrides parent
cd modules/experimental
echo "personal" > .claude-profile

# Detection walks up the tree and finds nearest .claude-profile
claude-vault detect
# Output: personal (from modules/experimental/.claude-profile)
```

### Team Project Setup

```bash
# Initialize project for the team
claude-vault init work

# Add .claude-profile to .gitignore
cat .gitignore | grep .claude-profile
# Output: .claude-profile (already added by init command)

# Team members can:
# 1. Clone the repo
# 2. Run: claude-vault init their-profile
# 3. Each developer uses their own profile
```

## CI/CD Integration

### GitHub Actions

```yaml
name: Test with Claude

on: [push]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Install claude-vault
        run: |
          curl -L https://github.com/rtzr/claude-vault/releases/latest/download/claude-vault-x86_64-unknown-linux-gnu -o claude-vault
          chmod +x claude-vault
          sudo mv claude-vault /usr/local/bin/

      - name: Setup profile
        env:
          CLAUDE_API_KEY: ${{ secrets.CLAUDE_API_KEY }}
        run: |
          echo "$CLAUDE_API_KEY" | claude-vault add ci
          claude-vault default ci

      - name: Run tests
        run: claude-vault exec npm test
```

### GitLab CI

```yaml
test:
  image: node:18
  before_script:
    - curl -L https://github.com/rtzr/claude-vault/releases/latest/download/claude-vault-x86_64-unknown-linux-gnu -o /usr/local/bin/claude-vault
    - chmod +x /usr/local/bin/claude-vault
    - echo "$CLAUDE_API_KEY" | claude-vault add ci
    - claude-vault default ci
  script:
    - claude-vault exec npm test
```

### Docker Integration

```dockerfile
FROM node:18

# Install claude-vault
RUN curl -L https://github.com/rtzr/claude-vault/releases/latest/download/claude-vault-x86_64-unknown-linux-gnu -o /usr/local/bin/claude-vault && \
    chmod +x /usr/local/bin/claude-vault

# Setup profile at runtime
COPY setup-profile.sh /usr/local/bin/
RUN chmod +x /usr/local/bin/setup-profile.sh

WORKDIR /app
COPY . .

ENTRYPOINT ["/usr/local/bin/setup-profile.sh"]
CMD ["npm", "start"]
```

`setup-profile.sh`:
```bash
#!/bin/bash
set -e

# Setup profile from environment variable
if [ -n "$CLAUDE_API_KEY" ]; then
  echo "$CLAUDE_API_KEY" | claude-vault add production
  claude-vault default production
fi

# Execute the main command with the profile
exec claude-vault exec "$@"
```

## Tips and Tricks

### Quick Profile Switching

Add to your `~/.bashrc` or `~/.zshrc`:

```bash
# Function to quickly switch profiles
cvault() {
  if [ -z "$1" ]; then
    claude-vault list
  else
    eval $(claude-vault env --profile "$1")
    echo "Switched to profile: $1"
  fi
}

# Usage:
# cvault           # List profiles
# cvault personal  # Switch to personal profile
```

### Profile Status in Prompt

Add to your shell prompt to show current profile:

```bash
# For bash/zsh
claude_profile() {
  claude-vault detect 2>/dev/null || echo "none"
}

# Add to PS1/PROMPT
export PS1='[$(claude_profile)] $ '
```

### Scripting with claude-vault

```bash
#!/bin/bash

# Get the detected profile
PROFILE=$(claude-vault detect 2>/dev/null)

if [ $? -ne 0 ]; then
  echo "Error: No profile detected"
  echo "Please run: claude-vault init <profile>"
  exit 1
fi

echo "Using profile: $PROFILE"

# Run your commands
claude-vault exec claude --model sonnet "Analyze this code"
```

### Performance Optimization

```bash
# Cache is automatically managed (1-hour TTL)
# To force cache refresh, simply navigate out and back
cd ..
cd my-project

# Or remove and recreate .claude-profile
rm .claude-profile
claude-vault init work
```

## Next Steps

- [Claude Code Integration Guide](CLAUDE_CODE_INTEGRATION.md)
- [Troubleshooting Guide](TROUBLESHOOTING.md)
- [Best Practices](BEST_PRACTICES.md)
- [FAQ](FAQ.md)
