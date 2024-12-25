// Allowed since it's actually better to panic during chain setup when there is an error
#![allow(clippy::unwrap_used)]

use super::*;

#[rustfmt::skip]
pub fn localnet_config(single_authority: bool) -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;
    let account = |x| Ss58Codec::from_ss58check(x).unwrap();

    Ok(ChainSpec::builder(
        wasm_binary,
        Extensions {
            bad_blocks: Some(HashSet::new()),
            ..Default::default()
        },
    )
    .with_name          ("JangoAI")
    .with_protocol_id   ("jango-ai")
    .with_id            ("jango-ai")
    .with_chain_type    (ChainType::Development)
    // Give front-ends necessary data to present to users
    .with_properties({
        let mut properties = sc_service::Properties::new();
        properties.insert("tokenSymbol".into()  , "testJango".into());
        properties.insert("tokenDecimals".into(), 9.into());
        properties.insert("ss58Format".into()   , 42.into());
        properties
    })
    .with_genesis_config_patch(devnet_genesis(
        // Sudo account
        sr25519_account("Alice"),
        // Initial PoA authorities (Validators)
        // aura | grandpa
        if single_authority {
            // single authority allows you to run the network using a single node
            vec![authority_keys_from_seed("Alice")]
        } else {
            vec![
                authority_keys_from_seed("Alice"),
                authority_keys_from_seed("Bob"),
            ]
        },
        // balances
        vec![
            (sr25519_account("Alice")   , 1000_000_000_000_000u128),
            (sr25519_account("Bob")     , 1000_000_000_000_000u128),
            (sr25519_account("Charlie") , 1000_000_000_000_000u128),
            (sr25519_account("Dave")    , 2000_000_000_000u128),
            (sr25519_account("Ferdie")  , 2000_000_000_000u128),
            (sr25519_account("Eve")     , 2000_000_000_000u128),

            // Mohsen's local owner coldkey
            (account("5CwP1MPnA3vHmqmvzPampZmAg1m7FCaSJ3PnPpCy4wmruAFq"), 5000_000_000_000u128),
            // Mohsen's local miner coldkey
            (account("5GbcFmvaUTmfpL6MFfh9Xz7X6bj5cKBXqubgNHhKFiNmwFhy"), 5000_000_000_000u128),
            // Mohsen's local validator coldkey
            (account("5HTho5p3HQyuvni13e6qhc5Q8UpQZo6RNfgoxAmo3gcfRrJ8"), 5000_000_000_000u128),
        ],
        // trimvirates
        vec![
            sr25519_account("Alice"),
            sr25519_account("Bob"),
            sr25519_account("Charlie"),
        ],
        // expert_senate
        vec![
            sr25519_account("Dave"),
            sr25519_account("Ferdie"),
            sr25519_account("Eve"),
            AccountId32::from_str("5EHmyyPc69VEfVnaMABtHhxdFgkqLakTHxAVGt1eQ1iQ2MEo").unwrap(),
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
        }
    })
}
