import { Client, GatewayIntentBits, Interaction, ComponentType, ButtonStyle } from "discord.js";
import fetch from "node-fetch";

import { draw_tarot_cards, draw_trading_card, set_cooldown, EdenResponse, Quote, TradingCard } from "./collections";
import { build_trading_card_embed, build_tarot_card_embed, build_button } from "./constructors";
import * as DAPI from "./dapi";

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
			const buf = res.body.read();
			const quote: Quote = JSON.parse(buf.toString());
			if (quote.status && quote.status !== "200") {
				return;
			}
			const display_name: string = await DAPI.get_username(client, interaction.guild, quote.quotee);
			await interaction.reply({ content: `> ${quote.quote}\n—${display_name}` });
		} break;

		case "addquote": {
			const quote = interaction.options.get("quote")?.value;
			const quotee = interaction.options.get("user")?.user;
			// Avoid sending anything with bad data.
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
			const buf = res.body.read();
			const eres: EdenResponse = JSON.parse(buf.toString());
			if (eres.status && eres.status === "200") {
				await interaction.reply({ content : `${interaction.user} added a quote from ${quotee} to the quote database.` });
			}
		} break;

		case "findquote": {
			const query = interaction.options.get("text")?.value;
			const requester = interaction.user.id;
			if (typeof query !== "string") return;

			const res = await fetch("http://localhost:8080/db/quote/find", {
				method : "POST",
				headers : { "Content-Type" : "application/json" },
				body: JSON.stringify({ "query" : query , "requester" : requester })
			});
			const buf = res.body.read();
			const quote: Quote = JSON.parse(buf.toString());

			if (quote.status) {
				if (quote.status === "200") {
					const display_name: string = await DAPI.get_username(client, interaction.guild, quote.quotee);
					await interaction.reply({ content: `> ${quote.quote}\n —${display_name}` });
				}
				else if (quote.status === "404") {
					await interaction.reply({ content : "No matching quote found.", ephemeral : true });
				}
			}
		} break;

		case "unquote": {
			const query = interaction.options.get("text")?.value;
			const requester = interaction.user.id;
			if (typeof query !== "string") return;
			
			// First make a request to the "find" endpoint, to see if the quote exists.
			// If it does, ensure that the requester is either the quoter or the quotee.
			// An interactive button faciliates confirming the action to delete the quote, 
			// and the button is updated based on Eden's response to the "remove" call.

			const res = await fetch("http://localhost:8080/db/quote/find", {
				method : "POST",
				headers : { "Content-Type" : "application/json" },
				body: JSON.stringify({ "query" : query , "requester" : requester })
			});
			const buf = res.body.read();
			const quote: Quote = JSON.parse(buf.toString());

			if (quote.status === "404") {
				await interaction.reply({ content : "No matching quote found.", ephemeral : true });
			}
			else if (quote.status === "200") {
				if (requester === quote.quotee || requester === quote.quoter) {
					// Throw out a button on the reply. If it is clicked, carry on with a call to remove.
					const display_name: string = await DAPI.get_username(client, interaction.guild, quote.quotee);
					const confirm_btn = build_button("unquote_confirm", "Delete Quote", ButtonStyle.Danger, false);
					await interaction.reply({ content: `> ${quote.quote}\n—${display_name}` , components : [confirm_btn] , ephemeral : true});
					
					const collector = interaction.channel?.createMessageComponentCollector({ componentType: ComponentType.Button, time : 15000, max : 1 });
					collector?.on("collect", async i => {
						if (i.customId === "unquote_confirm" && i.user.id === interaction.user.id) {
							await i.deferUpdate();
							
							const res = await fetch("http://localhost:8080/db/quote/remove", {
								method : "POST",
								headers : { "Content-Type" : "application/json" },
								body: JSON.stringify({ "query" : quote.quote , "requester" : requester })
							});
							const buf = res.body.read();
							const eres: EdenResponse = JSON.parse(buf.toString());

							if (eres.status === "200") {
								// Update button to success
								const success_btn = build_button("unquote_success", "Successfully Removed!", ButtonStyle.Success, true);
								await i.editReply({ components : [success_btn] });
							}
							else {
								// Update button to error
								const error_btn = build_button("unquote_error", "Something went wrong.", ButtonStyle.Secondary, true);
								await i.editReply({ components : [error_btn] });
							}	
						}
					});
				}
				else {
					const quotee_name: string = await DAPI.get_username(client, interaction.guild, quote.quotee);
					const quoter_name: string = await DAPI.get_username(client, interaction.guild, quote.quoter);
					await interaction.reply({ content : `You cannot remove the following quote\n> ${quote.quote}\ndue to you not ` +
					`being the quoter (${quoter_name}) or the quotee (${quotee_name}).`, ephemeral : true });
				}
			}
		} break;
	}
});