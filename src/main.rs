use anyhow::{Context, Result};
use clap::Parser;
use colored::*;
use rusqlite::Connection;
use similar::{ChangeTag, TextDiff};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// The original SQLite database file
    file1: String,

    /// The new SQLite database file
    file2: String,
}

fn extract_schema(conn: &Connection) -> Result<String> {
    let mut stmt = conn.prepare(
        "SELECT type, name, sql FROM sqlite_master WHERE sql IS NOT NULL ORDER BY type, name",
    )?;
    let rows = stmt.query_map([], |row| {
        let type_: String = row.get(0)?;
        let name: String = row.get(1)?;
        let sql: String = row.get(2)?;
        Ok(format!("-- {} {}\n{};\n", type_, name, sql))
    })?;

    let mut schema = String::new();
    for row in rows {
        schema.push_str(&row?);
    }
    Ok(schema)
}

fn main() -> Result<()> {
    let args = Args::parse();

    let conn1 =
        Connection::open(&args.file1).with_context(|| format!("Failed to open {}", args.file1))?;
    let conn2 =
        Connection::open(&args.file2).with_context(|| format!("Failed to open {}", args.file2))?;

    let schema1 = extract_schema(&conn1)?;
    let schema2 = extract_schema(&conn2)?;

    if schema1 == schema2 {
        println!("{}", "Schemas are identical.".green());
        return Ok(());
    }

    let diff = TextDiff::from_lines(&schema1, &schema2);

    for change in diff.iter_all_changes() {
        let sign = match change.tag() {
            ChangeTag::Delete => "-".red(),
            ChangeTag::Insert => "+".green(),
            ChangeTag::Equal => " ".normal(),
        };
        print!("{} {}", sign, change);
    }

    Ok(())
}
