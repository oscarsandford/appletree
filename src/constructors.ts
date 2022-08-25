import { EmbedBuilder, ActionRowBuilder, ButtonBuilder } from "discord.js";
import { TarotCard, TradingCard } from "./collections";

export function build_trading_card_embed(tc: TradingCard): [EmbedBuilder, ActionRowBuilder] {
	/**
	 * Creates and returns a custom embed for a trading card.
	 * 
	 * @param tc - The trading card whose information is needed to create the embed.
	 * 
	 * @returns A 2-tuple with the two individual components of the embed.
	 */
	const row = new ActionRowBuilder()
		.addComponents(
			new ButtonBuilder()
				.setCustomId("primary")
				.setLabel("Add to Collection")
				// .setStyle("PRIMARY")
		);
	const card = new EmbedBuilder()
		.setTitle(tc.name + " +" + tc.level)
		.setDescription(":star:".repeat(tc.rank))
		.setImage(tc.imglink);
		// .setColor("DARK_GREEN")
		// .setFooter("August Trading Cards");
	return [card, row];
}

export function build_tarot_card_embed(mc: TarotCard, rc: TarotCard, ac: TarotCard): EmbedBuilder {
	/**
	 * Creates and returns a custom embed for a tarot card.
	 * 
	 * @param mc - The main card.
	 * @param rc - The reverse card (whose meaning signifies a warning).
	 * @param ac - The advice card (whose meaning gives some advice with respect to the reverse card).
	 * 
	 * @returns The constructed embed.
	 */
	return new EmbedBuilder()
		.setTitle(`${mc.numeral} : ${mc.name} ${mc.emoji}`)
		.setDescription(mc.description)
		.addFields(
			{ 
				name: `${rc.name} ${rc.emoji}`, 
				value: `${rc.reverse}...`, 
				inline: true 
			},
			{ 
				name: `${ac.name} ${ac.emoji}`, 
				value: `...${ac.advice}`, 
				inline: true 
			}
		)
		.setImage(mc.imglink);
		// .setColor("DARK_RED")
		// .setFooter("Tavern Arcana");
}