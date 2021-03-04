#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Encode, Decode};

use frame_support::traits::Randomness;
use pallet_contracts::chain_extension::{
	ChainExtension, Environment, Ext, InitState, RetVal, SysConfig, UncheckedFrom,
};
use sp_runtime::{DispatchError, RuntimeDebug};
use sp_std::prelude::*;
use primitives::Balance;

pub trait Config: pallet_contracts::Config + pallet_erc1155::Config {
	type Randomness: Randomness<Self::Hash>;
}

/// Result that returns a [`DispatchError`] on error.
pub type Result<T> = sp_std::result::Result<T, DispatchError>;



// func_id 1002
// do_create_tao(who: &T::AccountId, data: Vec<u8>) -> Result<T::TaoId, DispatchError>
#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug)]
pub struct CreateTaoInputParam<AccountId> {
	who: AccountId,
	data: Vec<u8>,
}

// func_id 1003
// do_create_token(
// 		who: &T::AccountId,
// 		tao_id: T::TaoId,
// 		token_id: T::TokenId,
// 		is_nf: bool,
// 		uri: Vec<u8>,
// 	)
#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug)]
pub struct CreateTokenInputParam<AccountId, TaoId, TokenId> {
	who: AccountId,
	tao_id: TaoId,
	token_id: TokenId,
	is_nf: bool,
	uri: Vec<u8>,
}


// func_id 1004
// do_set_approval_for_all(
// 		owner: &T::AccountId,
// 		operator: &T::AccountId,
// 		approved: bool,
// 	)
#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug)]
pub struct SetApprovalForAllInputParam<AccountId> {
	owner: AccountId,
	operator: AccountId,
	approved: bool,
}


// func_id 1005
// do_mint(
// 		to: &T::AccountId,
// 		tao_id: T::TaoId,
// 		token_id: T::TokenId,
// 		amount: Balance
// 	)
#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug)]
pub struct MintInputParam<AccountId, TaoId, TokenId, Balance> {
	to: AccountId,
	tao_id: TaoId,
	token_id: TokenId,
	amount: Balance,
}



// func_id 1006
// do_batch_mint(
// 		to: &T::AccountId,
// 		tao_id: T::TaoId,
// 		token_ids: Vec<T::TokenId>,
// 		amounts: Vec<Balance>
// 	)
#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug)]
pub struct BatchMintInputParam<AccountId, TaoId, TokenId, Balance> {
	to: AccountId,
	tao_id: TaoId,
	token_ids: Vec<TokenId>,
	amounts: Vec<Balance>,
}


// func_id 1007
// do_burn(
// 		from: &T::AccountId,
// 		tao_id: T::TaoId,
// 		token_id: T::TokenId,
// 		amount: Balance
// 	)
#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug)]
pub struct BurnInputParam<AccountId, TaoId, TokenId, Balance> {
	from: AccountId,
	tao_id: TaoId,
	token_id: TokenId,
	amount: Balance,
}

// func_id 1008
// do_batch_burn(
// 		from: &T::AccountId,
// 		tao_id: T::TaoId,
// 		token_ids: Vec<T::TokenId>,
// 		amounts: Vec<Balance>
// 	)
#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug)]
pub struct BatchBurnInputParam<AccountId, TaoId, TokenId, Balance> {
	from: AccountId,
	tao_id: TaoId,
	token_ids: Vec<TokenId>,
	amounts: Vec<Balance>,
}

// func_id 1009
// do_transfer_from(
// 		from: &T::AccountId,
// 		to: &T::AccountId,
// 		tao_id: T::TaoId,
// 		token_id: T::TokenId,
// 		amount: Balance
// 	)
#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug)]
pub struct TransferFromInputParam<AccountId, TaoId, TokenId, Balance> {
	from: AccountId,
	to: AccountId,
	tao_id: TaoId,
	token_id: TokenId,
	amount: Balance,
}

// func_id 1010
// do_batch_transfer_from(
// 		from: &T::AccountId,
// 		to: &T::AccountId,
// 		tao_id: T::TaoId,
// 		token_ids: Vec<T::TokenId>,
// 		amounts: Vec<Balance>
// 	)
#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug)]
pub struct BatchTransferFromInputParam<AccountId, TaoId, TokenId, Balance> {
	from: AccountId,
	to: AccountId,
	tao_id: TaoId,
	token_ids: Vec<TokenId>,
	amounts: Vec<Balance>,
}

// func_id 1011
// approved_or_owner(who: &T::AccountId, account: &T::AccountId) -> bool
#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug)]
pub struct ApprovedOrOwnerInputParam<AccountId> {
	who: AccountId,
	account: AccountId,
}

// func_id 1012
// is_approved_for_all(owner: &T::AccountId, operator: &T::AccountId) -> bool
#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug)]
pub struct IsApprovedForAllInputParam<AccountId> {
	owner: AccountId,
	operator: AccountId,
}
// func_id 1013
// fn balance_of(owner: &T::AccountId, tao_id: T::TaoId, token_id: T::TokenId) -> Balance
#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug)]
pub struct BalanceOfInputParam<AccountId, TaoId, TokenId> {
	owner: AccountId,
	tao_id: TaoId,
	token_id: TokenId,
}

// func_id 1014
// balance_of_batch(owners: &Vec<T::AccountId>, tao_id: T::TaoId, token_ids: Vec<T::TokenId>) -> Result<Vec<Balance>, DispatchError>
#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug)]
pub struct BalanceOfBatchInputParam<AccountId, TaoId, TokenId> {
	owners: Vec<AccountId>,
	tao_id: TaoId,
	token_ids: Vec<TokenId>,
}


/// chain extension of contract
pub struct SgcChainExtension;

impl<C: Config> ChainExtension<C> for SgcChainExtension {
	fn call<E>(func_id: u32, env: Environment<E, InitState>) -> Result<RetVal>
	where
		E: Ext<T = C>,
		<E::T as SysConfig>::AccountId: UncheckedFrom<<E::T as SysConfig>::Hash> + AsRef<[u8]>,
	{
		match func_id {
			1001 => {
				log::info!("run 1001");
				let mut env = env.buf_in_buf_out();
				let random_slice = <E::T as Config>::Randomness::random_seed().encode();
				// let random_slice = random_seed.encode();
				log::trace!(
					target: "runtime",
					"[ChainExtension]|call|func_id:{:}",
					func_id
				);
				env.write(&random_slice, false, None)
					.map_err(|_| DispatchError::Other("ChainExtension failed to call random"))?;
			}
			1002 => { // do_create_tao(who: &T::AccountId, data: Vec<u8>) -> Result<T::TaoId, DispatchError>
				log::info!("run 1002");
				let mut env = env.buf_in_buf_out();
				let caller = env.ext().caller().clone();
				log::info!("caller: {:?}", caller);

				let input: CreateTaoInputParam<
					<E::T as SysConfig>::AccountId
				 > = env.read_as()?;

				let tao_id: u64 = pallet_erc1155::Module::<E::T>::do_create_tao(&input.who, input.data)?.into();
				log::info!("balance: {:?}", tao_id);

				let weight = 100_000;
				env.charge_weight(weight)?;

				let tao_slice = tao_id.to_be_bytes();
				log::info!("balance_slice: {:?}", tao_slice);

				log::trace!(
					target: "runtime",
					"[ChainExtension]|call|func_id:{:}",
					func_id
				);

				env.write(&tao_slice, false, None)
					.map_err(|_| DispatchError::Other("ChainExtension failed to call create collection"))?;
			}
			1003 => { // // do_create_token(
						// 		who: &T::AccountId,
						// 		tao_id: T::TaoId,
						// 		token_id: T::TokenId,
						// 		is_nf: bool,
						// 		uri: Vec<u8>,
						// 	)
				log::info!("run 1003");
				let mut env = env.buf_in_buf_out();
				let caller = env.ext().caller().clone();
				log::info!("caller: {:?}", caller);

				let in_len = env.in_len();
				log::info!("in_len: {}", in_len);

				let mut buffer = vec![0u8; in_len as usize];

				env.read_into(&mut &mut buffer[..])?;
				log::info!("buffer: {:?}", buffer);

				let input: CreateTokenInputParam<
					<E::T as SysConfig>::AccountId,
					<E::T as pallet_erc1155::Config>::TaoId,
					<E::T as pallet_erc1155::Config>::TokenId,
				> = env.read_as()?;
				log::info!("input: {:?}", input);

				let weight = 100_000;
				env.charge_weight(weight)?;

				pallet_erc1155::Module::<E::T>::do_create_token(&input.who, input.tao_id, input.token_id, input.is_nf, input.uri)?;
			}
			1004 => {
				// do_set_approval_for_all(
				// 		owner: &T::AccountId,
				// 		operator: &T::AccountId,
				// 		approved: bool,
				// 	)
				log::info!("run 1004");
				let mut env = env.buf_in_buf_out();
				let caller = env.ext().caller().clone();
				log::info!("caller: {:?}", caller);

				let in_len = env.in_len();
				log::info!("in_len: {}", in_len);

				let mut buffer = vec![0u8; in_len as usize];

				env.read_into(&mut &mut buffer[..])?;
				log::info!("buffer: {:?}", buffer);

				let input: SetApprovalForAllInputParam<
					<E::T as SysConfig>::AccountId,
				> = env.read_as()?;
				log::info!("input: {:?}", input);

				let weight = 100_000;
				env.charge_weight(weight)?;

				pallet_erc1155::Module::<E::T>::do_set_approval_for_all(&input.owner, &input.operator, input.approved)?;

			}
			1005 => {
				// do_mint(
				// 		to: &T::AccountId,
				// 		tao_id: T::TaoId,
				// 		token_id: T::TokenId,
				// 		amount: Balance
				// 	)

				log::info!("run 1005");
				let mut env = env.buf_in_buf_out();
				let caller = env.ext().caller().clone();
				log::info!("caller: {:?}", caller);

				let in_len = env.in_len();
				log::info!("in_len: {}", in_len);

				let mut buffer = vec![0u8; in_len as usize];

				env.read_into(&mut &mut buffer[..])?;
				log::info!("buffer: {:?}", buffer);

				let input: MintInputParam<
					<E::T as SysConfig>::AccountId,
					<E::T as pallet_erc1155::Config>::TaoId,
					<E::T as pallet_erc1155::Config>::TokenId,
					Balance,
				> = env.read_as()?;
				log::info!("input: {:?}", input);

				let weight = 100_000;
				env.charge_weight(weight)?;

				pallet_erc1155::Module::<E::T>::do_mint(&input.to, input.tao_id, input.token_id, input.amount)?;

			}
			1006 => {
				// do_batch_mint(
				// 		to: &T::AccountId,
				// 		tao_id: T::TaoId,
 				// 		token_ids: Vec<T::TokenId>,
 				// 		amounts: Vec<Balance>
 				// 	)
				log::info!("run 1006");
				let mut env = env.buf_in_buf_out();
				let caller = env.ext().caller().clone();
				log::info!("caller: {:?}", caller);

				let in_len = env.in_len();
				log::info!("in_len: {}", in_len);

				let mut buffer = vec![0u8; in_len as usize];

				env.read_into(&mut &mut buffer[..])?;
				log::info!("buffer: {:?}", buffer);

				let input: BatchMintInputParam<
					<E::T as SysConfig>::AccountId,
					<E::T as pallet_erc1155::Config>::TaoId,
					<E::T as pallet_erc1155::Config>::TokenId,
					Balance,
				> = env.read_as()?;
				log::info!("input: {:?}", input);

				let weight = 100_000;
				env.charge_weight(weight)?;

				pallet_erc1155::Module::<E::T>::do_batch_mint(&input.to, input.tao_id, input.token_ids, input.amounts)?;

			}
			1007 => {

				// do_burn(
				// 		from: &T::AccountId,
				// 		tao_id: T::TaoId,
				// 		token_id: T::TokenId,
				// 		amount: Balance
				// 	)

				log::info!("run 1007");
				let mut env = env.buf_in_buf_out();
				let caller = env.ext().caller().clone();
				log::info!("caller: {:?}", caller);

				let in_len = env.in_len();
				log::info!("in_len: {}", in_len);

				let mut buffer = vec![0u8; in_len as usize];

				env.read_into(&mut &mut buffer[..])?;
				log::info!("buffer: {:?}", buffer);

				let input: BurnInputParam<
					<E::T as SysConfig>::AccountId,
					<E::T as pallet_erc1155::Config>::TaoId,
					<E::T as pallet_erc1155::Config>::TokenId,
					Balance,
				> = env.read_as()?;
				log::info!("input: {:?}", input);

				let weight = 100_000;
				env.charge_weight(weight)?;

				pallet_erc1155::Module::<E::T>::do_burn(&input.from, input.tao_id, input.token_id, input.amount)?;
			}
			1008 => {
				// do_batch_burn(
				// 		from: &T::AccountId,
				// 		tao_id: T::TaoId,
				// 		token_ids: Vec<T::TokenId>,
				// 		amounts: Vec<Balance>
				// 	)
				log::info!("run 1008");
				let mut env = env.buf_in_buf_out();
				let caller = env.ext().caller().clone();
				log::info!("caller: {:?}", caller);

				let in_len = env.in_len();
				log::info!("in_len: {}", in_len);

				let mut buffer = vec![0u8; in_len as usize];

				env.read_into(&mut &mut buffer[..])?;
				log::info!("buffer: {:?}", buffer);

				let input: BatchBurnInputParam<
					<E::T as SysConfig>::AccountId,
					<E::T as pallet_erc1155::Config>::TaoId,
					<E::T as pallet_erc1155::Config>::TokenId,
					Balance,
				> = env.read_as()?;
				log::info!("input: {:?}", input);

				let weight = 100_000;
				env.charge_weight(weight)?;

				pallet_erc1155::Module::<E::T>::do_batch_burn(&input.from, input.tao_id, input.token_ids, input.amounts)?;
			}
			1009 => {
				// do_transfer_from(
				// 		from: &T::AccountId,
				// 		to: &T::AccountId,
				// 		tao_id: T::TaoId,
				// 		token_id: T::TokenId,
				// 		amount: Balance
				// 	)
				log::info!("run 1009");
				let mut env = env.buf_in_buf_out();
				let caller = env.ext().caller().clone();
				log::info!("caller: {:?}", caller);

				let in_len = env.in_len();
				log::info!("in_len: {}", in_len);

				let mut buffer = vec![0u8; in_len as usize];

				env.read_into(&mut &mut buffer[..])?;
				log::info!("buffer: {:?}", buffer);

				let input: TransferFromInputParam<
					<E::T as SysConfig>::AccountId,
					<E::T as pallet_erc1155::Config>::TaoId,
					<E::T as pallet_erc1155::Config>::TokenId,
					Balance,
				> = env.read_as()?;
				log::info!("input: {:?}", input);

				let weight = 100_000;
				env.charge_weight(weight)?;

				pallet_erc1155::Module::<E::T>::do_transfer_from(&input.from, &input.to, input.tao_id, input.token_id, input.amount)?;
			}
			1010 => {
				// do_batch_transfer_from(
				// 		from: &T::AccountId,
				// 		to: &T::AccountId,
				// 		tao_id: T::TaoId,
				// 		token_ids: Vec<T::TokenId>,
				// 		amounts: Vec<Balance>
				// 	)
				log::info!("run 1010");
				let mut env = env.buf_in_buf_out();
				let caller = env.ext().caller().clone();
				log::info!("caller: {:?}", caller);

				let in_len = env.in_len();
				log::info!("in_len: {}", in_len);

				let mut buffer = vec![0u8; in_len as usize];

				env.read_into(&mut &mut buffer[..])?;
				log::info!("buffer: {:?}", buffer);

				let input: BatchTransferFromInputParam<
					<E::T as SysConfig>::AccountId,
					<E::T as pallet_erc1155::Config>::TaoId,
					<E::T as pallet_erc1155::Config>::TokenId,
					Balance,
				> = env.read_as()?;
				log::info!("input: {:?}", input);

				let weight = 100_000;
				env.charge_weight(weight)?;

				pallet_erc1155::Module::<E::T>::do_batch_transfer_from(&input.from, &input.to, input.tao_id, input.token_ids, input.amounts)?;
			}
			1011 => {
				// approved_or_owner(who: &T::AccountId, account: &T::AccountId) -> bool
				log::info!("run 1011");
				let mut env = env.buf_in_buf_out();
				let caller = env.ext().caller().clone();
				log::info!("caller: {:?}", caller);

				let input: ApprovedOrOwnerInputParam<
					<E::T as SysConfig>::AccountId
				> = env.read_as()?;

				let ret: bool = pallet_erc1155::Module::<E::T>::approved_or_owner(&input.who, &input.account);
				let ret = ret as u8;
				log::info!("balance: {:?}", ret);

				let weight = 100_000;
				env.charge_weight(weight)?;

				let ret_slice = ret.to_be_bytes();
				log::info!("balance_slice: {:?}", ret_slice);

				log::trace!(
					target: "runtime",
					"[ChainExtension]|call|func_id:{:}",
					func_id
				);

				env.write(&ret_slice, false, None)
					.map_err(|_| DispatchError::Other("ChainExtension failed to call create collection"))?;

			}
			1012 => {
				// is_approved_for_all(owner: &T::AccountId, operator: &T::AccountId) -> bool
				log::info!("run 1012");
				let mut env = env.buf_in_buf_out();
				let caller = env.ext().caller().clone();
				log::info!("caller: {:?}", caller);

				let input: ApprovedOrOwnerInputParam<
					<E::T as SysConfig>::AccountId
				> = env.read_as()?;

				let ret: bool = pallet_erc1155::Module::<E::T>::is_approved_for_all(&input.who, &input.account);
				let ret = ret as u8;
				log::info!("ret: {:?}", ret);

				let weight = 100_000;
				env.charge_weight(weight)?;

				let ret_slice = ret.to_be_bytes();
				log::info!("ret_slice: {:?}", ret_slice);

				log::trace!(
					target: "runtime",
					"[ChainExtension]|call|func_id:{:}",
					func_id
				);

				env.write(&ret_slice, false, None)
					.map_err(|_| DispatchError::Other("ChainExtension failed to call create collection"))?;
			}
			1013 => {
				// fn balance_of(owner: &T::AccountId, tao_id: T::TaoId, token_id: T::TokenId) -> Balance
				log::info!("run 1011");
				let mut env = env.buf_in_buf_out();
				let caller = env.ext().caller().clone();
				log::info!("caller: {:?}", caller);

				let input: BalanceOfInputParam<
					<E::T as SysConfig>::AccountId,
					<E::T as pallet_erc1155::Config>::TaoId,
					<E::T as pallet_erc1155::Config>::TokenId,
				> = env.read_as()?;

				let balance: u128 = pallet_erc1155::Module::<E::T>::balance_of(&input.owner, input.tao_id, input.token_id);
				log::info!("balance: {:?}", balance);

				let weight = 100_000;
				env.charge_weight(weight)?;

				let balance_slice = balance.to_be_bytes();
				log::info!("balance_slice: {:?}", balance_slice);

				log::trace!(
					target: "runtime",
					"[ChainExtension]|call|func_id:{:}",
					func_id
				);

				env.write(&balance_slice, false, None)
					.map_err(|_| DispatchError::Other("ChainExtension failed to call create collection"))?;
			}
			// 1014 => {
			// 	// balance_of_batch(owners: &Vec<T::AccountId>, tao_id: T::TaoId, token_ids: Vec<T::TokenId>) -> Result<Vec<Balance>, DispatchError>
			// 	log::info!("run 1011");
			// 	let mut env = env.buf_in_buf_out();
			// 	let caller = env.ext().caller().clone();
			// 	log::info!("caller: {:?}", caller);
			//
			// 	let input: ApprovedOrOwnerInputParam<
			// 		<E::T as SysConfig>::AccountId
			// 	> = env.read_as()?;
			//
			// 	let ret: bool = pallet_erc1155::Module::<E::T>::approved_or_owner(&input.who, input.account);
			// 	let ret = ret as u8;
			// 	log::info!("balance: {:?}", ret);
			//
			// 	let weight = 100_000;
			// 	env.charge_weight(weight)?;
			//
			// 	let ret_slice = ret.to_be_bytes();
			// 	log::info!("balance_slice: {:?}", ret_slice);
			//
			// 	log::trace!(
			// 		target: "runtime",
			// 		"[ChainExtension]|call|func_id:{:}",
			// 		func_id
			// 	);
			//
			// 	env.write(&ret_slice, false, None)
			// 		.map_err(|_| DispatchError::Other("ChainExtension failed to call create collection"))?;
			// }

			_ => {
				log::error!("call an unregistered `func_id`, func_id:{:}", func_id);
				return Err(DispatchError::Other("Unimplemented func_id"));
			}
		}
		Ok(RetVal::Converging(0))
	}

	fn enabled() -> bool {
		true
	}
}
