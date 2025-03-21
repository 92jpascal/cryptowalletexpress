


// Feature
// Webhook Notifications for Transaction Confirmations


use reqwest::Client;
use serde::Serialize;

// Struct for webhook payload
#[derive(Serialize)]
pub struct WebhookPayload {
	pub tx_id: String,
	pub status: String,
	pub currency String,
	pub amount: f64,
	pub address: String,
}

// Function to send webhook when a transaction is confirmed
// "reqwest::Client" allows for reuse of connections
// and improved performance
pub async fn send_transaction_webhook(webhook url: &str, payload: &WebhookPayload) -> Result<(), reqwest::Error> {
	let client = Client::new();
	let res = client.post(webhook_url)
		.json(payload)
		.send()
		.await?;

	println!("Webhook sent. Status: {:?}", res.status());
	Ok(())
}
