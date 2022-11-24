# Apple

Apple is a Node.js bot application written in TypeScript that uses [Discord.js](https://discord.js.org/) to retrieve data from Discord servers that the bot has been added to. Data includes user messages and the inputs from slash commands.

## Development
If you simply run `npm start`, it will transcompile the TypeScript and run the resulting JavaScript using `node`. This will enable the `dev` environment, which assumes Eden will be on the local network. Otherwise, if the environment is `production`, Eden will be linked on the Docker subnet.

## Registering Slash Commands
In order to make use of slash commands, you must run the script to register them with Discord's API using the following npm command:
```sh
npm run seed
```
The propagation time may vary, but expect it to take a while, especially when running the global configuration. See `scripts/seed.js` for more info on how to do this.
