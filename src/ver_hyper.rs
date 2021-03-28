use hyper::{Body, Request, Response, Server, StatusCode, service::{make_service_fn, service_fn}};
use simple_server::GenericResult;
use tokio::fs::read_to_string;

async fn paths(req: Request<Body>) -> Result<Response<Body>, hyper::Error>{
	let mut path = req.uri().path();
	if path == "/" {path = "home";}

	let response = if let Ok(content) = read_to_string(format!("web_files/{}.html", path)).await {
		Response::new(Body::from(content))
	} else {
		let mut response = Response::new(Body::from(read_to_string("web_files/404.html").await.unwrap()));
		*response.status_mut() = StatusCode::NOT_FOUND;

		response
	};

	Ok(response)
}

#[tokio::main]
async fn main() -> GenericResult<()> {
	let addr = ([192, 168, 1, 6], 10800).into();
	let service = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(paths)) });
	let server = Server::bind(&addr).serve(service);
	println!("Listening on http://{}", addr);
	server.await?;

	Ok(())
}
