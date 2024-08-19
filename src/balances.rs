use std::collections::BTreeMap;
use num::{traits::CheckedAdd, CheckedSub, Zero};

pub trait Config: crate::system::Config {
	type Balance: Zero + CheckedAdd + CheckedSub + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    // A simple storage mapping from accounts (`String`) to their balances (`u128`).
    balances: BTreeMap<T::AccountId, T::Balance>,
}

#[macros::call]
impl<T: Config> Pallet<T> {
    /// Transfer `amount` from one account to another.
	/// This function verifies that `from` has at least `amount` balance to transfer,
	/// and that no mathematical overflows occur.
    pub fn transfer(
        &mut self, 
        caller: T::AccountId,
        to: T::AccountId,
        amount: T::Balance,
    ) -> crate::support::DispatchResult {
        let caller_balance = self.balance(&caller);
        let to_balance = self.balance(&to);

        let new_caller_balance = caller_balance
            .checked_sub(&amount)
            .ok_or("insufficient balance")?;

        let new_to_balance = to_balance
            .checked_add(&amount)
            .ok_or("overflow when adding to balance")?;

        self.set_balance(&caller, new_caller_balance);
        self.set_balance(&to, new_to_balance);

        Ok(())
    }
}

impl<T: Config> Pallet<T> {
    /// Create a new instance of the balances module.
    pub fn new () -> Self {
        Self {
            balances: BTreeMap::new()
        }
    }

    /// Set the balance of an account `who` to some `amount`.
    pub fn set_balance (&mut self, who: &T::AccountId, amount: T::Balance) {
        self.balances.insert(who.clone(), amount);
    }

    /// Get the balance of an account `who`.
	/// If the account has no stored balance, we return zero.    
    pub fn balance (&mut self, who: &T::AccountId) -> T::Balance {
        *self.balances.get(who).unwrap_or(&T::Balance::zero())
    }    
}

#[cfg(test)]
mod tests {
    // use crate::balances::Pallet;

    use crate::system;

    struct TestConfig;
    impl system::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    impl super::Config for TestConfig {
        type Balance = u128;
    }

    #[test]
	fn init_balances() {
		let mut balances: super::Pallet<TestConfig> = super::Pallet::new();

		assert_eq!(balances.balance(&"alice".to_string()), 0);
		balances.set_balance(&"alice".to_string(), 100);
		assert_eq!(balances.balance(&"alice".to_string()), 100);
		assert_eq!(balances.balance(&"bob".to_string()), 0);
	}

	#[test]
	fn transfer_balance() {
		let mut balances: super::Pallet<TestConfig> = super::Pallet::new();

		assert_eq!(
			balances.transfer("alice".to_string(), "bob".to_string(), 51),
			Err("insufficient balance")
		);

		balances.set_balance(&"alice".to_string(), 100);
		assert_eq!(balances.transfer("alice".to_string(), "bob".to_string(), 51), Ok(()));
		assert_eq!(balances.balance(&"alice".to_string()), 49);
		assert_eq!(balances.balance(&"bob".to_string()), 51);

		assert_eq!(
			balances.transfer("alice".to_string(), "bob".to_string(), 51),
			Err("insufficient balance")
		);
	}

    #[test]
    fn transfer_insufficient_balance() {
        let alice = "alice".to_string();
        let bob = "bob".to_string();

        let mut balances: super::Pallet<TestConfig> = super::Pallet::new();
        
        balances.set_balance(&alice, 100);

        let result = balances.transfer(alice.clone(), bob.clone(), 110);

        assert_eq!(result, Err("insufficient balance"));
        assert_eq!(balances.balance(&alice), 100);
        assert_eq!(balances.balance(&bob), 0);
    }

    #[test]
    fn transfer_overflow_balance() {
        let alice = "alice".to_string();
        let bob = "bob".to_string();

        let mut balances: super::Pallet<TestConfig> = super::Pallet::new();
        
        balances.set_balance(&alice, 100);
        balances.set_balance(&bob, u128::MAX);

        let result = balances.transfer(alice.clone(), bob.clone(), 1);

        assert_eq!(result, Err("overflow when adding to balance"));
        assert_eq!(balances.balance(&alice), 100);
        assert_eq!(balances.balance(&bob), u128::MAX);
    }
}