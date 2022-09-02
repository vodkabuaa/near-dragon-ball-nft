

use near_units::{parse_gas, parse_near};
use serde_json::json;
use workspaces::prelude::*;
use workspaces::{network::Sandbox, Account, Contract, Worker, AccountDetails};

const NFT_WASM_FILEPATH: &str = "../out/main.wasm";
pub const DEFAULT_DEPOSIT: u128 = 6760000000000000000000 as u128;
pub const DEFAULT_GAS: u128 = 300000000000000 as u128;


pub async fn mint_nft(
    user: &Account,
    nft_contract: &Contract,
    worker: &Worker<Sandbox>,
    token_id: &str,
) -> anyhow::Result<()> { 
    let request_payload = json!({
        "token_id": token_id,
        "receiver_id": user.id(),
        "metadata": {
            "title": "Grumpy Cat",
            "description": "Not amused.",
            "media": "https://www.adamsdrafting.com/wp-content/uploads/2018/06/More-Grumpy-Cat.jpg"
        },
    });

    user.call(&worker, nft_contract.id(), "nft_mint")
        .args_json(request_payload)?
        .deposit(DEFAULT_DEPOSIT)
        .transact()
        .await?;
    
    Ok(())
}

pub async fn get_user_balance(
    user: &Account,
    worker: &Worker<Sandbox>,
) -> anyhow::Result<u128> {
    let details: AccountDetails = user.view_account(worker).await?;
    Ok(details.balance)
}

pub async fn get_nft_token_info(
    nft_contract: &Contract,
    worker: &Worker<Sandbox>,
    token_id: &str,
) -> anyhow::Result<serde_json::Value> {
    let token_info: serde_json::Value = nft_contract
        .call(&worker, "nft_token")
        .args_json(json!({"token_id": token_id}))?
        .transact()
        .await?
        .json()?;

    Ok(token_info)
}

pub fn round_to_near_dp(
    amount: u128,
    sf: u128,
) -> String {
    let near_amount = amount as f64 / 1_000_000_000_000_000_000_000_000.0;  // yocto in 1 NEAR
    return format!("{:.1$}", near_amount, sf as usize);
} 


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // initiate environemnt
    let worker = workspaces::sandbox().await?;

    // deploy contracts
    let nft_wasm = std::fs::read(NFT_WASM_FILEPATH)?;
    let nft_contract = worker.dev_deploy(&nft_wasm).await?;

    // create accounts
    let owner = worker.root_account().unwrap();
    let alice = owner
        .create_subaccount(&worker, "alice")
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await?
        .into_result()?;

    // Initialize contracts
    nft_contract
        .call(&worker, "new_default_meta")
        .args_json(serde_json::json!({
            "owner_id": owner.id()
        }))?
        .transact()
        .await?;

    test_nft_metadata_view(&owner, &nft_contract, &worker).await?;
    test_nft_mint_call(&owner, &alice, &nft_contract, &worker).await?;        

    Ok(())    
}


async fn test_nft_metadata_view(
    owner: &Account,
    contract: &Contract,
    worker: &Worker<Sandbox>,
) -> anyhow::Result<()> {
    let expected = json!({
        "base_uri": serde_json::Value::Null,
        "icon": serde_json::Value::Null,
        "name":  "Dragon Ball NFT Contract v1",
        "reference": serde_json::Value::Null,
        "reference_hash": serde_json::Value::Null,
        "spec": "nft-1.0.0",
        "symbol": "Dragon Ball".to_string(),
    });

    let res: serde_json::Value = owner
        .call(&worker, contract.id(), "nft_metadata")
        .args_json(json!({  "account_id": owner.id()  }))?
        .transact()
        .await?
        .json()?;
    assert_eq!(res, expected);
    println!("      Passed ✅ test_nft_metadata_view");
    Ok(())
}

async fn test_nft_mint_call(
    owner: &Account,
    user: &Account,
    contract: &Contract,
    worker: &Worker<Sandbox>,
) -> anyhow::Result<()> {
    let request_payload = json!({
        "receiver_id": user.id(),
    });

    user.call(&worker, contract.id(), "nft_mint")
        .args_json(request_payload)?
        .deposit(parse_near!("1.1 N"))
        .transact()
        .await?;

    let tokens: serde_json::Value = owner
        .call(&worker, contract.id(), "nft_tokens")
        .args_json(serde_json::json!({}))?
        .transact()
        .await?
        .json()?;
    println!("      Passed ✅ test_nft_mint_call");
    Ok(())
}