import * as cards_json from "./resources/cards.json";
import * as tarot_json from "./resources/tavernarcana.json";
export const trading_cards: Array<TradingCard> = cards_json;
export const tarot_cards: Array<TarotCard> = tarot_json;
export const card_weights: Array<number> = [0.19, 0.3, 0.4, 0.1, 0.01];

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

export interface Quote {
	status: string,
	quote: string,
	quotee: string,
	quoter: string,
	qweight: string
}


export function draw_trading_card(): TradingCard {
	let rand_rank = random_index_weighted(card_weights);
	let selected_cards = trading_cards.filter((c) => c.rank === rand_rank);
	let rand_card = selected_cards[Math.floor(Math.random() * selected_cards.length)];
	rand_card.level = 0;
	return rand_card;
}

export function draw_tarot_cards(drawer: string): [TarotCard, TarotCard, TarotCard] {
	let selected_cards = new Array<TarotCard>;
	// Get three unique cards
	//  - first one is primary
	//  - second is its reverse, or warning
	//  - third is some more advice
	while (selected_cards.length < 3) {
		let c: TarotCard;
		do {
			c = tarot_cards[Math.floor(Math.random() * tarot_cards.length)];
		} while (c.imglink == "" || selected_cards.includes(c));
		selected_cards.push(c);
	}
	// For lucky primary draws on The World. Yes, this is a JoJo reference!
	if (selected_cards[0]["id"] === 21) {
		selected_cards[0]["description"] = `I, ${drawer}, have a dream!`;
	}
	return [selected_cards[0], selected_cards[1], selected_cards[2]];
}

export function set_cooldown(target: string, set: Set<string>, timeout: number): void {
	set.add(target);
	setTimeout(() => {
		set.delete(target);
	}, timeout);
}

function random_index_weighted(weights: Array<number>): number {
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