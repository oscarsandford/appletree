use rusqlite::Connection;
use serde_json::{json, Value};
use rand::{thread_rng, distributions::WeightedIndex, distributions::Distribution};

use crate::types::*;

// TODO: rewrite this to drop the SQLdb methods, instead making them functions that take a Connection as an argument.

pub struct SQLdb {
	pub conn: Connection
}

impl SQLdb {
	pub fn new(dbfpath: &str) -> Result<SQLdb, EdenErr> {
		let conn = self::Connection::open(dbfpath)?;
		// For SQLite3, we have to force FK constraints on entry.
		conn.pragma_update(None, "foreign_keys", true)?;
		Ok( SQLdb { conn } )
	}

	fn set_xp(&self, uid: &String, delta: i32) -> Result<Value, EdenErr> {
		// A private function to set XP and update level as needed.
		let n = self.conn.execute("UPDATE users SET xp = xp + ?1 WHERE id = ?2", (delta, uid))?;
		// Add the user if this operation did not update an existing record.
		if n == 0 {
			self.conn.execute("INSERT INTO users (id, xp) VALUES(?1, ?2)", (uid, delta))?;
		}
		let new_xp: u32 = self.conn.query_row("SELECT xp FROM users WHERE id = ?", [uid], |row| row.get(0)).unwrap_or(0);
		if new_xp != 0 {
			let new_lvl = ((new_xp / 314) as f32).sqrt() as u16;
			self.conn.execute("UPDATE users SET lvl = ?1 WHERE id = ?2", (new_lvl, uid))?;
		}
		Ok(json!({ "status" : "200" }))
	}

	pub fn quote_draw(&self) -> Result<Value, EdenErr> {
		let mut stmt = self.conn.prepare("SELECT * FROM quotes")?;
		let retrieved_quotes = stmt.query_map([], |row| {
			Ok(Quote {
				quote: row.get(0)?,
				quotee: row.get(1)?,
				quoter: row.get(2)?,
				qweight: row.get(3)?,
			})
		})?;
		let mut quotes = Vec::new();
		for q in retrieved_quotes {
			quotes.push(q?);
		}
		let mut rng = thread_rng();
		// Weighted selection. 
		let dist = WeightedIndex::new(quotes.iter().map(|q| q.qweight))?;
		let ridx: usize = dist.sample(&mut rng);
		let quote = &quotes[ridx];

		// We have to use f64 because SQLite REAL values are stored in IEEE 754 Binary-64 format (https://www.sqlite.org/floatingpoint.html).
		let sum: f64 = self.conn.query_row("SELECT SUM(qweight) FROM quotes", [], |row| row.get(0)).unwrap_or(1.0);

		// Reduce the weight of the randomly selected quote, so it is less likely to be pulled next.
		self.conn.execute("UPDATE quotes SET qweight = qweight * 0.5 WHERE quote = ?", [&quote.quote])?;
		// Normalize the weights so that the sum of qweights is close enough to 1.
		self.conn.execute("UPDATE quotes SET qweight = qweight / ?", [sum])?;

		Ok(json!({
			"status" : "200",
			"quote" : quote.quote,
			"quotee" : quote.quotee,
			"quoter" : quote.quoter,
			"qweight" : quote.qweight
		}))
	}

	pub fn quote_find(&self, req_json: Value) -> Result<Value, EdenErr> {
		// Make the substring lowercase so that we are matching case in the query.
		let req: Request = serde_json::from_value(req_json)?;
		let quote_subst = req.query.replace("\\", "").replace("\"", "").to_lowercase();

		// This is super bad, but I cannot figure out how to format the query params otherwise. 
		// I don't know if I can trust this query to be sanitized/validated down the pipeline. Fix later.
		let query = format!("SELECT * FROM quotes WHERE LOWER(quote) LIKE '%{}%' LIMIT 1", quote_subst);

		let mut stmt = self.conn.prepare(&query)?;
		let mut res = stmt.query_map([], |row| {
			Ok(Quote {
				quote: row.get(0)?,
				quotee: row.get(1)?,
				quoter: row.get(2)?,
				qweight: row.get(3)?,
			})
		})?;
		
		match res.next() {
			None => Ok(json!({ "status" : "404" })), 
			Some(el) => {
				if let Ok(quote) = el {
					Ok(json!({
						"status" : "200",
						"quote" : &quote.quote,
						"quotee" : &quote.quotee,
						"quoter" : &quote.quoter,
						"qweight" : &quote.qweight
					}))
				}
				else {
					Ok(json!({ "status" : "500" }))
				}
			},
		}
	}

	pub fn quote_add(&self, req_json: Value) -> Result<Value, EdenErr> {
		let quote: Quote = serde_json::from_value(req_json)?;
		// Make sure the quotee and quoter are already users. If they 
		// are, the PK constraint will disallow these insertions.
		self.conn.execute("INSERT INTO users (id) VALUES(?)", [&quote.quotee]).unwrap_or_default();
		self.conn.execute("INSERT INTO users (id) VALUES(?)", [&quote.quoter]).unwrap_or_default();
		// The weight of this quote as it is inserted is the average of the weights as they are now.
		let avgw: f64 = self.conn.query_row("SELECT AVG(qweight) FROM quotes", [], |row| row.get(0)).unwrap_or(quote.qweight);
		self.conn.execute(
			"INSERT INTO quotes (quote, quotee, quoter, qweight) VALUES (?1, ?2, ?3, ?4)", 
			(&quote.quote, &quote.quotee, &quote.quoter, &avgw))?;
		// Getting quoted gives a flat 100 XP.
		Ok(self.set_xp(&quote.quotee, 100)?)
	}

	pub fn quote_remove(&self, req_json: Value) -> Result<Value, EdenErr> {
		let req: Request = serde_json::from_value(req_json)?;
		// Make the substring lowercase so that we are matching case in the query.
		let quote_subst = req.query.replace("\\", "").replace("\"", "").to_lowercase();
		
		// This is super bad, but I cannot figure out how to format the query params otherwise. 
		// I don't know if I can trust this query to be sanitized/validated down the pipeline. Fix later.
		let query = format!("SELECT * FROM quotes WHERE LOWER(quote) LIKE '%{}%' LIMIT 1", quote_subst);

		let mut stmt = self.conn.prepare(&query)?;
		let mut res = stmt.query_map([], |row| {
			Ok(Quote {
				quote: row.get(0)?,
				quotee: row.get(1)?,
				quoter: row.get(2)?,
				qweight: row.get(3)?,
			})
		})?;

		match res.next() {
			None => Ok(json!({ "status" : "404" })), 
			Some(el) => {
				if let Ok(quote) = el {
					// Only delete the quote if the requester is the quotee or the quoter.
					let status = if quote.quotee == req.requester || quote.quoter == req.requester { "200" } else { "403" };
					if status == "200" {
						self.conn.execute(
							"DELETE FROM quotes WHERE ( quote = ?1 ) AND ( quotee = ?2 ) AND ( quoter = ?3 )", 
							(&quote.quote, &quote.quotee, &quote.quoter))?;
					}
					Ok(json!({
						"status" : status,
						"quote" : &quote.quote,
						"quotee" : &quote.quotee,
						"quoter" : &quote.quoter,
						"qweight" : &quote.qweight
					}))
				}
				else {
					Ok(json!({ "status" : "500" }))
				}
			},
		}
	}

	pub fn get_user(&self, req_json: Value) -> Result<Value, EdenErr> {
		let req: Request = serde_json::from_value(req_json)?;
		let user = self.conn.query_row("SELECT * FROM users WHERE id = ?", [&req.requester], |row| {
			Ok(User{
				id: row.get(0)?,
				lvl: row.get(1)?,
				xp: row.get(2)?,
				credit: row.get(3)?,
				bg: row.get(4)?,
			})
		});
		match user {
			Ok(user) => {
				Ok(json!({
					"status" : "200",
					"id" : &user.id,
					"lvl" : &user.lvl,
					"xp" : &user.xp,
					"credit" : &user.credit,
					"bg" : &user.bg,
				}))
			},
			Err(_) => Ok(json!({ "status" : "404" })),
		}
	}

	pub fn set_user_xp(&self, req_json: Value) -> Result<Value, EdenErr> {
		let req: Request = serde_json::from_value(req_json)?;
		let delta = req.query.parse::<i32>().unwrap_or(0);
		Ok(self.set_xp(&req.requester, delta)?)
	}

	pub fn set_user_credit(&self, req_json: Value) -> Result<Value, EdenErr> {
		let req: Request = serde_json::from_value(req_json)?;
		let delta = req.query.parse::<i32>().unwrap_or(0);
		let n = self.conn.execute("UPDATE users SET credit = credit + ?1 WHERE id = ?2", (delta, &req.requester))?;
		if n == 0 {
			self.conn.execute("INSERT INTO users (id, credit) VALUES(?1, ?2)", (&req.requester, delta))?;
		}
		Ok(json!({ "status" : "200" }))
	}

	pub fn set_user_bg(&self, req_json: Value) -> Result<Value, EdenErr> {
		let req: Request = serde_json::from_value(req_json)?;
		// TODO: add some sanitation to make sure the payload is a valid image URL.
		let n = self.conn.execute("UPDATE users SET bg = ?1 WHERE id = ?2", (&req.query, &req.requester))?;
		if n == 0 {
			self.conn.execute("INSERT INTO users (id, bg) VALUES(?1, ?2)", (&req.requester, &req.query))?;
		}
		Ok(json!({ "status" : "200" }))
	}

	pub fn card_draw(&self) -> Result<Value, EdenErr> {
		let mut stmt = self.conn.prepare("SELECT prob FROM idprobs WHERE id > 0 AND id < 6 ORDER BY id")?;
		let weight_rows = stmt.query_map([], |row| {
			Ok( row.get(0)? )
		})?;
		let weights: Vec<f32> = weight_rows.map(|r| r.unwrap_or_default()).collect();

		let mut rng = thread_rng();
		let dist = WeightedIndex::new(weights.iter())?;
		let rrank: usize = dist.sample(&mut rng) + 1;

		let card = self.conn.query_row("SELECT * FROM cards WHERE crank = ? ORDER BY RANDOM() LIMIT 1", [&rrank], |row| {
			Ok(Card{
				csrc: row.get(0)?,
				cname: row.get(1)?,
				crank: row.get(2)?,
				element: row.get(3)?,
				atk: row.get(4)?,
				lufa: row.get(5)?,
				def: row.get(6)?,
				lufd: row.get(7)?,
				utl: row.get(8)?,
				lufu: row.get(9)?,
				subjct: row.get(10)?,
				adder: row.get(11)?,
				tradable: row.get(12)?,
			})
		})?;
		// TODO: maybe instead of having the struct, just do something here like row.get(x)? as f32
		Ok(json!({
			"status" : "200",
			"csrc" : &card.csrc,
			"cname" : &card.cname,
			"crank" : &card.crank,
			"element" : &card.element,
			"atk" : &card.atk,
			"lufa" : &card.lufa,
			"def" : &card.def,
			"lufd" : &card.lufd,
			"utl" : &card.utl,
			"lufu" : &card.lufu,
			"subjct" : &card.subjct,
			"adder" : &card.adder,
			"tradable" : &card.tradable,
		}))
	}

	pub fn card_add(&self, req_json: Value) -> Result<Value, EdenErr> {
		let c: Card = serde_json::from_value(req_json)?;
		// Make sure the subject of the card and its adder are registered.
		self.conn.execute("INSERT INTO users (id) VALUES(?)", [&c.subjct]).unwrap_or_default();
		self.conn.execute("INSERT INTO users (id) VALUES(?)", [&c.adder]).unwrap_or_default();
		self.conn.execute(
			"INSERT INTO cards VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)", 
			(&c.csrc, &c.cname, &c.crank, &c.element, &c.atk, &c.lufa, &c.def, &c.lufd, &c.utl, 
			&c.lufu, &c.subjct, &c.adder, &c.tradable))?;
		Ok(json!({ "status" : "200" }))
	}

	pub fn item_get(&self, req_json: Value) -> Result<Value, EdenErr> {
		let i: Item = serde_json::from_value(req_json)?;
		// Return everything for now (i.e. everything with i.ownr).
		let mut stmt = self.conn.prepare("SELECT cname,crank,element,lvl FROM cards JOIN items ON cards.csrc = items.src WHERE ownr = ? ORDER BY crank DESC")?;
		let rows = stmt.query_map([&i.ownr], |row| {
			Ok([
				row.get(0)?,
				row.get(1).unwrap_or(0).to_string(),
				row.get(2)?,
				row.get(3).unwrap_or(0).to_string(),
			])
		})?;
		let mut items = Vec::new();
		for r in rows {
			items.push(r?);
		}
		Ok(json!({ 
			"status" : "200",
			"payload" : items,
		}))
	}

	pub fn item_add(&self, req_json: Value) -> Result<Value, EdenErr> {
		let i: Item = serde_json::from_value(req_json)?;
		// Make sure the adder is registered.
		self.conn.execute("INSERT INTO users (id) VALUES(?)", [&i.ownr]).unwrap_or_default();

		// Increment the item level on insertion if already present, otherwise add it (default lvl is 0).
		let n = self.conn.execute("UPDATE items SET lvl = lvl + 1 WHERE src = ?1 AND ownr = ?2", (&i.src, &i.ownr))?;
		if n == 0 {
			self.conn.execute("INSERT INTO items (src, ownr) VALUES (?1, ?2)", (&i.src, &i.ownr))?;
		}
		Ok(json!({ "status" : "200" }))
	}
}
