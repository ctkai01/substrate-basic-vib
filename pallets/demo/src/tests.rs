use crate::{mock::*, Error};

use frame_support::{assert_noop, assert_ok};
use frame_system::pallet_prelude::*;

#[test]
fn check_error_young_create_student() {
	let name = String::from("Nam").as_bytes().to_vec();
	let age = 20;

	new_test_ext().execute_with(|| {
		assert_noop!(
			DemoModule::create_student(Origin::signed(1), name, age),
			Error::<Test>::TooYoung
		);
	})
}

#[test]
fn check_current_id_after_create_student() {
	let name = String::from("Nam").as_bytes().to_vec();
	let age = 21;
	new_test_ext().execute_with(|| {
		let current_id = DemoModule::student_id();
		assert_ok!(DemoModule::create_student(Origin::signed(1), name, age));
		assert_eq!(DemoModule::student_id(), current_id + 1);
	})
}

#[test]
fn inserted_new_student() {
	let name = String::from("Nam").as_bytes().to_vec();
	let age = 21;

	new_test_ext().execute_with(|| {
		let current_id = DemoModule::student_id();
		let account = ensure_signed(Origin::signed(1)).ok().unwrap();
		assert_ok!(DemoModule::create_student(Origin::signed(1), name.clone(), age));
		let student_after_created = DemoModule::student(current_id + 1).unwrap();
		assert_eq!(student_after_created.account, account);
		assert_eq!(student_after_created.age, age);
		assert_eq!(student_after_created.name, name);
	})
}

// #[test]
// fn check_event_create_student() {
// 	let name = String::from("Nam").as_bytes().to_vec();
// 	let age = 21;
// 	new_test_ext().execute_with(|| {
// 		assert_ok!(DemoModule::create_student(Origin::signed(1), name.clone(), age));
// 		// System::assert_last_event(mock::Event::DemoModule(Event::StudentStored(crate::Event::StudentStored(name, age)));
// 		// System::assert_last_event(mock::Event::DemoModule(Event::<Test>::StudentStored(c)));
// 		// System::assert_last_event(mock::Event::DemoModule(Event::<Test>::StudentStored(name, age)));
// 	})
// }
