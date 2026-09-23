# sqlitediff

Diffs two SQLite database files directly and reports schema differences. Fills the gap left by dumping `.schema` by hand and running standard text diffs.

## Status

**built, untested**: the `sqlite_master` extraction and diff logic is built. It has not been run against real database files in this sandbox environment.

## Installation

```sh
cargo install --path .
```

## Usage

```sh
sqlitediff original.sqlite modified.sqlite
```
