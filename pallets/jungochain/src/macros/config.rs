#![allow(clippy::crate_in_macro_def)]

use frame_support::pallet_macros::pallet_section;
/// A [`pallet_section`] that defines the errors for a pallet.
/// This can later be imported into the pallet using [`import_section`].
#[pallet_section]
mod config {
    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// call type
        type RuntimeCall: Parameter
            + Dispatchable<RuntimeOrigin = Self::RuntimeOrigin>
            + From<Call<Self>>
            + IsType<<Self as frame_system::Config>::RuntimeCall>
            + From<frame_system::Call<Self>>;

        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// A sudo-able call.
        type SudoRuntimeCall: Parameter
            + UnfilteredDispatchable<RuntimeOrigin = Self::RuntimeOrigin>
            + GetDispatchInfo;

        /// Origin checking for council majority
        type CouncilOrigin: EnsureOrigin<Self::RuntimeOrigin>;

        ///  Currency type that will be used to place deposits on neurons
        type Currency: fungible::Balanced<Self::AccountId, Balance = u64>
            + fungible::Mutate<Self::AccountId>;

        /// Senate members with members management functions.
        type SenateMembers: crate::MemberManagement<Self::AccountId>;

        /// Interface to allow other pallets to control who can register identities
        type TriumvirateInterface: crate::CollectiveInterface<Self::AccountId, Self::Hash, u32>;

        /// The scheduler type used for scheduling delayed calls.
        type Scheduler: ScheduleAnon<
            BlockNumberFor<Self>,
            LocalCallOf<Self>,
            PalletsOriginOf<Self>,
            Hasher = Self::Hashing,
        >;

        /// the preimage to store the call data.
        type Preimages: QueryPreimage<H = Self::Hashing> + StorePreimage;

        // =================================
        // ==== Initial Value Constants ====
        // =================================

        /// Default total currency issuance.
        #[pallet::constant]
        type InitialIssuance: Get<u64>;
        /// Minimum number of UIDs a subnet validator must set weights on
        #[pallet::constant]
        type InitialMinAllowedWeights: Get<u16>;
        /// Initial Emission Ratio. TODO: study
        #[pallet::constant]
        type InitialEmissionValue: Get<u16>;
        /// Max wight that validator can be set for miners in a subnet
        #[pallet::constant]
        type InitialMaxWeightsLimit: Get<u16>;
        /// Tempo is the block interval to run epoch for each subnet, (emission will propagate at each epoch)
        #[pallet::constant]
        type InitialTempo: Get<u16>;
        /// Initial Difficulty.
        #[pallet::constant]
        type InitialDifficulty: Get<u64>;
        /// Initial Max Difficulty.
        #[pallet::constant]
        type InitialMaxDifficulty: Get<u64>;
        /// Initial Min Difficulty.
        #[pallet::constant]
        type InitialMinDifficulty: Get<u64>;
        /// Initial RAO Recycled.
        #[pallet::constant]
        type InitialRAORecycledForRegistration: Get<u64>;
        /// Initial Burn.
        #[pallet::constant]
        type InitialBurn: Get<u64>;
        /// Maximum cost to register on this subnet
        #[pallet::constant]
        type InitialMaxBurn: Get<u64>;
        /// Minimum cost to register on this subnet
        #[pallet::constant]
        type InitialMinBurn: Get<u64>;
        /// Number of blocks after which the recycle register cost and the pow_register difficulty are recalculated
        #[pallet::constant]
        type InitialAdjustmentInterval: Get<u16>;
        /// Initial bonds moving average.
        #[pallet::constant]
        type InitialBondsMovingAverage: Get<u64>;
        /// The target number of registrations desired in a AdjustmentInterval period
        #[pallet::constant]
        type InitialTargetRegistrationsPerInterval: Get<u16>;
        /// Rho constant.
        #[pallet::constant]
        type InitialRho: Get<u16>;
        /// Kappa constant.
        #[pallet::constant]
        type InitialKappa: Get<u16>;
        /// Max allowed neuron, each neuron has an uid
        #[pallet::constant]
        type InitialMaxAllowedUids: Get<u16>;
        /// Initial validator context pruning length.
        #[pallet::constant]
        type InitialValidatorPruneLen: Get<u64>;
        /// Initial scaling law power. TODO: study
        #[pallet::constant]
        type InitialScalingLawPower: Get<u16>;
        /// The immunity period is the number of blocks given to a subnet miner or a subnet validator
        /// at a UID before they are considered available for deregistration.
        #[pallet::constant]
        type InitialImmunityPeriod: Get<u16>;
        /// If a subnet validator has not set weights on the blockchain for activity_cutoff duration,
        /// then the Yuma Consensus will consider this subnet validator as offline
        #[pallet::constant]
        type InitialActivityCutoff: Get<u16>;
        /// Maximum allowed registrations in a subnet per block
        #[pallet::constant]
        type InitialMaxRegistrationsPerBlock: Get<u16>;
        /// Initial pruning score for each neuron.
        #[pallet::constant]
        type InitialPruningScore: Get<u16>;
        /// Initial maximum allowed validators per network.
        #[pallet::constant]
        type InitialMaxAllowedValidators: Get<u16>;
        /// Initial default delegation take.
        #[pallet::constant]
        type InitialDefaultDelegateTake: Get<u16>;
        /// Initial minimum delegation take.
        #[pallet::constant]
        type InitialMinDelegateTake: Get<u16>;
        /// Initial default childkey take.
        #[pallet::constant]
        type InitialDefaultChildKeyTake: Get<u16>;
        /// Initial minimum childkey take.
        #[pallet::constant]
        type InitialMinChildKeyTake: Get<u16>;
        /// Initial maximum childkey take.
        #[pallet::constant]
        type InitialMaxChildKeyTake: Get<u16>;
        /// Minimum required version of the subnet validator code
        #[pallet::constant]
        type InitialWeightsVersionKey: Get<u64>;
        /// Initial serving rate limit.
        #[pallet::constant]
        type InitialServingRateLimit: Get<u64>;
        /// Initial transaction rate limit.
        #[pallet::constant]
        type InitialTxRateLimit: Get<u64>;
        /// Initial delegate take transaction rate limit.
        #[pallet::constant]
        type InitialTxDelegateTakeRateLimit: Get<u64>;
        /// Initial childkey take transaction rate limit.
        #[pallet::constant]
        type InitialTxChildKeyTakeRateLimit: Get<u64>;
        /// Initial percentage of total stake required to join senate.
        #[pallet::constant]
        type InitialSenateRequiredStakePercentage: Get<u64>;
        /// Initial adjustment alpha on burn and pow.
        /// TODO: (https://docs.bittensor.com/subnets/subnet-hyperparameters#adjustment_alpha)
        #[pallet::constant]
        type InitialAdjustmentAlpha: Get<u64>;
        /// Initial network immunity period
        #[pallet::constant]
        type InitialNetworkImmunityPeriod: Get<u64>;
        /// Initial minimum allowed network UIDs
        #[pallet::constant]
        type InitialNetworkMinAllowedUids: Get<u16>;
        /// Minimum Subnet Registration Cost
        #[pallet::constant]
        type InitialNetworkMinLockCost: Get<u64>;
        /// Initial network subnet cut.
        #[pallet::constant]
        type InitialSubnetOwnerCut: Get<u16>;
        /// Used to calcualte Subnet Reg Cost
        #[pallet::constant]
        type InitialNetworkLockReductionInterval: Get<u64>;
        /// Initial max allowed subnets
        #[pallet::constant]
        type InitialSubnetLimit: Get<SubnetCount>;
        #[pallet::constant]
        /// Initial number of reserved subnets
        type InitialFirstReservedNetuids: Get<SubnetCount>;
        /// Minimum block interval to be allowed to register a Subnet
        #[pallet::constant]
        type InitialNetworkRateLimit: Get<u64>;
        /// Initial target stakes per interval issuance.
        #[pallet::constant]
        type InitialTargetStakesPerInterval: Get<u64>;
        /// Cost of swapping a hotkey.
        #[pallet::constant]
        type KeySwapCost: Get<u64>;
        /// The upper bound for the alpha parameter. Used for Liquid Alpha.
        #[pallet::constant]
        type AlphaHigh: Get<u16>;
        /// The lower bound for the alpha parameter. Used for Liquid Alpha.
        #[pallet::constant]
        type AlphaLow: Get<u16>;
        /// A flag to indicate if Liquid Alpha is enabled.
        #[pallet::constant]
        type LiquidAlphaOn: Get<bool>;
        /// Initial network max stake.
        #[pallet::constant]
        type InitialNetworkMaxStake: Get<u64>;
        /// Initial hotkey emission tempo.
        #[pallet::constant]
        type InitialHotkeyEmissionTempo: Get<u64>;
        /// Coldkey swap schedule duartion.
        #[pallet::constant]
        type InitialColdkeySwapScheduleDuration: Get<BlockNumberFor<Self>>;
        /// Dissolve network schedule duration
        #[pallet::constant]
        type InitialDissolveNetworkScheduleDuration: Get<BlockNumberFor<Self>>;
        /// Root subnet Tempo
        #[pallet::constant]
        type RootTempo: Get<u16>;
        /// Default Stake Interval
        #[pallet::constant]
        type DefaultStakeInterval: Get<u64>;
        /// Default Weights Set Rate Limit
        #[pallet::constant]
        type DefaultWeightsSetRateLimit: Get<u64>;
    }
}
