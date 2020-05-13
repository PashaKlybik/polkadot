// Copyright 2017-2020 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Tests to make sure that Polkadot's weights and fees match what we
//! expect from Substrate.
//!
//! These test are not meant to be exhaustive, as it is inevitable that
//! weights in Substrate will change. Instead they are supposed to provide
//! some sort of indicator that calls we consider important (e.g Balances::transfer)
//! have not suddenly changed from under us.
//!
//! NOTE: All the tests assume RocksDB as the RuntimeDbWeight type
//! which gives us the following weights:
//!  - Read: 25 * WEIGHT_PER_MICROS = 25 * 100_000_000,
//!  - Write: 100 * WEIGHT_PER_MICROS = 25 * 100_000_000,

use frame_support::weights::{constants::*, GetDispatchInfo};
use keyring::AccountKeyring;
use polkadot_runtime::constants::currency::*;
use polkadot_runtime::{self, Runtime};
use primitives::AccountId;
use runtime_common::MaximumBlockWeight;

use democracy::Call as DemocracyCall;
use elections_phragmen::Call as PhragmenCall;
use session::Call as SessionCall;
use staking::Call as StakingCall;
use system::Call as SystemCall;
use treasury::Call as TreasuryCall;

#[test]
fn sanity_check_weight_per_second_is_as_expected() {
	// This value comes from Substrate, we want to make sure that if it
	// ever changes we don't accidently break Polkadot
	assert_eq!(WEIGHT_PER_SECOND, 1_000_000_000_000)
}

#[test]
fn sanity_check_weight_per_milli_is_as_expected() {
	// This value comes from Substrate, we want to make sure that if it
	// ever changes we don't accidently break Polkadot
	assert_eq!(WEIGHT_PER_MILLIS, 1_000_000_000)
}

#[test]
fn sanity_check_weight_per_micros_is_as_expected() {
	// This value comes from Substrate, we want to make sure that if it
	// ever changes we don't accidently break Polkadot
	assert_eq!(WEIGHT_PER_MICROS, 1_000_000)
}

#[test]
fn sanity_check_weight_per_nanos_is_as_expected() {
	// This value comes from Substrate, we want to make sure that if it
	// ever changes we don't accidently break Polkadot
	assert_eq!(WEIGHT_PER_NANOS, 1_000)
}

#[test]
fn weight_of_balances_transfer_is_correct() {
	// #[weight = T::DbWeight::get().reads_writes(1, 1) + 70_000_000]
	let expected_weight = 195_000_000;

	let weight = polkadot_runtime::BalancesCall::transfer::<Runtime>(Default::default(), Default::default())
		.get_dispatch_info()
		.weight;
	assert_eq!(weight, expected_weight);
}

#[test]
fn weight_of_balances_set_balance_is_correct() {
	// #[weight = T::DbWeight::get().reads_writes(1, 1) + 35_000_000]
	let expected_weight = 160_000_000;

	let weight = polkadot_runtime::BalancesCall::set_balance::<Runtime>(
		Default::default(),
		Default::default(),
		Default::default(),
	)
	.get_dispatch_info()
	.weight;

	assert_eq!(weight, expected_weight);
}

#[test]
fn weight_of_balances_force_transfer_is_correct() {
	// #[weight = T::DbWeight::get().reads_writes(2, 2) + 70_000_000]
	let expected_weight = 320_000_000;

	let weight = polkadot_runtime::BalancesCall::force_transfer::<Runtime>(
		Default::default(),
		Default::default(),
		Default::default(),
	)
	.get_dispatch_info()
	.weight;

	assert_eq!(weight, expected_weight);
}

#[test]
fn weight_of_balances_transfer_keep_alive_is_correct() {
	// #[weight = T::DbWeight::get().reads_writes(1, 1) + 50_000_000]
	let expected_weight = 175_000_000;

	let weight = polkadot_runtime::BalancesCall::transfer_keep_alive::<Runtime>(Default::default(), Default::default())
		.get_dispatch_info()
		.weight;

	assert_eq!(weight, expected_weight);
}

#[test]
fn weight_of_timestap_set_is_correct() {
	// #[weight = T::DbWeight::get().reads_writes(2, 1) + 9_000_000]
	let expected_weight = 159_000_000;
	let weight = polkadot_runtime::TimestampCall::set::<Runtime>(Default::default()).get_dispatch_info().weight;

	assert_eq!(weight, expected_weight);
}

#[test]
fn weight_of_staking_bond_is_correct() {
	let controller: AccountId = AccountKeyring::Alice.into();

	// #[weight = 500_000_000]
	let expected_weight = 500_000_000;
	let weight = StakingCall::bond::<Runtime>(controller, 1 * DOLLARS, Default::default()).get_dispatch_info().weight;

	assert_eq!(weight, expected_weight);
}

#[test]
fn weight_of_staking_bond_extra_is_correct() {
	// #[weight = 500_000_000]
	let expected_weight = 500_000_000;
	let weight = StakingCall::bond_extra::<Runtime>(1 * DOLLARS).get_dispatch_info().weight;

	assert_eq!(weight, expected_weight);
}

#[test]
fn weight_of_staking_unbond_is_correct() {
	// #[weight = 400_000_000]
	let expected_weight = 400_000_000;
	let weight = StakingCall::unbond::<Runtime>(Default::default()).get_dispatch_info().weight;

	assert_eq!(weight, expected_weight);
}

#[test]
fn weight_of_staking_widthdraw_unbonded_is_correct() {
	// #[weight = 400_000_000]
	let expected_weight = 400_000_000;
	let weight = StakingCall::withdraw_unbonded::<Runtime>().get_dispatch_info().weight;

	assert_eq!(weight, expected_weight);
}

#[test]
fn weight_of_staking_validate_is_correct() {
	// #[weight = 750_000_000]
	let expected_weight = 750_000_000;
	let weight = StakingCall::validate::<Runtime>(Default::default()).get_dispatch_info().weight;

	assert_eq!(weight, expected_weight);
}

#[test]
fn weight_of_staking_nominate_is_correct() {
	// #[weight = 750_000_000]
	let expected_weight = 750_000_000;
	let weight = StakingCall::nominate::<Runtime>(vec![]).get_dispatch_info().weight;

	assert_eq!(weight, expected_weight);
}

#[test]
fn weight_of_system_set_code_is_correct() {
	// #[weight = (T::MaximumBlockWeight::get(), DispatchClass::Operational)]
	let expected_weight = MaximumBlockWeight::get();
	let weight = SystemCall::set_code::<Runtime>(vec![]).get_dispatch_info().weight;

	assert_eq!(weight, expected_weight);
}

#[test]
fn weight_of_system_set_code_without_checks_is_correct() {
	// #[weight = (T::MaximumBlockWeight::get(), DispatchClass::Operational)]
	let expected_weight = MaximumBlockWeight::get();
	let weight = SystemCall::set_code_without_checks::<Runtime>(vec![]).get_dispatch_info().weight;

	assert_eq!(weight, expected_weight);
}

#[test]
fn weight_of_system_set_storage_is_correct() {
	let storage_items = vec![(vec![12], vec![34])];

	// #[weight = FunctionOf(
	// 	|(items,): (&Vec<KeyValue>,)| {
	// 		T::DbWeight::get().writes(items.len() as Weight)
	// 			.saturating_add((items.len() as Weight).saturating_mul(600_000))
	// 	},
	// 	DispatchClass::Operational,
	// 	Pays::Yes,
	// )]
	let expected_weight = 100_600_000;
	let weight = SystemCall::set_storage::<Runtime>(storage_items).get_dispatch_info().weight;

	assert_eq!(weight, expected_weight);
}

#[test]
fn weight_of_system_remark_is_correct() {
	// #[weight = 700_000]
	let expected_weight = 700_000;
	let weight = SystemCall::remark::<Runtime>(vec![]).get_dispatch_info().weight;

	assert_eq!(weight, expected_weight);
}

#[test]
fn weight_of_session_set_keys_is_correct() {
	// #[weight = 200_000_000
	// 	+ T::DbWeight::get().reads(2 + T::Keys::key_ids().len() as Weight)
	// 	+ T::DbWeight::get().writes(1 + T::Keys::key_ids().len() as Weight)]
	//
	// Polkadot has five possible session keys, so we default to key_ids.len() = 5
	let expected_weight = 975_000_000;
	let weight = SessionCall::set_keys::<Runtime>(Default::default(), Default::default()).get_dispatch_info().weight;

	assert_eq!(weight, expected_weight);
}

#[test]
fn weight_of_session_purge_keys_is_correct() {
	// #[weight = 120_000_000
	// 	+ T::DbWeight::get().reads_writes(2, 1 + T::Keys::key_ids().len() as Weight)]
	//
	// Polkadot has five possible session keys, so we default to key_ids.len() = 5
	let expected_weight = 770_000_000;
	let weight = SessionCall::purge_keys::<Runtime>().get_dispatch_info().weight;

	assert_eq!(weight, expected_weight);
}

#[test]
fn weight_of_democracy_propose_is_correct() {
	// #[weight = 5_000_000_000]
	let expected_weight = 5_000_000_000;
	let weight = DemocracyCall::propose::<Runtime>(Default::default(), Default::default()).get_dispatch_info().weight;

	assert_eq!(weight, expected_weight);
}

#[test]
fn weight_of_democracy_vote_is_correct() {
	use democracy::AccountVote;
	let vote = AccountVote::Standard { vote: Default::default(), balance: Default::default() };

	// #[weight = 200_000_000]
	let expected_weight = 200_000_000;
	let weight = DemocracyCall::vote::<Runtime>(Default::default(), vote).get_dispatch_info().weight;

	assert_eq!(weight, expected_weight);
}

#[test]
fn weight_of_democracy_enact_proposal_is_correct() {
	// #[weight = T::MaximumBlockWeight::get()]
	let expected_weight = MaximumBlockWeight::get();
	let weight =
		DemocracyCall::enact_proposal::<Runtime>(Default::default(), Default::default()).get_dispatch_info().weight;

	assert_eq!(weight, expected_weight);
}

#[test]
fn weight_of_phragment_vote_is_correct() {
	// #[weight = 100_000_000]
	let expected_weight = 100_000_000;
	let weight = PhragmenCall::vote::<Runtime>(Default::default(), Default::default()).get_dispatch_info().weight;

	assert_eq!(weight, expected_weight);
}

#[test]
fn weight_of_phragment_submit_candidacy_is_correct() {
	// #[weight = 500_000_000]
	let expected_weight = 500_000_000;
	let weight = PhragmenCall::submit_candidacy::<Runtime>().get_dispatch_info().weight;

	assert_eq!(weight, expected_weight);
}

#[test]
fn weight_of_phragment_renounce_candidacy_is_correct() {
	// #[weight = (2_000_000_000, DispatchClass::Operational)]
	let expected_weight = 2_000_000_000;
	let weight = PhragmenCall::renounce_candidacy::<Runtime>().get_dispatch_info().weight;

	assert_eq!(weight, expected_weight);
}

#[test]
fn weight_of_treasury_propose_spend_is_correct() {
	// #[weight = 120_000_000 + T::DbWeight::get().reads_writes(1, 2)]
	let expected_weight = 345_000_000;
	let weight =
		TreasuryCall::propose_spend::<Runtime>(Default::default(), Default::default()).get_dispatch_info().weight;

	assert_eq!(weight, expected_weight);
}

#[test]
fn weight_of_treasury_approve_proposal_is_correct() {
	// #[weight = (34_000_000 + T::DbWeight::get().reads_writes(2, 1), DispatchClass::Operational)]
	let expected_weight = 184_000_000;
	let weight = TreasuryCall::approve_proposal::<Runtime>(Default::default()).get_dispatch_info().weight;

	assert_eq!(weight, expected_weight);
}

#[test]
fn weight_of_treasury_tip_is_correct() {
	// #[weight = 68_000_000 + 2_000_000 * T::Tippers::max_len() as Weight
	// 	+ T::DbWeight::get().reads_writes(2, 1)]
	let expected_weight = 244_000_000;
	let weight = TreasuryCall::tip::<Runtime>(Default::default(), Default::default()).get_dispatch_info().weight;

	assert_eq!(weight, expected_weight);
}