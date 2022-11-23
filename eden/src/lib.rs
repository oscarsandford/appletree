mod db;
mod types;

pub mod net;
pub mod tests;

use std::{net::TcpStream, io::{Read, BufReader, Write}};
use serde_json::{json, Value};

use crate::db::SQLdb;
use crate::types::EdenErr;

const RECV_BUFFER_SIZE: usize = 1024;
// The cfg!(debug_assertions) macro indicates we use the first value if we are running 
// a debug binary. If it is a release binary, or otherwise, we use production values.
pub const DB_PATH: &str = if cfg!(debug_assertions) {"./db/user.db"} else {"/db/eden/user.db"};
pub const ADDRESS: &str = if cfg!(debug_assertions) {"127.0.0.1:8080"} else {"0.0.0.0:8080"};
pub const NWORKERS: usize = if cfg!(debug_assertions) {4} else {8};


fn handle(buf: &[u8]) -> Vec<u8> {
	// Bytes to JSON.
	let bufst = String::from_utf8_lossy(&buf).replace("\0", "");
	let buflns = bufst.split("\r\n").collect::<Vec<&str>>();
	let bodyst = buflns.last().unwrap_or(&"");

	let path = buflns.first().unwrap_or(&"").split_whitespace().nth(1).unwrap_or("/");
	let body_json: Value = serde_json::from_str(bodyst).unwrap_or(json!({}));

	println!("[ REQ {} <-- ] : {:?}", path, &body_json);

	let payload_json = match SQLdb::new(DB_PATH) {
		Ok(db) => {
			let res_json: Result<Value, EdenErr> = match path {
				"/db/quote/draw" => db.quote_draw(),
				"/db/quote/find" => db.quote_find(body_json),
				"/db/quote/add" => db.quote_add(body_json),
				"/db/quote/remove" => db.quote_remove(body_json),
				"/db/user" => db.get_user(body_json),
				"/db/user/xp" => db.set_user_xp(body_json),
				"/db/user/credit" => db.set_user_credit(body_json),
				"/db/user/bg" => db.set_user_bg(body_json),
				"/db/card/draw" => db.card_draw(),
				"/db/card/add" => db.card_add(body_json),
				"/db/item" => db.item_get(body_json),
				"/db/item/add" => db.item_add(body_json),
				_ => Ok(json!({"status":"404"})),
			};

			if let Err(e) = db.conn.close() {
				eprintln!("<!> Database closure error: \n{:?}", e.1);
			};

			match res_json {
				Ok(val) => val,
				Err(e) => json!({"status":"500","payload" : format!("{e:?}")}),
			}
		},
		Err(e) => {
			eprintln!("<!> Database connection error: \n{:?}", e);
			json!({"status":"500","payload" : format!("{e:?}")})
		}
	};

	if let Ok(payload) = serde_json::to_vec(&payload_json) {
		println!("[ RES {} --> ] : {}", path, String::from_utf8_lossy(&payload));
		let mut res = b"HTTP/1.1 200 OK\n\r\n\r".to_vec();
		res.extend(&payload);
		res
	}
	else {
		b"HTTP/1.1 500 INTERNAL SERVER ERROR\n\r\n\r".to_vec()
	}
}

pub fn receive(mut stream: TcpStream) {
	let mut buf_reader = BufReader::new(&mut stream);
	let mut buf = [' ' as u8; RECV_BUFFER_SIZE];
	if let Err(e) = buf_reader.read(&mut buf) {
		eprintln!("<!> Socket read failed: {:?}", e);
	};
	let res = handle(&buf);
	if let Err(e) = stream.write_all(&res) {
		eprintln!("<!> Socket write failed: {:?}", e);
	}
}