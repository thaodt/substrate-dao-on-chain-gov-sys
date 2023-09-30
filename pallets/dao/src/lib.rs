#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		pallet_prelude::*,
		traits::{Currency, ReservableCurrency},
		PalletId,
	};
	use frame_system::pallet_prelude::*;
	use sp_runtime::traits::AtLeast32BitUnsigned;

	#[pallet::pallet]
	pub struct Pallet<T>(PhantomData<T>);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	pub type Proposals<T: Config> =
		StorageMap<_, Blake2_128Concat, T::ProposalId, Proposal<T::AccountId, T::Balance>>;

	#[pallet::storage]
	pub type Votes<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::ProposalId,
		Blake2_128Concat,
		T::AccountId,
		Vote<T::AccountId>,
	>;

	// #[pallet::storage]
	// pub type DaoConfig<T: Config> = StorageValue<_, DaoConfig<T::BlockNumber, T::Balance>>;

	#[pallet::storage]
	pub type Masks<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, T::Hash>;

	#[pallet::storage]
	pub type RevealedValues<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u32>;

	#[pallet::storage]
	pub type ActiveBots<T: Config> = StorageValue<_, u32>;

	// pub type DaoConfig<BlockNumber, Balance> = (BlockNumber, Balance);

	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
	pub struct Proposal<AccountId, Balance> {
		proposer: AccountId,
		value: Balance,
		description: Vec<u8>,
	}

	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
	pub struct DaoConfig<BlockNumber, Balance> {
		block_number: BlockNumber,
		balance: Balance,
	}

	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
	pub struct Vote<AccountId> {
		voter: AccountId,
		is_support: bool,
	}

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		#[pallet::constant]
		type PalletId: Get<PalletId>;
		/// type currency for this pallet, it is used to reserve the storage, staking, and reward
		/// for randomness.
		type Currency: ReservableCurrency<Self::AccountId>;
		/// Type representing the weight of this pallet
		type WeightInfo: WeightInfo;

		type ProposalId: Parameter + Default + Copy;
		type Balance: Parameter + Member + AtLeast32BitUnsigned + Default + Copy;
		type BlockNumber: Parameter + Member + AtLeast32BitUnsigned + Default + Copy;
	}

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		MaskSubmitted(T::AccountId),
		ValueRevealed(T::AccountId),
		RandomNumberGenerated(T::AccountId, u32),
		Voted(T::AccountId, T::ProposalId, bool),
		ProposalCreated(T::AccountId, T::ProposalId, T::Balance),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		CanNotSub,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(10_000)] // TODO: update as auto weight!
		pub fn create_proposal(
			origin: OriginFor<T>,
			value: T::Balance,
			description: Vec<u8>,
		) -> DispatchResult {
			let proposer = ensure_signed(origin)?;

			// Check if the proposer has enough balance
			ensure!(T::Currency::free_balance(&proposer) >= value, Error::<T>::InsufficientBalance);

			// Generate a new proposal ID
			let proposal_id = T::ProposalId::unique_saturated_from(0);

			// Create a new proposal
			let proposal = Proposal { proposer: proposer.clone(), value, description };

			// Store the proposal
			Proposals::<T>::insert(proposal_id, proposal);

			// Emit the ProposalCreated event
			Self::deposit_event(Event::ProposalCreated(proposer, proposal_id, value));

			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(10_000)] // TODO: update as auto weight!
		pub fn vote(
			origin: OriginFor<T>,
			proposal_id: T::ProposalId,
			is_support: bool,
		) -> DispatchResult {
			let voter = ensure_signed(origin)?;

			// Check if the proposal exists
			ensure!(Proposals::<T>::contains_key(proposal_id), Error::<T>::InvalidProposalId);

			// Create a new vote
			let vote = Vote { voter: voter.clone(), is_support };

			// Store the vote
			Votes::<T>::insert(proposal_id, voter, vote);

			// Emit the Voted event
			Self::deposit_event(Event::Voted(voter, proposal_id, is_support));

			Ok(())
		}
	}
}
