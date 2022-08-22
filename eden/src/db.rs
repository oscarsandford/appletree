use rusqlite::{Connection, Result, Error, params};
use serde_json::{json, Value};
use rand::{thread_rng, Rng, distributions::WeightedIndex, distributions::Distribution};

struct Quote {
	id: u16,
	quote: String,
	quotee: String,
	quoter: String,
	qweight: f32,
}

pub struct SQLdb {
	conn: Connection
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
				id: row.get(0)?,
				quote: row.get(1)?,
				quotee: row.get(2)?,
				quoter: row.get(3)?,
				qweight: row.get(4)?,
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

		// Reduce the weight of the randomly selected quote, so it is less likely to be pulled next.
		// TODO: Do we need to sanitize, or is this safe?
		if let Ok(n) = self.conn.execute("UPDATE quotes SET qweight = qweight * 0.5 WHERE id = ?1", params![quote.id]) {
			println!("[Eden] Updated {} rows.", n);
		};

		Ok(json!({
			"status" : "OK",
			"quote" : quote.quote,
			"quotee" : quote.quotee
		}))
	}

	pub fn quote_find(&self, qsubstr: String) -> Result<Value, Box<dyn std::error::Error>> {
		let mut stmt = self.conn.prepare("SELECT * FROM quotes WHERE quote LIKE '%?%' LIMIT 1")?;
		let quote = stmt.query_and_then([qsubstr], |row| {
			Ok(Quote {
				id: row.get(0)?,
				quote: row.get(1)?,
				quotee: row.get(2)?,
				quoter: row.get(3)?,
				qweight: row.get(4)?,
			})
		})?.next()??;

		// TODO: maybe we want to send back the quote id? Since it could be a good unique identifier for removing quotes.
		Ok(json!({
			"status" : "OK",
			"quote" : quote.quote,
			"quotee" : quote.quotee
		}))
	}

	pub fn quote_add(&self, qjson: Value) -> Result<Value, Box<dyn std::error::Error>> {
		let quote: Quote = serde_json::from_value(qjson)?;
		let n = self.conn.execute("INSERT INTO quotes (quote, quotee, quoter, qweight", 
			(&quote.quote, &quote.quotee, &quote.quoter, &quote.qweight)
		)?;
		println!("[Eden] Updated {} rows.", n);
		Ok(json!({ "status" : "OK" }))
	}

	// pub fn quote_remove(&self, qsubstr: String) -> Result<Value, Box<dyn std::error::Error>> {

	// }
}