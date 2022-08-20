// Barebones web server skeleton code.
// This code is more designed for serving HTML files, 
// but will be adapted to be an interface to a SQL db.
// We will use https://docs.rs/rust-sqlite/latest/sqlite3/.

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::fs;
use std::error::Error;

const ADDR: &'static str = "127.0.0.1";
const PORT: u16 = 8000;
const PUBLIC_PFX: &'static str = "";
const GET: &'static [u8; 16] = b"GET / HTTP/1.1\r\n";


fn handle_request(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
	let mut buf = [0; 1024];
	stream.read(&mut buf)?;

	let (status, resource_name) = if buf.starts_with(GET) {
		("HTTP/1.1 200 OK", "hello.html")
	} else {
		("HTTP/1.1 404 NOT FOUND", "404.html")
	};
	
	let content = fs::read_to_string(format!("{}/{}", PUBLIC_PFX, resource_name))?;
	let response = format!(
		"{}\r\nContent-Length: {}\r\n\r\n{}",
		status,
		content.len(),
		content
	);
	stream.write(response.as_bytes())?;
	stream.flush()?;
	
	Ok(())
}


fn main() -> std::io::Result<()> {
	let listener = TcpListener::bind(format!("{}:{}", ADDR, PORT))?;
	
	for stream in listener.incoming() {
		match stream {
			Ok(stream) => {
				println!("Connection established!");
				handle_request(stream).unwrap_or_else(|e| {
					eprintln!("Err: something went wrong while handling the request.\n{:?}", e);
				});
			},
			Err(e) => eprintln!("Err: connection refused.\n{:?}", e),
		}
	}
	Ok(())
}