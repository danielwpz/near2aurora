use near2aurora::{parse_output, Param, SubmitResult, Token};
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env,
    json_types::U128,
    near_bindgen, AccountId, PanicOnDefault, Promise,
};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Demo {
    aurora: AccountId,
}

#[near_bindgen]
impl Demo {
    #[init]
    pub fn new(aurora: String) -> Self {
        Self {
            aurora: AccountId::new_unchecked(aurora),
        }
    }

    pub fn info(&self) -> AccountId {
        self.aurora.clone()
    }

    pub fn approve(&self, token: String, amount: U128, spender: String) -> Promise {
        #[allow(deprecated)]
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
                },
            ],
            outputs: vec![Param {
                name: "result".into(),
                kind: near2aurora::ParamType::Bool,
                internal_type: None,
            }],
            constant: None,
            state_mutability: near2aurora::StateMutability::NonPayable,
        };

        near2aurora::call(
            &self.aurora,
            &token,
            &method,
            &[
                Token::Address(spender.parse().unwrap()),
                Token::Uint(amount.0.into()),
            ],
            None,
            None,
        )
        .then(Self::ext(env::current_account_id()).on_approve())
    }

    #[private]
    pub fn on_approve(
        &self,
        #[serializer(borsh)]
        #[callback_unwrap]
        result: SubmitResult,
    ) -> bool {
        match parse_output(result) {
            Result::Ok(val) => !val.is_zero(),
            Result::Err(err) => {
                env::log_str(&err);
                false
            }
        }
    }
}
