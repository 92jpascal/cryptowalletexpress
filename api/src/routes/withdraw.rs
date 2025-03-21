


use axum::response::IntoResponse;

use axum::{Json, Router, routing::post};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize)]
pub struct WithdrawRequest {
	currency: String,
	address: String,
	amount: f64,
}

#[derive(Debug, Serialize)]
pub struct WithdrawResponse {
	status: String,
	tx_hash: String,
}

pub fn routes() -> Router {
	Router::new().route("/", post(withdraw_handler))
}

async fn withdraw_handler(Json(payload): Json<WithdrawRequest>) -> Json<WithdrawResponse> {
	// TODO: Add validation, balance check, blockchain broadcast
	Json(WithdrawResponse {
		status: "pending".to_string(),
		tx_hash: "0xmocktxhash123".to_string(),
	})
}
