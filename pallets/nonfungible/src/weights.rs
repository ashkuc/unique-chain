// Template adopted from https://github.com/paritytech/substrate/blob/master/.maintain/frame-weight-template.hbs

//! Autogenerated weights for pallet_nonfungible
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-10-21, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 128

// Executed Command:
// target/release/nft
// benchmark
// --pallet
// pallet-nonfungible
// --wasm-execution
// compiled
// --extrinsic
// *
// --template
// .maintain/frame-weight-template.hbs
// --steps=50
// --repeat=20
// --output=./pallets/nonfungible/src/weights.rs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_nonfungible.
pub trait WeightInfo {
	fn create_item() -> Weight;
	fn create_multiple_items(b: u32, ) -> Weight;
	fn burn_item() -> Weight;
	fn transfer() -> Weight;
	fn approve() -> Weight;
	fn transfer_from() -> Weight;
	fn set_variable_metadata(b: u32, ) -> Weight;
}

/// Weights for pallet_nonfungible using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Nonfungible TokensMinted (r:1 w:1)
	// Storage: Nonfungible AccountBalance (r:1 w:1)
	// Storage: Nonfungible TokenData (r:0 w:1)
	// Storage: Nonfungible Owned (r:0 w:1)
	fn create_item() -> Weight {
		(16_902_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Nonfungible TokensMinted (r:1 w:1)
	// Storage: Nonfungible AccountBalance (r:1 w:1)
	// Storage: Nonfungible TokenData (r:0 w:4)
	// Storage: Nonfungible Owned (r:0 w:4)
	fn create_multiple_items(b: u32, ) -> Weight {
		(15_860_000 as Weight)
			// Standard Error: 5_000
			.saturating_add((3_916_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			.saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(b as Weight)))
	}
	// Storage: Nonfungible TokenData (r:1 w:1)
	// Storage: Nonfungible TokensBurnt (r:1 w:1)
	// Storage: Nonfungible Allowance (r:1 w:0)
	// Storage: Nonfungible Owned (r:0 w:1)
	fn burn_item() -> Weight {
		(17_966_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Nonfungible TokenData (r:1 w:1)
	// Storage: Nonfungible AccountBalance (r:2 w:2)
	// Storage: Nonfungible Allowance (r:1 w:0)
	// Storage: Nonfungible Owned (r:0 w:2)
	fn transfer() -> Weight {
		(23_886_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: Nonfungible TokenData (r:1 w:0)
	// Storage: Nonfungible Allowance (r:1 w:1)
	fn approve() -> Weight {
		(14_697_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Nonfungible Allowance (r:1 w:1)
	// Storage: Nonfungible TokenData (r:1 w:1)
	// Storage: Nonfungible AccountBalance (r:2 w:2)
	// Storage: Nonfungible Owned (r:0 w:2)
	fn transfer_from() -> Weight {
		(28_001_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	// Storage: Nonfungible TokenData (r:1 w:1)
	fn set_variable_metadata(_b: u32, ) -> Weight {
		(6_380_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Nonfungible TokensMinted (r:1 w:1)
	// Storage: Nonfungible AccountBalance (r:1 w:1)
	// Storage: Nonfungible TokenData (r:0 w:1)
	// Storage: Nonfungible Owned (r:0 w:1)
	fn create_item() -> Weight {
		(16_902_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: Nonfungible TokensMinted (r:1 w:1)
	// Storage: Nonfungible AccountBalance (r:1 w:1)
	// Storage: Nonfungible TokenData (r:0 w:4)
	// Storage: Nonfungible Owned (r:0 w:4)
	fn create_multiple_items(b: u32, ) -> Weight {
		(15_860_000 as Weight)
			// Standard Error: 5_000
			.saturating_add((3_916_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes((2 as Weight).saturating_mul(b as Weight)))
	}
	// Storage: Nonfungible TokenData (r:1 w:1)
	// Storage: Nonfungible TokensBurnt (r:1 w:1)
	// Storage: Nonfungible Allowance (r:1 w:0)
	// Storage: Nonfungible Owned (r:0 w:1)
	fn burn_item() -> Weight {
		(17_966_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: Nonfungible TokenData (r:1 w:1)
	// Storage: Nonfungible AccountBalance (r:2 w:2)
	// Storage: Nonfungible Allowance (r:1 w:0)
	// Storage: Nonfungible Owned (r:0 w:2)
	fn transfer() -> Weight {
		(23_886_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	// Storage: Nonfungible TokenData (r:1 w:0)
	// Storage: Nonfungible Allowance (r:1 w:1)
	fn approve() -> Weight {
		(14_697_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Nonfungible Allowance (r:1 w:1)
	// Storage: Nonfungible TokenData (r:1 w:1)
	// Storage: Nonfungible AccountBalance (r:2 w:2)
	// Storage: Nonfungible Owned (r:0 w:2)
	fn transfer_from() -> Weight {
		(28_001_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(6 as Weight))
	}
	// Storage: Nonfungible TokenData (r:1 w:1)
	fn set_variable_metadata(_b: u32, ) -> Weight {
		(6_380_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
}