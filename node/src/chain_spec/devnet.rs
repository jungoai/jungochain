// Allowed since it's actually better to panic during chain setup when there is an error
#![allow(clippy::unwrap_used)]

use super::*;

const JUNGO: u128 = 1_000_000_000;
const EXPERT_SENATE_COLD1: &str = "5CPJGQVZr44ejcSALUJ7D96wQev4wawVJR65Di2rwXiwihPt";
const EXPERT_SENATE_COLD2: &str = "5EaEWaRyrockHfMVQofdTfZSn3qGj9rHdd1ZYxy42kuH397W";
const EXPERT_SENATE_COLD3: &str = "5G9ApVQgUJB97Z5D16EzfwewaYZ3v7sAGHuP7qbaWubpsZR7";
const EXPERT_SENATE_HOT1: &str = "5CQ1DRkQyHzEL3DW1EnSwWQnCi9CAGH5XBLtn4AEMbRWZ4T8";
const EXPERT_SENATE_HOT2: &str = "5GEjzN1P1uPH1zqR2n8ppZ96vMcaSZ6Geaz1pyL2mYPHRqbZ";
const EXPERT_SENATE_HOT3: &str = "5FUEpp4JZKWBZ4kHBn4e1VucKsQz665benM5wUtAoFN9J91p";
const FAUCET: &str = "5DcdQh3fPix14t6GdNQgv6ECD5EbSj8Z5dpu8EboKvF8AkKy";
const SUDO: &str = "5DVGrQ3aEA6rcRGFuwDJSrD1vbPt5K588KnLTsCM6sMGSFhF";

#[rustfmt::skip]
pub fn devnet_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;
    let account = |x| Ss58Codec::from_ss58check(x).unwrap();

    Ok(ChainSpec::builder(
        wasm_binary,
        Extensions {
            bad_blocks: Some(HashSet::new()),
            ..Default::default()
        },
    )
    .with_name          ("JungoAI")
    .with_protocol_id   ("jungo-ai")
    .with_id            ("jungo-ai")
    .with_chain_type    (ChainType::Development)
    // Give front-ends necessary data to present to users
    .with_properties({
        let mut properties = sc_service::Properties::new();
        properties.insert("tokenSymbol".into()  , "testJUNGO".into());
        properties.insert("tokenDecimals".into(), 9.into());
        properties.insert("ss58Format".into()   , 42.into());
        properties
    })
    .with_genesis_config_patch(devnet_genesis(
        // Sudo account
        account(SUDO),
        // Initial PoA authorities (Validators)
        // aura | grandpa
        vec![
            authority_keys_from_ss58( // node01
                "5HpYhZmtNZqrxeDhFW2x2CHYmesE99TfcKiquVgMnAKF6SrX", // aura
                "5EGiscqsnQUvMoHUKH5M73CumaCrihXYYzaNVfcUF8eDwdgA", // grandpa
            ),
            authority_keys_from_ss58( // node02
                "5FBq8Y7j9Y8hGK47j39W64be5goTfUF9oEg9iB63NwF57mNn",
                "5GyqaEaBFWjyRF4kZvrXX4b1VZjqb7YWexKmbU5cTTnVrP2w",
            ),
            authority_keys_from_ss58( // node03
                "5ELVnCbqXZ3QpaNjE71sGv3qN1s3zSLCC363f7qkHxnXu22W",
                "5HWuVHfJP1VjH8mJEK8VQhFmFmdtJqpgYW3V2Ae9WRRaUZAs",
            ),
            authority_keys_from_ss58( // node04
                "5CUauLtuzEXZYkBEPcNAwFeUjk8c98B43SMy4nQj8N5uJM1m",
                "5G5p7xKTvLc1ZPQQuYB7WbaXUcVmMysPsURp9Pny8QodoVWH",
            ),
            authority_keys_from_ss58( // node05
                "5Gq8MizVyfh2Wg7erRky5KdFYeTEuKes5o9WJfZogXTe6UPn",
                "5CxqoXZya9U5wNPuPKDbA6jYRFThAF8on5fjee734ApgBCHg",
            ),
        ],
        // balances
        vec![
            // Expert senate cold keys
            (account(EXPERT_SENATE_COLD1), 103 * JUNGO),
            (account(EXPERT_SENATE_COLD2), 103 * JUNGO),
            (account(EXPERT_SENATE_COLD3), 103 * JUNGO),
            // Faucet
            (account(FAUCET), 5_000_000 * JUNGO),
        ],
        // trimvirates
        vec![],
        // expert_senate
        vec![
            AccountId32::from_str(EXPERT_SENATE_HOT1).unwrap(),
            AccountId32::from_str(EXPERT_SENATE_HOT2).unwrap(),
            AccountId32::from_str(EXPERT_SENATE_HOT3).unwrap(),
        ],
        vec![],
        0,
    ))
    .build())
}

#[allow(clippy::too_many_arguments)]
fn devnet_genesis(
    sudo_key: AccountId,
    authorities: Vec<(AuraId, GrandpaId)>,
    balances: Vec<(AccountId, u128)>,
    trimvirates: Vec<AccountId>,
    expert_senate: Vec<AccountId>,
    _stakes: Vec<(AccountId, Vec<(AccountId, (u64, u16))>)>,
    _balances_issuance: u64,
) -> serde_json::Value {
    let (auras, grandpas): (Vec<AuraId>, Vec<GrandpaId>) = authorities.into_iter().unzip();

    serde_json::json!({
        "balances": {
            "balances": balances,
        },
        "aura": {
            "authorities": auras,
        },
        "grandpa": {
            "authorities": grandpas.into_iter().map(|x| (x, 1)).collect::<Vec<_>>()
        },
        "sudo": {
            "key": Some(sudo_key),
        },
        "triumvirateMembers": {
            "members": trimvirates
        },
        "expertSenate": {
            "members": expert_senate
        },
        "evmChainId": { "chainId": 4222 },
    })
}
