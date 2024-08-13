use std::collections::BTreeMap;

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet {
    /// The current block number.
    block_number: u32,
    /// A map from an account to their nonce.
    nonce: BTreeMap<String, u32>,
}

impl Pallet {
    pub fn new() -> Self {
        Self {
            block_number: 0,
            nonce: BTreeMap::new(),
        }
    }

    /// Get the current block number.
	pub fn block_number(&self) -> u32 {
		/* TODO: Return the current block number. */
		self.block_number
	}

	// This function can be used to increment the block number.
	// Increases the block number by one.
	pub fn inc_block_number(&mut self) {
		/* TODO: Increment the current block number by one. */
		// crashes when block_number overflows
		self.block_number = self.block_number.checked_add(1).unwrap();
	}

	// Increment the nonce of an account. This helps us keep track of how many transactions each
	// account has made.
	pub fn inc_nonce(&mut self, who: &String) {
		/* TODO: Get the current nonce of `who`, and increment it by one. */
		let nonce = self.nonce.get(who).unwrap_or(&0);
		self.nonce.insert(who.clone(), nonce + 1);
	}

	pub fn get_nonce(&mut self, who: &String) -> u32 {
		*self.nonce.get(who).unwrap_or(&0)
	}
}

#[cfg(test)]
mod tests {
	#[test]
    fn init_system() {
		let system = super::Pallet::new();
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
		let mut system = super::Pallet::new();
		system.inc_block_number();
		assert_eq!(system.block_number(), 1);
	}

	#[test]
	fn inc_nonce() {
		let alice = String::from("alice");
		let mut system = super::Pallet::new();
		system.inc_nonce(&alice);

		assert_eq!(system.get_nonce(&alice), 1);
	}
}