import { Client, GatewayIntentBits, Interaction } from "discord.js";
import fetch from "node-fetch";

import { draw_tarot_cards, draw_trading_card, set_cooldown, Quote, TradingCard } from "./collections";
import { build_trading_card_embed, build_tarot_card_embed } from "./constructors";
import { get_username } from "./dapi";

import dotenv from "dotenv";
dotenv.config();

const client: Client<boolean> = new Client({ intents : [GatewayIntentBits.Guilds] });
client.login(process.env.APL_DISCORDJS);

const recently_drawn = {
	"card" : new Set<string>(),
	"tarot" : new Set<string>()
};

client.on("ready", () => {
	console.log("> Appletree has bloomed.");
});

client.on("interactionCreate", async (interaction: Interaction) => {
	if (!interaction.isCommand()) return;
	switch (interaction.commandName) {
		case "drawaugust": {
			if (recently_drawn.card.has(interaction.user.id)) {
				await interaction.reply({ content: "Draw on cooldown.", ephemeral: true });
				return;
			}
			const trading_card: TradingCard = draw_trading_card();
			const [card, row] = build_trading_card_embed(trading_card);
			await interaction.reply({ embeds : [card], components : [] });
			set_cooldown(interaction.user.id, recently_drawn.card, 600000);
		} break;

		case "drawtarot": {
			if (recently_drawn.tarot.has(interaction.user.id)) {
				await interaction.reply({ content: "Tarot draw on cooldown.", ephemeral: true });
				return;
			}
			const [mc, rc, ac] = draw_tarot_cards(interaction.user.username);
			const card = build_tarot_card_embed(mc, rc, ac);
			await interaction.reply({ embeds : [card] });
			set_cooldown(interaction.user.id, recently_drawn.tarot, 600000);
		} break;

		case "quote": {
			const res = await fetch("http://localhost:8080/db/quote/draw", {
				method : "POST",
				headers : { "Content-Type" : "application/json" },
				body: JSON.stringify({})
			});
			const json = (await res.json()) as any; // Hacky! Replace with a better workaround later. 
				// source: https://github.com/node-fetch/node-fetch/issues/1262#issuecomment-910417511
			if (json["status"] && json["status"] !== "200") {
				return;
			}
			const quote: Quote = JSON.parse(json);
			const display_name: string = await get_username(client, interaction.guild, quote.quotee);

			await interaction.reply({ content: `> ${quote.quote}\n —${display_name}` });
		} break;

		case "addquote": {
			const quote = interaction.options.get("quote")?.value;
			const quotee = interaction.options.get("user")?.user;
			// Avoid sending bad data.
			if (quote === undefined || quotee === undefined || typeof quote !== "string") return;

			const body = {
				"quote" : quote,
				"quotee" : quotee.id,
				"quoter" : interaction.user.id,
				"qweight" : 0.5
			};

			const res = await fetch("http://localhost:8080/db/quote/add", {
				method : "POST",
				headers : { "Content-Type" : "application/json" },
				body: JSON.stringify(body)
			});
			const json = (await res.json()) as any;
			if (json["status"] && json["status"] === "200") {
				await interaction.reply({ content : `${interaction.user} added a quote from ${quotee} to the database.` });
			}
		} break;

		case "findquote": {
			const query = interaction.options.get("text")?.value;
			if (typeof query !== "string") return;

			const res = await fetch("http://localhost:8080/db/quote/find", {
				method : "POST",
				headers : { "Content-Type" : "application/json" },
				body: JSON.stringify({ "query" : query })
			});
			const json = (await res.json()) as any;

			if (json["status"] && json["status"] === "200") {
				const quote: Quote = JSON.parse(json);
				const display_name: string = await get_username(client, interaction.guild, quote.quotee);
				await interaction.reply({ content: `> ${quote.quote}\n —${display_name}` });
			} else if (json["status"] && json["status"] === "204") {
				await interaction.reply({content: "No matching quote found.", ephemeral: true});
			}
		} break;

		case "unquote": {
			// Remove quote. Interface with Eden. This is not yet implemented on Eden.
		} break;
	}
});