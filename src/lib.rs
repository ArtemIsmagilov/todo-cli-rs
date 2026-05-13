use jiff::civil::{Date, DateTime};
use rusqlite::Connection;
use super_table::Table;

struct Task {
    id: i64,
    title: String,
    note: Option<String>,
    due: Option<Date>,
    subtask_id: Option<i64>,
    group_name: Option<String>,
    priority: String,
    completed: bool,
    created_at: DateTime,
    updated_at: DateTime,
}

fn get_conn() -> rusqlite::Result<Connection> {
    let conn = Connection::open("todo.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks(
id INTEGER PRIMARY KEY AUTOINCREMENT
, title TEXT NOT NULL
, note TEXT
, due TEXT
, subtask_id INTEGER REFERENCES tasks(id) ON DELETE SET NULL
, group_name TEXT
, priority TEXT DEFAULT low CHECK(priority IN ('low', 'medium', 'high', 'critical'))
, completed BOOLEAN DEFAULT FALSE
, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
, updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
)",
        (),
    )?;
    Ok(conn)
}

pub fn cmd_list(sort: Option<&String>, order: Option<&String>) -> rusqlite::Result<()> {
    let query = format!(
        "SELECT
            id
            , title
            , note
            , due
            , subtask_id
            , group_name
            , priority
            , completed
            , created_at
            , updated_at
        FROM tasks 
        ORDER BY {} {}",
        sort.unwrap(),
        order.unwrap()
    );
    let conn = get_conn()?;
    let mut stmt = conn.prepare(&query)?;
    let tasks = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            title: row.get(1)?,
            note: row.get(2)?,
            due: row.get(3)?,
            subtask_id: row.get(4)?,
            group_name: row.get(5)?,
            priority: row.get(6)?,
            completed: row.get(7)?,
            created_at: row.get(8)?,
            updated_at: row.get(9)?,
        })
    })?;
    let mut table = Table::new();
    table.add_row(vec![
        "ID",
        "Title",
        "Note",
        "Due",
        "Subtask",
        "Group",
        "Priority",
        "Completed",
        "Created",
        "Updated",
    ]);
    for task in tasks {
        let t = task?;
        table.add_row(vec![
            t.id.to_string(),
            t.title,
            t.note.unwrap_or_default(),
            t.due.map(|d| d.to_string()).unwrap_or_default(),
            t.subtask_id.map(|id| id.to_string()).unwrap_or_default(),
            t.group_name.unwrap_or_default(),
            t.priority.to_string(),
            if t.completed { "✓" } else { "✗" }.to_string(),
            t.created_at.to_string(),
            t.updated_at.to_string(),
        ]);
    }
    println!("{table}");
    Ok(())
}

pub fn cmd_add(
    title: Option<&String>,
    note: Option<&String>,
    due: Option<&Date>,
    subtask: Option<&i64>,
    group_name: Option<&String>,
    priority: Option<&String>,
) -> rusqlite::Result<()> {
    let conn = get_conn()?;
    conn.execute(
        "INSERT INTO tasks (title, note, due, subtask_id, group_name, priority)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        (
            title,
            note,
            due,
            subtask,
            group_name,
            priority.unwrap_or(&String::from("low")),
        ),
    )?;
    println!("Task added");
    Ok(())
}

pub fn cmd_edit(
    id: Option<&i64>,
    title: Option<&String>,
    note: Option<&String>,
    due: Option<&Date>,
    subtask: Option<&i64>,
    group_name: Option<&String>,
    priority: Option<&String>,
    completed: Option<&bool>,
) -> rusqlite::Result<()> {
    if title.is_none()
        && note.is_none()
        && due.is_none()
        && subtask.is_none()
        && group_name.is_none()
        && priority.is_none()
        && completed.is_none()
    {
        println!("empty edit");
        return Ok(());
    }
    let conn = get_conn()?;
    conn.execute(
        "UPDATE tasks SET
            title = COALESCE(?1, title),
            note = COALESCE(?2, note),
            due = COALESCE(?3, due),
            subtask_id = COALESCE(?4, subtask_id),
            group_name = COALESCE(?5, group_name),
            priority = COALESCE(?6, priority),
            completed = COALESCE(?7, completed),
            updated_at = CURRENT_TIMESTAMP
         WHERE id = ?8",
        (
            title, note, due, subtask, group_name, priority, completed, id,
        ),
    )?;
    println!("Task updated");
    Ok(())
}

pub fn cmd_find(
    id: Option<&String>,
    title: Option<&String>,
    note: Option<&String>,
    due: Option<&String>,
    subtask: Option<&String>,
    group_name: Option<&String>,
    priority: Option<&String>,
    completed: Option<&String>,
) -> rusqlite::Result<()> {
    let fields = [
        ("CAST(id AS TEXT)", id),
        ("title", title),
        ("note", note),
        ("due", due),
        ("CAST(subtask_id AS TEXT)", subtask),
        ("group_name", group_name),
        ("priority", priority),
        ("CAST(completed AS TEXT)", completed),
    ];

    let mut clauses = Vec::new();
    let mut params = Vec::new();

    for (col, val) in fields {
        if let Some(v) = val {
            clauses.push(format!("{col} LIKE ?"));
            params.push(format!("%{}%", v));
        }
    }

    if clauses.is_empty() {
        println!("empty search");
        return Ok(());
    }

    let query = format!(
        "SELECT id, title, note, due, subtask_id, group_name, priority, completed, created_at, updated_at
         FROM tasks WHERE {}
         ORDER BY id ASC",
        clauses.join(" AND "),
    );

    let conn = get_conn()?;
    let mut stmt = conn.prepare(&query)?;
    let tasks = stmt.query_map(rusqlite::params_from_iter(params.iter()), |row| {
        Ok(Task {
            id: row.get(0)?,
            title: row.get(1)?,
            note: row.get(2)?,
            due: row.get(3)?,
            subtask_id: row.get(4)?,
            group_name: row.get(5)?,
            priority: row.get(6)?,
            completed: row.get(7)?,
            created_at: row.get(8)?,
            updated_at: row.get(9)?,
        })
    })?;

    let mut table = Table::new();
    table.add_row(vec![
        "ID",
        "Title",
        "Note",
        "Due",
        "Subtask",
        "Group",
        "Priority",
        "Completed",
        "Created",
        "Updated",
    ]);
    for task in tasks {
        let t = task?;
        table.add_row(vec![
            t.id.to_string(),
            t.title,
            t.note.unwrap_or_default(),
            t.due.map(|d| d.to_string()).unwrap_or_default(),
            t.subtask_id.map(|id| id.to_string()).unwrap_or_default(),
            t.group_name.unwrap_or_default(),
            t.priority.to_string(),
            if t.completed { "✓" } else { "✗" }.to_string(),
            t.created_at.to_string(),
            t.updated_at.to_string(),
        ]);
    }
    println!("{table}");
    Ok(())
}

pub fn cmd_drop() {
    match std::fs::remove_file("todo.db") {
        Ok(()) => println!("Tasks dropped"),
        Err(e) => println!("Tasks aleady dropped. {:?}", e),
    }
}

pub fn cmd_rm(id: Option<&i64>) -> rusqlite::Result<()> {
    let conn = get_conn()?;
    conn.execute("DELETE FROM tasks WHERE id = ?", (id,))?;
    println!("Task removed");
    Ok(())
}
