


// Feature Addition
// Simulated Cold Wallet Transfers

use crate::db::models::{Transaction, Wallet};

// Cold wallet threshold (e.g., auto-sweep if > 1.0 BTC or ETH)
const COLD_THRESHOLD: f64 = 1.0;

// Funtion to check if cold wallet sweep is needed
// For testing/staging environments, simulating transfer
// behavior to avoid exposing keys
pub fn needs_cold_storage_transfer(wallet: &Wallet) -> bool {
	wallet.balance > COLD_THRESHOLD
}

// Function to move funds to a "cold wallet" entry
pub fn simulate_cold_transfer(wallet: &mut Wallet) -> Transaction {
	let swept_amount = wallet.balance - 0.1;
	wallet.balance = 0.1;

	Transaction {
		tx_id: "cold_tx_mock".into(),
		type_: "cold_transfer".into(),
		currency: wallet.currency.clone(),
		amount: swept_amount,
		status: "simulated".into(),
		address: "cold_wallet_sim".into(),
	}
}
