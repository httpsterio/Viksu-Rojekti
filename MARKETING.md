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

---

## 📑 12-Post LinkedIn Campaign

## Post 1: The "why files" argument

Database lock-in is a choice we’ve been conditioned to accept as inevitable. We assume that if we want a Kanban board, we have to upload our entire roadmap to a third-party server, sign up for a subscription, and pray their API stays stable. But your project data shouldn't be a row in someone else's SQL database. It should be a file on your disk.

When I started building Rojekti, the first decision was that every card is just a Markdown file with YAML frontmatter. This sounds like a limitation until you realize it’s a superpower. Because your board is just a folder of files, it is instantly portable. You can open a card in Neovim, VS Code, or even Notepad if you’re having a bad day. You can use Grep to search your entire backlog in milliseconds. You can version control your board with Git and see exactly when a task moved from doing to done.

Longevity is the real goal here. If Viksu as a company disappeared tomorrow, your board would still work. There is no export process because there is no proprietary format to export from. Your work belongs to you, and it stays on your filesystem where it belongs. It’s a simple shift, but once you start managing your projects as files, going back to a cloud-only SaaS feels like stepping into a cage.

How often do you actually audit the longevity of the tools you use for your most important work?

## Post 2: Local-first as a philosophy

Most modern apps treat "offline mode" as a secondary, degraded state. It's a fallback for when the internet fails, usually accompanied by a limited feature set and a stressful "Syncing..." spinner the moment you reconnect. I wanted to flip that script. In Rojekti, local is the primary state. The cloud isn't just optional; it's non-existent. 

Building a genuinely local-first tool changes your relationship with your productivity. There is zero latency. No waiting for a round-trip to a server in Virginia just to move a card from one lane to another. It works on a plane, in a basement, or during a total ISP outage. But more importantly, it respects your privacy by default. You don't need to read a 50-page privacy policy to know where your data is going. It's in the folder you created.

We’ve traded a lot of autonomy for the convenience of the cloud. We've accepted that our tools can be taken away from us or changed without our consent. Rojekti is an attempt to take some of that autonomy back. It’s built with Rust and Tauri to be fast and light, staying out of your way and letting you focus on the work rather than the platform. It's not a cloud app with an offline mode; it's a desktop app that respects your machine.

Does the "always-on" requirement of modern tools actually help you work, or is it just another form of digital tethering?

## Post 3: CLI + GUI as equals

Most desktop tools treat their Command Line Interface as a tacked-on afterthought—a limited set of shortcuts for power users that never quite matches what the GUI can do. In Rojekti, the CLI and the GUI are equal citizens. They are built on the same Rust core, reading the same files, with identical capabilities.

This parity exists because developers don't work in just one environment. If you're deep in a terminal session, you shouldn't have to reach for the mouse, switch windows, and navigate a web UI just to add a quick bug report or check your next task. You just type a command and keep moving. But when you need that bird's-eye view of your sprint, the Vue-powered GUI is right there with a beautiful, tactile Kanban interface.

This design decision wasn't just about human convenience, though. By making the CLI a first-class citizen, we’ve made the board inherently scriptable. You can pipe outputs into it, trigger card creations from build scripts, or let your local automation tools manage your workflow. It’s a tool that fits into your existing ecosystem rather than forcing you to build a new one around it. It's about meeting the developer wherever they happen to be at that moment.

What’s one task in your current project management tool that you wish you could just do with a quick terminal command?

## Post 4: AI/agent collaboration angle

We are entering the era of the AI agent, yet most of our project management tools are still built for a world where only humans interact with boards. If you want an AI to manage your tasks in a traditional SaaS tool, you’re looking at complex API integrations, oauth flows, and rate limits. Rojekti solves this by being incredibly "dumb" in the best way possible: everything is a file.

Because Rojekti stores cards as plain Markdown files, an AI agent doesn't need a plugin or a special API key to help you. If you’re using an agent to write code, that same agent can read your `cards/` directory, understand the context of a task, perform the work, and then update the card's YAML frontmatter to mark it as done. It can leave technical notes in the Markdown body or even create new sub-tasks based on what it discovered during implementation.

This creates a seamless loop between the human operator and the AI assistant. You can move a card to an "Agent" lane in the GUI, and your local AI script can pick it up via the CLI, execute the task, and move it to "Review." There’s no friction, no middleware, and no data leaving your machine. It’s a project management system designed for the way we are actually starting to build software today.

If your project board was easily readable by an AI agent right now, what's the first repetitive task you'd delegate?

## Post 5: The Trello-to-Jira trap

There is a recurring pattern in the life of a solo founder or a small dev team. You start with something like Trello because it’s simple and visual. Then, as the project grows, you realize you need more structure—priorities, epics, metadata, custom fields. Trello feels too shallow, so you migrate to Jira or a similar heavy-duty tool. Suddenly, you’re spending more time managing the tool than the project. The mental overhead of notifications, complex workflows, and enterprise features becomes a burden.

Rojekti is built to live in the "Goldilocks zone" between those two extremes. It gives you the visual clarity of a Kanban board and the structured data of YAML, without the bloated "social network" features of a team-centric SaaS. It's project management for people who actually have work to do. 

We don't have @mentions because you're probably working alone or in a tiny, high-trust group. We don't have an activity feed because you already know what you did today. We have statuses, epics, tags, and priorities—the essential pillars of organization—delivered in a fast, offline package that stays out of your way. It's about providing enough structure to be useful, but not so much that it becomes a chore.

At what point does a project management tool stop being a helper and start being a distraction for you?

## Post 6: A specific technical decision

When we decided how to store data in Rojekti, we didn't look at SQL or NoSQL databases. We looked at how developers already manage information. We chose Markdown with YAML frontmatter. It’s the same format used by Obsidian, Jekyll, Hugo, and almost every modern static site generator. This was a very deliberate choice to ensure Rojekti wasn't an island.

By using this format, your project board immediately becomes part of a much larger ecosystem. Your cards aren't just cards; they're documents. You can point Obsidian at your Rojekti cards directory and suddenly your tasks are part of your second brain, linkable and searchable alongside your research notes. You can use standard CLI tools like `yq` to query your board metadata or `grep` to scan your descriptions.

This is what we mean by "transparent data." We aren't hiding your information behind a proprietary schema or a binary blob. We’re using the closest thing the tech world has to a universal language for structured text. It makes the app harder to build in some ways—we have to handle filesystem watchers and file conflicts gracefully—but it makes the tool infinitely more valuable for the user. It’s about building on top of standards rather than trying to create new ones.

Do you prefer tools that keep data in a standard format, or are you okay with proprietary databases if the UI is good?

## Post 7: "Your data survives us"

The lifespan of the average SaaS startup is significantly shorter than the lifespan of a successful project. We’ve all seen great tools get acquired and killed, or simply shut down because the unit economics didn't work out. When that happens, you’re usually given a 30-day window to export your data into a messy JSON file that you’ll probably never look at again. Your history, your notes, and your context are effectively lost.

Rojekti is built on a different premise: your data should survive us. Because Viksu is local-first, our company’s existence is decoupled from your ability to work. If our website went dark tomorrow, your Rojekti app would keep running. Your files would still be on your hard drive. Your CLI would still work. You wouldn't lose a single byte of progress.

This is "digital sustainability." We believe that the tools you use for your life’s work shouldn't be dependent on a venture capital cycle or a corporate pivot. By building on Tauri and Rust, we've created a tiny, efficient binary that you can keep on a thumb drive if you want to. It’s a tool designed for the long haul, respecting the fact that your projects are more important than the software used to track them.

How much of your current project history is stored in a tool that could realistically disappear in the next two years?

## Post 8: The single-user problem

Almost every project management tool on the market is built with "teams" as the default unit of measure. They are designed for coordination, which means they are filled with features like comments, notifications, activity streams, and permission levels. But if you’re a solo founder or a developer working on a personal project, these features aren't just useless—they’re active noise. They create a mental overhead that makes the act of "checking the board" feel like an administrative task.

Rojekti is designed for the solo operator. We’ve stripped away the "social" layer of project management to focus on the "management" part. There are no notifications because you don't need to be notified about your own actions. There is no comment section because you can just write your notes directly into the Markdown body of the card. The interface is optimized for high-speed personal throughput.

When you remove the need to perform productivity for others, the tool becomes much sharper. You can organize things exactly how your brain works, using Finnish slang for your project names or unconventional epic structures, without worrying about how it looks to a manager. It’s a private workspace for your thoughts and your progress. It turns Kanban back into what it was meant to be: a visual way to manage flow, not a social network for work.

Do you find that "collaboration" features in tools actually slow you down when you're working solo?

## Post 9: Real-time sync without a server

One of the coolest technical challenges in Rojekti was implementing real-time synchronization between the GUI and the CLI without using a server or a local database. Usually, if you edit a file in a terminal, a GUI app wouldn't know about it until you manually hit refresh. That's a friction point we wanted to eliminate to make the CLI and GUI feel like a single, cohesive tool.

We solved this using a native filesystem watcher built in Rust. The app monitors your `rojekti/` folder for OS-level file events. If an AI agent or a shell script updates a card via the CLI, the Rust backend detects the change, debounces it to avoid thrashing, and sends an event to the Vue frontend to reload the board. It happens in the background, instantly, with no websockets and no internet required.

This creates a "live" feeling that is rare in local-file apps. You can have the GUI open on one monitor and your terminal on the other. Run a command to move a card, and you'll see it slide across the board half a second later. It’s a simple architecture that leverages the power of the operating system rather than trying to reinvent the wheel with a server-side state. It’s clean, it’s fast, and it just works.

What's a technical "simple" solution you've implemented recently that felt more robust than a complex one?

## Post 10: The name

We chose the name **Rojekti** because it’s Finnish slang for "project"—specifically the kind of project you’re actually excited to work on. The company name, **Viksu**, means "smart" or "clever." We wanted to name things in our own language because there’s a certain honesty in not pretending to be another generic Silicon Valley SaaS. 

Rojekti isn't trying to be the "all-in-one workspace for the enterprise." It’s a smart, focused tool for people who value their time and their data. By keeping the name personal and a bit unconventional, we’re signaling who this tool is for. It’s for the developer who appreciates a well-written CLI, the indie hacker who wants to own their roadmap, and the solo operator who prefers clever simplicity over corporate complexity.

Building this has been a project of passion, born out of a frustration with the current state of productivity software. We’re not hiding behind marketing jargon or hype. We’re just Viksu, building Rojekti, and trying to make project management a little bit more fixed and a lot more clever. 

Do you prefer when products have a clear, perhaps even "niche" identity, or do you like things that try to appeal to everyone?

## Post 11: What "done" looks like

In most Kanban tools, when a card moves to "Done," it eventually gets archived and disappears into a searchable but mostly forgotten graveyard. In Rojekti, moving a card to done is just a metadata change in a Markdown file. The card stays in your `cards/` directory. It remains a part of your project's permanent record, perfectly formatted and ready to be committed to Git.

This turns your project board into a living history of your development process. Every technical note, every "Lessons Learned" section, and every priority shift is preserved in a human-readable format. Years from now, you won't need to log into a defunct SaaS account to remember why you made a specific architectural decision. You’ll just open the `.md` file in your repository.

By treating cards as documents, we bridge the gap between "task management" and "documentation." You don't need a separate system to track your progress and your notes; they are the same thing. It’s a more holistic way to look at building software, where the "path to done" is just as important as the final product. Your board becomes a knowledge base that grows alongside your code.

How much of your project's "why" is currently buried in closed tickets that you'll never look at again?

## Post 12: An honest take on where it's at

Rojekti is early. I built the first version because I was tired of chasing my own data across a dozen different cloud platforms and I wanted a tool that respected my filesystem. It’s not a Jira killer. It’s not trying to replace Notion's infinite flexibility or Linear's team-scale polish. It is a very specific tool for a very specific kind of person.

If you’re someone who loves the terminal, lives in Markdown, and wants to own your work without a monthly subscription, Rojekti might be for you. It’s built with a focus on performance, parity between CLI and GUI, and AI-friendliness. It’s still evolving, and there are plenty of features yet to come, but the core philosophy will never change: your data is yours, and your tools should stay out of your way.

I’m building this in public, sharing the technical decisions and the mistakes along the way. I’m not interested in building the biggest project management tool in the world—just the right one for people who think like I do. If you value autonomy and local-first software, I’d love to have you along for the journey. 

What’s the one "missing feature" in your current workflow that would make you switch to a local-first tool tomorrow?
