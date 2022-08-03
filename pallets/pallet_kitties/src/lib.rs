#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
use serde::{Serialize, Deserialize};
use core::fmt::Debug;
use frame_system::pallet_prelude::*;
use frame_support::{dispatch::fmt, inherent::Vec, pallet_prelude::*, traits::Randomness, traits::UnixTime};
use log::info;
#[frame_support::pallet]
pub mod pallet {

	pub use super::*;

	#[derive(TypeInfo, Default, Encode, Decode, Clone)]
	#[scale_info(skip_type_params(T))]

	pub struct Kitty<T: Config> {
		dna: Vec<u8>,
		owner: T::AccountId,
		price: u32,
		gender: Gender,
		created_date: u64,
	}

	impl<T: Config> fmt::Debug for Kitty<T> {
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
			f.debug_struct("Kitty")
				.field("dna", &self.dna)
				.field("owner", &self.owner)
				.field("price", &self.price)
				.field("gender", &self.gender)
				.field("created_date", &self.created_date)
				.finish()
		}
	}

	#[derive(TypeInfo, Encode, Decode, Debug, Clone, Serialize, Deserialize)]

	pub enum Gender {
		Male,
		Female,
	}

	impl Default for Gender {
		fn default() -> Self {
			Gender::Male
		}
	}

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type TimeProvider: UnixTime;
		type DnaRandomness: Randomness<Self::Hash, Self::BlockNumber>;

		#[pallet::constant]
		type MaxKitty: Get<u8>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn kitties_number)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type KittiesNumber<T> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	pub(super) type Kitties<T: Config> =
		StorageMap<_, Blake2_128Concat, Vec<u8>, Kitty<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn owners_kitty)]
	pub(super) type OwnersKitty<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, Vec<Vec<u8>>, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		KittyStored(Vec<u8>, u32),
		TransferKitty(Vec<u8>, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		NotExistKitty,
		LimitKitty,
	}

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub kitties: Vec<(T::AccountId, Vec<u8>, u32, Gender, u64)>
	}

	#[cfg(feature = "std")]
	impl <T: Config> Default for GenesisConfig<T> {
		fn default() -> GenesisConfig<T> {
			GenesisConfig {
				kitties: Vec::new()
			}
		}
	}

	#[pallet::genesis_build]
	impl <T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			KittiesNumber::<T>::put(self.kitties.len() as u32);
			
			let (account_id, dna, price, gender, created_date) = self.kitties[0].clone();
			let _kitty = Kitty::<T> {
				dna: dna.clone(),
				gender,
				price,
				owner: account_id.clone(),
				created_date
			};
			let mut new_kitties = Vec::new();
			new_kitties.push(dna.clone());
			<Kitties<T>>::insert(dna, _kitty);
			<OwnersKitty<T>>::insert(account_id, new_kitties);
		}
	}

	pub trait Test {}
	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create kitty
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_kitty(origin: OriginFor<T>, price: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins

			// Account extrinsics
			let who = ensure_signed(origin)?;
			let account_kitty_number = <OwnersKitty<T>>::get(&who).len();
			ensure!(account_kitty_number < T::MaxKitty::get() as usize, Error::<T>::LimitKitty);

			// Generate timestamp now
			let time_now = T::TimeProvider::now();

			let mut kitties_count = KittiesNumber::<T>::get();

			let dna_random = Self::generate_random_adn(&kitties_count).encode();
			// Generate gender
			let gender = Self::gen_gender(&dna_random)?;

			// Create new kitty
			let kitty = Kitty {
				dna: dna_random.clone(),
				gender,
				price,
				owner: who.clone(),
				created_date: time_now.as_secs(),
			};
			info!("Kitties: {:?}", kitty);

			// Update kitty count
			kitties_count += 1;

			KittiesNumber::<T>::put(kitties_count);
			
			// Add new kitty into storage map kitties
			<Kitties<T>>::insert(dna_random.clone(), kitty);

			// Check account has any kitty
			let check_exist_owner = <OwnersKitty<T>>::contains_key(who.clone());

			// Update OwnerKitty based account has any kitty
			if check_exist_owner {
				<OwnersKitty<T>>::mutate(who.clone(), |_kitties_vec| {
					_kitties_vec.push(dna_random.clone());
				})
			} else {
				let mut new_kitties = Vec::new();
				new_kitties.push(dna_random.clone());
				<OwnersKitty<T>>::insert(who.clone(), new_kitties);
			}

			//Emit an event.
			Self::deposit_event(Event::KittyStored(dna_random, price));

			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		#[pallet::weight(32_005_000 + T::DbWeight::get().reads_writes(3,3))]
		pub fn transfer_kitty(
			origin: OriginFor<T>,
			dna: Vec<u8>,
			_to_account: T::AccountId,
		) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins

			// Account extrinsics
			let _who = ensure_signed(origin)?;

			let account_transfer_kitty_number = <OwnersKitty<T>>::get(&_to_account).len();
			ensure!(
				account_transfer_kitty_number < T::MaxKitty::get() as usize,
				Error::<T>::LimitKitty
			);

			// Check exist dna kitty
			let mut _check_exist_kitty = <Kitties<T>>::get(&dna);

			// not exist dna kitty then error
			ensure!(_check_exist_kitty.is_some(), Error::<T>::NotExistKitty);

			// Get kitty by dna
			let _kitty_by_dna = _check_exist_kitty.unwrap();

			// Update add dna's kitty for the account being transferred
			<OwnersKitty<T>>::mutate(&_to_account, |_kitties_vec| {
				_kitties_vec.push(dna.clone());
			});

			// Update remove dna's kitty for the account transferred
			<OwnersKitty<T>>::mutate(&_kitty_by_dna.owner, |_kitties_vec| {
				_kitties_vec.retain(|kitty_dna| kitty_dna != &dna);
			});

			// Update new owner of kitty
			<Kitties<T>>::mutate(&dna, |_kitty| match _kitty {
				Some(kitty) => kitty.owner = _to_account.clone(),
				None => {},
			});

			//Emit an event.
			Self::deposit_event(Event::TransferKitty(dna, _to_account));

			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
	}
}

impl<T: Config> Pallet<T> {
	pub fn gen_gender(dna: &Vec<u8>) -> Result<Gender, Error<T>> {
		let mut res = Gender::Female;

		if dna.len() % 2 == 0 {
			res = Gender::Male;
		}

		Ok(res)
	}

	pub fn generate_random_adn(hash: &u32) -> T::Hash {
		let encode_hash = hash.encode();
		let (random_value, _) = T::DnaRandomness::random(&encode_hash);
		random_value
	}
}
