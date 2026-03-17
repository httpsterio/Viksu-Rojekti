# Tauri + Vue + TypeScript

This template should help get you started developing with Vue 3 and TypeScript in Vite. The template uses Vue 3 `<script setup>` SFCs, check out the [script setup docs](https://v3.vuejs.org/api/sfc-script-setup.html#sfc-script-setup) to learn more.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Vue - Official](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)



## Commandline Use (for humans or agents)

- **1. Initialize a new board**
  Creates board.yaml and the tickets/ folder in the current directory.
  `.\rojekti.exe init --name "Project Name" --prefix PROJ`

- **2. List all tickets**
  Prints a summary of all tickets (ID, Title, Status, Priority) to stdout.
  `.\rojekti.exe list`

- **3. Show ticket details**
  Prints the full metadata and Markdown body of a specific ticket.
  `.\rojekti.exe show --id PROJ-001`

- **4. Create a new ticket**
  Creates a new .md file in the tickets/ directory and updates the index.
  `.\rojekti.exe create --title "My Ticket Title" --priority high --tags "bug,ui" --status todo --epic feature-id`
  - --title (Required)
  - --priority (Optional: low, medium, high, critical)
  - --tags (Optional: comma-separated)
  - --status (Optional: lane name)
  - --epic (Optional: epic ID)

- **5. Rebuild the index**
  Manually regenerates index.yaml from the files in the tickets/ folder. Useful if you've edited ticket files directly or via another tool.
  `.\rojekti.exe rebuild-index`

- **6. Reorder a lane**
  Renormalizes the position values for all tickets in a specific lane to 1.0, 2.0, 3.0, etc.
  `.\rojekti.exe reorder --lane backlog`

- **7. Launch the GUI**
  Running the executable without any subcommands opens the Kanban interface.
  `.\rojekti.exe`