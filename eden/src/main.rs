mod db;

use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde_json::json;

use db::SQLdb;


fn handle(buf: &mut [u8]) {
	// Bytes to JSON
	let bufst = String::from_utf8_lossy(&buf).replace("\0", "");
	let buflns = bufst.split("\r\n").collect::<Vec<&str>>();
	let bodyst = buflns.last().unwrap_or(&"");

	let path = buflns.first().unwrap_or(&"").split_whitespace().nth(1).unwrap_or("/");
	let body_json: serde_json::Value = serde_json::from_str(bodyst).unwrap_or(json!({}));

	// TODO: ideally, we can make use of this db connection multiple times.
	let db = SQLdb::new("./db/user.db").unwrap(); // Fix this unwrap

	let res_json: serde_json::Value = match path {
		"/db/quote/draw" => db.quote_draw().unwrap_or(json!({"status":"500"})),
		"/db/quote/find" => db.quote_find(body_json["qsubstr"].to_string()).unwrap_or(json!({"status":"500"})),
		"/db/quote/add" => db.quote_add(body_json).unwrap_or(json!({"status":"500"})),
		// "/db/quote/remove" => {},
		_ => {json!({"status":"404"})},
	};
	println!("res(json): {:?}", res_json);

	// Write response: overwrite the bytes in the buffer, and pad with zeroes.
	if let Ok(res_bytes) = serde_json::to_vec(&res_json) {
		for i in 0..1024 {
			buf[i] = if i < res_bytes.len() { res_bytes[i] } else { 0 as u8 };
		}
	}
	println!("new buf: {:?}", String::from_utf8_lossy(&buf));
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("[Eden] Starting");
	let listener = TcpListener::bind("127.0.0.1:8080").await?;

	loop {
		let (mut socket, _) = listener.accept().await?;

		tokio::spawn(async move {
			let mut buf = [0; 1024];

			// Read data from socket. Writeback to socket.
			loop {
				let n = match socket.read(&mut buf).await {
					Ok(n) if n == 0 => {return;},
					Ok(n) => n,
					Err(e) => {
						eprintln!("[Eden] Socket read failed: {:?}", e);
						return;
					}
				};

				handle(&mut buf);

				if let Err(e) = socket.write_all(&buf[0..n]).await {
					eprintln!("[Eden] Socket write failed: {:?}", e);
					return;
				}
			}
		});
	}
}