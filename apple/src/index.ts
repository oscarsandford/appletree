import { Client, GatewayIntentBits, Interaction, ComponentType, ButtonStyle, Message } from "discord.js";
import fetch from "node-fetch";

import { draw_tarot_cards, draw_trading_card, set_cooldown, EdenResponse, Quote, UserData, TradingCard, Card, Item } from "./collections";
import { build_trading_card_embed, build_tarot_card_embed, build_button, build_user_embed, build_card_embed, elemap } from "./constructors";
import * as DAPI from "./dapi";

import dotenv from "dotenv";
dotenv.config();

const client: Client<boolean> = new Client({ intents : [GatewayIntentBits.Guilds, GatewayIntentBits.GuildMessages] });
client.login(process.env.APL_DISCORDJS);

const recency_cache = {
	"card" : new Set<string>(),
	"tarot" : new Set<string>(),
	"msgs" : new Set<string>(),
};

const eden = process.env.EDEN || "http://127.0.0.1:8080";

client.on("ready", () => {
	const env = process.env.NODE_ENV || "dev";
	console.log(`Appletree has bloomed.\n - Environment: ${env}\n - Eden Address: ${eden}`);
});

client.on("messageCreate", async (message: Message) => {
	// Collect 25-35 XP up to once every minute while messaging.
	if (!recency_cache.msgs.has(message.author.id)) {
		set_cooldown(message.author.id, recency_cache.msgs, 60000);
		const body = { 
			"query" : (Math.floor(Math.random()*11)+25).toString(), 
			"requester" : message.author.id 
		};
		console.log("[Apple] /db/user/xp <--", body);
		const res = await fetch(`${eden}/db/user/xp`, {
			method : "POST",
			headers : { "Content-Type" : "application/json" },
			body: JSON.stringify(body)
		});
		const buf = res.body.read();
		const eres: EdenResponse = JSON.parse(buf.toString());
		console.log("[Apple] /db/user/xp -->", eres);
	}
});

client.on("interactionCreate", async (interaction: Interaction) => {
	if (!interaction.isCommand()) return;

	switch (interaction.commandName) {
		case "drawaugust": {
			if (recency_cache.card.has(interaction.user.id)) {
				await interaction.reply({ content: "Draw on cooldown.", ephemeral: true });
				return;
			}
			set_cooldown(interaction.user.id, recency_cache.card, 600000);
			const trading_card: TradingCard = draw_trading_card();
			const [card, row] = build_trading_card_embed(trading_card);
			await interaction.reply({ embeds : [card], components : [] });
		} break;

		case "drawtarot": {
			if (recency_cache.tarot.has(interaction.user.id)) {
				await interaction.reply({ content: "Tarot draw on cooldown.", ephemeral: true });
				return;
			}
			set_cooldown(interaction.user.id, recency_cache.tarot, 600000);
			const [mc, rc, ac] = draw_tarot_cards(interaction.user.username);
			const card = build_tarot_card_embed(mc, rc, ac);
			await interaction.reply({ embeds : [card] });
		} break;

		case "quote": {
			const res = await fetch(`${eden}/db/quote/draw`, {
				method : "POST",
				headers : { "Content-Type" : "application/json" },
				body: JSON.stringify({})
			});
			const buf = res.body.read();
			const quote: Quote = JSON.parse(buf.toString());
			if (quote.status && quote.status !== "200") {
				return;
			}
			const display_name: string = await DAPI.get_user_name(client, interaction.guild, quote.quotee);
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

			const res = await fetch(`${eden}/db/quote/add`, {
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

			const res = await fetch(`${eden}/db/quote/find`, {
				method : "POST",
				headers : { "Content-Type" : "application/json" },
				body: JSON.stringify({ "query" : query , "requester" : requester })
			});
			const buf = res.body.read();
			const quote: Quote = JSON.parse(buf.toString());

			if (quote.status) {
				if (quote.status === "200") {
					const display_name: string = await DAPI.get_user_name(client, interaction.guild, quote.quotee);
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

			const res = await fetch(`${eden}/db/quote/find`, {
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
					const display_name: string = await DAPI.get_user_name(client, interaction.guild, quote.quotee);
					const confirm_btn = build_button("unquote_confirm", "Delete Quote", ButtonStyle.Danger, false);
					await interaction.reply({ content: `> ${quote.quote}\n—${display_name}` , components : [confirm_btn] , ephemeral : true});
					
					const collector = interaction.channel?.createMessageComponentCollector({ componentType: ComponentType.Button, time : 15000, max : 1 });
					collector?.on("collect", async i => {
						if (i.customId === "unquote_confirm" && i.user.id === interaction.user.id) {
							await i.deferUpdate();
							
							const res = await fetch(`${eden}/db/quote/remove`, {
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
					const quotee_name: string = await DAPI.get_user_name(client, interaction.guild, quote.quotee);
					const quoter_name: string = await DAPI.get_user_name(client, interaction.guild, quote.quoter);
					await interaction.reply({ content : `You cannot remove the following quote\n> ${quote.quote}\ndue to you not ` +
					`being the quoter (${quoter_name}) or the quotee (${quotee_name}).`, ephemeral : true });
				}
			}
		} break;

		case "profile": {
			// Returns embed of user info.
			const res = await fetch(`${eden}/db/user`, {
				method : "POST",
				headers : { "Content-Type" : "application/json" },
				body: JSON.stringify({ "query" : "", "requester" : interaction.user.id })
			});
			const buf = res.body.read();
			const udata: UserData = JSON.parse(buf.toString());
			if (udata.status === "200") {
				const uname = await DAPI.get_user_name(client, interaction.guild, udata.id);
				const uicon = await DAPI.get_user_avatar(client, interaction.guild, udata.id);
				const user_card = build_user_embed(uname, uicon, interaction.user.accentColor, udata);
				await interaction.reply({ embeds : [user_card] });
			}
		} break;

		case "background": {
			const url = interaction.options.get("url")?.value;
			if (typeof url !== "string") return;

			const res = await fetch(`${eden}/db/user/bg`, {
				method : "POST",
				headers : { "Content-Type" : "application/json" },
				body: JSON.stringify({ "query" : url, "requester" : interaction.user.id })
			});
			const buf = res.body.read();
			const eres: EdenResponse = JSON.parse(buf.toString());
			if (eres.status && eres.status === "200") {
				await interaction.reply({ content : "Your profile background has been updated.", ephemeral : true });
			}
			else {
				await interaction.reply({ content : "Something went wrong while updating your profile background.", ephemeral : true });
			}
		} break;

		case "card": {
			console.log("draw card cache (before): ", recency_cache.card);
			if (recency_cache.card.has(interaction.user.id)) {
				await interaction.reply({ content: "Draw on cooldown.", ephemeral: true });
				return;
			}
			set_cooldown(interaction.user.id, recency_cache.card, 1200000);
			const res = await fetch(`${eden}/db/card/draw`, {
				method : "POST",
				headers : { "Content-Type" : "application/json" },
				body: JSON.stringify({})
			});

			const buf = res.body.read();
			const card: Card = JSON.parse(buf.toString());

			if (card.status && card.status === "200") {
				const uicon = await DAPI.get_user_avatar(client, interaction.guild, card.subjct);
				const embed = build_card_embed(card, <Item>{lvl : 0, xp : 0}, uicon);
				await interaction.reply({ embeds : [embed] });

				const body = {
					"src" : card.csrc,
					"ownr" : interaction.user.id,
					"lvl" : 0,
					"xp" : 0
				};
				console.log("[Apple] /db/item/add <--", body);
				const res = await fetch(`${eden}/db/item/add`, {
					method : "POST",
					headers : { "Content-Type" : "application/json" },
					body: JSON.stringify(body)
				});
				const buf = res.body.read();
				const eres: EdenResponse = JSON.parse(buf.toString());
				console.log("[Apple] /db/item/add -->", eres);
			}
			console.log("draw card cache (after): ", recency_cache.card);
		} break;

		case "addcard": {
			const subject = interaction.options.get("subject")?.user;
			const name = interaction.options.get("name")?.value;
			const element = interaction.options.get("element")?.value;
			const image = interaction.options.get("image")?.value;
			// Avoid sending anything with bad data. This check is ugly!
			if (name === undefined || typeof name !== "string" ||
				element === undefined || typeof element !== "string" ||
				image === undefined || typeof image !== "string" || subject === undefined
			) return;

			const body = {
				"csrc" : image,
				"cname" : name,
				"crank" : 3,
				"element" : element,
				"atk" : 0,
				"lufa" : 0.0,
				"def" : 0.0,
				"lufd" : 0.0,
				"utl" : 0,
				"lufu" : 0.0,
				"subjct" : subject.id,
				"adder" : interaction.user.id,
				"tradable" : 1,
			};

			const res = await fetch(`${eden}/db/card/add`, {
				method : "POST",
				headers : { "Content-Type" : "application/json" },
				body: JSON.stringify(body)
			});
			const buf = res.body.read();
			const eres: EdenResponse = JSON.parse(buf.toString());
			if (eres.status && eres.status === "200") {
				await interaction.reply({ content : `Added a card titled "${name}" depicting ${subject} to the database.`});
			}
		} break;

		case "collection": {
			const body = {
				"src" : "",
				"ownr" : interaction.user.id,
				"lvl" : 0,
				"xp" : 0
			}
			const res = await fetch(`${eden}/db/item`, {
				method : "POST",
				headers : { "Content-Type" : "application/json" },
				body: JSON.stringify(body)
			});
			const buf = res.body.read();
			const eres: EdenResponse = JSON.parse(buf.toString());
			let st = "";
			let count = 0;
			if (eres.status && eres.status === "200") {
				eres.payload.forEach(el => {
					if (el.length === 4 && st.length < 1800 && (el[2] === "air" || el[2] ===  "earth" || el[2] === "fire" || el[2] === "water")) {
						st += `(${el[1]}:star:)  ${elemap[el[2]][1]}  LVL ${el[3]} -  *${el[0]}*\n`;
						count += parseInt(el[3]);
					}
				});
			}
			let header = `__Card Collection__ (${count} pulled)\n`;
			await interaction.reply({ content : header + st });
		} break;
	}
});
