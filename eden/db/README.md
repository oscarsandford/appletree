# Database

The database files that Eden can interface with are not included with the source code. You will have to construct them yourself.

Use `sqlite3` with the `.sql` files in this directory in order to create the database `.db` files necessary for Eden's back end to access the databases.

## Quick Start

To set up the database for user information (such as quotes and card collections), open SQLite and read the schema into memory. Then, save the 
database in memory to a database file.
```
$ sqlite3
sqlite> .read user_schema.sql
sqlite> .save user.db
```
Place the `.db` files in `/db/eden`, so that the Eden program can access them. This is a shared volume when Eden is ran as a Docker container.

## Notes

For Sqlite 3.x, you need to make the following query every time when connecting to a database in order to enforce foreign key constraints. This is done in the Rust program, but keep this in mind if you want to access the database directly. You can add this line to a file `~/.sqliterc`, and it will automatically run when launching Sqlite.
```sql
PRAGMA foreign_keys = ON;
```