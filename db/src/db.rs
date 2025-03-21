


use tokio_postgres::{NoTls, Client};

pub async fn connect_db() -> Client {
	let (client, connection) = tokio_postgres::connect("host=localhost user=postgres dbname=wallet", NoTls).await.unwrap();
	tokio::spawn(async move {
		connection.await.unwrap();
	});
	client
}
