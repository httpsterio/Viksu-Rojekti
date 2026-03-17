# Rojekti CLI Usage

Run these from your terminal in the directory where `rojekti.exe` is located (or provide the full path to the executable).

### 1. Initialize a new board
Creates the `rojekti/` folder with `rojekti.config.yaml` in the current directory.
```powershell
.\rojekti.exe init --name "Project Name" --prefix PROJ
```

### 2. List all cards
Prints a summary of all cards (ID, Title, Status, Priority) to stdout.
```powershell
.\rojekti.exe list
```

### 3. Show card details
Prints the full metadata and Markdown body of a specific card.
```powershell
.\rojekti.exe show --id PROJ-001
```

### 4. Create a new card
Creates a new `.md` file in the `rojekti/cards/` directory and updates the index.
```powershell
.\rojekti.exe create --title "My Card Title" --priority high --tags "bug,ui" --status todo --epic feature-id
```
*   `--title` (Required)
*   `--priority` (Optional: low, medium, high, critical)
*   `--tags` (Optional: comma-separated)
*   `--status` (Optional: lane name)
*   `--epic` (Optional: epic ID)

### 5. Rebuild the index
Manually regenerates `rojekti.index.yaml` from the files in the `rojekti/cards/` folder.
```powershell
.\rojekti.exe rebuild-index
```

### 6. Reorder a lane
Renormalizes the `position` values for all cards in a specific lane to 1.0, 2.0, 3.0, etc.
```powershell
.\rojekti.exe reorder --lane backlog
```

### 7. Launch the GUI
Running the executable without any subcommands opens the Kanban interface.
```powershell
.\rojekti.exe
```
