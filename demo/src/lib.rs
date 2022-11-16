
use near2aurora::{Param, Argument};
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    near_bindgen, PanicOnDefault, AccountId, json_types::U128,
};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Demo {
    aurora: AccountId
}

#[near_bindgen]
impl Demo {
    #[init]
    pub fn new(
        aurora: String
    ) -> Self {
        Self { aurora: AccountId::new_unchecked(aurora) }
    }

    pub fn info(&self) -> AccountId {
        self.aurora.clone()
    }

    pub fn approve(
        &self,
        token: String,
        amount: U128,
        spender: String,
    ) {
        let method = near2aurora::Function {
            name: "approve".to_string(),
            inputs: vec![
                Param {
                    name: "spender".to_string(),
                    kind: near2aurora::ParamType::Address,
                    internal_type: None,
                },
                Param {
                    name: "amount".to_string(),
                    kind: near2aurora::ParamType::Uint(256),
                    internal_type: None,
                }
            ],
            outputs: vec![
                Param {
                    name: "result".into(),
                    kind: near2aurora::ParamType::Bool,
                    internal_type: None,
                }
            ],
            constant: None,
            state_mutability: near2aurora::StateMutability::NonPayable
        };

        near2aurora::call(
            &self.aurora,
            &token,
            &method,
            &vec![Argument::Address(spender), Argument::Uint256(amount.into())],
            None,
            None,
        );
    }
}
