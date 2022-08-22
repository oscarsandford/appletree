# Database

The database files that Eden can interface with are not included with the source code. You will have to construct them yourself.

Use `sqlite3` with the `.schema.sql` files in this directory in order to create the database `.db` files necessary for Eden's back end to access the databases.

## Quick Start

To set up the database for user information (such as quotes and card collections), open SQLite and read the schema into memory. Then, save the 
database in memory to a database file.
```
$ sqlite3
sqlite> .read user.schema.sql
sqlite> .save user.db
