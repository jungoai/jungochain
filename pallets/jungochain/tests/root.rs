#![allow(clippy::indexing_slicing, clippy::unwrap_used)]

use crate::mock::*;
use frame_support::{assert_err, assert_ok};
use frame_system::Config;
use frame_system::{EventRecord, Phase};
use pallet_jungochain::{migrations, SubnetIdentity};
use pallet_jungochain::{Error, FirstReservedNetuids};
use pallet_jungochain::{SubnetIdentities, SubnetIdentityOf};
use sp_core::{Get, H256, U256};

mod mock;

#[allow(dead_code)]
fn record(event: RuntimeEvent) -> EventRecord<RuntimeEvent, H256> {
    EventRecord {
        phase: Phase::Initialization,
        event,
        topics: vec![],
    }
}

#[test]
fn test_root_register_network_exist() {
    new_test_ext(1).execute_with(|| {
        migrations::migrate_create_root_network::migrate_create_root_network::<Test>();
        let hotkey_account_id: U256 = U256::from(1);
        let coldkey_account_id = U256::from(667);
        assert_ok!(JungochainModule::root_register(
            <<Test as Config>::RuntimeOrigin>::signed(coldkey_account_id),
            hotkey_account_id,
        ));
    });
}

// SKIP_WASM_BUILD=1 RUST_LOG=info cargo test --test root -- test_root_register_normal_on_root_fails --exact --nocapture
#[test]
fn test_root_register_normal_on_root_fails() {
    new_test_ext(1).execute_with(|| {
        migrations::migrate_create_root_network::migrate_create_root_network::<Test>();
        // Test fails because normal registrations are not allowed
        // on the root network.
        let root_netuid: u16 = 0;
        let hotkey_account_id: U256 = U256::from(1);
        let coldkey_account_id = U256::from(667);

        // Burn registration fails.
        JungochainModule::set_burn(root_netuid, 0);
        JungochainModule::add_balance_to_coldkey_account(&coldkey_account_id, 1);
        assert_eq!(
            JungochainModule::burned_register(
                <<Test as Config>::RuntimeOrigin>::signed(coldkey_account_id),
                root_netuid,
                hotkey_account_id
            ),
            Err(Error::<Test>::RegistrationNotPermittedOnRootSubnet.into())
        );
        // Pow registration fails.
        let block_number: u64 = JungochainModule::get_current_block_as_u64();
        let (nonce, work): (u64, Vec<u8>) = JungochainModule::create_work_for_block_number(
            root_netuid,
            block_number,
            0,
            &hotkey_account_id,
        );
        assert_eq!(
            JungochainModule::register(
                <<Test as frame_system::Config>::RuntimeOrigin>::signed(hotkey_account_id),
                root_netuid,
                block_number,
                nonce,
                work,
                hotkey_account_id,
                coldkey_account_id,
            ),
            Err(Error::<Test>::RegistrationNotPermittedOnRootSubnet.into())
        );
    });
}

// SKIP_WASM_BUILD=1 RUST_LOG=info cargo test --test root -- test_root_register_stake_based_pruning_works --exact --nocapture
#[test]
fn test_root_register_stake_based_pruning_works() {
    new_test_ext(1).execute_with(|| {
        migrations::migrate_create_root_network::migrate_create_root_network::<Test>();
        // Add two networks.
        let root_netuid: u16 = 0;
        let other_netuid: u16 = 1;
        add_network(other_netuid, 0, 0);

        // Set params to allow all registrations to subnet.
        JungochainModule::set_burn(other_netuid, 0);
        JungochainModule::set_max_registrations_per_block(other_netuid, 256);
        JungochainModule::set_target_registrations_per_interval(other_netuid, 256);

        JungochainModule::set_max_registrations_per_block(root_netuid, 1000);
        JungochainModule::set_target_registrations_per_interval(root_netuid, 1000);

        // Register 128 accounts with stake to the other network.
        for i in 0..128 {
            let hot: U256 = U256::from(i);
            let cold: U256 = U256::from(i);
            // Add balance
            JungochainModule::add_balance_to_coldkey_account(&cold, 1000 + (i as u64));
            // Register
            assert_ok!(JungochainModule::burned_register(
                <<Test as Config>::RuntimeOrigin>::signed(cold),
                other_netuid,
                hot
            ));
            // Add stake on other network
            assert_ok!(JungochainModule::add_stake(
                <<Test as Config>::RuntimeOrigin>::signed(cold),
                hot,
                1000 + (i as u64)
            ));
            // Check successful registration.
            assert!(JungochainModule::get_uid_for_net_and_hotkey(other_netuid, &hot).is_ok());
            // Check that they are NOT all delegates
            assert!(!JungochainModule::hotkey_is_delegate(&hot));
        }

        // Register the first 64 accounts with stake to the root network.
        for i in 0..64 {
            let hot: U256 = U256::from(i);
            let cold: U256 = U256::from(i);
            assert_ok!(JungochainModule::root_register(
                <<Test as Config>::RuntimeOrigin>::signed(cold),
                hot,
            ));
            // Check successful registration.
            assert!(JungochainModule::get_uid_for_net_and_hotkey(root_netuid, &hot).is_ok());
            // Check that they are all delegates
            assert!(JungochainModule::hotkey_is_delegate(&hot));
        }

        // Register the second 64 accounts with stake to the root network.
        // Replaces the first 64
        for i in 64..128 {
            let hot: U256 = U256::from(i);
            let cold: U256 = U256::from(i);
            assert_ok!(JungochainModule::root_register(
                <<Test as Config>::RuntimeOrigin>::signed(cold),
                hot,
            ));
            // Check successful registration.
            assert!(JungochainModule::get_uid_for_net_and_hotkey(root_netuid, &hot).is_ok());
        }

        // Register the first 64 accounts again, this time failing because they
        // don't have enough stake.
        for i in 0..64 {
            let hot: U256 = U256::from(i);
            let cold: U256 = U256::from(i);
            assert_eq!(
                JungochainModule::root_register(
                    <<Test as Config>::RuntimeOrigin>::signed(cold),
                    hot,
                ),
                Err(Error::<Test>::StakeTooLowForRoot.into())
            );
            // Check for unsuccessful registration.
            assert!(JungochainModule::get_uid_for_net_and_hotkey(root_netuid, &hot).is_err());
            // Check that they are NOT senate members
            assert!(!JungochainModule::is_senate_member(&hot));
        }
    });
}

// SKIP_WASM_BUILD=1 RUST_LOG=info cargo test --test root -- test_root_set_weights --exact --nocapture
#[test]
fn test_root_set_weights() {
    new_test_ext(1).execute_with(|| {
        System::set_block_number(0);
        migrations::migrate_create_root_network::migrate_create_root_network::<Test>();

        let n: usize = 10;
        let root_netuid: u16 = 0;
        JungochainModule::set_max_registrations_per_block(root_netuid, n as u16);
        JungochainModule::set_target_registrations_per_interval(root_netuid, n as u16);
        JungochainModule::set_max_allowed_uids(root_netuid, n as u16);
        for i in 0..n {
            let hotkey_account_id: U256 = U256::from(i);
            let coldkey_account_id: U256 = U256::from(i + 456);
            JungochainModule::add_balance_to_coldkey_account(
                &coldkey_account_id,
                1_000_000_000_000_000,
            );
            assert_ok!(JungochainModule::root_register(
                <<Test as Config>::RuntimeOrigin>::signed(coldkey_account_id),
                hotkey_account_id,
            ));
            assert_ok!(JungochainModule::add_stake(
                <<Test as Config>::RuntimeOrigin>::signed(coldkey_account_id),
                hotkey_account_id,
                1000
            ));
        }

        log::info!("subnet limit: {:?}", JungochainModule::get_max_subnets());
        log::info!(
            "current subnet count: {:?}",
            JungochainModule::get_num_subnets()
        );

        // Lets create n networks
        for i in 1..n {
            log::debug!("Adding network with netuid: {}", unid(i as u16));
            assert_ok!(JungochainModule::register_network(
                <<Test as Config>::RuntimeOrigin>::signed(U256::from(i + 456)),
            ));
        }

        // Test that signing with hotkey will fail.
        for i in 0..n {
            let hotkey = U256::from(i);
            let uids: Vec<u16> = vec![unid_or_zero(i as u16)];
            let values: Vec<u16> = vec![1];
            assert_err!(
                JungochainModule::set_root_weights(
                    <<Test as Config>::RuntimeOrigin>::signed(hotkey),
                    hotkey,
                    uids,
                    values,
                    0,
                ),
                Error::<Test>::NonAssociatedColdKey
            );
        }

        // Test that signing an unassociated coldkey will fail.
        let unassociated_coldkey = U256::from(612);
        for i in 0..n {
            let hotkey = U256::from(i);
            let uids: Vec<u16> = vec![unid_or_zero(i as u16)];
            let values: Vec<u16> = vec![1];
            assert_err!(
                JungochainModule::set_root_weights(
                    <<Test as Config>::RuntimeOrigin>::signed(unassociated_coldkey),
                    hotkey,
                    uids,
                    values,
                    0,
                ),
                Error::<Test>::NonAssociatedColdKey
            );
        }

        // Set weights into diagonal matrix.
        for i in 0..n {
            let hotkey = U256::from(i);
            let coldkey = U256::from(i + 456);
            let uids: Vec<u16> = vec![unid_or_zero(i as u16)];
            let values: Vec<u16> = vec![1];
            assert_ok!(JungochainModule::set_root_weights(
                <<Test as Config>::RuntimeOrigin>::signed(coldkey),
                hotkey,
                uids,
                values,
                0,
            ));
        }
        // Run the root epoch
        log::debug!("Running Root epoch");
        JungochainModule::set_tempo(root_netuid, 1);
        assert_ok!(JungochainModule::root_epoch(1_000_000_000));
        // Check that the emission values have been set.
        for i in 1..n {
            log::debug!("check emission for netuid: {}", unid(i as u16));
            assert_eq!(
                JungochainModule::get_subnet_emission_value(unid(i as u16)),
                99_999_999
            );
        }
        step_block(2);
        // Check that the pending emission values have been set.
        for i in 1..n {
            log::debug!(
                "check pending emission for netuid {} has pending {}",
                unid(i as u16),
                JungochainModule::get_pending_emission(unid(i as u16))
            );
            assert_eq!(
                JungochainModule::get_pending_emission(unid(i as u16)),
                199_999_998
            );
        }
        step_block(1);
        for i in 1..n {
            log::debug!(
                "check pending emission for netuid {} has pending {}",
                unid(i as u16),
                JungochainModule::get_pending_emission(unid(i as u16))
            );
            assert_eq!(
                JungochainModule::get_pending_emission(unid(i as u16)),
                299_999_997
            );
        }
        let step = JungochainModule::blocks_until_next_epoch(
            10,
            1000,
            JungochainModule::get_current_block_as_u64(),
        );
        step_block(step as u16);
        assert_eq!(JungochainModule::get_pending_emission(10), 0);
    });
}

// SKIP_WASM_BUILD=1 RUST_LOG=info cargo test --test root -- test_root_set_weights --exact --nocapture
#[test]
fn test_root_set_weights_out_of_order_netuids() {
    new_test_ext(1).execute_with(|| {
        System::set_block_number(0);
        migrations::migrate_create_root_network::migrate_create_root_network::<Test>();

        let n: usize = 10;
        let root_netuid: u16 = 0;
        JungochainModule::set_max_registrations_per_block(root_netuid, n as u16);
        JungochainModule::set_target_registrations_per_interval(root_netuid, n as u16);
        JungochainModule::set_max_allowed_uids(root_netuid, n as u16);
        for i in 0..n {
            let hotkey_account_id: U256 = U256::from(i);
            let coldkey_account_id: U256 = U256::from(i);
            JungochainModule::add_balance_to_coldkey_account(
                &coldkey_account_id,
                1_000_000_000_000_000,
            );
            assert_ok!(JungochainModule::root_register(
                <<Test as Config>::RuntimeOrigin>::signed(coldkey_account_id),
                hotkey_account_id,
            ));
            assert_ok!(JungochainModule::add_stake(
                <<Test as Config>::RuntimeOrigin>::signed(coldkey_account_id),
                hotkey_account_id,
                1000
            ));
        }

        log::info!("subnet limit: {:?}", JungochainModule::get_max_subnets());
        log::info!(
            "current subnet count: {:?}",
            JungochainModule::get_num_subnets()
        );

        // Lets create n networks
        for i in 1..n {
            log::debug!("Adding network with netuid: {}", unid(i as u16));

            if unid(i as u16) % 2 == 0 {
                assert_ok!(JungochainModule::register_network(
                    <<Test as Config>::RuntimeOrigin>::signed(U256::from(i)),
                ));
            } else {
                add_network(unid(i as u16 * 10), 1000, 0)
            }
        }

        log::info!("netuids: {:?}", JungochainModule::get_all_subnet_netuids());
        log::info!(
            "root network count: {:?}",
            JungochainModule::get_subnetwork_n(0)
        );

        let subnets = JungochainModule::get_all_subnet_netuids();
        // Set weights into diagonal matrix.
        for (i, netuid) in subnets.iter().enumerate() {
            let uids: Vec<u16> = vec![*netuid];
            let values: Vec<u16> = vec![1];

            let coldkey = U256::from(i);
            let hotkey = U256::from(i);
            assert_ok!(JungochainModule::set_root_weights(
                <<Test as Config>::RuntimeOrigin>::signed(coldkey),
                hotkey,
                uids,
                values,
                0,
            ));
        }
        // Run the root epoch
        log::debug!("Running Root epoch");
        JungochainModule::set_tempo(root_netuid, 1);
        assert_ok!(JungochainModule::root_epoch(1_000_000_000));
        // Check that the emission values have been set.
        for netuid in subnets.iter() {
            log::debug!("check emission for netuid: {}", netuid);
            assert_eq!(
                JungochainModule::get_subnet_emission_value(*netuid),
                99_999_999
            );
        }
        step_block(2);
        // Check that the pending emission values have been set.
        for netuid in subnets.iter() {
            if *netuid == 0 {
                continue;
            }

            log::debug!(
                "check pending emission for netuid {} has pending {}",
                netuid,
                JungochainModule::get_pending_emission(*netuid)
            );
            assert_eq!(JungochainModule::get_pending_emission(*netuid), 199_999_998);
        }
        step_block(1);
        for netuid in subnets.iter() {
            if *netuid == 0 {
                continue;
            }

            log::debug!(
                "check pending emission for netuid {} has pending {}",
                netuid,
                JungochainModule::get_pending_emission(*netuid)
            );
            assert_eq!(JungochainModule::get_pending_emission(*netuid), 299_999_997);
        }
        let step = JungochainModule::blocks_until_next_epoch(
            9,
            1000,
            JungochainModule::get_current_block_as_u64(),
        );
        step_block(step as u16);
        assert_eq!(JungochainModule::get_pending_emission(9), 0);
    });
}

#[test]
fn test_root_subnet_creation_deletion() {
    new_test_ext(1).execute_with(|| {
        System::set_block_number(0);
        migrations::migrate_create_root_network::migrate_create_root_network::<Test>();
        // Owner of subnets.
        let owner: U256 = U256::from(0);

        // Add a subnet.
        JungochainModule::add_balance_to_coldkey_account(&owner, 1_000_000_000_000_000);
        // last_lock: 100000000000, min_lock: 100000000000, last_lock_block: 0, lock_reduction_interval: 2, current_block: 0, mult: 1 lock_cost: 100000000000
        assert_ok!(JungochainModule::register_network(
            <<Test as Config>::RuntimeOrigin>::signed(owner),
        ));
        // last_lock: 100000000000, min_lock: 100000000000, last_lock_block: 0, lock_reduction_interval: 2, current_block: 0, mult: 1 lock_cost: 100000000000
        assert_eq!(JungochainModule::get_network_lock_cost(), 100_000_000_000);
        step_block(1);
        // last_lock: 100000000000, min_lock: 100000000000, last_lock_block: 0, lock_reduction_interval: 2, current_block: 1, mult: 1 lock_cost: 100000000000
        assert_ok!(JungochainModule::register_network(
            <<Test as Config>::RuntimeOrigin>::signed(owner),
        ));
        // last_lock: 100000000000, min_lock: 100000000000, last_lock_block: 1, lock_reduction_interval: 2, current_block: 1, mult: 2 lock_cost: 200000000000
        assert_eq!(JungochainModule::get_network_lock_cost(), 200_000_000_000); // Doubles from previous subnet creation
        step_block(1);
        // last_lock: 100000000000, min_lock: 100000000000, last_lock_block: 1, lock_reduction_interval: 2, current_block: 2, mult: 2 lock_cost: 150000000000
        assert_eq!(JungochainModule::get_network_lock_cost(), 150_000_000_000); // Reduced by 50%
        step_block(1);
        // last_lock: 100000000000, min_lock: 100000000000, last_lock_block: 1, lock_reduction_interval: 2, current_block: 3, mult: 2 lock_cost: 100000000000
        assert_eq!(JungochainModule::get_network_lock_cost(), 100_000_000_000); // Reduced another 50%
        step_block(1);
        // last_lock: 100000000000, min_lock: 100000000000, last_lock_block: 1, lock_reduction_interval: 2, current_block: 4, mult: 2 lock_cost: 100000000000
        assert_eq!(JungochainModule::get_network_lock_cost(), 100_000_000_000); // Reaches min value
        assert_ok!(JungochainModule::register_network(
            <<Test as Config>::RuntimeOrigin>::signed(owner),
        ));
        // last_lock: 100000000000, min_lock: 100000000000, last_lock_block: 4, lock_reduction_interval: 2, current_block: 4, mult: 2 lock_cost: 200000000000
        assert_eq!(JungochainModule::get_network_lock_cost(), 200_000_000_000); // Doubles from previous subnet creation
        step_block(1);
        // last_lock: 100000000000, min_lock: 100000000000, last_lock_block: 4, lock_reduction_interval: 2, current_block: 5, mult: 2 lock_cost: 150000000000
        assert_ok!(JungochainModule::register_network(
            <<Test as Config>::RuntimeOrigin>::signed(owner),
        ));
        // last_lock: 150000000000, min_lock: 100000000000, last_lock_block: 5, lock_reduction_interval: 2, current_block: 5, mult: 2 lock_cost: 300000000000
        assert_eq!(JungochainModule::get_network_lock_cost(), 300_000_000_000); // Doubles from previous subnet creation
        step_block(1);
        // last_lock: 150000000000, min_lock: 100000000000, last_lock_block: 5, lock_reduction_interval: 2, current_block: 6, mult: 2 lock_cost: 225000000000
        assert_ok!(JungochainModule::register_network(
            <<Test as Config>::RuntimeOrigin>::signed(owner),
        ));
        // last_lock: 225000000000, min_lock: 100000000000, last_lock_block: 6, lock_reduction_interval: 2, current_block: 6, mult: 2 lock_cost: 450000000000
        assert_eq!(JungochainModule::get_network_lock_cost(), 450_000_000_000); // Increasing
        step_block(1);
        // last_lock: 225000000000, min_lock: 100000000000, last_lock_block: 6, lock_reduction_interval: 2, current_block: 7, mult: 2 lock_cost: 337500000000
        assert_ok!(JungochainModule::register_network(
            <<Test as Config>::RuntimeOrigin>::signed(owner),
        ));
        // last_lock: 337500000000, min_lock: 100000000000, last_lock_block: 7, lock_reduction_interval: 2, current_block: 7, mult: 2 lock_cost: 675000000000
        assert_eq!(JungochainModule::get_network_lock_cost(), 675_000_000_000); // Increasing.
        assert_ok!(JungochainModule::register_network(
            <<Test as Config>::RuntimeOrigin>::signed(owner),
        ));
        // last_lock: 337500000000, min_lock: 100000000000, last_lock_block: 7, lock_reduction_interval: 2, current_block: 7, mult: 2 lock_cost: 675000000000
        assert_eq!(JungochainModule::get_network_lock_cost(), 1_350_000_000_000); // Double increasing.
        assert_ok!(JungochainModule::register_network(
            <<Test as Config>::RuntimeOrigin>::signed(owner),
        ));
        assert_eq!(JungochainModule::get_network_lock_cost(), 2_700_000_000_000); // Double increasing again.

        // Now drop it like its hot to min again.
        step_block(1);
        assert_eq!(JungochainModule::get_network_lock_cost(), 2_025_000_000_000); // 675_000_000_000 decreasing.
        step_block(1);
        assert_eq!(JungochainModule::get_network_lock_cost(), 1_350_000_000_000); // 675_000_000_000 decreasing.
        step_block(1);
        assert_eq!(JungochainModule::get_network_lock_cost(), 675_000_000_000); // 675_000_000_000 decreasing.
        step_block(1);
        assert_eq!(JungochainModule::get_network_lock_cost(), 100_000_000_000); // 675_000_000_000 decreasing with 100000000000 min
    });
}

#[test]
fn test_network_pruning() {
    new_test_ext(1).execute_with(|| {
        System::set_block_number(0);
        migrations::migrate_create_root_network::migrate_create_root_network::<Test>();

        assert_eq!(JungochainModule::get_total_issuance(), 0);

        let n: usize = 10;
        let root_netuid: u16 = 0;
        JungochainModule::set_max_registrations_per_block(root_netuid, n as u16);
        JungochainModule::set_target_registrations_per_interval(root_netuid, n as u16);
        JungochainModule::set_max_allowed_uids(root_netuid, n as u16 + 1);
        JungochainModule::set_tempo(root_netuid, 1);
        // No validators yet.
        assert_eq!(JungochainModule::get_subnetwork_n(root_netuid), 0);

        for i in 0..n {
            let hot: U256 = U256::from(i);
            let cold: U256 = U256::from(i);
            let uids: Vec<u16> = (0..i as u16).map(unid_or_zero).collect();
            let values: Vec<u16> = vec![1; i];
            JungochainModule::add_balance_to_coldkey_account(&cold, 1_000_000_000_000_000);
            assert_ok!(JungochainModule::root_register(
                <<Test as Config>::RuntimeOrigin>::signed(cold),
                hot
            ));
            assert_ok!(JungochainModule::add_stake(
                <<Test as Config>::RuntimeOrigin>::signed(cold),
                hot,
                1_000
            ));
            assert_ok!(JungochainModule::register_network(
                <<Test as Config>::RuntimeOrigin>::signed(cold),
            ));
            log::debug!("Adding network with netuid: {}", unid(i as u16 + 1));
            assert!(JungochainModule::if_subnet_exist(unid(i as u16 + 1)));
            assert!(JungochainModule::is_hotkey_registered_on_network(
                root_netuid,
                &hot
            ));
            assert!(JungochainModule::get_uid_for_net_and_hotkey(root_netuid, &hot).is_ok());
            assert_ok!(JungochainModule::set_root_weights(
                <<Test as Config>::RuntimeOrigin>::signed(cold),
                hot,
                uids,
                values,
                0
            ));
            JungochainModule::set_tempo(unid(i as u16 + 1), 1);
            JungochainModule::set_burn(unid(i as u16 + 1), 0);
            assert_ok!(JungochainModule::burned_register(
                <<Test as Config>::RuntimeOrigin>::signed(cold),
                unid(i as u16 + 1),
                hot
            ));
            assert_eq!(
                JungochainModule::get_subnetwork_n(root_netuid),
                i as u16 + 1
            );
        }
        // Stakes
        // 0 : 10_000
        // 1 : 9_000
        // 2 : 8_000
        // 3 : 7_000
        // 4 : 6_000
        // 5 : 5_000
        // 6 : 4_000
        // 7 : 3_000
        // 8 : 2_000
        // 9 : 1_000

        step_block(1);
        assert_ok!(JungochainModule::root_epoch(1_000_000_000));
        assert_eq!(JungochainModule::get_subnet_emission_value(0), 385_861_815);
        assert_eq!(
            JungochainModule::get_subnet_emission_value(unid(1)),
            249_435_914
        );
        assert_eq!(
            JungochainModule::get_subnet_emission_value(unid(2)),
            180_819_837
        );
        assert_eq!(
            JungochainModule::get_subnet_emission_value(unid(3)),
            129_362_980
        );
        assert_eq!(
            JungochainModule::get_subnet_emission_value(unid(4)),
            50_857_187
        );
        assert_eq!(
            JungochainModule::get_subnet_emission_value(unid(5)),
            3_530_356
        );
        step_block(1);
        assert_eq!(JungochainModule::get_pending_emission(0), 0); // root network gets no pending emission.
        assert_eq!(JungochainModule::get_pending_emission(unid(1)), 249_435_914);
        assert_eq!(JungochainModule::get_pending_emission(unid(2)), 0); // This has been drained.
        assert_eq!(JungochainModule::get_pending_emission(unid(3)), 129_362_980);
        assert_eq!(JungochainModule::get_pending_emission(unid(4)), 0); // This network has been drained.
        assert_eq!(JungochainModule::get_pending_emission(unid(5)), 3_530_356);
        step_block(1);
    });
}

#[rustfmt::skip]
#[test]
fn test_network_prune_results() {
    new_test_ext(1).execute_with(|| {
        migrations::migrate_create_root_network::migrate_create_root_network::<Test>();

        JungochainModule::set_network_immunity_period(3);
        JungochainModule::set_network_min_lock(0);
        JungochainModule::set_network_rate_limit(0);

        let owner: U256 = U256::from(0);
        JungochainModule::add_balance_to_coldkey_account(&owner, 1_000_000_000_000_000);

        // --------------------------------------------------------------------
        // -- User subnet

        assert_ok!(JungochainModule::register_network(
            <<Test as Config>::RuntimeOrigin>::signed(owner),
        ));
        step_block(3);

        assert_ok!(JungochainModule::register_network(
            <<Test as Config>::RuntimeOrigin>::signed(owner),
        ));
        step_block(3);

        assert_ok!(JungochainModule::register_network(
            <<Test as Config>::RuntimeOrigin>::signed(owner),
        ));
        step_block(3);

        // lowest emission
        JungochainModule::set_emission_values(
            &[unid(1u16), unid(2u16), unid(3u16)],
            vec![  5u64,       4u64,       4u64],
        )
        .unwrap();
        assert_eq!(JungochainModule::get_user_subnet_to_prune(), unid(2u16));

        // equal emission, creation date
        JungochainModule::set_emission_values(
            &[unid(1u16), unid(2u16), unid(3u16)],
            vec![  5u64,       5u64,       4u64]
        )
        .unwrap();
        assert_eq!(JungochainModule::get_user_subnet_to_prune(), unid(3u16));

        // equal emission, creation date
        JungochainModule::set_emission_values(
            &[unid(1u16), unid(2u16), unid(3u16)],
            vec![  4u64,       5u64,       5u64]
        )
        .unwrap();
        assert_eq!(JungochainModule::get_user_subnet_to_prune(), unid(1u16));

        // --------------------------------------------------------------------
        // -- Reserved subnet

        assert_ok!(JungochainModule::register_reserved_subnet(
            <<Test as Config>::RuntimeOrigin>::signed(owner),
        ));
        step_block(3);

        assert_ok!(JungochainModule::register_reserved_subnet(
            <<Test as Config>::RuntimeOrigin>::signed(owner),
        ));
        step_block(3);

        assert_ok!(JungochainModule::register_reserved_subnet(
            <<Test as Config>::RuntimeOrigin>::signed(owner),
        ));
        step_block(3);

        // lowest emission
        JungochainModule::set_emission_values(
               &[1u16, 2u16, 3u16],
            vec![5u64, 4u64, 4u64]
        )
        .unwrap();
        assert_eq!(JungochainModule::get_reserved_subnet_to_prune(), 2u16);

        // equal emission, creation date
        JungochainModule::set_emission_values(
               &[1u16, 2u16, 3u16],
            vec![5u64, 5u64, 4u64]
        )
        .unwrap();
        assert_eq!(JungochainModule::get_reserved_subnet_to_prune(), 3u16);

        // equal emission, creation date
        JungochainModule::set_emission_values(
               &[1u16, 2u16, 3u16],
            vec![4u64, 5u64, 5u64]
        )
        .unwrap();
        assert_eq!(JungochainModule::get_reserved_subnet_to_prune(), 1u16);
    });
}

#[test]
fn test_weights_after_network_pruning() {
    new_test_ext(1).execute_with(|| {
        migrations::migrate_create_root_network::migrate_create_root_network::<Test>();

        assert_eq!(JungochainModule::get_total_issuance(), 0);

        // Set up N subnets, with max N + 1 allowed UIDs
        let n: usize = 2;
        let root_netuid: u16 = 0;
        JungochainModule::set_network_immunity_period(3);
        JungochainModule::set_max_registrations_per_block(root_netuid, n as u16);
        JungochainModule::set_max_subnets(n as u16);
        JungochainModule::set_weights_set_rate_limit(root_netuid, 0_u64);

        // No validators yet.
        assert_eq!(JungochainModule::get_subnetwork_n(root_netuid), 0);

        for i in 0..n {
            // Register a validator
            let cold: U256 = U256::from(i);

            JungochainModule::add_balance_to_coldkey_account(&cold, 1_000_000_000_000);

            // Register a network
            assert_ok!(JungochainModule::register_network(
                <<Test as Config>::RuntimeOrigin>::signed(cold),
            ));

            log::debug!("Adding network with netuid: {}", unid(i as u16 + 1));
            assert!(JungochainModule::if_subnet_exist(unid(i as u16 + 1)));
            step_block(3);
        }

        // Register a validator in subnet 0
        let hot: U256 = U256::from((n as u64) - 1);
        let cold: U256 = U256::from((n as u64) - 1);

        assert_ok!(JungochainModule::root_register(
            <<Test as Config>::RuntimeOrigin>::signed(cold),
            hot
        ));
        assert_ok!(JungochainModule::add_stake(
            <<Test as Config>::RuntimeOrigin>::signed(cold),
            hot,
            1_000
        ));

        // Let's give these subnets some weights
        let uids: Vec<u16> = (0..(n as u16) + 1).map(unid_or_zero).collect();
        let values: Vec<u16> = vec![4u16, 2u16, 6u16];
        log::info!("uids set: {:?}", uids);
        log::info!("values set: {:?}", values);
        log::info!("In netuid: {:?}", root_netuid);
        assert_ok!(JungochainModule::set_root_weights(
            <<Test as Config>::RuntimeOrigin>::signed(cold),
            hot,
            uids,
            values,
            0
        ));

        log::info!(
            "Root network weights before extra network registration: {:?}",
            JungochainModule::get_root_weights()
        );
        log::info!("Max subnets: {:?}", JungochainModule::get_max_subnets());
        let i = (n as u16) + 1;
        // let _hot: U256 = U256::from(i);
        let cold: U256 = U256::from(i);

        JungochainModule::add_balance_to_coldkey_account(&cold, 1_000_000_000_000_000_000);
        let subnet_to_prune = JungochainModule::get_user_subnet_to_prune();

        // User subnet 1 should be pruned here.
        assert_eq!(subnet_to_prune, unid(1));
        log::info!("Removing subnet: {:?}", subnet_to_prune);

        // Check that the weights have been set appropriately.
        let latest_weights = JungochainModule::get_root_weights();
        log::info!("Weights before register network: {:?}", latest_weights);
        // We expect subnet 1 to be deregistered as it is oldest and has lowest emissions
        assert_eq!(latest_weights[0][1], 21845);

        assert_ok!(JungochainModule::register_network(
            <<Test as Config>::RuntimeOrigin>::signed(cold),
        ));

        // Subnet should not exist, as it would replace a previous subnet.
        assert!(!JungochainModule::if_subnet_exist(unid(i + 1)));

        log::info!(
            "Root network weights: {:?}",
            JungochainModule::get_root_weights()
        );

        let latest_weights = JungochainModule::get_root_weights();
        log::info!(
            "Weights after register network: {:?}",
            JungochainModule::get_root_weights()
        );

        // Subnet 0 should be kicked, and thus its weight should be 0
        assert_eq!(latest_weights[0][1], 0);
    });
}

/// This test checks the halving mechanism of the emission schedule.
/// Run this test using the following command:
/// `cargo test --package pallet-jungochain --test root test_issance_bounds`
#[test]
fn test_issuance_bounds() {
    new_test_ext(1).execute_with(|| {
        // Simulate 100 halvings convergence to 21M. Note that the total issuance never reaches 21M because of rounding errors.
        // We converge to 20_999_999_989_500_000 (< 1 TAO away).
        let n_halvings: usize = 100;
        let mut total_issuance: u64 = 0;
        for _ in 0..n_halvings {
            let block_emission_10_500_000x: u64 =
                JungochainModule::get_block_emission_for_issuance(total_issuance).unwrap()
                    * 10_500_000;
            total_issuance += block_emission_10_500_000x;
        }
        assert_eq!(total_issuance, 20_999_999_989_500_000);
    })
}

/// This test checks the halving mechanism of the emission schedule.
/// Run this test using the following command:
/// `cargo test --package pallet-jungochain --test root test_halving`
#[test]
fn test_halving() {
    new_test_ext(1).execute_with(|| {
        let expected_emissions: [(u64, u64); 43] = [
            (0, 1_000_000_000), // Testing at zero issuance.
            (1_776_000, 1_000_000_000),
            (1_776_000_000, 1_000_000_000),
            (1_776_000_000_000, 1_000_000_000),
            (10_500_000_000_000_000, 500_000_000), // First halving event
            (10_999_999_000_000_000, 500_000_000),
            (11_000_000_000_000_000, 500_000_000),
            (12_000_999_000_000_000, 500_000_000),
            (15_749_999_000_000_000, 500_000_000),
            (15_800_000_000_000_000, 250_000_000), // Second halving event
            (16_400_999_000_000_000, 250_000_000),
            (16_499_999_000_000_000, 250_000_000),
            (17_624_999_000_000_000, 250_000_000),
            (18_400_000_000_000_000, 125_000_000), // Third halving event
            (19_312_500_000_000_000, 125_000_000),
            (19_700_000_000_000_000, 62_500_000), // Fourth halving event
            (19_906_249_000_000_000, 62_500_000),
            (20_400_000_000_000_000, 31_250_000), // Fifth halving event
            (20_500_000_000_000_000, 31_250_000),
            (20_700_000_000_000_000, 15_625_000), // Sixth halving event
            (20_800_000_000_000_000, 15_625_000),
            (20_900_000_000_000_000, 7_812_500), // Seventh halving event
            (20_917_970_000_000_000, 3_906_250), // Eighth halving event
            (20_958_985_000_000_000, 1_953_125), // Ninth halving event
            (20_979_493_000_000_000, 976_562),   // Tenth halving event
            (20_989_747_000_000_000, 488_281),   // Eleventh halving event
            (20_994_874_000_000_000, 244_140),   // Twelfth halving event
            (20_997_437_000_000_000, 122_070),   // Thirteenth halving event
            (20_998_719_000_000_000, 61_035),    // Fourteenth halving event
            (20_999_360_000_000_000, 30_517),    // Fifteenth halving event
            (20_999_680_000_000_000, 15_258),    // Sixteenth halving event
            (20_999_840_000_000_000, 7_629),     // Seventeenth halving event
            (20_999_920_000_000_000, 3_814),     // Eighteenth halving event
            (20_999_960_000_000_000, 1_907),     // Nineteenth halving event
            (20_999_980_000_000_000, 953),       // Twentieth halving event
            (20_999_990_000_000_000, 476),       // Twenty-first halving event
            (20_999_990_500_000_000, 476),
            (20_999_995_000_000_000, 238), // Twenty-second halving event
            (20_999_998_000_000_000, 119), // Twenty-third halving event
            (20_999_999_000_000_000, 59),  // Twenty-fourth halving event
            (21_000_000_000_000_000, 0),   // Total supply reached, emissions stop
            (21_100_000_000_000_000, 0),   // Just for fun
            (u64::MAX, 0),                 // Testing bounds
        ];

        for (issuance, expected_emission) in expected_emissions.iter() {
            JungochainModule::set_total_issuance(*issuance);
            step_block(1);

            let current_emission = JungochainModule::get_block_emission().unwrap();
            assert_eq!(
                current_emission, *expected_emission,
                "Incorrect emission {} at total issuance {}",
                current_emission, issuance
            );
        }
    });
}

#[test]
fn test_get_emission_across_entire_issuance_range() {
    new_test_ext(1).execute_with(|| {
        let total_supply: u64 = pallet_jungochain::TotalSupply::<Test>::get();
        let original_emission: u64 = pallet_jungochain::DefaultBlockEmission::<Test>::get();
        let halving_issuance: u64 = total_supply / 2;

        let mut issuance = 0;

        // Issuance won't reach total supply.
        while issuance <= 20_900_000_000_000_000 {
            JungochainModule::set_total_issuance(issuance);

            let issuance_f64 = issuance as f64;
            let h = f64::log2(1.0 / (1.0 - issuance_f64 / (2.0 * halving_issuance as f64)));
            let h = h.floor();
            let emission_percentage = f64::powf(2.0, -h);

            let expected_emission: u64 = if issuance < total_supply {
                (original_emission as f64 * emission_percentage) as u64
            } else {
                0
            };
            assert_eq!(
                JungochainModule::get_block_emission().unwrap(),
                expected_emission,
                "Issuance: {}",
                issuance_f64
            );

            issuance += expected_emission;
        }
    });
}

#[test]
fn test_dissolve_network_ok() {
    new_test_ext(1).execute_with(|| {
        let netuid: u16 = 30;
        let hotkey = U256::from(1);

        add_network(netuid, 0, 0);
        let owner_coldkey = JungochainModule::get_subnet_owner(netuid);
        register_ok_neuron(netuid, hotkey, owner_coldkey, 3);

        assert!(JungochainModule::if_subnet_exist(netuid));
        assert_ok!(JungochainModule::dissolve_network(
            RuntimeOrigin::root(),
            owner_coldkey,
            netuid
        ));
        assert!(!JungochainModule::if_subnet_exist(netuid))
    });
}

#[test]
fn test_dissolve_network_refund_coldkey_ok() {
    new_test_ext(1).execute_with(|| {
        let netuid: u16 = 30;
        let hotkey = U256::from(1);
        let subnet_locked_balance = 1000;

        add_network(netuid, 0, 0);
        let owner_coldkey = JungochainModule::get_subnet_owner(netuid);
        register_ok_neuron(netuid, hotkey, owner_coldkey, 3);

        JungochainModule::set_subnet_locked_balance(netuid, subnet_locked_balance);
        let coldkey_balance = JungochainModule::get_coldkey_balance(&owner_coldkey);

        assert!(JungochainModule::if_subnet_exist(netuid));
        assert_ok!(JungochainModule::dissolve_network(
            RuntimeOrigin::root(),
            owner_coldkey,
            netuid
        ));
        assert!(!JungochainModule::if_subnet_exist(netuid));

        let coldkey_new_balance = JungochainModule::get_coldkey_balance(&owner_coldkey);

        assert!(coldkey_new_balance > coldkey_balance);
        assert_eq!(coldkey_new_balance, coldkey_balance + subnet_locked_balance);
    });
}

#[test]
fn test_dissolve_network_not_owner_err() {
    new_test_ext(1).execute_with(|| {
        let netuid: u16 = 30;
        let hotkey = U256::from(1);
        let owner_coldkey = U256::from(2);
        let random_coldkey = U256::from(3);

        add_network(netuid, 0, 0);
        register_ok_neuron(netuid, hotkey, owner_coldkey, 3);

        assert_err!(
            JungochainModule::dissolve_network(RuntimeOrigin::root(), random_coldkey, netuid),
            Error::<Test>::NotSubnetOwner
        );
    });
}

#[test]
fn test_dissolve_network_does_not_exist_err() {
    new_test_ext(1).execute_with(|| {
        let netuid: u16 = 30;
        let coldkey = U256::from(2);

        assert_err!(
            JungochainModule::dissolve_network(RuntimeOrigin::root(), coldkey, netuid),
            Error::<Test>::SubNetworkDoesNotExist
        );
    });
}

#[test]
fn test_user_add_network_with_identity_fields_ok() {
    new_test_ext(1).execute_with(|| {
        let coldkey_1 = U256::from(1);
        let coldkey_2 = U256::from(2);
        let balance_1 = JungochainModule::get_network_lock_cost() + 10_000;

        let subnet_name_1: Vec<u8> = b"GenericSubnet1".to_vec();
        let github_repo_1: Vec<u8> = b"GenericSubnet1.com".to_vec();
        let subnet_contact_1: Vec<u8> = b"https://www.GenericSubnet1.co".to_vec();

        let identity_value_1: SubnetIdentity = SubnetIdentityOf {
            subnet_name: subnet_name_1.clone(),
            github_repo: github_repo_1.clone(),
            subnet_contact: subnet_contact_1.clone(),
        };

        let subnet_name_2: Vec<u8> = b"DistinctSubnet2".to_vec();
        let github_repo_2: Vec<u8> = b"https://github.com/DistinctRepo2".to_vec();
        let subnet_contact_2: Vec<u8> = b"https://contact2.example.com".to_vec();

        let identity_value_2: SubnetIdentity = SubnetIdentityOf {
            subnet_name: subnet_name_2.clone(),
            github_repo: github_repo_2.clone(),
            subnet_contact: subnet_contact_2.clone(),
        };

        JungochainModule::add_balance_to_coldkey_account(&coldkey_1, balance_1);

        assert_ok!(JungochainModule::user_add_network(
            RuntimeOrigin::signed(coldkey_1),
            Some(identity_value_1.clone()),
            false,
        ));

        let balance_2 = JungochainModule::get_network_lock_cost() + 10_000;
        JungochainModule::add_balance_to_coldkey_account(&coldkey_2, balance_2);

        assert_ok!(JungochainModule::user_add_network(
            RuntimeOrigin::signed(coldkey_2),
            Some(identity_value_2.clone()),
            false,
        ));

        let reserved_subnet_count = FirstReservedNetuids::<Test>::get();
        // after_reserved_netuid
        let arid = |id| id + reserved_subnet_count;

        let stored_identity_1: SubnetIdentity = SubnetIdentities::<Test>::get(arid(1)).unwrap();
        assert_eq!(stored_identity_1.subnet_name, subnet_name_1);
        assert_eq!(stored_identity_1.github_repo, github_repo_1);
        assert_eq!(stored_identity_1.subnet_contact, subnet_contact_1);

        let stored_identity_2: SubnetIdentity = SubnetIdentities::<Test>::get(arid(2)).unwrap();
        assert_eq!(stored_identity_2.subnet_name, subnet_name_2);
        assert_eq!(stored_identity_2.github_repo, github_repo_2);
        assert_eq!(stored_identity_2.subnet_contact, subnet_contact_2);

        // Now remove the first network.
        assert_ok!(JungochainModule::user_remove_network(coldkey_1, arid(1)));

        // Verify that the first network and identity have been removed.
        assert!(SubnetIdentities::<Test>::get(arid(1)).is_none());

        // Ensure the second network and identity are still intact.
        let stored_identity_2_after_removal: SubnetIdentity =
            SubnetIdentities::<Test>::get(arid(2)).unwrap();
        assert_eq!(stored_identity_2_after_removal.subnet_name, subnet_name_2);
        assert_eq!(stored_identity_2_after_removal.github_repo, github_repo_2);
        assert_eq!(
            stored_identity_2_after_removal.subnet_contact,
            subnet_contact_2
        );
    });
}
