# TaskMaster
Track tasks with statuses such as
- *Pending* (not started yet)
- *In Progress* (started but not finished)
- *Complete*
---
Task data **persists between sessions** because the list is serialized to a .json file
in your home directory in the file: `~/Tasks/todo.json`

Similarly any errors are **logged and datestamped** to the file: `~/Tasks/bin/logs.txt`
