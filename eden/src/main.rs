mod db;
mod types;

use std::{net::TcpListener, io::{Read, BufReader, Write}};
use serde_json::{json, Value};
use chrono::Local;

use db::SQLdb;
use crate::types::EdenErr;

fn handle(buf: &mut [u8]) {
	// Bytes to JSON.
	let bufst = String::from_utf8_lossy(&buf).replace("\0", "");
	let buflns = bufst.split("\r\n").collect::<Vec<&str>>();
	let bodyst = buflns.last().unwrap_or(&"");

	let path = buflns.first().unwrap_or(&"").split_whitespace().nth(1).unwrap_or("/");
	let body_json: Value = serde_json::from_str(bodyst).unwrap_or(json!({}));

	println!("({}) [Eden] req: ({}) {:?}", Local::now(), path, &body_json);

	// I think it's better to open a new connection for each request. Keeps things atomic.
	let db = match SQLdb::new("/db/eden/user.db") {
		Ok(x) => x,
		Err(_) => {
			eprintln!("({}) [Eden] Database connection error. Request handling aborted.", Local::now());
			return;
		}
	};

	let res: Result<Value, EdenErr> = match path {
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

	let res_json: Value = match res {
		Ok(val) => val,
		Err(e) => json!({"status":"500","payload" : format!("{e:?}")}),
	};

	// Write response: overwrite the bytes in the buffer, and pad with zeroes.
	if let Ok(res_bytes) = serde_json::to_vec(&res_json) {
		// Prepend a simple HTTP/1.1 header. We know this header will always be 19 bytes.
		let header = b"HTTP/1.1 200 OK\n\r\n\r";
		for i in 0..19 {
			buf[i] = header[i];
		}
		for i in 0..8173 {
			buf[19+i] = if i < res_bytes.len() { res_bytes[i] } else { ' ' as u8 };
			// Having a `0 as u8` instead of `' ' as u8` for the longest time cost so much grief on the front end.
		}
		println!("({}) [Eden] res: {}\n", Local::now(), String::from_utf8_lossy(&res_bytes));
	}

	// TODO: This should be done in a neater fashion.
	match db.conn.close() {
		Ok(_) => {},
		Err(_) => {
			eprintln!("({}) [Eden] Database closure error. Request handling aborted.", Local::now());
			return;
		}
	};
}

fn main() {
	let addr = if cfg!(debug_assertions) {"127.0.0.1:8080"} else {"0.0.0.0:8080"};
	if let Ok(listener) = TcpListener::bind(addr) {
		println!("({}) [Eden] Listening on {:?}", Local::now(), addr);
		let mut buf = [0u8; 8192];
		for stream in listener.incoming() {
			if let Ok(mut stream) = stream {
				let mut buf_reader = BufReader::new(&mut stream);
				if let Err(e) = buf_reader.read(&mut buf) {
					eprintln!("({}) [Eden] Socket read failed: {:?}", Local::now(), e);
				}
				handle(&mut buf);
				if let Err(e) = stream.write_all(&buf) {
					eprintln!("({}) [Eden] Socket write failed: {:?}", Local::now(), e);
				}
			}
		}
	}
}