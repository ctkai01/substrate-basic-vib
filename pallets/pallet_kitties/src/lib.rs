#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

use frame_support::inherent::Vec;
use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use log;
#[frame_support::pallet]
pub mod pallet {
	use core::fmt::Debug;

	pub use super::*;

	#[derive(TypeInfo, Default, Encode, Decode, Debug, Clone)]
	#[scale_info(skip_type_params(T))]

	pub struct Kitty<T: Config> {
		dna: Vec<u8>,
		owner: T::AccountId,
		price: u32,
		gender: Gender,
	}

	#[derive(TypeInfo, Encode, Decode, Debug, Clone)]

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
	}
	pub trait Test {}
	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_kitty(origin: OriginFor<T>, dna: Vec<u8>, price: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;
			let gender = Self::gen_gender(dna.clone())?;

			let kitty = Kitty { dna: dna.clone(), gender, price, owner: who.clone() };
			let mut kitties_count = KittiesNumber::<T>::get();
			kitties_count += 1;

			KittiesNumber::<T>::put(kitties_count);

			<Kitties<T>>::insert(dna.clone(), kitty);

			let check_exist_owner = <OwnersKitty<T>>::contains_key(who.clone());
			if check_exist_owner {
				<OwnersKitty<T>>::mutate(who.clone(), |_kitties_vec| {
					_kitties_vec.push(dna.clone());
				})
			} else {
				let mut new_kitties = Vec::new();
				new_kitties.push(dna.clone());
				<OwnersKitty<T>>::insert(who.clone(), new_kitties);
			}

			//Emit an event.
			Self::deposit_event(Event::KittyStored(dna, price));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn transfer_kitty(
			origin: OriginFor<T>,
			dna: Vec<u8>,
			_to_account: T::AccountId,
		) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let _who = ensure_signed(origin)?;

			let mut _check_exist_kitty = <Kitties<T>>::get(&dna);
			ensure!(_check_exist_kitty.is_some(), Error::<T>::NotExistKitty);

			let _kitty_by_dna = _check_exist_kitty.unwrap();

			<OwnersKitty::<T>>::mutate(&_to_account, |_kitties_vec| {
				_kitties_vec.push(dna.clone());
			});

			<OwnersKitty::<T>>::mutate(&_kitty_by_dna.owner, |_kitties_vec| {
				_kitties_vec.retain(|kitty_dna| {
					kitty_dna != &dna
				});
			});

			<Kitties::<T>>::mutate(&dna, |_kitty| {
				match _kitty {
					Some(kitty) => kitty.owner = _to_account.clone(),
					None => {}
				}
			});

			
			//Emit an event.
			Self::deposit_event(Event::TransferKitty(dna, _to_account));

			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
	}
}

impl<T> Pallet<T> {
	fn gen_gender(dna: Vec<u8>) -> Result<Gender, Error<T>> {
		let mut res = Gender::Female;

		if dna.len() % 2 == 0 {
			res = Gender::Male;
		}

		Ok(res)
	}
}
