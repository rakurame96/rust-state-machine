use std::collections::BTreeMap;

pub struct Pallet {
    // A simple storage mapping from accounts (`String`) to their balances (`u128`).
    balances: BTreeMap<String, u128>,
}

impl Pallet {
    /// Create a new instance of the balances module.
    pub fn new () -> Self {
        Self {
            balances: BTreeMap::new()
        }
    }

    /// Set the balance of an account `who` to some `amount`.
    pub fn set_balances (&mut self, who: String, amount: u128) {
        self.balances.insert(who.clone(), amount);
    }

    /// Get the balance of an account `who`.
	/// If the account has no stored balance, we return zero.    
    pub fn balance (&mut self, who: String) -> u128 {
        *self.balances.get(&who).unwrap_or(&0)
    }

    /// Transfer `amount` from one account to another.
	/// This function verifies that `from` has at least `amount` balance to transfer,
	/// and that no mathematical overflows occur.
    pub fn transfer(
        &mut self, 
        caller: String,
        to: String,
        amount: u128,
    ) -> Result<(), &'static str> {
        /* TODO:
			- Get the balance of account `caller`.
			- Get the balance of account `to`.

			- Use safe math to calculate a `new_caller_balance`.
			- Use safe math to calculate a `new_to_balance`.

			- Insert the new balance of `caller`.
			- Insert the new balance of `to`.
		*/
        let caller_balance = self.balance(caller.clone());
        let to_balance = self.balance(to.clone());

        let new_caller_balance = caller_balance
            .checked_sub(amount)
            .ok_or("insufficient balance")?;

        let new_to_balance = to_balance
            .checked_add(amount)
            .ok_or("overflow when adding to balance")?;

        self.set_balances(caller, new_caller_balance);
        self.set_balances(to, new_to_balance);



        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::u128;

    use super::Pallet;

    #[test]
    fn init_balances() {
        let mut balances = Pallet::new();
        
        assert_eq!(balances.balance("alice".to_string()), 0);
        balances.set_balances("alice".to_string(), 100);
        assert_eq!(balances.balance("alice".to_string()), 100);
        assert_eq!(balances.balance("bob".to_string()), 0);
    }

    #[test]
    fn transfer_balance() {
        let alice = "alice".to_string();
        let bob = "bob".to_string();

        let mut balances = Pallet::new();
        
        balances.set_balances(alice.clone(), 100);
        let _ = balances.transfer(alice.clone(), bob.clone(), 90);

        assert_eq!(balances.balance(alice), 10);
        assert_eq!(balances.balance(bob), 90);
    }

    #[test]
    fn transfer_insufficient_balance() {
        let alice = "alice".to_string();
        let bob = "bob".to_string();

        let mut balances = Pallet::new();
        
        balances.set_balances(alice.clone(), 100);

        let result = balances.transfer(alice.clone(), bob.clone(), 110);

        assert_eq!(result, Err("insufficient balance"));
        assert_eq!(balances.balance(alice), 100);
        assert_eq!(balances.balance(bob), 0);
    }

    #[test]
    fn transfer_overflow_balance() {
        let alice = "alice".to_string();
        let bob = "bob".to_string();

        let mut balances = Pallet::new();
        
        balances.set_balances(alice.clone(), 100);
        balances.set_balances(bob.clone(), u128::MAX);

        let result = balances.transfer(alice.clone(), bob.clone(), 1);

        assert_eq!(result, Err("overflow when adding to balance"));
        assert_eq!(balances.balance(alice), 100);
        assert_eq!(balances.balance(bob), u128::MAX);
    }
}