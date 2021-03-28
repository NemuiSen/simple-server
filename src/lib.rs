pub type GenericResult<T> = Result<T, Box<dyn std::error::Error>>;

// things for ver_tcplistener
pub const   OK: &str = "HTTP/1.1 200 OK\r\n\r\n";
pub const E404: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";

pub fn chunk_from_chars(str_val: &str, pos: usize, start: char, end: char) -> Option<&str> {
	let start_pos = str_val[      pos..].find(start)?+      pos;
	let   end_pos = str_val[start_pos..].find(  end)?+start_pos;
	Some(&str_val[start_pos..end_pos])
}