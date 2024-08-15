use std::{collections::BTreeMap, ops::AddAssign};
use num::{traits::Zero, One};

pub trait Config {
	type AccountId: Ord + Clone;
	type BlockNumber: Zero + One + AddAssign + Copy;
	type Nonce: Zero + One + Copy;
	// and more if needed
}

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet<T: Config> {
    /// The current block number.
    block_number: T::BlockNumber,
    /// A map from an account to their nonce.
    nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            block_number: T::BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }

    /// Get the current block number.
	pub fn block_number(&self) -> T::BlockNumber {
		/* TODO: Return the current block number. */
		self.block_number
	}

	// This function can be used to increment the block number.
	// Increases the block number by one.
	pub fn inc_block_number(&mut self) {
		/* TODO: Increment the current block number by one. */
		// crashes when block_number overflows
		self.block_number += T::BlockNumber::one()
	}

	// Increment the nonce of an account. This helps us keep track of how many transactions each
	// account has made.
	pub fn inc_nonce(&mut self, who: &T::AccountId) {
		/* TODO: Get the current nonce of `who`, and increment it by one. */
		let nonce = *self.nonce.get(who).unwrap_or(&T::Nonce::zero());
		self.nonce.insert(who.clone(), nonce + T::Nonce::one());
	}

	pub fn get_nonce(&mut self, who: &T::AccountId) -> T::Nonce {
		*self.nonce.get(who).unwrap_or(&T::Nonce::zero())
	}
}

#[cfg(test)]
mod tests {
    struct TestConfig;

	impl super::Config for TestConfig {
		type AccountId = String;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	#[test]
    fn init_system() {
		let system: super::Pallet<TestConfig> = super::Pallet::new();
		assert_eq!(system.block_number(), 0);
		/* TODO: Create a test which checks the following:
			- Increment the current block number.
			- Increment the nonce of `alice`.
			- Check the block number is what we expect.
			- Check the nonce of `alice` is what we expect.
		*/
	}

	#[test]
	fn inc_block_number() {
		let mut system: super::Pallet<TestConfig> = super::Pallet::new();
		system.inc_block_number();
		assert_eq!(system.block_number(), 1);
	}

	#[test]
	fn inc_nonce() {
		let alice = String::from("alice");
		let mut system: super::Pallet<TestConfig> = super::Pallet::new();
		system.inc_nonce(&alice);

		assert_eq!(system.get_nonce(&alice), 1);
	}
}