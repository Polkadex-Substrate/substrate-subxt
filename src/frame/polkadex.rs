// Copyright 2019-2020 Parity Technologies (UK) Ltd.
// This file is part of substrate-subxt.
//
// subxt is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// subxt is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with substrate-subxt.  If not, see <http://www.gnu.org/licenses/>.

//! Implements support for the polkadex module.

use sp_runtime::traits::{AtLeast32Bit, MaybeSerialize, Member, AtLeast32BitUnsigned, MaybeSerializeDeserialize};
use core::marker::PhantomData;
use frame_support::Parameter;
use frame_support::sp_std::fmt::Debug;
use crate::frame::system::{
    System,
    SystemEventsDecoder,
};
use codec::{
    Decode,
    Encode,
};

/// The subset of the `polkadex::Trait` that a client must implement.
#[module]
pub trait Trait: frame_system::Trait + pallet_generic_asset::Trait {}

/// Creates a new orderbook for given assets
#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct RegisterNewOrderbookCall{
    pub quote_asset_id: u32,
    pub base_asset_id: u32
}