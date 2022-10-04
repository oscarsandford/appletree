use rusqlite::Connection;
use serde_json::{json, Value};
use rand::{thread_rng, distributions::WeightedIndex, distributions::Distribution};

use crate::types::*;


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
		self.conn.execute(
			"INSERT INTO quotes (quote, quotee, quoter, qweight) VALUES (?1, ?2, ?3, ?4)", 
			(&quote.quote, &quote.quotee, &quote.quoter, &quote.qweight))?;
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
		})?;
		Ok(json!({
			"status" : "200",
			"id" : &user.id,
			"lvl" : &user.lvl,
			"xp" : &user.xp,
			"credit" : &user.credit,
			"bg" : &user.bg,
		}))
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

	// pub fn add_card(&self, req_json: Value) -> Result<Value, Box<dyn std::error::Error>> {
		
	// }
}