---
name: check
description: Run the full validation loop for the spacey crate — cargo fmt, cargo clippy with warnings-as-errors, then cargo test. Use after making code changes to verify they format, lint clean, and pass tests.
---

Run the project's validation loop, in this order. Stop and report at the first failing step; do not continue past a failure.

1. `cargo fmt` — apply formatting.
2. `cargo clippy -- -D warnings` — lint; **warnings are errors**. Fix any reported issues before proceeding.
3. `cargo test` — run the test suite (includes `test_collision_logic` and `test_update_performance` in `src/tui/game.rs`).

If all three pass, report success concisely. If any step fails, show the relevant output and fix the underlying issue, then re-run from `cargo fmt`.
