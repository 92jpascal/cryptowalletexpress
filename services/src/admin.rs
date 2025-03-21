


// Feature Addition
// Admin Retry Mechanism for Failed Withdrawals

use crate::db::{get_failed_withdrawals, retry_withdrawal_tx};

// Retry all failed withdrawals after a certain time period
// Manual retries for admins to control fraud and improve audits
pub async fn retry_failed_withdrawals() {
	let failed = get_failed_withdrawals().await;

	for tx in failed {
		match retry_withdrawal_tx(&tx).await {
			// Consider: using "?" instead of code below
			Ok(new_tx_id) => println!("Retried tx {} -> {}", tx.tx_id, new_tx_id),
			Err(e) => println!("Retry failed for {}: {}", tx.tx_id, e),
		}
	}
}

