


use axum::response::IntoResponse;

use axum::{response::Json, routing::get, Router};
use serde_json::{json, Value};

pub fn routes() -> Router {
	Router::new().route("/", get(get_transactions))
}

async fn get_transactions() -> impl IntoResponse {
	Json(json!([
		{
			"tx_id": "0xabc123",
			"type": "deposit",
			"currency": "ETH",
			"amount": "0.5"
		},
		{
			"tx_id": "0xdef456",
			"type": "withdrawal",
			"currency": "BTC",
			"amount": "0.1"
		}
	]))
}
