# Rojekti Marketing Strategy & Content

**Company:** Viksu  
**Product:** Rojekti (Local-first Kanban for Developers & Privacy-Seekers)

---

## 🎯 Core Value Propositions
1. **Ownership**: Your data never leaves your disk. No cloud, no subscription, no outages.
2. **Hybrid Workflow**: A beautiful PrimeVue GUI for visual planning + a high-speed Rust CLI for terminal productivity.
3. **Transparent Data**: Everything is a Markdown file with YAML frontmatter. Readable by humans, indexable by machines.
4. **LLM Ready**: Designed to be manipulated by AI agents via the CLI while humans use the GUI.

---

## 📱 Social Media Content (LinkedIn/Medium Style)

### Post 1: The Death of Cloud-Lock-In
**Headline: Is your productivity tool holding your data hostage?**

We’ve all been there. You open your project management tool, only to see a "503 Service Unavailable" or a "Maintenance" screen. Or worse, you realize your sensitive project roadmap is sitting on someone else’s server, subject to their privacy policy changes and price hikes.

At **Viksu**, we believe you should own your work. That’s why we built **Rojekti**. 

Rojekti is a local-first Kanban board. There is no "Syncing..." spinner because there is no cloud. Your cards are stored as simple Markdown files on your own machine. 

Why does this matter?
- **Speed**: Local disk I/O is always faster than a REST API.
- **Privacy**: Your business secrets stay on your hardware.
- **Longevity**: 10 years from now, even if we vanished, you could still read your cards with any text editor.

Stop renting your workspace. Own it. 

#LocalFirst #Productivity #Privacy #Kanban #Viksu #Rojekti

---

### Post 2: Terminal Flow vs. Visual Overview
**Headline: Why choose between a CLI and a GUI?**

Developers live in the terminal. Managers live in the browser. Usually, that means a constant friction of context-switching or outdated boards. 

With **Rojekti**, we’ve bridged the gap. 

Built with **Tauri** and **Rust**, Rojekti provides a lightning-fast CLI for the moments you're deep in code.
`$ rojekti create --title "Fix race condition in watcher" --priority critical`

But when it’s time for the weekly standup or a deep-dive planning session, the **Rojekti GUI** provides a beautiful, drag-and-drop Kanban experience powered by PrimeVue. 

The best part? They share the same heartbeat. Every change in the CLI is reflected in the GUI in real-time thanks to our native file-watching system. 

Productivity isn't about the tool you use; it's about the tool that gets out of your way.

#RustLang #WebDev #CLI #Kanban #DeveloperExperience #Viksu

---

### Post 3: Markdown is the New Database
**Headline: Your project board should be as readable as your code.**

Most project management tools use proprietary databases or complex JSON blobs hidden behind an API. If you want to run a custom script against your data, you’re stuck learning their rate limits and authentication schemes.

**Rojekti** takes a different approach: **Markdown is the database.**

Every card on your Rojekti board is a `.md` file. The metadata (status, priority, epic) lives in a clean YAML frontmatter. The description is just... Markdown.

This opens up a world of "Meta-Productivity":
- **Grep your board**: Find every card mentioning "API" across all lanes in milliseconds.
- **Version control**: Put your `rojekti/` folder in Git. See exactly who moved a card and when.
- **AI-Collaboration**: AI agents can read and write to your board via the CLI, acting as an automated project assistant that actually understands your files.

It’s simple. It’s transparent. It’s Rojekti.

#Markdown #Git #OpenSource #ProductivityHacks #AI #Viksu

---

## 💡 Marketing Ideas & Growth Channels

### 1. The "Dotfile" Community
Market Rojekti as part of a user's "Second Brain" or "Dotfiles." Encourage users to share screenshots of their board themes and their `rojekti.config.yaml` on subreddits like `r/unixporn` or `r/productivity`.

### 2. "Build in Public" Log
Since Rojekti is local-first, create a public GitHub repository where the Rojekti team’s *actual* Rojekti board is committed daily. People can see the progress of the app by reading the Markdown files themselves.

### 3. Developer Influencer Outreach
Reach out to "No-Cloud" and "Privacy" advocates in the tech space. Offer them a tool that respects their data and fits their terminal-heavy workflow.

### 4. Integration Templates
Create a series of "Workflow Templates" (e.g., a Template for Indie Hackers, a Template for Creative Writers, a Template for Security Researchers) that users can download and drop into their `rojekti/` folder to get started instantly.

### 5. AI Agent Integration
Write a guide on how to hook up an LLM (like GPT-4 or Claude) to the Rojekti CLI. Show users how they can have an AI "summarize the backlog" or "auto-assign priorities" based on card descriptions. 
