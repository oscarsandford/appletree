# Appletree in Eden

Appletree is [Applebot](https://github.com/oscarsandford/applebot) upgraded with TypeScript, as well as its own local Rust web server and SQL database called Eden.

## Prerequisites
Install [`node`](https://nodejs.org/en/), [`npm`](https://www.npmjs.com/), and [Rust](https://www.rust-lang.org/).

## Build

See the instructions for building the database files under `eden/db`.

~~`launch.sh` builds and runs both Apple and Eden. It uses the [`nohup`](https://www.gnu.org/software/coreutils/manual/html_node/nohup-invocation.html#nohup-invocation) command to run Apple and Eden as independent background processes that will persist after logout (e.g. a remote connection). It writes output to log files in their respective directories.~~

~~`clean.sh` helps automate cleaning the workspace and copying the project repository to `/tmp` where the `.git` files are then removed. This results in a slim project directory that can be moved around and set up with the launch script. This script will likely be deprecated once Docker is introduced.~~

With Docker, this setup README will be changed when everything is complete.

## Seed
In order to make use of slash commands, you must run the script to register them with Discord's API using the following npm command:
```sh
npm run seed
```
The propagation time may vary, but expect it to take a while, especially when running the global configuration. See `scripts/seed.js` for more info on how to do this.