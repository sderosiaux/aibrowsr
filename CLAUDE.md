# aibrowsr

Single Rust binary for browser automation via CDP. Built for AI agents.
~3.5K lines Rust, zero runtime dependencies.

## Architecture

```
CLI (clap) → CDP Client (WebSocket) → Chrome
```

- `src/cdp/` — WebSocket transport, message correlation, CDP types
- `src/commands/` — goto, click, fill, inspect, eval, text, wait, screenshot, tabs, etc.
- `src/element.rs` — uid/selector/coordinate resolution → CDP input dispatch
- `src/element_ref.rs` — ElementRef abstraction (decouples from CDP internals)
- `src/snapshot.rs` — Accessibility tree → compact text with uid identifiers
- `src/session.rs` — JSON session persistence (~/.aibrowsr/sessions.json)
- `src/browser.rs` — Chrome launch, auto-discovery, profile management
- `src/daemon.rs` — Optional micro-daemon (Unix only), heartbeat, crash recovery
- `npm/` — npm distribution wrapper (postinstall downloads native binary)

## Build & Test

```bash
cargo build
cargo test
cargo clippy -- -D warnings  # zero warnings policy
```

## Release

```bash
./scripts/release.sh 0.2.0
# → bumps version, tags, pushes
# → GitHub Actions builds 5 platform binaries
# → Creates GitHub Release
# → Publishes to npm (needs NPM_TOKEN secret)
```

## Key Design Decisions

- **Headless by default** — `--headed` for debug. Agents never see Chrome windows.
- **3 targeting modes** — uid (from inspect), CSS selector (`--selector`), coordinates (`--xy`)
- **ElementRef abstraction** — session stores `{"type":"backendNode","id":N}` not raw CDP ids
- **inspect filters** — StaticText/InlineTextBox stripped by default (66% token reduction)
- **--json mode** — structured output, errors exit 0 with `{"ok":false}` for agent parsing
- **Session reuse** — checks WebSocket connectivity, kills headed browser if agent wants headless
- **Self-healing errors** — every error includes a `hint` suggesting the next action

## Gotchas

- CDP `rename_all = "camelCase"` fails on acronyms: use `#[serde(rename = "backendDOMNodeId")]`
- Browser-level WebSocket only supports `Target.*`. Page-level commands need `/json/list` → page WS.
- `Accessibility.getFullAXTree` returns a flat list with parentId/childIds, not a tree.
- Some AXRelatedNode fields may be missing (use `Option<T>` + `#[serde(default)]` everywhere).

## Linting

Zero warnings enforced. Clippy pedantic + nursery enabled with targeted suppressions in Cargo.toml.
Warnings = errors in CI (`cargo clippy -- -D warnings`).
