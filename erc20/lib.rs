#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod erc20 {

	use ink::storage::Mapping;

	#[ink(storage)]
	#[derive(Default)]
	pub struct Erc20 {
		total_supply: Balance,
		balances: Mapping<AccountId, Balance>,
		allowances: Mapping<(AccountId, AccountId), Balance>,
	}

	#[ink(event)]
	pub struct Transfer {
		#[link(topic)]
		from: AccountId,
		#[link(topic)]
		to: AccountId,
		value: Balance,
	}

	#[ink(event)]
	pub struct Approval {
		#[link(topic)]
		from: AccountId,
		#[link(topic)]
		to: AccountId,
		value: Balance,
	}

	#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
	#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
	pub enum Error {
		BalanceTooLow,
		AllowanceTooLow,
	}

	type Result<T> = core::result::Result<T, Error>;

	impl Erc20 {
		#[ink(constructor)]
		pub fn new(total_supply: Balance) -> Self {
			let mut balances = Mapping::new();

			balances.insert(Self::env().caller(), &total_supply);

			Self { total_supply, balances, ..Default::default() }
		}

		#[ink(message)]
		pub fn total_supply(&self) -> Balance {
			self.total_supply
		}

		#[ink(message)]
		pub fn balance_of(&self, who: AccountId) -> Balance {
			self.balances.get(&who).unwrap_or_default()
		}

		#[ink(message)]
		pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
			let sender = self.env().caller();
			self.transfer_helper(&sender, &to, value)
		}

		#[ink(message)]
		pub fn transfer_from(
			&mut self,
			from: AccountId,
			to: AccountId,
			value: Balance,
		) -> Result<()> {
			let sender = self.env().caller();
			let mut allowance = self.allowances.get(&(from, sender)).unwrap_or_default();

			if allowance < value {
				return Err(Error::AllowanceTooLow)
			}

			self.allowances.insert(&(from, sender), &(allowance - value));

			self.transfer_helper(&from, &to, value)
		}

		#[ink(message)]
		pub fn approve(&mut self, to: AccountId, value: Balance) -> Result<()> {
			let sender = self.env().caller();

			self.allowances.insert(&(sender, to), &(value));

			self.env().emit_event(Approval { from: sender, to, value });

			Ok(())
		}

		pub fn transfer_helper(
			&mut self,
			from: &AccountId,
			to: &AccountId,
			value: Balance,
		) -> Result<()> {
			let balance_from = self.balance_of(*from);
			let balance_to = self.balance_of(*to);

			if value > balance_from {
				return Err(Error::BalanceTooLow)
			}

			self.balances.insert(from, &(balance_from - value));
			self.balances.insert(to, &(balance_to + value));

			self.env().emit_event(Transfer { from: *from, to: *to, value });

			Ok(())
		}
	}

	/// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
	/// module and test functions are marked with a `#[test]` attribute.
	/// The below code is technically just normal Rust code.
	#[cfg(test)]
	mod tests {}

	/// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
	///
	/// When running these you need to make sure that you:
	/// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
	/// - Are running a Substrate node which contains `pallet-contracts` in the background
	#[cfg(all(test, feature = "e2e-tests"))]
	mod e2e_tests {
		/// Imports all the definitions from the outer scope so we can use them here.
		use super::*;

		/// A helper function used for calling contract messages.
		use ink_e2e::build_message;

		/// The End-to-End test `Result` type.
		type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;
	}
}
