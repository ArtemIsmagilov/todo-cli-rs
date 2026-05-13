# Fast Rust CLI ToDo manager

```bash
cargo run -- help
Usage: todo-cli-rs
       todo-cli-rs list [OPTIONS]
       todo-cli-rs add [OPTIONS] <TITLE>
       todo-cli-rs edit [OPTIONS] <ID>
       todo-cli-rs rm <ID>
       todo-cli-rs find [OPTIONS]
       todo-cli-rs drop
       todo-cli-rs help [COMMAND]...

Options:
  -h, --help     Print help
  -V, --version  Print version

todo-cli-rs list:
List all tasks. Example: todo-cli-rs list -s id -o asc
  -s, --sort <FIELD>   sort tasks by field [default: id] [possible values: id, title, note, due, subtask, group_name, priority, completed, created_at, updated_at]
  -o, --order <ORDER>  sort order asc/desc [default: asc] [possible values: asc, desc]
  -h, --help           Print help

todo-cli-rs add:
Add a new task. Example: todo-cli-rs add 'Task title'
  -n, --note <NOTE>          task description
  -d, --due <DATE>           task expiration date. Format: YYYY-MM-DD
  -s, --subtask <ID>         subtask for main task
  -g, --group_name <GROUP>   task group
  -p, --priority <PRIORITY>  task priority [possible values: low, medium, high, critical]
  -h, --help                 Print help
  <TITLE>                    task title

todo-cli-rs edit:
Edit task. Example: todo-cli-rs edit 1 -t 'Task title'
  -t, --title <TITLE>        edit task title
  -n, --note <NOTE>          edit task description
  -d, --due <DATE>           edit task expiration date
  -s, --subtask <ID>         edit subtask for main task
  -g, --group_name <GROUP>   edit task group
  -p, --priority <PRIORITY>  edit task priority [possible values: low, medium, high, critical]
  -c, --completed <BOOL>     set task as done or undone [possible values: true, false]
  -h, --help                 Print help
  <ID>                       set ID for edit task

todo-cli-rs rm:
remove task. Example: todo-cli-rs rm 1
  -h, --help  Print help
  <ID>        remove task by ID

todo-cli-rs find:
Find tasks. Example: todo-cli-rs find --id 1 --title text
      --id <PATTERN>          search by id
      --title <PATTERN>       search by title
      --note <PATTERN>        search by note
      --due <PATTERN>         search by due date
      --subtask <PATTERN>     search by subtask id
      --group_name <PATTERN>  search by group
      --priority <PATTERN>    search by priority
      --completed <PATTERN>   search by completed status
  -h, --help                  Print help

todo-cli-rs drop:
Drop the tasks. Example: todo-cli-rs drop
  -h, --help  Print help

todo-cli-rs help:
Print this message or the help of the given subcommand(s)
  [COMMAND]...  Print help for the subcommand(s)

```

## References
- https://github.com/dezoito/rust-todo-list
- https://github.com/sioodmy/todo
- https://github.com/GothenburgBitFactory/taskwarrior
