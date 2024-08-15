mod balances;
mod system;

mod types {
    pub type AccountId = String;
    pub type Balance = u128;
    pub type BlockNumber = u32;
    pub type Nonce = u32;
}

impl system::Config for Runtime {
    type AccountId = types::AccountId;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
    type Balance = types::Balance;
}
// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
#[derive(Debug)]
pub struct Runtime {
    system: system::Pallet<Runtime>,
    balances: balances::Pallet<Runtime>,
}

impl Runtime {
	// Create a new instance of the main Runtime, by creating a new instance of each pallet.
	fn new() -> Self {
		Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
        }
	}
}

fn main() {
	let mut runtime = Runtime::new();

    // initialize the runtime
    let alice = "alice".to_string();
    let bob: String = "bob".to_string();
    let charlie = "charlie".to_string();

    runtime.balances.set_balance(&alice, 100);
    
	// start emulating a block
	runtime.system.inc_block_number();
    assert_eq!(runtime.system.block_number(), 1);

	// first transaction
	runtime.system.inc_nonce(&alice);
    let _result = runtime.balances
        .transfer(alice.clone(), bob.clone(), 30)
        .map_err(|e| println!("Error: {:?}", e));
    assert_eq!(_result, Ok(()));
    assert_eq!(runtime.balances.balance(&alice), 70);
    assert_eq!(runtime.balances.balance(&bob), 30);
    assert_eq!(runtime.system.get_nonce(&alice), 1);

	// second transaction
	runtime.system.inc_nonce(&alice);
    /* TODO: Increment the nonce of `alice` again. */
    let _result = runtime.balances
        .transfer(alice.clone(), charlie.clone(), 20)
        .map_err(|e| println!("Err: {:?}", e));
    assert_eq!(_result, Ok(()));
    assert_eq!(runtime.balances.balance(&alice), 50);
    assert_eq!(runtime.balances.balance(&charlie), 20);
    assert_eq!(runtime.system.get_nonce(&alice), 2);
	/* TODO: Execute another balance transfer, this time from `alice` to `charlie` for 20. */

    println!("Runtime: {:#?}", runtime);
}
