
//! Autogenerated weights for `pallet_admin_utils`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-12-01, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `morpheus`, CPU: `AMD EPYC 7513 32-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("local")`, DB CACHE: `1024`

// Executed Command:
// ./target/release/node-subtensor
// benchmark
// pallet
// --chain=local
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_admin_utils
// --extrinsic=*
// --steps
// 50
// --repeat
// 20
// --output=pallets/admin-utils/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_admin_utils`.
pub trait WeightInfo {
	fn swap_authorities(a: u32, ) -> Weight;
    fn sudo_set_min_delegate_take() -> Weight;
    fn sudo_set_default_take() -> Weight;
	fn sudo_set_serving_rate_limit() -> Weight;
	fn sudo_set_max_difficulty() -> Weight;
	fn sudo_set_min_difficulty() -> Weight;
	fn sudo_set_weights_set_rate_limit() -> Weight;
	fn sudo_set_weights_version_key() -> Weight;
	fn sudo_set_bonds_moving_average() -> Weight;
	fn sudo_set_max_allowed_validators() -> Weight;
	fn sudo_set_difficulty() -> Weight;
	fn sudo_set_adjustment_interval() -> Weight;
	fn sudo_set_target_registrations_per_interval() -> Weight;
	fn sudo_set_activity_cutoff() -> Weight;
	fn sudo_set_rho() -> Weight;
	fn sudo_set_kappa() -> Weight;
	fn sudo_set_max_allowed_uids() -> Weight;
	fn sudo_set_min_allowed_weights() -> Weight;
	fn sudo_set_validator_prune_len() -> Weight;
	fn sudo_set_scaling_law_power() -> Weight;
	fn sudo_set_immunity_period() -> Weight;
	fn sudo_set_max_weight_limit() -> Weight;
	fn sudo_set_max_registrations_per_block() -> Weight;
	fn sudo_set_max_burn() -> Weight;
	fn sudo_set_min_burn() -> Weight;
	fn sudo_set_network_registration_allowed() -> Weight;
	fn sudo_set_tempo() -> Weight;
	fn sudo_set_commit_reveal_weights_interval() -> Weight;
	fn sudo_set_commit_reveal_weights_enabled() -> Weight;
}

/// Weights for `pallet_admin_utils` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: System Digest (r:1 w:1)
	/// Proof Skipped: System Digest (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Aura Authorities (r:0 w:1)
	/// Proof: Aura Authorities (max_values: Some(1), max_size: Some(1025), added: 1520, mode: MaxEncodedLen)
	/// The range of component `a` is `[0, 32]`.
	fn swap_authorities(a: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `632`
		//  Estimated: `1127`
		// Minimum execution time: 11_490_000 picoseconds.
		Weight::from_parts(20_410_228, 1127)
			// Standard Error: 8_309
			.saturating_add(Weight::from_parts(199_399, 0).saturating_mul(a.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: SubtensorModule DefaultTake (r:0 w:1)
	/// Proof Skipped: SubtensorModule DefaultTake (max_values: Some(1), max_size: None, mode: Measured)
	fn sudo_set_default_take() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `655`
		//  Estimated: `655`
		// Minimum execution time: 26_770_000 picoseconds.
		Weight::from_parts(27_199_000, 655)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: SubtensorModule DefaultTake (r:0 w:1)
	/// Proof Skipped: SubtensorModule DefaultTake (max_values: Some(1), max_size: None, mode: Measured)
	fn sudo_set_min_delegate_take() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `655`
		//  Estimated: `655`
		// Minimum execution time: 26_770_000 picoseconds.
		Weight::from_parts(27_199_000, 655)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: SubtensorModule ServingRateLimit (r:0 w:1)
	/// Proof Skipped: SubtensorModule ServingRateLimit (max_values: None, max_size: None, mode: Measured)
	fn sudo_set_serving_rate_limit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `655`
		//  Estimated: `655`
		// Minimum execution time: 27_700_000 picoseconds.
		Weight::from_parts(28_290_000, 655)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: SubtensorModule NetworksAdded (r:1 w:0)
	/// Proof Skipped: SubtensorModule NetworksAdded (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubtensorModule MaxDifficulty (r:0 w:1)
	/// Proof Skipped: SubtensorModule MaxDifficulty (max_values: None, max_size: None, mode: Measured)
	fn sudo_set_max_difficulty() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1111`
		//  Estimated: `4697`
		// Minimum execution time: 46_450_000 picoseconds.
		Weight::from_parts(47_279_000, 4697)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: SubtensorModule NetworksAdded (r:1 w:0)
	/// Proof Skipped: SubtensorModule NetworksAdded (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubtensorModule MinDifficulty (r:0 w:1)
	/// Proof Skipped: SubtensorModule MinDifficulty (max_values: None, max_size: None, mode: Measured)
	fn sudo_set_min_difficulty() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1111`
		//  Estimated: `4697`
		// Minimum execution time: 46_110_000 picoseconds.
		Weight::from_parts(46_909_000, 4697)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: SubtensorModule NetworksAdded (r:1 w:0)
	/// Proof Skipped: SubtensorModule NetworksAdded (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubtensorModule WeightsSetRateLimit (r:0 w:1)
	/// Proof Skipped: SubtensorModule WeightsSetRateLimit (max_values: None, max_size: None, mode: Measured)
	fn sudo_set_weights_set_rate_limit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1111`
		//  Estimated: `4697`
		// Minimum execution time: 46_349_000 picoseconds.
		Weight::from_parts(46_970_000, 4697)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: SubtensorModule NetworksAdded (r:1 w:0)
	/// Proof Skipped: SubtensorModule NetworksAdded (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubtensorModule WeightsVersionKey (r:0 w:1)
	/// Proof Skipped: SubtensorModule WeightsVersionKey (max_values: None, max_size: None, mode: Measured)
	fn sudo_set_weights_version_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1111`
		//  Estimated: `4697`
		// Minimum execution time: 45_940_000 picoseconds.
		Weight::from_parts(47_460_000, 4697)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: SubtensorModule NetworksAdded (r:1 w:0)
	/// Proof Skipped: SubtensorModule NetworksAdded (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubtensorModule BondsMovingAverage (r:0 w:1)
	/// Proof Skipped: SubtensorModule BondsMovingAverage (max_values: None, max_size: None, mode: Measured)
	fn sudo_set_bonds_moving_average() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1111`
		//  Estimated: `4697`
		// Minimum execution time: 46_099_000 picoseconds.
		Weight::from_parts(47_510_000, 4697)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: SubtensorModule NetworksAdded (r:1 w:0)
	/// Proof Skipped: SubtensorModule NetworksAdded (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubtensorModule MaxAllowedUids (r:1 w:0)
	/// Proof Skipped: SubtensorModule MaxAllowedUids (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubtensorModule MaxAllowedValidators (r:0 w:1)
	/// Proof Skipped: SubtensorModule MaxAllowedValidators (max_values: None, max_size: None, mode: Measured)
	fn sudo_set_max_allowed_validators() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1154`
		//  Estimated: `8412`
		// Minimum execution time: 52_599_000 picoseconds.
		Weight::from_parts(53_640_000, 8412)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: SubtensorModule NetworksAdded (r:1 w:0)
	/// Proof Skipped: SubtensorModule NetworksAdded (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubtensorModule Difficulty (r:0 w:1)
	/// Proof Skipped: SubtensorModule Difficulty (max_values: None, max_size: None, mode: Measured)
	fn sudo_set_difficulty() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1111`
		//  Estimated: `4697`
		// Minimum execution time: 46_240_000 picoseconds.
		Weight::from_parts(47_130_000, 4697)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: SubtensorModule NetworksAdded (r:1 w:0)
	/// Proof Skipped: SubtensorModule NetworksAdded (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubtensorModule AdjustmentInterval (r:0 w:1)
	/// Proof Skipped: SubtensorModule AdjustmentInterval (max_values: None, max_size: None, mode: Measured)
	fn sudo_set_adjustment_interval() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1111`
		//  Estimated: `4697`
		// Minimum execution time: 45_430_000 picoseconds.
		Weight::from_parts(46_790_000, 4697)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: SubtensorModule NetworksAdded (r:1 w:0)
	/// Proof Skipped: SubtensorModule NetworksAdded (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubtensorModule TargetRegistrationsPerInterval (r:0 w:1)
	/// Proof Skipped: SubtensorModule TargetRegistrationsPerInterval (max_values: None, max_size: None, mode: Measured)
	fn sudo_set_target_registrations_per_interval() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1111`
		//  Estimated: `4697`
		// Minimum execution time: 45_899_000 picoseconds.
		Weight::from_parts(47_099_000, 4697)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: SubtensorModule NetworksAdded (r:1 w:0)
	/// Proof Skipped: SubtensorModule NetworksAdded (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubtensorModule ActivityCutoff (r:0 w:1)
	/// Proof Skipped: SubtensorModule ActivityCutoff (max_values: None, max_size: None, mode: Measured)
	fn sudo_set_activity_cutoff() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1111`
		//  Estimated: `4697`
		// Minimum execution time: 46_029_000 picoseconds.
		Weight::from_parts(46_759_000, 4697)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: SubtensorModule NetworksAdded (r:1 w:0)
	/// Proof Skipped: SubtensorModule NetworksAdded (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubtensorModule Rho (r:0 w:1)
	/// Proof Skipped: SubtensorModule Rho (max_values: None, max_size: None, mode: Measured)
	fn sudo_set_rho() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `903`
		//  Estimated: `4281`
		// Minimum execution time: 30_980_000 picoseconds.
		Weight::from_parts(31_820_000, 4281)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: SubtensorModule NetworksAdded (r:1 w:0)
	/// Proof Skipped: SubtensorModule NetworksAdded (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubtensorModule Kappa (r:0 w:1)
	/// Proof Skipped: SubtensorModule Kappa (max_values: None, max_size: None, mode: Measured)
	fn sudo_set_kappa() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1111`
		//  Estimated: `4697`
		// Minimum execution time: 45_620_000 picoseconds.
		Weight::from_parts(46_440_000, 4697)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: SubtensorModule NetworksAdded (r:1 w:0)
	/// Proof Skipped: SubtensorModule NetworksAdded (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubtensorModule SubnetworkN (r:1 w:0)
	/// Proof Skipped: SubtensorModule SubnetworkN (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubtensorModule MaxAllowedUids (r:0 w:1)
	/// Proof Skipped: SubtensorModule MaxAllowedUids (max_values: None, max_size: None, mode: Measured)
	fn sudo_set_max_allowed_uids() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1117`
		//  Estimated: `8301`
		// Minimum execution time: 50_270_000 picoseconds.
		Weight::from_parts(51_149_000, 8301)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: SubtensorModule NetworksAdded (r:1 w:0)
	/// Proof Skipped: SubtensorModule NetworksAdded (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubtensorModule MinAllowedWeights (r:0 w:1)
	/// Proof Skipped: SubtensorModule MinAllowedWeights (max_values: None, max_size: None, mode: Measured)
	fn sudo_set_min_allowed_weights() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1111`
		//  Estimated: `4697`
		// Minimum execution time: 45_990_000 picoseconds.
		Weight::from_parts(47_390_000, 4697)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: SubtensorModule NetworksAdded (r:1 w:0)
	/// Proof Skipped: SubtensorModule NetworksAdded (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubtensorModule ValidatorPruneLen (r:0 w:1)
	/// Proof Skipped: SubtensorModule ValidatorPruneLen (max_values: None, max_size: None, mode: Measured)
	fn sudo_set_validator_prune_len() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1111`
		//  Estimated: `4697`
		// Minimum execution time: 45_939_000 picoseconds.
		Weight::from_parts(46_960_000, 4697)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: SubtensorModule NetworksAdded (r:1 w:0)
	/// Proof Skipped: SubtensorModule NetworksAdded (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubtensorModule ScalingLawPower (r:0 w:1)
	/// Proof Skipped: SubtensorModule ScalingLawPower (max_values: None, max_size: None, mode: Measured)
	fn sudo_set_scaling_law_power() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1111`
		//  Estimated: `4697`
		// Minimum execution time: 45_480_000 picoseconds.
		Weight::from_parts(46_590_000, 4697)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: SubtensorModule NetworksAdded (r:1 w:0)
	/// Proof Skipped: SubtensorModule NetworksAdded (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubtensorModule ImmunityPeriod (r:0 w:1)
	/// Proof Skipped: SubtensorModule ImmunityPeriod (max_values: None, max_size: None, mode: Measured)
	fn sudo_set_immunity_period() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1111`
		//  Estimated: `4697`
		// Minimum execution time: 45_289_000 picoseconds.
		Weight::from_parts(46_679_000, 4697)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: SubtensorModule NetworksAdded (r:1 w:0)
	/// Proof Skipped: SubtensorModule NetworksAdded (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubtensorModule MaxWeightsLimit (r:0 w:1)
	/// Proof Skipped: SubtensorModule MaxWeightsLimit (max_values: None, max_size: None, mode: Measured)
	fn sudo_set_max_weight_limit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1111`
		//  Estimated: `4697`
		// Minimum execution time: 45_850_000 picoseconds.
		Weight::from_parts(46_589_000, 4697)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: SubtensorModule NetworksAdded (r:1 w:0)
	/// Proof Skipped: SubtensorModule NetworksAdded (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubtensorModule MaxRegistrationsPerBlock (r:0 w:1)
	/// Proof Skipped: SubtensorModule MaxRegistrationsPerBlock (max_values: None, max_size: None, mode: Measured)
	fn sudo_set_max_registrations_per_block() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1111`
		//  Estimated: `4697`
		// Minimum execution time: 45_330_000 picoseconds.
		Weight::from_parts(46_490_000, 4697)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: SubtensorModule NetworksAdded (r:1 w:0)
	/// Proof Skipped: SubtensorModule NetworksAdded (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubtensorModule MaxBurn (r:0 w:1)
	/// Proof Skipped: SubtensorModule MaxBurn (max_values: None, max_size: None, mode: Measured)
	fn sudo_set_max_burn() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1111`
		//  Estimated: `4697`
		// Minimum execution time: 45_390_000 picoseconds.
		Weight::from_parts(46_339_000, 4697)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: SubtensorModule NetworksAdded (r:1 w:0)
	/// Proof Skipped: SubtensorModule NetworksAdded (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubtensorModule MinBurn (r:0 w:1)
	/// Proof Skipped: SubtensorModule MinBurn (max_values: None, max_size: None, mode: Measured)
	fn sudo_set_min_burn() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1111`
		//  Estimated: `4697`
		// Minimum execution time: 45_189_000 picoseconds.
		Weight::from_parts(46_109_000, 4697)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: SubtensorModule NetworkPowRegistrationAllowed (r:0 w:1)
	/// Proof Skipped: SubtensorModule NetworkPowRegistrationAllowed (max_values: None, max_size: None, mode: Measured)
	fn sudo_set_network_registration_allowed() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `655`
		//  Estimated: `655`
		// Minimum execution time: 33_600_000 picoseconds.
		Weight::from_parts(34_599_000, 655)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: SubtensorModule NetworksAdded (r:1 w:0)
	/// Proof Skipped: SubtensorModule NetworksAdded (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubtensorModule Tempo (r:0 w:1)
	/// Proof Skipped: SubtensorModule Tempo (max_values: None, max_size: None, mode: Measured)
	fn sudo_set_tempo() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1111`
		//  Estimated: `4697`
		// Minimum execution time: 44_739_000 picoseconds.
		Weight::from_parts(45_489_000, 4697)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn sudo_set_commit_reveal_weights_interval() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1111`
		//  Estimated: `4697`
		// Minimum execution time: 46_450_000 picoseconds.
		Weight::from_parts(47_279_000, 4697)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn sudo_set_commit_reveal_weights_enabled() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1111`
		//  Estimated: `4697`
		// Minimum execution time: 46_450_000 picoseconds.
		Weight::from_parts(47_279_000, 4697)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: System Digest (r:1 w:1)
	/// Proof Skipped: System Digest (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Aura Authorities (r:0 w:1)
	/// Proof: Aura Authorities (max_values: Some(1), max_size: Some(1025), added: 1520, mode: MaxEncodedLen)
	/// The range of component `a` is `[0, 32]`.
	fn swap_authorities(a: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `632`
		//  Estimated: `1127`
		// Minimum execution time: 11_490_000 picoseconds.
		Weight::from_parts(20_410_228, 1127)
			// Standard Error: 8_309
			.saturating_add(Weight::from_parts(199_399, 0).saturating_mul(a.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: SubtensorModule DefaultTake (r:0 w:1)
	/// Proof Skipped: SubtensorModule DefaultTake (max_values: Some(1), max_size: None, mode: Measured)
	fn sudo_set_default_take() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `655`
		//  Estimated: `655`
		// Minimum execution time: 26_770_000 picoseconds.
		Weight::from_parts(27_199_000, 655)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: SubtensorModule DefaultTake (r:0 w:1)
	/// Proof Skipped: SubtensorModule DefaultTake (max_values: Some(1), max_size: None, mode: Measured)
	fn sudo_set_min_delegate_take() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `655`
		//  Estimated: `655`
		// Minimum execution time: 26_770_000 picoseconds.
		Weight::from_parts(27_199_000, 655)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: SubtensorModule ServingRateLimit (r:0 w:1)
	/// Proof Skipped: SubtensorModule ServingRateLimit (max_values: None, max_size: None, mode: Measured)
	fn sudo_set_serving_rate_limit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `655`
		//  Estimated: `655`
		// Minimum execution time: 27_700_000 picoseconds.
		Weight::from_parts(28_290_000, 655)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: SubtensorModule NetworksAdded (r:1 w:0)
	/// Proof Skipped: SubtensorModule NetworksAdded (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubtensorModule MaxDifficulty (r:0 w:1)
	/// Proof Skipped: SubtensorModule MaxDifficulty (max_values: None, max_size: None, mode: Measured)
	fn sudo_set_max_difficulty() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1111`
		//  Estimated: `4697`
		// Minimum execution time: 46_450_000 picoseconds.
		Weight::from_parts(47_279_000, 4697)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: SubtensorModule NetworksAdded (r:1 w:0)
	/// Proof Skipped: SubtensorModule NetworksAdded (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubtensorModule MinDifficulty (r:0 w:1)
	/// Proof Skipped: SubtensorModule MinDifficulty (max_values: None, max_size: None, mode: Measured)
	fn sudo_set_min_difficulty() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1111`
		//  Estimated: `4697`
		// Minimum execution time: 46_110_000 picoseconds.
		Weight::from_parts(46_909_000, 4697)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: SubtensorModule NetworksAdded (r:1 w:0)
	/// Proof Skipped: SubtensorModule NetworksAdded (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubtensorModule WeightsSetRateLimit (r:0 w:1)
	/// Proof Skipped: SubtensorModule WeightsSetRateLimit (max_values: None, max_size: None, mode: Measured)
	fn sudo_set_weights_set_rate_limit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1111`
		//  Estimated: `4697`
		// Minimum execution time: 46_349_000 picoseconds.
		Weight::from_parts(46_970_000, 4697)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: SubtensorModule NetworksAdded (r:1 w:0)
	/// Proof Skipped: SubtensorModule NetworksAdded (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubtensorModule WeightsVersionKey (r:0 w:1)
	/// Proof Skipped: SubtensorModule WeightsVersionKey (max_values: None, max_size: None, mode: Measured)
	fn sudo_set_weights_version_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1111`
		//  Estimated: `4697`
		// Minimum execution time: 45_940_000 picoseconds.
		Weight::from_parts(47_460_000, 4697)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: SubtensorModule NetworksAdded (r:1 w:0)
	/// Proof Skipped: SubtensorModule NetworksAdded (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubtensorModule BondsMovingAverage (r:0 w:1)
	/// Proof Skipped: SubtensorModule BondsMovingAverage (max_values: None, max_size: None, mode: Measured)
	fn sudo_set_bonds_moving_average() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1111`
		//  Estimated: `4697`
		// Minimum execution time: 46_099_000 picoseconds.
		Weight::from_parts(47_510_000, 4697)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: SubtensorModule NetworksAdded (r:1 w:0)
	/// Proof Skipped: SubtensorModule NetworksAdded (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubtensorModule MaxAllowedUids (r:1 w:0)
	/// Proof Skipped: SubtensorModule MaxAllowedUids (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubtensorModule MaxAllowedValidators (r:0 w:1)
	/// Proof Skipped: SubtensorModule MaxAllowedValidators (max_values: None, max_size: None, mode: Measured)
	fn sudo_set_max_allowed_validators() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1154`
		//  Estimated: `8412`
		// Minimum execution time: 52_599_000 picoseconds.
		Weight::from_parts(53_640_000, 8412)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: SubtensorModule NetworksAdded (r:1 w:0)
	/// Proof Skipped: SubtensorModule NetworksAdded (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubtensorModule Difficulty (r:0 w:1)
	/// Proof Skipped: SubtensorModule Difficulty (max_values: None, max_size: None, mode: Measured)
	fn sudo_set_difficulty() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1111`
		//  Estimated: `4697`
		// Minimum execution time: 46_240_000 picoseconds.
		Weight::from_parts(47_130_000, 4697)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: SubtensorModule NetworksAdded (r:1 w:0)
	/// Proof Skipped: SubtensorModule NetworksAdded (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubtensorModule AdjustmentInterval (r:0 w:1)
	/// Proof Skipped: SubtensorModule AdjustmentInterval (max_values: None, max_size: None, mode: Measured)
	fn sudo_set_adjustment_interval() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1111`
		//  Estimated: `4697`
		// Minimum execution time: 45_430_000 picoseconds.
		Weight::from_parts(46_790_000, 4697)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: SubtensorModule NetworksAdded (r:1 w:0)
	/// Proof Skipped: SubtensorModule NetworksAdded (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubtensorModule TargetRegistrationsPerInterval (r:0 w:1)
	/// Proof Skipped: SubtensorModule TargetRegistrationsPerInterval (max_values: None, max_size: None, mode: Measured)
	fn sudo_set_target_registrations_per_interval() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1111`
		//  Estimated: `4697`
		// Minimum execution time: 45_899_000 picoseconds.
		Weight::from_parts(47_099_000, 4697)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: SubtensorModule NetworksAdded (r:1 w:0)
	/// Proof Skipped: SubtensorModule NetworksAdded (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubtensorModule ActivityCutoff (r:0 w:1)
	/// Proof Skipped: SubtensorModule ActivityCutoff (max_values: None, max_size: None, mode: Measured)
	fn sudo_set_activity_cutoff() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1111`
		//  Estimated: `4697`
		// Minimum execution time: 46_029_000 picoseconds.
		Weight::from_parts(46_759_000, 4697)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: SubtensorModule NetworksAdded (r:1 w:0)
	/// Proof Skipped: SubtensorModule NetworksAdded (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubtensorModule Rho (r:0 w:1)
	/// Proof Skipped: SubtensorModule Rho (max_values: None, max_size: None, mode: Measured)
	fn sudo_set_rho() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `903`
		//  Estimated: `4281`
		// Minimum execution time: 30_980_000 picoseconds.
		Weight::from_parts(31_820_000, 4281)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: SubtensorModule NetworksAdded (r:1 w:0)
	/// Proof Skipped: SubtensorModule NetworksAdded (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubtensorModule Kappa (r:0 w:1)
	/// Proof Skipped: SubtensorModule Kappa (max_values: None, max_size: None, mode: Measured)
	fn sudo_set_kappa() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1111`
		//  Estimated: `4697`
		// Minimum execution time: 45_620_000 picoseconds.
		Weight::from_parts(46_440_000, 4697)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: SubtensorModule NetworksAdded (r:1 w:0)
	/// Proof Skipped: SubtensorModule NetworksAdded (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubtensorModule SubnetworkN (r:1 w:0)
	/// Proof Skipped: SubtensorModule SubnetworkN (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubtensorModule MaxAllowedUids (r:0 w:1)
	/// Proof Skipped: SubtensorModule MaxAllowedUids (max_values: None, max_size: None, mode: Measured)
	fn sudo_set_max_allowed_uids() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1117`
		//  Estimated: `8301`
		// Minimum execution time: 50_270_000 picoseconds.
		Weight::from_parts(51_149_000, 8301)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: SubtensorModule NetworksAdded (r:1 w:0)
	/// Proof Skipped: SubtensorModule NetworksAdded (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubtensorModule MinAllowedWeights (r:0 w:1)
	/// Proof Skipped: SubtensorModule MinAllowedWeights (max_values: None, max_size: None, mode: Measured)
	fn sudo_set_min_allowed_weights() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1111`
		//  Estimated: `4697`
		// Minimum execution time: 45_990_000 picoseconds.
		Weight::from_parts(47_390_000, 4697)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: SubtensorModule NetworksAdded (r:1 w:0)
	/// Proof Skipped: SubtensorModule NetworksAdded (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubtensorModule ValidatorPruneLen (r:0 w:1)
	/// Proof Skipped: SubtensorModule ValidatorPruneLen (max_values: None, max_size: None, mode: Measured)
	fn sudo_set_validator_prune_len() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1111`
		//  Estimated: `4697`
		// Minimum execution time: 45_939_000 picoseconds.
		Weight::from_parts(46_960_000, 4697)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: SubtensorModule NetworksAdded (r:1 w:0)
	/// Proof Skipped: SubtensorModule NetworksAdded (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubtensorModule ScalingLawPower (r:0 w:1)
	/// Proof Skipped: SubtensorModule ScalingLawPower (max_values: None, max_size: None, mode: Measured)
	fn sudo_set_scaling_law_power() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1111`
		//  Estimated: `4697`
		// Minimum execution time: 45_480_000 picoseconds.
		Weight::from_parts(46_590_000, 4697)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: SubtensorModule NetworksAdded (r:1 w:0)
	/// Proof Skipped: SubtensorModule NetworksAdded (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubtensorModule ImmunityPeriod (r:0 w:1)
	/// Proof Skipped: SubtensorModule ImmunityPeriod (max_values: None, max_size: None, mode: Measured)
	fn sudo_set_immunity_period() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1111`
		//  Estimated: `4697`
		// Minimum execution time: 45_289_000 picoseconds.
		Weight::from_parts(46_679_000, 4697)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: SubtensorModule NetworksAdded (r:1 w:0)
	/// Proof Skipped: SubtensorModule NetworksAdded (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubtensorModule MaxWeightsLimit (r:0 w:1)
	/// Proof Skipped: SubtensorModule MaxWeightsLimit (max_values: None, max_size: None, mode: Measured)
	fn sudo_set_max_weight_limit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1111`
		//  Estimated: `4697`
		// Minimum execution time: 45_850_000 picoseconds.
		Weight::from_parts(46_589_000, 4697)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: SubtensorModule NetworksAdded (r:1 w:0)
	/// Proof Skipped: SubtensorModule NetworksAdded (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubtensorModule MaxRegistrationsPerBlock (r:0 w:1)
	/// Proof Skipped: SubtensorModule MaxRegistrationsPerBlock (max_values: None, max_size: None, mode: Measured)
	fn sudo_set_max_registrations_per_block() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1111`
		//  Estimated: `4697`
		// Minimum execution time: 45_330_000 picoseconds.
		Weight::from_parts(46_490_000, 4697)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: SubtensorModule NetworksAdded (r:1 w:0)
	/// Proof Skipped: SubtensorModule NetworksAdded (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubtensorModule MaxBurn (r:0 w:1)
	/// Proof Skipped: SubtensorModule MaxBurn (max_values: None, max_size: None, mode: Measured)
	fn sudo_set_max_burn() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1111`
		//  Estimated: `4697`
		// Minimum execution time: 45_390_000 picoseconds.
		Weight::from_parts(46_339_000, 4697)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: SubtensorModule NetworksAdded (r:1 w:0)
	/// Proof Skipped: SubtensorModule NetworksAdded (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubtensorModule MinBurn (r:0 w:1)
	/// Proof Skipped: SubtensorModule MinBurn (max_values: None, max_size: None, mode: Measured)
	fn sudo_set_min_burn() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1111`
		//  Estimated: `4697`
		// Minimum execution time: 45_189_000 picoseconds.
		Weight::from_parts(46_109_000, 4697)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: SubtensorModule NetworkPowRegistrationAllowed (r:0 w:1)
	/// Proof Skipped: SubtensorModule NetworkPowRegistrationAllowed (max_values: None, max_size: None, mode: Measured)
	fn sudo_set_network_registration_allowed() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `655`
		//  Estimated: `655`
		// Minimum execution time: 33_600_000 picoseconds.
		Weight::from_parts(34_599_000, 655)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: SubtensorModule NetworksAdded (r:1 w:0)
	/// Proof Skipped: SubtensorModule NetworksAdded (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubtensorModule Tempo (r:0 w:1)
	/// Proof Skipped: SubtensorModule Tempo (max_values: None, max_size: None, mode: Measured)
	fn sudo_set_tempo() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1111`
		//  Estimated: `4697`
		// Minimum execution time: 44_739_000 picoseconds.
		Weight::from_parts(45_489_000, 4697)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn sudo_set_commit_reveal_weights_interval() -> Weight {
		// -- Extrinsic Time --
		// Model:
		// Time ~=    20.42
		//               µs
		// Reads = 1
		// Writes = 1
		// Recorded proof Size = 456
		Weight::from_parts(20_420_000, 456)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn sudo_set_commit_reveal_weights_enabled() -> Weight {
		// -- Extrinsic Time --
		// Model:
		// Time ~=    19.78
		//               µs
		// Reads = 1
		// Writes = 1
		// Recorded proof Size = 456
		Weight::from_parts(19_780_000, 456)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}