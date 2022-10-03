import cards_json from "./resources/cards.json";
import tarot_json from "./resources/tavernarcana.json";
const trading_cards: Array<TradingCard> = cards_json;
const tarot_cards: Array<TarotCard> = tarot_json;
const card_weights: Array<number> = [0.19, 0.3, 0.4, 0.1, 0.01];

export interface TradingCard {
	id: number,
	name: string,
	rank: number,
	level: number,
	imglink: string,
	deck: string
}

export interface TarotCard {
	id: number,
	numeral: string,
	name: string,
	emoji: string,
	imglink: string,
	description: string,
	reverse: string,
	advice: string,
	deck: string
}

export interface EdenResponse {
	status: string,
}

export interface Quote extends EdenResponse {
	quote: string,
	quotee: string,
	quoter: string,
	qweight: number
}

export interface UserData extends EdenResponse {
	id: string,
	lvl: number,
	xp: number,
	credit: number,
	bg: string,
}

export function draw_trading_card(): TradingCard {
	/**
	 * Returns a trading card randomly drawn based on rates for each rank.
	 * 
	 * @returns A randomly drawn trading card.
	 */
	let rand_rank = random_index_weighted(card_weights);
	let selected_cards = trading_cards.filter((c) => c.rank === rand_rank);
	let rand_card = selected_cards[Math.floor(Math.random() * selected_cards.length)];
	rand_card.level = 0;
	return rand_card;
}

export function draw_tarot_cards(drawer: string): [TarotCard, TarotCard, TarotCard] {
	/**
	 * Draws 3 tarot cards: a main card, a reverse card, and a advice card.
	 * 
	 * @param drawer - The Discord username of the user who issued the command.
	 * @returns A TarotCard 3-tuple with the main, reverse, and advice cards, respectively.
	 */
	let uniq = new Set<TarotCard>;
	do {
		let c = tarot_cards[Math.floor(Math.random() * tarot_cards.length)];
		if (c.imglink != "") uniq.add(c);
	} while (uniq.size < 3);

	let cards = Array.from(uniq);
	return [cards[0], cards[1], cards[2]];
}

export function set_cooldown(uid: string, set: Set<string>, timeout: number): void {
	/**
	 * Adds the target's UID to a given set for a given time (in ms) to track cooldown.
	 * 
	 * @param uid - The Discord user id in string form.
	 * @param set - The set to add the uid to.
	 * @param set - How long to wait (in ms) before removing the uid from the set.
	 */
	set.add(uid);
	setTimeout(() => {
		set.delete(uid);
	}, timeout);
}

function random_index_weighted(weights: Array<number>): number {
	/**
	 * Returns a randomly chosen index based on a set of weights.
	 * Based on this Python implementation: https://stackoverflow.com/a/10803136.
	 * 
	 * @param weights - An array of floating point weights.
	 * @returns An integer index.
	 */
	let r = Math.random();
	let cumul_sum = new Array(weights.length);
	for (let i = 1; i < cumul_sum.length; i++) {
		cumul_sum[i] = weights.slice(0, i).reduce((x, y) => x + y);
	}
	for (let i = 0; i < cumul_sum.length; i++) {
		if (cumul_sum[i] > r) return i;
	}
	return 0;
}
