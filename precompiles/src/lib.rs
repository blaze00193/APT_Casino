// This file is part of Web3Games.

// Copyright (C) 2021-2022 Web3Games https://web3games.org
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

#![cfg_attr(not(feature = "std"), no_std)]

use codec::Decode;
use frame_support::dispatch::{Dispatchable, GetDispatchInfo, PostDispatchInfo};
use pallet_evm::{Precompile, PrecompileHandle, PrecompileResult, PrecompileSet};
use pallet_evm_precompile_bn128::{Bn128Add, Bn128Mul, Bn128Pairing};
use pallet_evm_precompile_dispatch::Dispatch;
use pallet_evm_precompile_modexp::Modexp;
use pallet_evm_precompile_sha3fips::Sha3FIPS256;
use pallet_evm_precompile_simple::{ECRecover, ECRecoverPublicKey, Identity, Ripemd160, Sha256};
use pallet_support::AccountMapping;
use sp_core::H160;
use sp_std::{marker::PhantomData, prelude::*};

mod exchange;
mod marketplace;
mod token_fungible;
mod token_multi;
mod token_non_fungible;

pub use exchange::ExchangeExtension;
pub use marketplace::MarketplaceExtension;
pub use token_fungible::FungibleTokenExtension;
pub use token_multi::MultiTokenExtension;
pub use token_non_fungible::NonFungibleTokenExtension;

/// Function Selector of "create": 0x42ecabc0
pub const TOKEN_FUNGIBLE_CREATE_SELECTOR: &[u8] = &[66u8, 236u8, 171u8, 192u8];

/// Function Selector of "create": 0xe9d0638d //233,208,99,141
pub const TOKEN_NON_FUNGIBLE_CREATE_SELECTOR: &[u8] = &[233u8, 208u8, 99u8, 141u8];

/// Function Selector of "create": 0xcf5ba53f //207,91,165,63
pub const TOKEN_MULTI_CREATE_SELECTOR: &[u8] = &[207u8, 91u8, 165u8, 63u8];

/// Fungible Token prefix with 0xFFFFFFFF.
pub const FT_PRECOMPILE_ADDRESS_PREFIX: &[u8] = &[255u8; 4];

/// Non Fungible Token prefix with 0xFEFFFFFF.
pub const NFT_PRECOMPILE_ADDRESS_PREFIX: &[u8] = &[254u8, 255u8, 255u8, 255u8];

/// Multi Token prefix with 0xFDFFFFFF.
pub const MT_PRECOMPILE_ADDRESS_PREFIX: &[u8] = &[253u8, 255u8, 255u8, 255u8];

#[derive(Debug, Clone, Copy)]
pub struct Web3GamesPrecompiles<R>(PhantomData<R>);

impl<R> Web3GamesPrecompiles<R>
where
	R: pallet_evm::Config,
{
	pub fn new() -> Self {
		Self(Default::default())
	}
	pub fn used_addresses() -> sp_std::vec::Vec<H160> {
		sp_std::vec![1, 2, 3, 4, 5, 6, 7, 8, 1024, 1025, 1026]
			.into_iter()
			.map(|x| hash(x))
			.collect()
	}
}

impl<R> PrecompileSet for Web3GamesPrecompiles<R>
where
	R::Call: Dispatchable<PostInfo = PostDispatchInfo> + GetDispatchInfo + Decode,
	<R::Call as Dispatchable>::Origin: From<Option<R::AccountId>>,
	R: pallet_evm::Config
		+ pallet_token_fungible::Config
		+ pallet_token_non_fungible::Config
		+ pallet_token_multi::Config
		+ pallet_exchange::Config
		+ pallet_marketplace::Config,
	R::Call: From<pallet_token_fungible::Call<R>>,
	R::Call: From<pallet_token_non_fungible::Call<R>>,
	R::Call: From<pallet_token_multi::Call<R>>,
	R::Call: From<pallet_exchange::Call<R>>,
	R::Call: From<pallet_marketplace::Call<R>>,
	<R as pallet_token_fungible::Config>::FungibleTokenId: From<u128> + Into<u128>,
	<R as pallet_exchange::Config>::PoolId: From<u128> + Into<u128>,
	<R as pallet_token_non_fungible::Config>::NonFungibleTokenId: From<u128> + Into<u128>,
	<R as pallet_token_non_fungible::Config>::TokenId: From<u128> + Into<u128>,
	<R as pallet_token_multi::Config>::MultiTokenId: From<u128> + Into<u128>,
	<R as pallet_token_multi::Config>::TokenId: From<u128> + Into<u128>,
	R: AccountMapping<R::AccountId>,
{
	fn execute(&self, handle: &mut impl PrecompileHandle) -> Option<PrecompileResult> {
		match handle.code_address() {
			// Ethereum precompiles :
			a if a == hash(1) => Some(ECRecover::execute(handle)),
			a if a == hash(2) => Some(Sha256::execute(handle)),
			a if a == hash(3) => Some(Ripemd160::execute(handle)),
			a if a == hash(4) => Some(Identity::execute(handle)),
			a if a == hash(5) => Some(Modexp::execute(handle)),
			a if a == hash(6) => Some(Bn128Add::execute(handle)),
			a if a == hash(7) => Some(Bn128Mul::execute(handle)),
			a if a == hash(8) => Some(Bn128Pairing::execute(handle)),

			// Non-Web3Games specific nor Ethereum precompiles
			a if a == hash(1024) => Some(Sha3FIPS256::execute(handle)),
			a if a == hash(1025) => Some(Dispatch::<R>::execute(handle)),
			a if a == hash(1026) => Some(ECRecoverPublicKey::execute(handle)),

			// Web3Games precompiles
			a if a == hash(1027) => ExchangeExtension::<R>::new().execute(handle),
			a if a == hash(1028) => MarketplaceExtension::<R>::new().execute(handle),
			a if &a.to_fixed_bytes()[0..4] == FT_PRECOMPILE_ADDRESS_PREFIX =>
			// Some(<FungibleTokenExtension<R> as Precompile>::execute(handle)),
				FungibleTokenExtension::<R>::new().execute(handle),
			a if &a.to_fixed_bytes()[0..4] == NFT_PRECOMPILE_ADDRESS_PREFIX =>
			// Some(<NonFungibleTokenExtension<R> as Precompile>::execute(handle)),
				NonFungibleTokenExtension::<R>::new().execute(handle),
			a if &a.to_fixed_bytes()[0..4] == MT_PRECOMPILE_ADDRESS_PREFIX =>
				MultiTokenExtension::<R>::new().execute(handle),

			// Not support
			_ => None,
		}
	}

	fn is_precompile(&self, address: H160) -> bool {
		Self::used_addresses().contains(&address)
	}
}

pub fn hash(a: u64) -> H160 {
	H160::from_low_u64_be(a)
}
