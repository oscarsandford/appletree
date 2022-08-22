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

		// Reduce the weight of the randomly selected quote, so it is less likely to be pulled next.
		if let Ok(n) = self.conn.execute("UPDATE quotes SET qweight = qweight * 0.5 WHERE quote = ?1", params![quote.quote]) {
			println!("[Eden] Updated {} rows.", n);
		};

		Ok(json!({
			"status" : "OK",
			"quote" : quote.quote,
			"quotee" : quote.quotee
		}))
	}

	pub fn quote_find(&self, qsubstr: String) -> Result<Value, Box<dyn std::error::Error>> {
		let substr = qsubstr.replace("\\", "").replace("\"", "");

		// This is super bad, but I cannot figure out how to format the query params otherwise. Fix later.
		let query = format!("SELECT * FROM quotes WHERE quote LIKE '%{}%' LIMIT 1", substr);

		let mut stmt = self.conn.prepare(&query)?;
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
		if quotes.len() == 0 {
			Ok(json!({ "status" : "204" }))
		}
		else {
			Ok(json!({
				"status" : "200",
				"quote" : &quotes[0].quote,
				"quotee" : &quotes[0].quotee
			}))
		}
	}

	pub fn quote_add(&self, qjson: Value) -> Result<Value, Box<dyn std::error::Error>> {
		let quote: Quote = serde_json::from_value(qjson)?;
		println!("[Eden:quotes] Adding {:?}", &quote);
		let n = self.conn.execute("INSERT INTO quotes (quote, quotee, quoter, qweight) VALUES (?1, ?2, ?3, ?4)", 
			(&quote.quote, &quote.quotee, &quote.quoter, &quote.qweight)
		)?;
		println!("[Eden:quotes] Updated {} rows.", n);
		Ok(json!({ "status" : "200" }))
	}

	// pub fn quote_remove(&self, qsubstr: String) -> Result<Value, Box<dyn std::error::Error>> {

	// }
}