# Appletree in Eden

Appletree is [Applebot](https://github.com/oscarsandford/applebot) upgraded with TypeScript, as well as its own local Rust web server and SQL database called Eden.

## Installation
Install [`node`](https://nodejs.org/en/) and [`npm`](https://www.npmjs.com/). Clone this repository and, with it as the current working directory, run:
```
npm ci
```

## Launch
Eden is first started as a background process, and Applebot is spawned afterwards. There are a few ways to do this:
* `./launch.sh` checks for a release build of Eden, compiles if necessary, then launches Eden and Apple.
* `./launch.sh build` forces compilation of a new release build before launch.

You may need to make use of `kill` or `pkill`, in order to clean up Eden if the main process exits prematurely.

## Seed
In order to make use of slash commands, you must run the script to register them with Discord's API using the following npm command:
```sh
npm run seed
```
The propagation time may vary, but expect it to take a while, especially when running the global configuration. See `scripts/seed.js` for more info on how to do this.