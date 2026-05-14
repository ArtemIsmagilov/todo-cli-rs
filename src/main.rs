use clap::{ArgAction, Command, arg, command, value_parser};
use jiff::civil::Date;

use todo_cli_rs::{cmd_add, cmd_drop, cmd_edit, cmd_find, cmd_list, cmd_rm};

fn main() -> rusqlite::Result<()> {
    let matches = command!()
        .subcommand(
            Command::new("list")
                .about("List all tasks. Example: todo-cli-rs list -s id -o asc")
                .arg(
                    arg!(
                        -s --sort <FIELD> "sort tasks by field"
                    )
                    .value_parser([
                        "id",
                        "title",
                        "note",
                        "due",
                        "subtask",
                        "group_name",
                        "priority",
                        "completed",
                        "created_at",
                        "updated_at",
                    ])
                    .default_value("id")
                    .required(false),
                )
                .arg(
                    arg!(-o --order <ORDER> "sort order asc/desc")
                        .value_parser(["asc", "desc"])
                        .default_value("asc")
                        .required(false),
                ),
        )
        .subcommand(
            Command::new("add")
                .about("Add a new task. Example: todo-cli-rs add 'Task title'")
                .arg(arg!(<TITLE> "task title").required(true))
                .arg(arg!(-n --note <NOTE> "task description").required(false))
                .arg(
                    arg!(-d --due <DATE> "task expiration date. Format: YYYY-MM-DD")
                        .required(false)
                        .value_parser(value_parser!(Date)),
                )
                .arg(
                    arg!(-s --subtask <ID> "subtask for main task")
                        .required(false)
                        .value_parser(value_parser!(i64)),
                )
                .arg(arg!(-g --group_name <GROUP> "task group").required(false))
                .arg(
                    arg!(
                        -p --priority <PRIORITY> "task priority"
                    )
                    .required(false)
                    .value_parser(["low", "medium", "high", "critical"]),
                ),
        )
        .subcommand(
            Command::new("edit")
                .about("Edit task. Example: todo-cli-rs edit 1 -t 'Task title'")
                .arg(
                    arg!(<ID> "set ID for edit task")
                        .required(true)
                        .value_parser(value_parser!(i64)),
                )
                .arg(arg!(-t --title <TITLE> "edit task title").required(false))
                .arg(arg!(-n --note <NOTE> "edit task description").required(false))
                .arg(
                    arg!(-d --due <DATE> "edit task expiration date")
                        .required(false)
                        .value_parser(value_parser!(Date)),
                )
                .arg(
                    arg!(-s --subtask <ID> "edit subtask for main task")
                        .required(false)
                        .value_parser(value_parser!(i64)),
                )
                .arg(arg!(-g --group_name <GROUP> "edit task group").required(false))
                .arg(
                    arg!(
                        -p --priority <PRIORITY> "edit task priority"
                    )
                    .required(false)
                    .value_parser(["low", "medium", "high", "critical"]),
                )
                .arg(
                    arg!(-c --completed <BOOL> "set task as done or undone")
                        .action(ArgAction::Set)
                        .required(false)
                        .value_parser(value_parser!(bool)),
                ),
        )
        .subcommand(
            Command::new("rm")
                .about("remove task. Example: todo-cli-rs rm 1")
                .arg(
                    arg!(<ID> "remove task by ID")
                        .required(true)
                        .value_parser(value_parser!(i64)),
                ),
        )
        .subcommand(
            Command::new("find")
                .about("Find tasks. Example: todo-cli-rs find --id 1 --title text")
                .arg(arg!(-i --id <PATTERN> "search by id").required(false))
                .arg(arg!(-t --title <PATTERN> "search by title").required(false))
                .arg(arg!(-n --note <PATTERN> "search by note").required(false))
                .arg(arg!(-d --due <PATTERN> "search by due date").required(false))
                .arg(arg!(-s --subtask <PATTERN> "search by subtask id").required(false))
                .arg(arg!(-g --group_name <PATTERN> "search by group").required(false))
                .arg(arg!(-p --priority <PATTERN> "search by priority").required(false))
                .arg(arg!(-c --completed <PATTERN> "search by completed status").required(false)),
        )
        .subcommand(Command::new("drop").about("Drop the tasks. Example: todo-cli-rs drop"))
        .flatten_help(true)
        .get_matches();

    match matches.subcommand() {
        Some(("list", sub_matches)) => cmd_list(
            sub_matches.get_one::<String>("sort"),
            sub_matches.get_one::<String>("order"),
        ),
        Some(("edit", sub_matches)) => cmd_edit(
            sub_matches.get_one::<i64>("ID"),
            sub_matches.get_one::<String>("title"),
            sub_matches.get_one::<String>("note"),
            sub_matches.get_one::<Date>("due"),
            sub_matches.get_one::<i64>("subtask"),
            sub_matches.get_one::<String>("group_name"),
            sub_matches.get_one::<String>("priority"),
            sub_matches.get_one::<bool>("completed"),
        ),
        Some(("add", sub_matches)) => cmd_add(
            sub_matches.get_one::<String>("TITLE"),
            sub_matches.get_one::<String>("note"),
            sub_matches.get_one::<Date>("due"),
            sub_matches.get_one::<i64>("subtask"),
            sub_matches.get_one::<String>("group_name"),
            sub_matches.get_one::<String>("priority"),
        ),
        Some(("rm", sub_matches)) => cmd_rm(sub_matches.get_one::<i64>("ID")),
        Some(("drop", _)) => {
            cmd_drop();
            Ok(())
        }
        Some(("find", sub_matches)) => cmd_find(
            sub_matches.get_one::<String>("id"),
            sub_matches.get_one::<String>("title"),
            sub_matches.get_one::<String>("note"),
            sub_matches.get_one::<String>("due"),
            sub_matches.get_one::<String>("subtask"),
            sub_matches.get_one::<String>("group_name"),
            sub_matches.get_one::<String>("priority"),
            sub_matches.get_one::<String>("completed"),
        ),
        _ => {
            println!("empty command");
            Ok(())
        }
    }
}
