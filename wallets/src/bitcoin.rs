

use bitcoin::util::key::PrivateKey;
use bitcoin::network::constants::Network;
use bitcoin::util::address::Address;

pub fn generate_btc_wallet() -> (String, String) {
	let priv_key = PrivateKey::new(rand::random(), Network::Testnet);
	let pub_key = priv_key.public_key(&secp256k1::Secp256k1::new());
	let address = Address::p2pkh(&pub_key, Network::Testnet);
	(address.to_string(), priv_key.to_wif())
}
