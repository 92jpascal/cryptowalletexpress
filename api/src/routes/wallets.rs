


use axum::response::IntoResponse;

use axum::{response::Json, routing::get, Router};
use serde_json::{json, Value};

pub fn routes() -> Router {
	Router::new().route("/", get(get_wallets))
}

async fn get_wallets() -> impl IntoResponse {
	Json(json!({
		"BTC": "abcdexampleaddressbtc",
		"ETC": "efgexampleaddresseth"
	}))
}
