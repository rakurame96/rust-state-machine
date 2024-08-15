use std::{collections::BTreeMap, ops::AddAssign};
use num::{traits::CheckedAdd, CheckedSub, Zero, One};

/*
	TODO: Define the common types used in this pallet:
		- `AccountID`
		- `BlockNumber`
		- `Nonce`

	Then update this pallet to use these common types.
*/

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet<AccountID, BlockNumber, Nonce> {
    /// The current block number.
    block_number: BlockNumber,
    /// A map from an account to their nonce.
    nonce: BTreeMap<AccountID, Nonce>,
}

impl<AccountID, BlockNumber,Nonce> Pallet<AccountID, BlockNumber,Nonce>
where
	AccountID: Ord + Clone,
	BlockNumber: Zero + One + CheckedAdd + CheckedSub + Copy + AddAssign,
	Nonce: Ord + Zero + One + Clone + Copy,
{
    pub fn new() -> Self {
        Self {
            block_number: BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }

    /// Get the current block number.
	pub fn block_number(&self) -> BlockNumber {
		/* TODO: Return the current block number. */
		self.block_number
	}

	// This function can be used to increment the block number.
	// Increases the block number by one.
	pub fn inc_block_number(&mut self) {
		/* TODO: Increment the current block number by one. */
		// crashes when block_number overflows
		self.block_number += BlockNumber::one()
	}

	// Increment the nonce of an account. This helps us keep track of how many transactions each
	// account has made.
	pub fn inc_nonce(&mut self, who: &AccountID) {
		/* TODO: Get the current nonce of `who`, and increment it by one. */
		let nonce = *self.nonce.get(who).unwrap_or(&Nonce::zero());
		self.nonce.insert(who.clone(), nonce + Nonce::one());
	}

	pub fn get_nonce(&mut self, who: &AccountID) -> Nonce {
		*self.nonce.get(who).unwrap_or(&Nonce::zero())
	}
}

#[cfg(test)]
mod tests {
    
	#[test]
    fn init_system() {
		let system: Pallet<String, u32, u32> = super::Pallet::new();
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
		let mut system: Pallet<String, u32, u32> = super::Pallet::new();
		system.inc_block_number();
		assert_eq!(system.block_number(), 1);
	}

	#[test]
	fn inc_nonce() {
		let alice = String::from("alice");
		let mut system: Pallet<String, u32, u32> = super::Pallet::new();
		system.inc_nonce(&alice);

		assert_eq!(system.get_nonce(&alice), 1);
	}
}