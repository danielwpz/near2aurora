use crate::ethereum_types::U256;
pub use aurora_engine_types::parameters::engine::{SubmitResult, TransactionStatus};
pub use ethabi::*;
use near_sdk::{
    borsh::{self, BorshSerialize},
    require, AccountId, Gas, Promise,
};

type AuroraAddress = [u8; 20];
type Wei = [u8; 32];

/// Indicates an error with signature `Error(String)`. For more information see:
/// * https://docs.soliditylang.org/en/v0.8.18/control-structures.html#assert-and-require
/// * https://www.4byte.directory/signatures/?bytes4_signature=0x08c379a0
pub const SOLIDITY_ERROR_SELECTOR: [u8; 4] = [8, 195, 121, 160];

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
    arguments: &[Token],
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
    let encoded_arguments: Vec<u8> = encode(arguments);

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

pub fn parse_output(result: SubmitResult) -> core::result::Result<U256, String> {
    match result.status {
        TransactionStatus::Succeed(bytes) => {
            let u256_result = U256::from_big_endian(&bytes);
            Ok(u256_result)
        }
        TransactionStatus::Revert(bytes) => {
            let error_message = format!("Revert: {}", parse_evm_revert_message(&bytes));
            Err(error_message)
        }
        TransactionStatus::OutOfGas => Err("Err: Out of gas".into()),
        TransactionStatus::OutOfFund => Err("Err: Out of Fund".into()),
        TransactionStatus::OutOfOffset => Err("Err: Out of offset".into()),
        TransactionStatus::CallTooDeep => Err("Err: Call too deep".into()),
    }
}

pub fn parse_evm_revert_message(input: &[u8]) -> String {
    if input.len() < 4 {
        return format!("0x{}", hex::encode(input));
    }

    let decoded = if input[0..4] == SOLIDITY_ERROR_SELECTOR {
        try_abi_parse_revert_message(input)
    } else {
        None
    };

    decoded.unwrap_or_else(|| format!("0x{}", hex::encode(input)))
}

fn try_abi_parse_revert_message(input: &[u8]) -> Option<String> {
    ethabi::decode(&[ethabi::ParamType::String], &input[4..])
        .ok()?
        .pop()?
        .into_string()
}

#[cfg(test)]
mod test {
    use ethabi::ethereum_types::U256;

    use super::*;

    #[test]
    fn test_method_signature_encode() {
        let approve_method = Function {
            name: "approve".to_string(),
            inputs: vec![
                Param {
                    name: "spender".to_string(),
                    kind: ParamType::Address,
                    internal_type: None,
                },
                Param {
                    name: "amount".to_string(),
                    kind: ParamType::Uint(256),
                    internal_type: None,
                },
            ],
            outputs: vec![Param {
                name: "result".into(),
                kind: ParamType::Bool,
                internal_type: None,
            }],
            constant: None,
            state_mutability: StateMutability::NonPayable,
        };

        // should be 0x095ea7b3
        assert_eq!(
            approve_method.short_signature().to_vec(),
            vec![0x09, 0x5e, 0xa7, 0xb3]
        );
    }

    #[test]
    fn test_token_encode() {
        let spender = Token::Address("ccc2b1ad21666a5847a804a73a41f904c4a4a0ec".parse().unwrap());
        let amount = Token::Uint(U256::max_value());

        let encoded = encode(&vec![spender, amount]);
        let hex_encoded = hex::encode(encoded);

        assert_eq!(
            hex_encoded,
            "000000000000000000000000ccc2b1ad21666a5847a804a73a41f904c4a4a0ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
        );
    }
}
