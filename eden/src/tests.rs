#[cfg(test)]
mod db_tests {
    use crate::db::SQLdb;
    use crate::handle;

    // It seems to be hard to write test for this application given 
    // that the live data is non-static, and we cannot really expect 
    // a definite outcome to compare to.
    
    // As far as I'm concerned, internal errors are handled in all 
    // the possible places, and Eden simply returns 500 to them.

    #[test]
    fn handle_empty_buffer() {
        let mut buf1 = [' ' as u8; 8192];
        let mut buf2 = [' ' as u8; 8192];
        let expected_res = b"HTTP/1.1 200 OK\n\r\n\r{\"status\":\"404\"}";
        handle(&mut buf1);
        for i in 0..35 {
            buf2[i] = expected_res[i];
        }
        assert_eq!(&buf1, &buf2);
    }

    #[test]
    fn card_draw() {
        let db = SQLdb::new("/db/eden/user.db").unwrap();
        let res = db.card_draw().unwrap();
        assert_eq!(res["status"], "200");
    }
}