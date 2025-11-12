# Phase 6: OAuth Token Support

## Status

- ✅ **Phase 6.1**: Core OAuth Support (COMPLETED)
- ✅ **Auto-refresh**: Token auto-refresh implemented (COMPLETED)
- ⏳ **Phase 6.2**: Environment Variable Support (PLANNED)
- ⏳ **Phase 6.3**: Linux Support (PLANNED)
- ⏳ **Phase 6.4**: Independent OAuth Flow (FUTURE)
- ⏳ **Phase 6.5**: Token Management UI (FUTURE)

## Overview

Add support for Claude.ai subscription (Pro/Max) OAuth tokens in addition to API keys, enabling users without API keys to use claude-vault.

## Problem Statement

**Current Limitation:**
- claude-vault only supports API key authentication (`ANTHROPIC_API_KEY`)
- Claude Pro/Max subscription users cannot use claude-vault
- OAuth token management is manual and error-prone

**User Need:**
- Store and manage OAuth tokens securely
- Auto-detect OAuth vs API key credentials
- Seamless integration with Claude Code OAuth login flow
- Support for token refresh/expiration

## Authentication Methods Comparison

| Method | Access | Storage | Expiration | Use Case |
|--------|--------|---------|------------|----------|
| API Key | Console PAYG | Keychain | Never | Programmatic access, CI/CD |
| OAuth Token | Subscription (Pro/Max) | Keychain + JSON | Yes (refresh) | Personal CLI usage |
| Bedrock/Vertex | Cloud provider | AWS/GCP creds | Provider-managed | Enterprise cloud |

## Technical Design

### 1. Credential Types

**New Enum:**
```rust
pub enum CredentialType {
    ApiKey,
    OAuthToken,
    // Future: Bedrock, Vertex
}

pub struct Credential {
    cred_type: CredentialType,
    value: String,
    metadata: CredentialMetadata,
}

pub struct CredentialMetadata {
    created_at: DateTime<Utc>,
    expires_at: Option<DateTime<Utc>>,
    refresh_token: Option<String>,
}
```

### 2. Storage Strategy

**macOS Keychain:**
- API Keys: `claude-vault-{profile}` (existing)
- OAuth Tokens: `claude-vault-oauth-{profile}`
- Refresh Tokens: `claude-vault-refresh-{profile}`

**Config File** (`~/.claude-vault/config.toml`):
```toml
[[profiles]]
name = "personal"
credential_type = "oauth"  # New field
description = "Personal Pro account"
created_at = "2024-01-15T12:00:00Z"
```

**Linux Support** (planned):
```json
// ~/.claude/.credentials.json format (Claude Code compatible)
{
  "oauth_tokens": {
    "personal": {
      "access_token": "...",
      "refresh_token": "...",
      "expires_at": "2024-01-15T13:00:00Z"
    }
  }
}
```

### 3. OAuth Flow Integration

**Two Approaches:**

#### Option A: Delegate to Claude Code (MVP)
```bash
# User workflow
claude /login              # Claude Code handles OAuth
claude-vault import oauth  # Import from Claude Code's storage
```

**Pros:**
- No need to implement OAuth flow
- Reuses Claude Code's existing auth
- Simple implementation

**Cons:**
- Requires Claude Code installed
- Limited control over token lifecycle

#### Option B: Independent OAuth Flow (Future)
```bash
# User workflow
claude-vault login personal  # Starts OAuth flow
# Opens browser to claude.ai/oauth/authorize
# Callback receives token
# Stores in keychain
```

**Pros:**
- Works without Claude Code
- Full control over auth lifecycle
- Better UX

**Cons:**
- Complex OAuth implementation
- Needs OAuth client credentials
- Token refresh logic required

### 4. Environment Variable Injection

**Current:**
```bash
export ANTHROPIC_API_KEY="sk-ant-..."
```

**With OAuth:**
```bash
# Option 1: Use same variable (Claude Code supports both)
export ANTHROPIC_API_KEY="oauth-token-..."

# Option 2: Use separate variable
export ANTHROPIC_OAUTH_TOKEN="oauth-token-..."
export ANTHROPIC_AUTH_TYPE="oauth"
```

**Investigation needed:**
- Which environment variables does Claude Code check?
- Does it auto-detect token type?

### 5. Command Changes

**New Commands:**

```bash
# Import OAuth token from Claude Code
claude-vault import oauth [--profile NAME]

# Login with OAuth (future)
claude-vault login [--profile NAME]

# Show credential type
claude-vault show personal
# Output:
# Profile: personal
# Type: OAuth Token
# Expires: 2024-01-15T13:00:00Z
# Description: Personal Pro account

# Refresh token (future)
claude-vault refresh personal
```

**Modified Commands:**

```bash
# Add now supports credential type
claude-vault add personal --type oauth
claude-vault add work --type api-key  # Explicit

# Exec auto-detects credential type
claude-vault exec claude "hello"
```

## Implementation Roadmap

### Phase 6.1: Core OAuth Support (2-3 days)

**Goals:**
- Add CredentialType enum and metadata
- Extend Keychain storage for OAuth tokens
- Import OAuth tokens from Claude Code storage
- Update profile commands to show credential type

**Tasks:**
1. ✅ Define data structures (Credential, CredentialType)
2. ✅ Update Profile struct with credential_type field
3. ✅ Extend keychain module for OAuth tokens
4. ✅ Implement `import oauth` command
5. ✅ Update `show` command to display credential type
6. ✅ Add expiration warnings
7. ✅ Update tests

**Deliverables:**
- Users can import OAuth tokens from Claude Code
- OAuth tokens stored securely in keychain
- Credential type visible in profile list
- Expiration warnings when tokens expire soon

### Phase 6.2: Environment Variable Support (1 day)

**Goals:**
- Inject OAuth tokens correctly for Claude Code
- Auto-detect credential type
- Handle token expiration

**Tasks:**
1. ✅ Research Claude Code environment variable usage
2. ✅ Update `env` command for OAuth tokens
3. ✅ Update `exec` command for OAuth tokens
4. ✅ Add token expiration checks
5. ✅ Update documentation

**Deliverables:**
- `claude-vault exec` works with OAuth tokens
- `claude-vault env` exports OAuth tokens correctly
- Expired token warnings

### Phase 6.3: Linux Support (2 days)

**Goals:**
- Read/write `~/.claude/.credentials.json`
- Cross-platform credential storage
- Claude Code compatibility

**Tasks:**
1. ✅ Implement JSON credential file handler
2. ✅ Add Linux-specific storage backend
3. ✅ Ensure compatibility with Claude Code format
4. ✅ Platform detection and fallback
5. ✅ Update tests for Linux

**Deliverables:**
- Linux users can use OAuth tokens
- Compatible with Claude Code's credential format
- Seamless platform switching

### Phase 6.4: Independent OAuth Flow (Future - 5-7 days)

**Goals:**
- Implement OAuth 2.0 flow
- Browser-based login
- Token refresh logic
- Remove Claude Code dependency

**Tasks:**
1. ⏳ Register OAuth application with Anthropic
2. ⏳ Implement OAuth 2.0 authorization code flow
3. ⏳ Add local HTTP server for callback
4. ⏳ Implement token refresh
5. ⏳ Add `login` command
6. ⏳ Add `refresh` command
7. ⏳ Handle headless environments (SSH, Docker)
8. ⏳ Comprehensive testing

**Deliverables:**
- `claude-vault login` starts OAuth flow
- Automatic token refresh
- Works without Claude Code
- Headless mode support

### Phase 6.5: Token Management (2 days)

**Goals:**
- Automatic token refresh
- Expiration monitoring
- Token lifecycle management

**Tasks:**
1. ⏳ Background token refresh daemon
2. ⏳ Expiration notifications
3. ⏳ Auto-refresh before expiration
4. ⏳ Token revocation support
5. ⏳ Audit logging

**Deliverables:**
- Tokens automatically refreshed
- Users notified of expiration
- Comprehensive token lifecycle management

## Technical Challenges

### 1. OAuth Client Registration

**Challenge:** Need OAuth client ID/secret from Anthropic

**Solutions:**
- Use Claude Code's OAuth client (if allowed)
- Register our own OAuth application
- Document the registration process

### 2. Token Refresh

**Challenge:** Refresh tokens expire and need rotation

**Solutions:**
- Store refresh token separately
- Implement refresh before expiration
- Handle refresh token expiration gracefully

### 3. Headless Environments

**Challenge:** OAuth requires browser for login

**Solutions:**
- SSH port forwarding (documented)
- Device code flow (if supported)
- Manual token paste option
- Pre-authenticated token import

### 4. Cross-Platform Storage

**Challenge:** Different storage mechanisms per platform

**Solutions:**
- Abstract storage interface
- Platform-specific implementations
- Fallback to file-based storage

### 5. Claude Code Compatibility

**Challenge:** Must work alongside Claude Code

**Solutions:**
- Read Claude Code's credential format
- Don't interfere with Claude Code's storage
- Document coexistence strategies

## Security Considerations

### Token Storage

**Best Practices:**
- ✅ Use platform keychain when available
- ✅ Encrypt file-based storage
- ✅ Restrict file permissions (600)
- ✅ Never log tokens
- ✅ Clear tokens on removal

### Token Handling

**Best Practices:**
- ✅ Never expose tokens in process list
- ✅ Use environment variables carefully
- ✅ Automatic token clearing on exit
- ✅ Secure token transmission

### Refresh Token Protection

**Critical:**
- Refresh tokens are long-lived
- Must be protected even more than access tokens
- Consider separate keychain entries
- Implement rotation

## User Experience

### Simple Workflow (MVP)

```bash
# Step 1: Login to Claude Code
claude /login
# Browser opens, user authenticates with Pro/Max account

# Step 2: Import to claude-vault
claude-vault import oauth --profile personal
# ✓ OAuth token imported from Claude Code
# ✓ Profile 'personal' created

# Step 3: Use normally
cd ~/my-project
claude-vault init personal
claude-vault exec claude "hello world"
```

### Future Workflow (Independent OAuth)

```bash
# Step 1: Login via claude-vault
claude-vault login personal
# Browser opens to claude.ai/oauth/authorize
# User authenticates with Pro/Max account
# ✓ OAuth token stored securely

# Step 2: Use normally
cd ~/my-project
claude-vault init personal
claude-vault exec claude "hello world"

# Token automatically refreshed when needed
```

## Testing Strategy

### Unit Tests

- ✅ CredentialType serialization
- ✅ OAuth token storage/retrieval
- ✅ Token expiration checks
- ✅ Credential type detection

### Integration Tests

- ✅ Import from Claude Code storage
- ✅ Environment variable injection
- ✅ Exec command with OAuth tokens
- ✅ Cross-platform compatibility

### Manual Testing

- ✅ Real Claude Code integration
- ✅ Browser OAuth flow
- ✅ Token refresh
- ✅ Expiration handling
- ✅ Multi-platform (macOS, Linux)

## Documentation Updates

### New Docs

- `docs/OAUTH_GUIDE.md` - OAuth authentication guide
- `docs/OAUTH_TROUBLESHOOTING.md` - OAuth-specific issues

### Updated Docs

- `README.md` - Add OAuth support to features
- `docs/USAGE_GUIDE.md` - OAuth workflows
- `docs/CLAUDE_CODE_INTEGRATION.md` - OAuth integration
- `docs/FAQ.md` - OAuth questions
- `docs/BEST_PRACTICES.md` - OAuth security

## Success Metrics

- ✅ Users with Pro/Max can use claude-vault
- ✅ OAuth tokens stored securely
- ✅ Seamless Claude Code integration
- ✅ Token refresh works automatically
- ✅ Documentation comprehensive
- ✅ Zero OAuth-related security issues

## Alternative Approaches Considered

### 1. Only Support API Keys

**Rejected:** Excludes subscription users, main use case

### 2. Wrapper Around Claude Code

**Rejected:** Too dependent on Claude Code implementation

### 3. Require Manual Token Entry

**Considered for MVP:** Simple but poor UX

### 4. Independent OAuth Implementation

**Future Goal:** Best UX but complex

## Dependencies

### External

- Claude Code (for import command)
- Anthropic OAuth endpoint
- Platform keychain APIs

### Internal

- Existing keychain module
- Profile management
- Config file format

## Timeline

### Phase 6.1: Core OAuth Support
- **Duration:** 2-3 days
- **Priority:** P0 (Critical)
- **Blockers:** None

### Phase 6.2: Environment Variable Support
- **Duration:** 1 day
- **Priority:** P0 (Critical)
- **Blockers:** Phase 6.1

### Phase 6.3: Linux Support
- **Duration:** 2 days
- **Priority:** P1 (High)
- **Blockers:** Phase 6.1

### Phase 6.4: Independent OAuth Flow
- **Duration:** 5-7 days
- **Priority:** P2 (Medium)
- **Blockers:** OAuth client registration

### Phase 6.5: Token Management
- **Duration:** 2 days
- **Priority:** P2 (Medium)
- **Blockers:** Phase 6.4

**Total Timeline:**
- **MVP (Phases 6.1-6.3):** 5-6 days
- **Full (All phases):** 10-14 days

## Questions to Resolve

1. ✅ How does Claude Code store OAuth tokens on macOS?
   - Answer: macOS Keychain

2. ✅ What environment variables does Claude Code check?
   - Answer: ANTHROPIC_API_KEY (supports both API key and OAuth token)

3. ⏳ Can we reuse Claude Code's OAuth client?
   - Need to: Contact Anthropic or reverse engineer

4. ⏳ What's the token refresh flow?
   - Need to: Test with real tokens

5. ⏳ How to handle headless environments?
   - Partial answer: SSH port forwarding documented

6. ⏳ Token expiration time?
   - Need to: Test with real tokens

## Next Steps

1. ✅ Get approval for Phase 6 plan
2. ✅ Start Phase 6.1 implementation
3. ⏳ Research Claude Code's OAuth storage format
4. ⏳ Implement OAuth token import
5. ⏳ Test with real Claude Pro/Max account
6. ⏳ Document OAuth usage
7. ⏳ Plan Phase 6.4 OAuth client registration

## References

- [Claude Code IAM Docs](https://code.claude.com/docs/en/iam)
- [Claude Code OAuth Issues](https://github.com/anthropics/claude-code/issues?q=oauth)
- [OAuth 2.0 RFC](https://datatracker.ietf.org/doc/html/rfc6749)
- [Keychain Services Programming Guide](https://developer.apple.com/documentation/security/keychain_services)
