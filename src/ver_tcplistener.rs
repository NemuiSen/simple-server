use std::{fs::read_to_string, io::{Read, Write}, net::{TcpListener, TcpStream}};
use simple_server::{E404, GenericResult, OK, chunk_from_chars};

fn handle_client(mut stream: TcpStream) -> GenericResult<()> {
	let mut buffer = [0; 1024];
	stream.read(&mut buffer)?;

	let msg = String::from_utf8_lossy(&buffer);
	let dir = if let Some (dir) = chunk_from_chars(&msg, 0, '/', ' ') {
		if dir == "/" {"home"} else {dir}
	} else { "" };

	if msg.starts_with("GET") {
		if let Ok(content) = read_to_string(format!("web_files/{}.html", dir)) {
			let response = format!("{}{}", OK, content);
			stream.write(response.as_bytes())?;
		} else {
			let content = read_to_string("web_files/404.html")?;
			let response = format!("{}{}", E404, content);
			stream.write(response.as_bytes())?;
		};
	}
	Ok(stream.flush().unwrap())
}

fn main() -> GenericResult<()> {
	let listener = TcpListener::bind("192.168.1.6:10800")?;
	println!("Listening on http://{}", listener.local_addr().unwrap());
	for stream in listener.incoming() {
		handle_client(stream?)?;
	}

	Ok(())
}