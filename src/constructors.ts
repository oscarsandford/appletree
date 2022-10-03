import { EmbedBuilder, ActionRowBuilder, ButtonBuilder, ButtonStyle } from "discord.js";
import { TarotCard, TradingCard, UserData } from "./collections";

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

export function build_button(id: string, label: string, style: ButtonStyle, disabled: boolean): ActionRowBuilder<ButtonBuilder> {
	/**
	 * A convenience function for making simple interactive buttons.
	 * 
	 * @param id - An id for uniquely identifying interactions with this button.
	 * @param label - A cosmetic string displayed with the button.
	 * @param style - The display colour of the button.
	 * @param disabled - Whether the button should be interactive or not.
	 * 
	 * @returns The constructed button.
	 */
	return new ActionRowBuilder<ButtonBuilder>()
		.addComponents(
			new ButtonBuilder()
				.setCustomId(id)
				.setLabel(label)
				.setStyle(style)
				.setDisabled(disabled)
		);
}

export function build_user_embed(uname: string, uicon: string, ucolr: number | null | undefined, udata: UserData): EmbedBuilder {
	/**
	 * A function to create profile embeds for user data.
	 * 
	 * @param uname - The user NAME, not id.
	 * @param uicon - The URL for the user avatar icon.
	 * @param ucolr - The accent color of the user profile.
	 * @param udata - The data for the user.
	 * 
	 * @returns The constructed embed.
	 */
	if (!ucolr || ucolr === undefined) ucolr = 0;

	return new EmbedBuilder()
		.setColor(ucolr)
		.setTitle(uname)
		.setThumbnail(uicon)
		.addFields(
			{ name : "Level", value: `${udata.lvl}`, inline : true },
			{ name : "XP", value: `${udata.xp}`, inline : true },
			{ name : "Credit", value: `${udata.credit}`, inline : true },
		)
		.setImage(udata.bg)
		.setTimestamp();
}