use serde::Deserialize;
use rand::distributions;


#[derive(Debug)]
pub enum EdenErr {
	SQLError(rusqlite::Error),
	JSONError(serde_json::Error),
	WeightError(distributions::WeightedError),
}
impl std::fmt::Display for EdenErr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			EdenErr::SQLError(e) => write!(f, "[!] SQL query error:\n{}", e),
			EdenErr::JSONError(e) => write!(f, "[!] JSON parsing error:\n{}", e),
			EdenErr::WeightError(e) => write!(f, "[!] Distribution error:\n{}", e),
		}
	}
}
impl From<rusqlite::Error> for EdenErr {
	fn from(e: rusqlite::Error) -> Self { EdenErr::SQLError(e) }
}
impl From<serde_json::Error> for EdenErr {
	fn from(e: serde_json::Error) -> Self { EdenErr::JSONError(e) }
}
impl From<distributions::WeightedError> for EdenErr {
	fn from(e: distributions::WeightedError) -> Self { EdenErr::WeightError(e) }
}


#[derive(Deserialize, Debug)]
pub struct Quote {
	// TODO: make Strings for discord ids [u8; 30]
	pub quote: String,
	pub quotee: String,
	pub quoter: String,
	pub qweight: f64,
}

#[derive(Deserialize, Debug)]
pub struct User {
	pub id: String,
	pub lvl: u16,
	pub xp: u32,
	pub credit: i32,
	pub bg: String,
}

#[derive(Deserialize, Debug)]
pub struct Card {
    pub csrc: String,
    pub cname: String,
	pub crank: u8,
	pub element: String,
	pub atk: u32,
	pub lufa: f32,
	pub def: f32,
	pub lufd: f32,
	pub utl: u32,
	pub lufu: f32,
	// TODO: see above
    pub subjct: String,
	pub adder: String,
	// This could be a bool, but SQLite cannot store bools, so it is an integer either 1 or 0.
	pub tradable: u8, 
}

#[derive(Deserialize, Debug)]
pub struct Item {
    pub src: String,
    pub ownr: String,
	pub lvl: u16,
    pub xp: u32,
}

#[derive(Deserialize, Debug)]
pub struct Request {
	pub query: String,
	pub requester: String,
}

// enum Payload {
// 	Text(String),
// 	Quote(Quote),
// 	Card(Card),
// }

// #[derive(Deserialize, Debug)]
// pub struct Request {
// 	pub payload: Payload,
// 	pub requester: String,
// }