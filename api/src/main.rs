


// If being used from inside a container, the Axum server is bound to
// localhost inside the container -- need to update binding
use axum::{
	routing::{get, post},
	Router,
};
use std::net::SocketAddr;

mod routes;

#[tokio::main]
async fn main() {
	println!("Hello, world!");

	let app = Router::new()
		.nest("/wallets", routes::wallets::routes())
		.nest("/transactions", routes::transactions::routes())
		.nest("/withdraw", routes::withdraw::routes());

	let addr = SocketAddr::from(([0, 0, 0, 0], 5901)); // 0.0.0.0 binds to all network interfaces,
							   // including Docker's external bridge
	println!("API running at http://{}", addr);

	axum::Server::bind(&addr)
		.serve(app.into_make_service())
		.await
		.unwrap();
}
