#[cfg(test)]
mod tests {
	use serde_json::json;
	use crate::db::SQLdb;

	const DEV_DB_PATH: &str = "./db/user.db";

	#[test]
	fn handle_empty_buffer() {}

	#[test]
	fn handle_bad_path() {}

	#[test]
	fn quote_draw_ok() {
		let db = SQLdb::new(DEV_DB_PATH).unwrap();
		let res = db.quote_draw().unwrap();
		assert_eq!(res["status"], "200");
	}

	#[test]
	fn card_draw_ok() {
		let db = SQLdb::new(DEV_DB_PATH).unwrap();
		let res = db.card_draw().unwrap();
		assert_eq!(res["status"], "200");
	}

	#[test]
	fn quote_not_found() {
		let db = SQLdb::new(DEV_DB_PATH).unwrap();
		let res = db.quote_find(json!({ 
			"query" : "asdfjkleibt", // Insert arbitrary text that is unlikely to be in the dev db.
			"requester" : "unknown" })).unwrap();
		assert_eq!(res["status"], "404");
	}

	#[test]
	fn quote_not_found_remove() {
		let db = SQLdb::new(DEV_DB_PATH).unwrap();
		let res = db.quote_remove(json!({ 
			"query" : "asdfjkleibt", // Insert arbitrary text that is unlikely to be in the dev db.
			"requester" : "unknown" 
		})).unwrap();
		assert_eq!(res["status"], "404");
	}

	#[test]
	fn user_does_not_exist() {
		let db = SQLdb::new(DEV_DB_PATH).unwrap();
		let res = db.get_user(json!({ 
			"query" : "", 
			"requester" : "aifhg389awg398rs3" // Insert arbitrary text that is unlikely to be in the dev db.
		})).unwrap();
		assert_eq!(res["status"], "404");
	}

	#[test]
	fn items_collection_does_not_exist() {
		let db = SQLdb::new(DEV_DB_PATH).unwrap();
		let res = db.item_get(json!({ 
			"src" : "", 
			"ownr" : "aifhg389awg398rs3", // Insert arbitrary text that is unlikely to be in the dev db.
			"lvl" : 0, 
			"xp": 0
		})).unwrap();
		assert_eq!(res["payload"], json!([]));
	}
}