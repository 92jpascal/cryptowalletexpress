


// Feature Addition
// ERC-20 (Ethereum) Token Balance Support

use ethers::providers::{Provider, Http};
use ethers::contract::Contract;
use ethers::types::{Address, U256};
use std::convert::TryFrom;
use std::str::FromStr;

// Standard ERC-20 ABI snippet
const ERC20_ABI: &str = r#"[{"constant":true,"inputs":[{"name":"_owner","type":"address"}],"name":"balanceOf","outputs"[{"name":"","type":"uint256"}],"type":"function"}]"#;

// Check on ERC-20 token balance
// Using provider and contract to make function
// reusable with ERC-20 contract address
pub async fn get_erc20_balance(
	wallet_address: &str,
	token_address: &str,
	rpc_url: &str,
) -> Result<U256, Box<dyn std::error::Error>> {
	let provider = Provider::<Http>::try_from(rpc_url)?;
	let contract_address = Address::from_str(token_address)?;
	let wallet_addr = Address::from_str(wallet_address)?;

	let contract = Contract::from_json(contract_address, ERC20_ABI.as_bytes(), provider)?;
	let balance: U256 = contract
		.method::<_, U256>("balanceOf", wallet_addr)?
		.call()
		.await?;

	Ok(balance)
}
