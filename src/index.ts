import { Client, GatewayIntentBits, Interaction, Snowflake } from "discord.js";
import { draw_tarot_cards, draw_trading_card, set_cooldown, TradingCard } from "./collections";
import { build_trading_card_embed, build_tarot_card_embed } from "./constructors";

import dotenv from "dotenv";
dotenv.config();

const client: Client<boolean> = new Client({ intents : [GatewayIntentBits.Guilds] });
client.login(process.env.APL_DISCORDJS);

const recently_drawn = {
	"card" : new Set<Snowflake>(),
	"tarot" : new Set<Snowflake>()
};

client.on("ready", () => {
	console.log("> Appletree has bloomed.");
});

client.on("interactionCreate", async (interaction: Interaction) => {
	if (!interaction.isCommand()) return;
	switch (interaction.commandName) {
		case "drawaugust":
			if (recently_drawn.card.has(interaction.user.id)) {
				await interaction.reply({content: "Draw on cooldown.", ephemeral: true});
				return;
			}
			const trading_card: TradingCard = draw_trading_card();
			const [card, row] = build_trading_card_embed(trading_card);
			await interaction.reply({ embeds : [card], components : [] });
			set_cooldown(interaction.user.id, recently_drawn.card, 600000);
			break;

		case "drawtarot":
			if (recently_drawn.tarot.has(interaction.user.id)) {
				await interaction.reply({content: "Tarot draw on cooldown.", ephemeral: true});
				return;
			}
			const [mc, rc, ac] = draw_tarot_cards(interaction.user.username);
			const tarot_card_embed = build_tarot_card_embed(mc, rc, ac);
			await interaction.reply({embeds : [tarot_card_embed]});
			set_cooldown(interaction.user.id, recently_drawn.tarot, 600000);
			break;

		case "quote":
			// Draw quote. Interface with Eden.
			break;

		case "addquote":
			// Add quote. Interface with Eden.
			break;

		case "findquote":
			// Add quote. Interface with Eden.
			break;

		case "unquote":
			// Add quote. Interface with Eden.
			break;
	}
});