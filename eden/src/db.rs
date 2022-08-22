use rusqlite::{Connection, Result, Error};
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
		// TODO: after drawing this quote, reduce the weight of selected quote, 
		// so that it is less likely to show up soon after.
		let dist = WeightedIndex::new(quotes.iter().map(|q| q.qweight))?;
		let ridx: usize = dist.sample(&mut rng);

		Ok(json!({
			"quote" : quotes[ridx].quote,
			"quotee" : quotes[ridx].quotee,
		}))
	}

	// fn quote_find(&self, quote: Quote) -> Value {

	// }

	// fn quote_add(&self, quote: Quote) -> Value {

	// }

	// fn quote_remove(&self, quote: Quote) -> Value {

	// }
}