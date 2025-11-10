# Best Practices

Security and operational best practices for using `claude-vault`.

## Table of Contents

- [Security](#security)
- [Profile Organization](#profile-organization)
- [Project Setup](#project-setup)
- [Team Collaboration](#team-collaboration)
- [Automation and CI/CD](#automation-and-cicd)
- [Performance](#performance)

## Security

### API Key Management

**✅ DO:**
- Use separate profiles for different trust levels (personal, work, production)
- Rotate API keys regularly (every 90 days recommended)
- Use profile descriptions to track key purpose and creation date
- Delete profiles immediately when no longer needed

```bash
# Good: Clear separation of concerns
claude-vault add personal --description "Personal projects (created: 2024-01)"
claude-vault add work --description "Company projects (created: 2024-01)"
claude-vault add prod --description "Production deployments (created: 2024-01)"
```

**❌ DON'T:**
- Share API keys between projects or people
- Store API keys in plain text files or environment files
- Commit `.claude-profile` with hardcoded keys (it should only contain profile names)
- Use production keys for development or testing

### Keychain Security

**✅ DO:**
- Always select "Always Allow" when prompted for keychain access
- Regularly audit keychain entries via Keychain Access.app
- Lock your keychain when not using your computer
- Use strong keychain passwords

**❌ DON'T:**
- Grant keychain access to untrusted applications
- Share your keychain password
- Disable keychain encryption

### .gitignore Protection

**✅ DO:**
Always ensure `.claude-profile` is in `.gitignore`:

```bash
# Verify it's ignored
git check-ignore .claude-profile
# Should output: .claude-profile

# If not, add it
echo ".claude-profile" >> .gitignore
git add .gitignore
git commit -m "chore: ignore .claude-profile"
```

**Note:** `claude-vault init` automatically adds `.claude-profile` to `.gitignore` if `.git` exists.

### Access Control

**✅ DO:**
- Use separate profiles for different access levels
- Implement principle of least privilege
- Monitor profile usage via `last_used` timestamps
- Remove unused profiles promptly

```bash
# Audit profile usage
claude-vault list
# Review "Last used" timestamps
# Remove stale profiles
claude-vault remove old-profile --yes
```

**❌ DON'T:**
- Use admin/production keys for development
- Share profile credentials across team members
- Leave test/temporary profiles active

## Profile Organization

### Naming Conventions

**✅ DO:**
Use clear, descriptive profile names:

```bash
# Good: Purpose-based names
claude-vault add personal
claude-vault add work-dev
claude-vault add work-prod
claude-vault add client-acme
claude-vault add experiment

# Good: Team member names (for personal use)
claude-vault add alice-dev
claude-vault add alice-prod
```

**❌ DON'T:**
```bash
# Bad: Unclear names
claude-vault add profile1
claude-vault add test
claude-vault add temp
claude-vault add asdf
```

### Profile Descriptions

**✅ DO:**
Add detailed descriptions:

```bash
# Good: Includes purpose, scope, and metadata
claude-vault add work-prod \
  --description "Production deployment key - Company X - Created: 2024-01-15"

claude-vault add client-acme \
  --description "Client ACME projects - Limited to their codebase only"
```

**❌ DON'T:**
```bash
# Bad: No description or vague
claude-vault add work
claude-vault add test --description "test"
```

### Default Profile Strategy

**✅ DO:**
Set a safe default for fallback:

```bash
# Use least privileged profile as default
claude-vault default personal

# Or use development profile
claude-vault default dev
```

**❌ DON'T:**
```bash
# Bad: Production keys as default
claude-vault default prod

# Bad: No default at all (in personal use)
# Always have a fallback
```

## Project Setup

### Directory Structure

**✅ DO:**
Initialize each project with appropriate profile:

```bash
# Project root structure
~/projects/
  ├── work/
  │   ├── api-service/
  │   │   └── .claude-profile  # Contains: work-dev
  │   └── frontend/
  │       └── .claude-profile  # Contains: work-dev
  ├── personal/
  │   └── blog/
  │       └── .claude-profile  # Contains: personal
  └── clients/
      └── acme/
          └── .claude-profile  # Contains: client-acme
```

**❌ DON'T:**
```bash
# Bad: No per-project profiles
~/projects/
  └── .claude-profile  # Single profile for everything
```

### Monorepo Setup

**✅ DO:**
Use profile hierarchy in monorepos:

```bash
# Root uses general profile
~/monorepo/
  ├── .claude-profile  # Contains: company

# Packages can override
  ├── packages/
  │   ├── core/
  │   │   └── .claude-profile  # Contains: company
  │   ├── experimental/
  │   │   └── .claude-profile  # Contains: company-dev
  │   └── admin/
  │       └── .claude-profile  # Contains: company-admin
```

### New Project Workflow

**✅ DO:**
Follow this workflow for new projects:

```bash
# 1. Create project
mkdir new-project && cd new-project
git init

# 2. Initialize profile
claude-vault init work-dev

# 3. Verify
claude-vault detect
# Output: Detected profile: work-dev

# 4. Verify .gitignore
cat .gitignore | grep .claude-profile
# Output: .claude-profile

# 5. First commit
git add .gitignore
git commit -m "chore: initial commit with profile config"
```

## Team Collaboration

### Shared Repositories

**✅ DO:**
```bash
# 1. Add .claude-profile to .gitignore
echo ".claude-profile" >> .gitignore

# 2. Document in README
# README.md
## Setup
1. Clone this repository
2. Run: claude-vault init your-profile
3. Verify: claude-vault detect

# 3. Each member uses their own profile
# Alice: claude-vault init alice
# Bob: claude-vault init bob
```

**❌ DON'T:**
```bash
# Bad: Commit .claude-profile with profile name
git add .claude-profile
git commit -m "Add profile"

# Bad: Share API keys in documentation
# README.md: Use API key sk-ant-xyz...
```

### Profile Standardization

**✅ DO:**
Establish team conventions:

```markdown
## Team Profile Guidelines

### Profile Naming
- Development: `{name}-dev`
- Staging: `{name}-staging`
- Production: `{name}-prod`

### Setup
```bash
# Add your profiles
claude-vault add alice-dev --description "Alice dev key"
claude-vault add alice-prod --description "Alice prod key"

# Initialize project
claude-vault init alice-dev
```

### Onboarding Script

**✅ DO:**
Create setup scripts for new team members:

```bash
#!/bin/bash
# setup-claude-vault.sh

set -e

echo "Setting up claude-vault profiles..."

# Get developer name
read -p "Enter your name: " DEV_NAME

# Setup profiles
echo "Adding development profile..."
claude-vault add "${DEV_NAME}-dev" \
  --description "Development key for $DEV_NAME"

echo "Adding production profile..."
claude-vault add "${DEV_NAME}-prod" \
  --description "Production key for $DEV_NAME"

# Set default
claude-vault default "${DEV_NAME}-dev"

echo "Setup complete! Available profiles:"
claude-vault list
```

## Automation and CI/CD

### Environment Variables

**✅ DO:**
Use secrets management in CI/CD:

```yaml
# GitHub Actions
env:
  CLAUDE_API_KEY: ${{ secrets.CLAUDE_API_KEY }}

# GitLab CI
variables:
  CLAUDE_API_KEY:
    description: "Claude API key for CI"
    protected: true
    masked: true
```

**❌ DON'T:**
```yaml
# Bad: Hardcoded keys
env:
  CLAUDE_API_KEY: "sk-ant-api-key-here"

# Bad: Keys in repository
# .env file with keys
```

### CI Profile Setup

**✅ DO:**
Create dedicated CI profiles:

```bash
# On CI machine
echo "$CLAUDE_API_KEY" | claude-vault add ci --description "CI/CD pipeline"
claude-vault default ci

# Or use explicit profile
claude-vault exec --profile ci your-command
```

### Docker Best Practices

**✅ DO:**
```dockerfile
# Multi-stage build
FROM rust:1.70 AS builder
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /target/release/claude-vault /usr/local/bin/

# Profile setup at runtime
CMD ["sh", "-c", "echo $CLAUDE_API_KEY | claude-vault add app && claude-vault exec app your-command"]
```

**❌ DON'T:**
```dockerfile
# Bad: Hardcode keys in image
ENV ANTHROPIC_API_KEY=sk-ant-...

# Bad: Copy keychain data
COPY ~/.claude-vault /root/.claude-vault
```

## Performance

### Cache Optimization

**✅ DO:**
- Let automatic caching work (1-hour TTL)
- Use project-level `.claude-profile` files for best performance
- Avoid frequent `detect` calls in loops

```bash
# Good: Detect once
PROFILE=$(claude-vault detect)
for file in *.txt; do
  claude-vault exec --profile "$PROFILE" process "$file"
done
```

**❌ DON'T:**
```bash
# Bad: Detect in every iteration
for file in *.txt; do
  claude-vault detect
  claude-vault exec process "$file"
done
```

### Shell Integration

**✅ DO:**
Load profile once at shell startup:

```bash
# .bashrc or .zshrc
if command -v claude-vault &> /dev/null; then
  eval $(claude-vault env 2>/dev/null) || true
fi
```

**❌ DON'T:**
```bash
# Bad: Load on every prompt
PROMPT_COMMAND='eval $(claude-vault env)'
```

### Batch Operations

**✅ DO:**
```bash
# Good: Export once, run multiple commands
eval $(claude-vault env --profile work)
npm test
npm run lint
npm run build

# Or use exec wrapper
claude-vault exec --profile work bash -c '
  npm test
  npm run lint
  npm run build
'
```

**❌ DON'T:**
```bash
# Bad: Separate exec for each command
claude-vault exec --profile work npm test
claude-vault exec --profile work npm run lint
claude-vault exec --profile work npm run build
```

## Maintenance

### Regular Audits

**✅ DO:**
Periodically review profiles:

```bash
# Monthly profile audit
claude-vault list

# Check for:
# - Unused profiles (old last_used)
# - Unclear descriptions
# - Test/temporary profiles

# Clean up
claude-vault remove unused-profile --yes
```

### Key Rotation

**✅ DO:**
Rotate keys regularly:

```bash
# Every 90 days
# 1. Generate new key from Anthropic dashboard
# 2. Remove old profile
claude-vault remove work --yes

# 3. Add with new key
claude-vault add work --description "Work profile (rotated: 2024-04-01)"

# 4. Test
claude-vault detect
claude-vault exec --profile work claude --version
```

### Backup Strategy

**✅ DO:**
```bash
# Backup config (without keys)
cp ~/.claude-vault/config.toml ~/.claude-vault/config.toml.backup

# Export profile list
claude-vault list > ~/.claude-vault/profiles-backup.txt
```

**❌ DON'T:**
```bash
# Bad: Backup keychain data
# Keys should stay in keychain only

# Bad: Backup to cloud storage
# Config file alone (without keys) is safe to backup
```

## Next Steps

- [FAQ](FAQ.md)
- [Usage Guide](USAGE_GUIDE.md)
- [Troubleshooting Guide](TROUBLESHOOTING.md)
- [Claude Code Integration](CLAUDE_CODE_INTEGRATION.md)
