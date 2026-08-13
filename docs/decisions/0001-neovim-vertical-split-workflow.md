# ADR-001: Automatic Neovim Vertical Split Workflow

## Status
Accepted

## Date
2026-08-13

## Context
When developers use LeetCode on the web, they constantly context-switch between browser windows (reading problem statements, viewing example inputs/outputs) and their IDE or text editor.
`leetrs` aims to provide a frictionless, terminal-native LeetCode workflow for terminal users, specifically targeting Neovim as the primary editor.

Key requirements:
1. Display the problem description (markdown) and code stub (solution file) simultaneously without manual pane management.
2. Maintain compatibility with standard terminal multiplexers (tmux) and standalone terminal emulators.
3. Allow configurable fallback to other editors (e.g., VS Code, Vim) for users who do not use Neovim.

## Decision
Implement automatic vertical split (`vsplit`) launching via Neovim command-line flags:

```bash
nvim <snake_slug>.md -c "vsplit <snake_slug>.<ext>"
```

The left pane displays `<snake_slug>.md` (80-column wrapped Markdown problem description), while the right pane displays `<snake_slug>.<ext>` (pre-populated code template with metadata header comment).

For non-Vim editors (e.g., `code`), both file paths are passed as positional arguments.

## Alternatives Considered

### 1. Internal TUI Split Editor
- Pros: Keeps the entire experience inside Ratatui without launching subprocesses.
- Cons: Re-inventing text editing in a custom TUI widget lacks LSP integration, Neovim keymaps, treesitter syntax highlighting, and personal editor plugins.
- Rejected: Developer productivity relies heavily on personal editor configuration and language servers.

### 2. Dual Terminal Windows
- Pros: Simple command launch per file.
- Cons: Spawns multiple terminal windows/tabs, cluttering desktop workspaces and losing cohesive context.
- Rejected: Poor user experience compared to a single side-by-side view.

## Consequences
- Requires `nvim` (0.9+) binary available in system `$PATH` for the default experience.
- Metadata header comments (`// id=... slug=... lang=...`) inside code stubs are required to link solution files back to LeetCode problem IDs during `leetrs test` and `leetrs submit`.
- Users gain full access to their personal Neovim plugins, keybindings, and LSP server features while solving problems.
