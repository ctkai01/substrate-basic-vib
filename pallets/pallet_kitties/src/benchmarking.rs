//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as KittiesPallet;
use frame_benchmarking::{benchmarks, whitelisted_caller, account};
use frame_system::RawOrigin;

benchmarks! {
	transfer_kitty {
		
		let caller: T::AccountId = whitelisted_caller();
		let receiver: T::AccountId = account("receiver", 0, 0);
		let price = 100;
		KittiesPallet::<T>::create_kitty(RawOrigin::Signed(caller.clone()).into(), price);
	
		let dna_kitties_caller = <OwnersKitty<T>>::get(&caller);
		let dna_kitty_created = &dna_kitties_caller[0];
		let count_kitties_caller = dna_kitties_caller.clone().len();
		let count_kitties_receiver = <OwnersKitty<T>>::get(&receiver).len();
	}: transfer_kitty(RawOrigin::Signed(caller.clone()), dna_kitty_created.clone(), receiver)
	verify {
		assert_eq!(KittiesNumber::<T>::get(), 1);
		assert_eq!(count_kitties_caller - 1, 0);
		assert_eq!(count_kitties_receiver + 1, 1);
	}

	impl_benchmark_test_suite!(KittiesPallet, crate::mock::new_test_ext(), crate::mock::Test);
}
