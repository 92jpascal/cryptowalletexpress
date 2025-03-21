


use ethers::signers::LocalWallet;
use rand::thread_rng;

pub fn generate_eth_wallet() -> (String, String) {
	let wallet = LocalWallet::new(&mut thread_rng());
	(wallet.address().to_string(), wallet.signer().to_string())
}
