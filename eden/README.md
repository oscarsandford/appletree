# Eden

Eden is a multithreaded REST web server primarily designed as an interface for Apple to fetch resources from locally hosted databases.

Why Rust? Because I wanted to learn how to make a web server in Rust!

See `/db` for instructions on how to set up the SQLite database. See `/scripts` for some sample scripts to help automate various setup and maintenance tasks.

## Development

In terms of developing Eden as a standalone system, there are a few commands to remember:
- Compile the project with `cargo build` (by default this will be a debug build)
- Run unit tests with `cargo test`

If running the debug build executable, the program will use the database files located at `./db` (the subdirectory in this repository). Otherwise (i.e. for release), the database files will be assumed to be located at the Linux path `/db/eden`. This will be the case when the program is running in a Docker image.
