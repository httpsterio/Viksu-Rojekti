# Claude Code Instructions

## Do not act without being asked

A session starting with context or a summary is not an instruction to do work. Wait for an explicit request. If it's unclear what the user wants, ask.

## Check git status before touching anything

Before making any edits, run `git status`. If there are uncommitted changes, understand what they are before writing on top of them. Do not modify files that have existing uncommitted work without calling it out first.

## Do not start implementation tasks without confirmation

For anything beyond a trivial fix, state what you plan to do and wait for a go-ahead. This applies especially to multi-file changes.

## Never commit without being explicitly asked

Do not stage or commit files unless the user asks. Do not amend existing commits.

## When verifying code, trace the full user journey

Do not read functions in isolation. Instead, walk through the feature as a user would experience it: the user takes an action, the frontend responds with specific state, that state is passed to the backend, the backend processes it through each function in sequence, and the final state on disk matches what the user expected.

At each handoff, ask what value is actually being passed and whether the receiving code still holds the assumptions it was written with. Mutations earlier in the chain often invalidate assumptions later — this is only visible by tracing the full flow.

---

## Project conventions

- Tauri commands live in `src-tauri/src/commands.rs`, file I/O in `storage.rs`, models in `models.rs`
- All Vue state lives in `src/composables/useBoard.ts` — components never call `invoke()` directly
- No `any` in TypeScript
- No comments that explain what code does — only why, and only when it's non-obvious
- Run `npm run dev` and builds from Windows, not WSL

## Dev environment

- WSL is used for editing and git, but `npm install` and `npm run dev` must be run from Windows due to native bindings
