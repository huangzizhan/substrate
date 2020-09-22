// This file is part of Substrate.

// Copyright (C) 2020 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use frame_support::{
	RuntimeDebug,
	dispatch::Parameter,
	traits::Get,
	weights::Weight,
};

/// A provider of the MMR's leaf data.
pub trait LeafDataProvider {
	/// A type that should end up in the leaf of MMR.
	type LeafData: Parameter;

	/// The method to return leaf data that should be placed
	/// in the leaf node appended MMR at this block.
	///
	/// This is being called by the `on_initialize` method of
	/// this pallet at the very beginning of each block.
	/// The second return value should indicate how much [Weight]
	/// was required to retrieve the data.
	fn leaf_data() -> (Self::LeafData, Weight);
}

impl LeafDataProvider for () {
	type LeafData = ();

	fn leaf_data() -> (Self::LeafData, Weight) {
		((), 0)
	}
}

impl<T: frame_system::Trait> LeafDataProvider for frame_system::Module<T> {
	type LeafData = T::Hash;

	fn leaf_data() -> (Self::LeafData, Weight) {
		let hash = Self::parent_hash();
		(hash, T::DbWeight::get().reads(1))
	}
}

// TODO [ToDr] Impl for all tuples
impl<A: LeafDataProvider, B: LeafDataProvider> LeafDataProvider for (A, B) {
	type LeafData = (A::LeafData, B::LeafData);

	fn leaf_data() -> (Self::LeafData, Weight) {
		let (a_leaf, a_weight) = A::leaf_data();
		let (b_leaf, b_weight) = B::leaf_data();

		((a_leaf, b_leaf), a_weight.saturating_add(b_weight))
	}
}

/// A MMR proof data for one of the leaves.
#[derive(codec::Encode, codec::Decode, RuntimeDebug, Clone, PartialEq, Eq)]
pub struct Proof<Hash> {
	/// The index of the leaf the proof is for.
	pub leaf_index: u64,
	/// Number of leaves in MMR, when the proof was generated.
	pub leaf_count: u64,
	/// Proof elements (hashes of inner nodes on the path to the leaf).
	pub items: Vec<Hash>,
}
