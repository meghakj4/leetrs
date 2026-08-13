# ADR-002: Cookie-Based Browser Authentication Strategy

## Status
Accepted

## Date
2026-08-13

## Context
LeetCode requires session authentication for querying user profile data, fetching GraphQL problem data, testing solutions against judge servers, and submitting official solutions.
LeetCode does not provide an official API token system for third-party CLIs. Authentication relies on two HTTP cookies: `LEETCODE_SESSION` and `csrftoken`.

Key requirements:
1. Low friction onboarding for users who are already logged into LeetCode in their web browser.
2. Secure credential storage on the local machine.
3. Fallback support for containerized (Snap/Flatpak), custom browser profiles, or headless server environments.

## Decision
Use automatic browser cookie extraction via the `rookie` Rust crate as the primary authentication mechanism for Chrome and Firefox, backed by an interactive manual token entry fallback (`dialoguer`).

Extracted or entered credentials are saved locally in JSON format (`~/.config/leetrs/credentials.json`).

## Alternatives Considered

### 1. Username/Password Login Prompts
- Pros: Direct authentication from the CLI.
- Cons: LeetCode uses Cloudflare bot protection, CAPTCHA challenges, and OAuth (Google/GitHub) logins which break headless password authentication.
- Rejected: Fragile and high risk of being blocked by anti-bot measures.

### 2. Chrome Extension Helper
- Pros: Native browser extension can relay cookies to a local localhost server.
- Cons: Requires users to install a custom browser extension outside standard package managers.
- Rejected: Adds unnecessary setup friction compared to direct SQLite cookie extraction via `rookie`.

## Consequences
- Automatic cookie extraction works seamlessly on macOS, Linux, and Windows for standard Chrome and Firefox browser installations.
- Users on containerized browsers (Snap/Flatpak) or custom profiles use the manual paste option (`leetrs auth` -> "Paste tokens manually").
- Active sessions persist until cookies expire on LeetCode's backend.
