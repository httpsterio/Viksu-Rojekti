# Prompt: LinkedIn Posts for Rojekti / Viksu

Use this prompt verbatim with Gemini or any LLM.

---

## PROMPT

Write 12 LinkedIn posts for a product called **Rojekti** by **Viksu**.

**What it is:**
Rojekti (Finnish: "Fiksu Projekti" — smart project) is a local-first, offline kanban board for
developers and solo operators. It's a desktop app built with Tauri (Rust + Vue). Cards are stored
as plain Markdown files with YAML frontmatter on your own filesystem — no cloud, no subscription,
no account. The same executable works as both a full GUI and a CLI, so boards can be read and
manipulated from the terminal or by AI agents directly.

**Core philosophy:**
- Your data is files. Always. A card is just a `.md` file you can open in any text editor.
- The CLI and the GUI are equal citizens. An AI agent can manage your board the same way a human
  can.
- Built for the solo operator or small team that finds Trello too shallow and Jira too heavy.
- File-based storage means your board is git-committable, diffable, scriptable, and survives any
  company shutting down.

**Audience:**
Developers, indie hackers, AI builders, solo founders, people who use tools like Obsidian,
Neovim, or spend time in the terminal. People who are tired of SaaS project management.

**Tone:**
- Insightful, direct, slightly opinionated. Not hype.
- Sounds like a thoughtful developer sharing something they built and believe in — not a marketing
  department.
- Short paragraphs. No corporate filler. No excessive emoji.
- Can reference specific technical decisions and explain the reasoning behind them like a dev
  would on a blog.
- Occasional dry wit is fine.

**Post variety — cover all of these angles, one or two posts each:**

1. **The "why files" argument** — Why storing cards as Markdown files on disk is a feature, not a
   limitation. Portability, longevity, git history, zero lock-in.

2. **Local-first as a philosophy** — The distinction between "offline mode" (a degraded cloud app)
   and genuinely local-first (the cloud is the degraded mode). What you gain when your tool
   doesn't need the internet to exist.

3. **CLI + GUI as equals** — Most desktop tools treat the CLI as an afterthought. In Rojekti, an
   AI agent and a human operator have identical capabilities over the same board. What that
   enables.

4. **AI/agent collaboration angle** — Cards are plain files. An AI agent can read your board,
   pick a task, do the work, update the card status, and leave notes — all without a plugin or
   API key. Why this matters for the way developers are starting to work.

5. **The Trello-to-Jira trap** — The gap between "too simple" and "too complex" in project
   management. Why most teams swing between the two and never find the right level of structure.
   What Rojekti tries to be instead.

6. **A specific technical decision** — The app stores cards as Markdown with YAML frontmatter.
   The same file format used by static site generators, Obsidian, Jekyll. This was deliberate.
   Your cards can be read by any tool in that ecosystem. Talk about the reasoning.

7. **"Your data survives us"** — SaaS tools shut down. Exports are lossy. What it means to build
   a tool where the company going under changes nothing about whether you can access your work.

8. **The single-user problem** — Project management tools are almost universally designed around
   teams. The mental overhead of @mentions, comments, activity feeds, and notifications is
   designed for coordination — but most of the time, it's just noise for someone working alone.
   What a tool built for one person can do differently.

9. **Real-time sync without a server** — Rojekti uses a filesystem watcher. If an AI agent edits
   a card via CLI while the GUI is open, the board refreshes automatically. No websocket, no
   server, no sync conflict UI. Just files and OS-level file events. Explain why this is
   interesting.

10. **The name** — Rojekti means "project" in Finnish slang. Viksu means "clever/smart." Why
    naming things in your own language is a small act of not pretending to be something you're
    not. (Keep this one short and a bit personal.)

11. **What "done" looks like** — A card is a `.md` file. When a feature ships, that card's status
    field changes to `done`. It's still there, readable, git-committable. You have a permanent
    record of every decision and note without a separate documentation system.

12. **An honest take on where it's at** — Rojekti is early. It's built by one person who wanted
    this tool to exist. It's not trying to replace Notion or Linear. It's trying to be the
    right tool for a specific kind of person who prefers their filesystem over someone else's
    database.

**Format for each post:**
- 150–280 words
- No headers or bullet points inside the post (flowing paragraphs only)
- End with a single low-key call to action or open question where it fits naturally — not
  "check out the link in bio" energy, more like a genuine prompt for discussion
- Label each post clearly: `## Post 1: [angle name]`

Do not write all 12 in the same voice or structure. Vary the opening hooks. Some can start with
a statement, some with a question, some with a short observation. Make them feel like they were
written on different days by someone who keeps thinking about the same problems from new angles.
