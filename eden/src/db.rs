use rusqlite::{Connection, Result, Error, params};
use serde::Deserialize;
use serde_json::{json, Value};
use rand::{thread_rng, distributions::WeightedIndex, distributions::Distribution};

#[derive(Deserialize, Debug)]
struct Quote {
	quote: String,
	quotee: String,
	quoter: String,
	qweight: f32,
}

#[derive(Deserialize, Debug)]
struct Request {
	query: String,
	requester: String,
}

pub struct SQLdb {
	pub conn: Connection
}

impl SQLdb {
	pub fn new(dbfpath: &str) -> Result<SQLdb, Error> {
		let conn = self::Connection::open(dbfpath)?;
		Ok( SQLdb { conn } )
	}

	pub fn quote_draw(&self) -> Result<Value, Box<dyn std::error::Error>> {
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
		let sum = self.conn.query_row_and_then("SELECT SUM(qweight) FROM quotes", [], |row| row.get(0)).unwrap_or(1.0);

		// Reduce the weight of the randomly selected quote, so it is less likely to be pulled next.
		if let Ok(n) = self.conn.execute("UPDATE quotes SET qweight = qweight * 0.5 WHERE quote = ?1", params![quote.quote]) {
			println!("[Eden] Adjusted drawn quote weight (updated {} rows).", n);
		};
		// Normalize the weights so that the sum of qweights is close enough to 1.
		if let Ok(n) = self.conn.execute("UPDATE quotes SET qweight = qweight / ?1", params![sum]) {
			println!("[Eden] Normalized quote weights (updated {} rows).", n);
		};

		Ok(json!({
			"status" : "200",
			"quote" : quote.quote,
			"quotee" : quote.quotee,
			"quoter" : quote.quoter,
			"qweight" : quote.qweight
		}))
	}

	pub fn quote_find(&self, req_json: Value) -> Result<Value, Box<dyn std::error::Error>> {
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

	pub fn quote_add(&self, req_json: Value) -> Result<Value, Box<dyn std::error::Error>> {
		let quote: Quote = serde_json::from_value(req_json)?;
		println!("[Eden:quotes] Adding {:?}", &quote);
		let n = self.conn.execute("INSERT INTO quotes (quote, quotee, quoter, qweight) VALUES (?1, ?2, ?3, ?4)", 
			(&quote.quote, &quote.quotee, &quote.quoter, &quote.qweight)
		)?;
		println!("[Eden:quotes] Updated {} rows.", n);
		Ok(json!({ "status" : "200" }))
	}

	pub fn quote_remove(&self, req_json: Value) -> Result<Value, Box<dyn std::error::Error>> {
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
						let n = self.conn.execute("DELETE FROM quotes WHERE ( quote = ?1 ) AND ( quotee = ?2 ) AND ( quoter = ?3 )", 
							(&quote.quote, &quote.quotee, &quote.quoter)
						)?;
						println!("[Eden:quotes] Updated {} rows.", n);
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
}