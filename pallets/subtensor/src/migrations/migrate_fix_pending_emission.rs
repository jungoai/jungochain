use super::*;
use alloc::string::String;
use frame_support::{traits::Get, weights::Weight};
use sp_core::crypto::Ss58Codec;
use sp_runtime::AccountId32;

fn get_account_id_from_ss58<T: Config>(ss58_str: &str) -> Result<T::AccountId, codec::Error> {
    let account =
        AccountId32::from_ss58check(ss58_str).map_err(|_| codec::Error::from("Invalid SS58"))?;
    let onchain_account = T::AccountId::decode(&mut account.as_ref())?;

    Ok(onchain_account)
}

/**
 * Migrates the pending emissions from the old hotkey to the new hotkey.
 * Also migrates the stake entry of (old_hotkey, 0x000) to the pending emissions of the new hotkey.
 */
fn migrate_pending_emissions_including_null_stake<T: Config>(
    old_hotkey: &T::AccountId,
    new_hotkey: &T::AccountId,
    migration_account: &T::AccountId,
) -> Weight {
    let mut weight = T::DbWeight::get().reads(0);
    let null_account = &DefaultAccount::<T>::get();
    weight.saturating_accrue(T::DbWeight::get().reads(1));

    // Get the pending emissions for the OLD hotkey
    let pending_emissions_old: u64 = PendingdHotkeyEmission::<T>::get(old_hotkey);
    PendingdHotkeyEmission::<T>::remove(old_hotkey);
    weight.saturating_accrue(T::DbWeight::get().reads(1));

    // Get the stake for the 0x000 key
    let null_stake = Stake::<T>::get(old_hotkey, null_account);
    weight.saturating_accrue(T::DbWeight::get().reads(1));
    // Remove
    Stake::<T>::remove(old_hotkey, null_account);
    weight.saturating_accrue(T::DbWeight::get().writes(1));

    let new_total_coldkey_stake =
        TotalColdkeyStake::<T>::get(null_account).saturating_sub(null_stake);
    if new_total_coldkey_stake == 0 {
        TotalColdkeyStake::<T>::remove(null_account);
    } else {
        TotalColdkeyStake::<T>::insert(null_account, new_total_coldkey_stake);
    }
    weight.saturating_accrue(T::DbWeight::get().reads_writes(1, 1));

    let new_staking_hotkeys = StakingHotkeys::<T>::get(null_account);
    let new_staking_hotkeys = new_staking_hotkeys
        .into_iter()
        .filter(|hk| hk != old_hotkey)
        .collect::<Vec<_>>();
    StakingHotkeys::<T>::insert(null_account, new_staking_hotkeys);
    weight.saturating_accrue(T::DbWeight::get().reads_writes(1, 1));

    // Insert the stake from the null account to the MIGRATION account under the OLD hotkey
    Stake::<T>::insert(old_hotkey, migration_account, null_stake);
    TotalColdkeyStake::<T>::insert(
        migration_account,
        TotalColdkeyStake::<T>::get(migration_account).saturating_add(null_stake),
    );
    let mut new_staking_hotkeys = StakingHotkeys::<T>::get(migration_account);
    if !new_staking_hotkeys.contains(old_hotkey) {
        new_staking_hotkeys.push(old_hotkey.clone());
    }
    StakingHotkeys::<T>::insert(migration_account, new_staking_hotkeys);
    weight.saturating_accrue(T::DbWeight::get().reads_writes(2, 3));

    // Get the pending emissions for the NEW hotkey
    let pending_emissions_new: u64 = PendingdHotkeyEmission::<T>::get(new_hotkey);
    weight.saturating_accrue(T::DbWeight::get().reads(1));

    // Add the pending emissions for the new hotkey and the old hotkey
    PendingdHotkeyEmission::<T>::insert(
        new_hotkey,
        pending_emissions_new.saturating_add(pending_emissions_old),
    );
    weight.saturating_accrue(T::DbWeight::get().writes(1));

    weight
}

pub fn do_migrate_fix_pending_emission<T: Config>() -> Weight {
    // Initialize the weight with one read operation.
    let mut weight = T::DbWeight::get().reads(1);

    let taostats_old_hotkey = "5Hddm3iBFD2GLT5ik7LZnT3XJUnRnN8PoeCFgGQgawUVKNm8";
    let taostats_new_hotkey = "5GKH9FPPnWSUoeeTJp19wVtd84XqFW4pyK2ijV2GsFbhTrP1";
    let migration_coldkey = "5D65DoFbapkYzJK17VRQo3HFs7FmMeicbaQern28UNDPypCT";

    let taostats_old_hk_account = get_account_id_from_ss58::<T>(taostats_old_hotkey);
    let taostats_new_hk_account = get_account_id_from_ss58::<T>(taostats_new_hotkey);
    let migration_ck_account = get_account_id_from_ss58::<T>(migration_coldkey);

    match (
        taostats_old_hk_account,
        taostats_new_hk_account,
        migration_ck_account.clone(),
    ) {
        (Ok(taostats_old_hk_acct), Ok(taostats_new_hk_acct), Ok(migration_ck_account)) => {
            weight.saturating_accrue(migrate_pending_emissions_including_null_stake::<T>(
                &taostats_old_hk_acct,
                &taostats_new_hk_acct,
                &migration_ck_account,
            ));
            log::info!("Migrated pending emissions from taostats old hotkey to new hotkey");
        }
        _ => {
            log::warn!("Failed to get account id from ss58 for taostats hotkeys");
            return weight;
        }
    }

    let datura_old_hotkey = "5FKstHjZkh4v3qAMSBa1oJcHCLjxYZ8SNTSz1opTv4hR7gVB";
    let datura_new_hotkey = "5GP7c3fFazW9GXK8Up3qgu2DJBk8inu4aK9TZy3RuoSWVCMi";

    let datura_old_hk_account = get_account_id_from_ss58::<T>(datura_old_hotkey);
    let datura_new_hk_account = get_account_id_from_ss58::<T>(datura_new_hotkey);

    match (
        datura_old_hk_account,
        datura_new_hk_account,
        migration_ck_account,
    ) {
        (Ok(datura_old_hk_acct), Ok(datura_new_hk_acct), Ok(migration_ck_account)) => {
            weight.saturating_accrue(migrate_pending_emissions_including_null_stake::<T>(
                &datura_old_hk_acct,
                &datura_new_hk_acct,
                &migration_ck_account,
            ));
            log::info!("Migrated pending emissions from datura old hotkey to new hotkey");
        }
        _ => {
            log::warn!("Failed to get account id from ss58 for datura hotkeys");
            return weight;
        }
    }

    weight
}
// Public migrate function to be called by Lib.rs on upgrade.
pub fn migrate_fix_pending_emission<T: Config>() -> Weight {
    let migration_name = b"fix_pending_emission".to_vec();

    // Initialize the weight with one read operation.
    let mut weight = T::DbWeight::get().reads(1);

    // Check if the migration has already run
    if HasMigrationRun::<T>::get(&migration_name) {
        log::info!(
            "Migration '{:?}' has already run. Skipping.",
            migration_name
        );
        return Weight::zero();
    }

    log::info!(
        "Running migration '{}'",
        String::from_utf8_lossy(&migration_name)
    );

    // Run the migration
    weight.saturating_accrue(do_migrate_fix_pending_emission::<T>());

    // Mark the migration as completed
    HasMigrationRun::<T>::insert(&migration_name, true);
    weight.saturating_accrue(T::DbWeight::get().writes(1));

    log::info!(
        "Migration '{:?}' completed. Marked in storage.",
        String::from_utf8_lossy(&migration_name)
    );

    // Return the migration weight.
    weight
}
