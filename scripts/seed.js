// This script updates the slash commands associated with this application.
// Set up a .env file with following fields:
// APL_DISCORDJS : the secret key for the bot
// APL_INSTANCE_UID : the account user ID for the bot.
// APL_TEST_GUILD (optional) : the guild ID for a test server. Slash 
//  commands update faster locally, so this may be more practical 
//  for "iterative development".

const { REST } = require('@discordjs/rest');
const { Routes } = require('discord-api-types/v9');
require('dotenv').config();

const commands = [
	{
		'name' : 'drawaugust',
		'description' : 'Draw an August card.'
	},
	{
		'name' : 'drawtarot',
		'description' : 'Draw a Tavern Tarot card.'
	},
	{
		'name' : 'collection',
		'description' : 'View your personal trading card collection.'
	},
	{
		'name' : 'profile',
		'description' : 'View your level, credit, and other attributes.'
	},
	{
		'name' : 'quote',
		'description' : 'Have Apple say a random quote.',
	},
	{
		'name' : 'addquote',
		'description' : 'Write a quote to the database.',
		'options' : [
			{
				'name' : 'user',
				'description' : 'The user who said the thing.',
				'type' : 9,
				'required' : true
			},
			{
				'name' : 'quote',
				'description' : 'What they said.',
				'type' : 3,
				'required' : true
			}
		]
	},
	{
		'name' : 'findquote',
		'description' : 'Retrieve the quote best matching the given text.',
		'options' : [
			{
				'name' : 'text',
				'description' : 'A substring of the quote.',
				'type' : 3,
				'required' : true
			}
		]
	},
	{
		'name' : 'unquote',
		'description' : 'Remove one of your quotes, or a quote you added.',
		'options' : [
			{
				'name' : 'text',
				'description' : 'A substring of the quote.',
				'type' : 3,
				'required' : true
			}
		]
	},
	{
		'name' : 'background',
		'description' : 'Set your profile background to an image at a given URL.',
		'options' : [
			{
				'name' : 'url',
				'description' : 'The image URL.',
				'type' : 3,
				'required' : true
			}
		]
	}
]

const rest = new REST({ version: '9' }).setToken(process.env.APL_DISCORDJS);

// LOCAL: slash command changes for test server only. Faster for development.
// rest.put(Routes.applicationGuildCommands(process.env.APL_INSTANCE_UID, process.env.APL_TEST_GUILD), { body: commands })
// 	.then(() => console.log('[Appletree] Successfully registered application commands LOCALLY.'))
// 	.catch(console.error);

// GLOBAL: propogate slash command changes globally.
rest.put(Routes.applicationCommands(process.env.APL_INSTANCE_UID), { body: commands })
	.then(() => console.log('[Appletree] Successfully registered application commands GLOBALLY.'))
	.catch(console.error);
