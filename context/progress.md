# Progress

## Session 1 — 2026-02-04

### What was done
- Set up the initial project: working CLI chat loop against Anthropic via Rig using `CLAUDE_3_5_HAIKU`.
- Added `/model` command — presents a numbered menu of available Anthropic models and rebuilds the agent on selection. Taught the builder pattern (`client.agent().preamble().build()`) and that the agent is immutable once built.
- Added `/history` command — prints the last 10 messages with truncation at 300 chars. Required digging into `Message`, `UserContent`, and `AssistantContent` enums to pattern-match and extract text. Learned that there are a lot of abstractions/variants between you and the raw message text.
- Added `.env.example`, `README.md`, `LICENSE` (MIT), and `context/project.md`.

### What was learned
- Rig's `Message` type is an enum with nested content enums (`UserContent`, `AssistantContent`) — getting to the actual text string requires matching through multiple layers.
- The agent is built via a builder and is not mutable after construction; swapping models means rebuilding it.
- The available Anthropic model constants live in `rig::providers::anthropic::completion`.

### Fixed
- Added `user_input.clear()` on the empty-input `continue` path — no more input accumulation.
- Moved `user_input.clear()` earlier (before the `agent.chat()` call) so it's always cleared regardless of path.
- Added `continue` after `/model` and `/history` blocks so they no longer fall through to the regular message send.

### Added later in session
- `/clear` command to reset chat history.
- `/help` command — lists all available commands with descriptions.
- `/history` improvements: shows empty-history message when there's nothing to show, re-reversed the `.rev().take().rev()` so messages display in chronological order, added `---` separators between messages.
- Fixed the `document` typo in the `UserContent::Document` branch.

### Ideas for next time
- Streaming responses — next Rig API surface worth exploring.

## Session 2 — 2026-02-05

### What was done
- Extracted configuration logic into `Config` module — removed model management from Config, simplified to just preamble and API key storage.
- Added `/summarize` command — asks the agent to summarize the conversation history.
- Created `ChatState` module to encapsulate chat history and user input management.
- Major architectural refactor: implemented command pattern with individual command modules (`commands/exit.rs`, `commands/model.rs`, etc.).
- Created `Input` enum for clean command routing and pattern matching.
- Extracted `Runner` module to handle main loop logic, separated from `main.rs`.
- Created `ui` module for shared UI utilities (`horizontal_line()`).
- Fixed integer underflow bug in model selection (input "0" would panic).
- Fixed input handling to preserve input across command interruptions (allows breaking out of `/model` selection to handle other commands).

### What was learned
- Command pattern in Rust: each command implements `Command` trait with async `execute()` method.
- Module organization: separating concerns into `chat/`, `commands/`, `runner`, `config`, and `ui`.
- Using enums for input routing makes the main loop clean and exhaustive.
- `checked_sub()` for safe integer arithmetic to avoid underflow panics.
- How to handle nested input loops (model selection) while preserving input state for parent loop.

### Architecture evolution
- **Before**: Monolithic `main.rs` with all logic inline (~220 lines).
- **After**:
  - `main.rs`: 13 lines, just bootstraps `Runner::run()`.
  - `chat/mod.rs`: `State` struct managing history, input, agent, model.
  - `chat/input.rs`: `Input` enum for command parsing.
  - `commands/`: Individual modules per command implementing `Command` trait.
  - `runner.rs`: Main loop logic with pattern matching on `Input`.
  - `config.rs`: Environment config (preamble, API key).
  - `ui.rs`: UI utilities.

### Polish & documentation
- Updated `README.md` with all available commands.
- Reorganized context files: `project.md` → `purpose.md`, moved commit guidelines to `context/commit_guidelines.md`.
- Refined `purpose.md` to focus on AI assistant rules and project philosophy (no code writing, educational first).

### Refinements
- Simplified `/exit` command — removed AI-generated farewell, now just prints "Farewell!" and breaks the loop.
- Split `Command` trait into `Command` (sync) and `AsyncCommand` (async) to reduce compile times for commands that don't need async.
- Updated UI styling: width from 100 to 50, horizontal line from `=` to `-`.
- Alphabetized command matching in runner for better organization.

### Ideas for next time
- Streaming responses — explore Rig's streaming API for real-time output.
- Tool use / function calling — if Rig supports it, add tools the agent can call.
- Export/import chat — save and load conversation history to/from JSON.
