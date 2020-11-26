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

use frame_support::sp_std::fmt::Debug;
use crate::frame::system::{
    System,
    SystemEventsDecoder,
};
use codec::{
    Decode,
    Encode,
};

use crate::frame::generic_asset::GenericAsset;
use crate::frame::generic_asset::GenericAssetEventsDecoder;

/// The subset of the `polkadex::Trait` that a client must implement.
#[module]
pub trait Polkadex: System  + GenericAsset{}

/// Creates a new orderbook for given assets
#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct RegisterNewOrderbookCall<T: Polkadex>{
    /// Type representing Quote Asset
    pub quote_asset_id: T::AssetId,
    /// Type representing Base Asset
    pub base_asset_id: T::AssetId
}

/// Submits a new trade
#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct SubmitOrder<T: Polkadex>{
    /// Trade Type
    pub order_type: OrderType,
    /// Market Id
    pub trading_pair: T::Hash,
    /// Trade Price
    pub price: T::Balance,
    /// Trade Quantity
    pub quantity: T::Balance,
}

/// Available Order types
#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode)]
pub enum OrderType {
    /// Limit Buy Order
    BidLimit,
    /// Market Buy Order
    BidMarket,
    /// Limit Sell Order
    AskLimit,
    /// Market Sell Order
    AskMarket,
}