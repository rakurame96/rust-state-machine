use support::Dispatch;

mod balances;
mod system;
mod support;

mod types {
    use crate::support;

    pub type AccountId = String;
    pub type Balance = u128;
    pub type BlockNumber = u32;
    pub type Nonce = u32;
    /* TODO: Define a concrete `Extrinsic` type using `AccountId` and `RuntimeCall`. */
	/* TODO: Define a concrete `Header` type using `BlockNumber`. */
	/* TODO: Define a concrete `Block` type using `Header` and `Extrinsic`. */
    pub type Extrinsic = support::Extrinsic<AccountId, crate::RuntimeCall>;
    pub type Header = support::Header<BlockNumber>;
    pub type Block = support::Block<Header, Extrinsic>;
}

pub enum RuntimeCall {
    Balances(balances::Call<Runtime>),
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

    // Execute a block of extrinsics. Increments the block number.
	fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
        self.system.inc_block_number();
        if self.system.block_number() != block.header.block_number {
            return Err("Block number mismatch");
        }

        for (i, support::Extrinsic { caller, call } ) in block.extrinsics.into_iter().enumerate() {
            // do stuff with `caller` and `call`
            self.system.inc_nonce(&caller);
            let _res = self.dispatch(caller, call).map_err(|e| eprintln!(
                "Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
                block.header.block_number, i, e
            ));
        }
		/* TODO:
			- Increment the system's block number.
			- Check that the block number of the incoming block matches the current block number,
			  or return an error.
			- Iterate over the extrinsics in the block...
				- Increment the nonce of the caller.
				- Dispatch the extrinsic using the `caller` and the `call` contained in the extrinsic.
				- Handle errors from `dispatch` same as we did for individual calls: printing any
				  error and capturing the result.
				- You can extend the error message to include information like the block number and
				  extrinsic number.
		*/
		Ok(())
	}
}

//also ADD THIS CODE TO YOUR main.rs file:
impl crate::support::Dispatch for Runtime {
	type Caller = <Runtime as system::Config>::AccountId;
	type Call = RuntimeCall;
	// Dispatch a call on behalf of a caller. Increments the caller's nonce.
	//
	// Dispatch allows us to identify which underlying module call we want to execute.
	// Note that we extract the `caller` from the extrinsic, and use that information
	// to determine who we are executing the call on behalf of.
	fn dispatch(
		&mut self,
		caller: Self::Caller,
		runtime_call: Self::Call,
	) -> support::DispatchResult {
		match runtime_call {
            RuntimeCall::Balances(call) => {
                self.balances.dispatch(caller, call)?;
            },
        }
        Ok(())
	}
}

fn main() {
	let mut runtime = Runtime::new();

    // initialize the runtime
    let alice = "alice".to_string();
    let bob: String = "bob".to_string();
    let charlie = "charlie".to_string();

    runtime.balances.set_balance(&alice, 100);

    let block_1 = types::Block {
        header: support::Header { block_number: 1 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::Balances(balances::Call::Transfer { to: bob.clone(), amount: 30 })
            },
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::Balances(balances::Call::Transfer { to: charlie.clone(), amount: 20 }),
            },
        ],
    };

    runtime.execute_block(block_1).expect("wrong block execution");

    println!("Runtime: {:#?}", runtime);
}
