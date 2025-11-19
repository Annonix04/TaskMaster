# TaskMaster
Track tasks with statuses such as
- *Pending* (not started yet)
- *In Progress* (started but not finished)
- *Complete*
---
## Notes:
Task data **persists between sessions** because the list is serialized to a .json file
in your home directory in the file: `~/Tasks/todo.json`

Similarly any errors are **logged and datestamped** to the file: `~/Tasks/bin/logs.txt`

---
## TODO:
- [x] Implement base functionality
- [x] Theme changing
- [ ] Allow for multiple task lists
- [ ] Add task counter for each list
- [ ] (optional) Style elements
