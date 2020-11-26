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

//! Implements support for the pallet_generic_assets module.

use sp_runtime::traits::{AtLeast32Bit, Member, AtLeast32BitUnsigned, MaybeSerializeDeserialize};
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

/// The subset of the `pallet_generic_asset::Trait` that a client must implement.
#[module]
pub trait GenericAsset: System {
    /// This type represents the balance in GenericAssets
    type Balance: Parameter + Member + AtLeast32BitUnsigned + Default + Copy + Debug +
    MaybeSerializeDeserialize;
    /// The type used to represent different assets
    type AssetId: Parameter + Member + AtLeast32Bit + Default + Copy;
}

/// Create a new asset
///
/// `transfer` will set the `FreeBalance` of the sender and receiver.
/// It will decrease the total issuance of the system by the `TransferFee`.
/// If the sender's account is below the existential deposit as a result
/// of the transfer, the account will be reaped.
#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct CreateCall< T: GenericAsset> {
    /// Asset Create Options
    pub options: AssetOptions<T::Balance,T::AccountId>
}

/// Transfer some liquid free balance to another account.
///
/// `transfer` will set the `FreeBalance` of the sender and receiver.
/// It will decrease the total issuance of the system by the `TransferFee`.
/// If the sender's account is below the existential deposit as a result
/// of the transfer, the account will be reaped.
#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct TransferCall<'a, T: GenericAsset> {
    /// AssetId to transfer
    #[codec(compact)]
    pub asset_id: T::AssetId,
    /// Destination of the transfer.
    pub to: &'a <T as System>::Address,
    /// Amount to transfer.
    #[codec(compact)]
    pub amount: T::Balance,
}

/// Transfer event.
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct TransferEvent<T: GenericAsset> {
    /// AssetId to transfer
    pub asset_id: T::AssetId,
    /// Account balance was transfered from.
    pub from: <T as System>::AccountId,
    /// Account balance was transfered to.
    pub to: <T as System>::AccountId,
    /// Amount of balance that was transfered.
    pub amount: T::Balance,
}


/// Asset creation options.
#[derive(Clone, Encode, Decode, PartialEq, Eq, Debug)]
pub struct AssetOptions<Balance, AccountId> {
    /// Initial issuance of this asset. All deposit to the creator of the asset.
    #[codec(compact)]
    pub initial_issuance: Balance,
    /// Which accounts are allowed to possess this asset.
    pub permissions: PermissionLatest<AccountId>,
}

/// Asset permissions
#[derive(Clone, Encode, Decode, PartialEq, Eq, Debug)]
pub struct PermissionsV1<AccountId> {
    /// Who have permission to update asset permission
    pub update: Owner<AccountId>,
    /// Who have permission to mint new asset
    pub mint: Owner<AccountId>,
    /// Who have permission to burn asset
    pub burn: Owner<AccountId>,
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, Debug)]
#[repr(u8)]
/// Versioned asset permission
pub enum PermissionVersionNumber {
    /// Versioned asset permission
    V1 = 0,
}

/// Versioned asset permission
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum PermissionVersions<AccountId> {
    /// Versioned asset permission
    V1(PermissionsV1<AccountId>),
}

/// Asset permission types
pub enum PermissionType {
    /// Permission to burn asset permission
    Burn,
    /// Permission to mint new asset
    Mint,
    /// Permission to update asset
    Update,
}

/// Alias to latest asset permissions
pub type PermissionLatest<AccountId> = PermissionsV1<AccountId>;

/// Owner of an asset.
#[derive(Clone, Encode, Decode, PartialEq, Eq, Debug)]
pub enum Owner<AccountId> {
    /// No owner.
    None,
    /// Owned by an AccountId
    Address(AccountId),
}