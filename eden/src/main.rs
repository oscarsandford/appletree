use eden::{
	receive, net::ThreadPool,
	ADDRESS, NWORKERS, DB_PATH,
};

fn main() {
	let listener = std::net::TcpListener::bind(ADDRESS).expect("Address should be valid");
	let pool = ThreadPool::new(NWORKERS);
	println!("Eden\n - Address: {}\n - Workers: {}\n - Database: {}", ADDRESS, NWORKERS, DB_PATH);
	for stream in listener.incoming() {
		if let Ok(stream) = stream {
			pool.execute(|| {
				receive(stream);
			});
		}
	}
}