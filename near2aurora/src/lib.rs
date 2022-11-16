pub use ethabi::*;
use near_sdk::{
    borsh::{self, BorshSerialize},
    AccountId, Gas, Promise, require,
};
pub use argument::Argument;

mod argument;

type AuroraAddress = [u8; 20];
type Wei = [u8; 32];

#[derive(borsh::BorshSerialize)]
struct CallArgs {
    /// Contract to be called
    pub contract: AuroraAddress,
    /// Amount of wei to attach
    pub value: Wei,
    /// Input data to pass to the contract
    /// Should be a vector of method signature followed by encoded arguments
    pub input: Vec<u8>,
}

impl CallArgs {
    fn serialize(&self) -> Vec<u8> {
        // Prepend byte one to signal enum version
        vec![vec![0], self.try_to_vec().unwrap()].concat()
    }
}

pub fn call(
    aurora_account_id: &AccountId,
    contract_address: &String,
    method: &Function,
    arguments: &Vec<Argument>,
    value: Option<Wei>,
    gas: Option<Gas>,
) -> Promise {
    let zero_value = [0; 32];
    let default_gas = Gas(20_000_000_000_000);

    require!(
        contract_address.len() == 40,
        "Invalid aurora address length"
    );
    let contract_address_encoded = hex::decode(contract_address).unwrap().to_vec();
    let mut contract: [u8; 20] = [0; 20];
    contract.copy_from_slice(&contract_address_encoded);

    let method_sig: Vec<u8> = method.short_signature().to_vec();
    let encoded_arguments: Vec<u8> = arguments.iter().flat_map(|arg| arg.encode()).collect();

    Promise::new(aurora_account_id.clone()).function_call(
        "call".to_string(),
        CallArgs {
            contract,
            value: value.unwrap_or(zero_value),
            input: vec![method_sig, encoded_arguments].concat(),
        }
        .serialize(),
        0,
        gas.unwrap_or(default_gas),
    )
}
