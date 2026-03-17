- App behavior when running from CLI and desktop GUI are inconsistent. CLI output has error:
```powershell
PS C:\Users\sami\Desktop\rojekti> .\rojekti.exe show --id roj-001
ID: roj-001
...
[0317/043128.111:ERROR:ui\gfx\win\window_impl.cc:124] Failed to unregister class Chrome_WidgetWin_0. Error = 1412
``` 

- Tickets can't be re-organized by dragging and dropping. 

- Tickets can't be moved between lanes.

- Settings can't be saved

- Running a CLI command doesn't refresh the app state. Tickets stay where they were despite being reorganized on the CLI end for example.

- Tags cannot be added (because they're not saved?) to tickets. In the edit view the tags section is empty. In settings the tags section is empty (even if a ticket has tags from CLI)

- Epics cannot be created.

- Priority filtering seems to work.

- Why is Markdown edit view wider than view? View and edit could and should be the same size.

- Lane order should be swappable in settings.

- Default window size doesn't fit five lanes, last lane has horizontal scroll. Should be responsive and resize to fit

- Rename tickets to cards

- Translations (extract text, replace hard-coded text)

- Rename tickets folder, move board.yaml and index.yaml to same folder as tickets. 