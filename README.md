# near2aurora
Helper module to call AURORA smart contracts from NEAR.

## API
```rust
fn call(
    aurora_account_id: &AccountId,
    contract_address: &String,
    method: &Function,
    arguments: &Vec<Token>,
    value: Option<Wei>,
    gas: Option<Gas>,
) -> Promise;
```

## Example
This example shows how to call the `approve` method on ERC20 tokens.

The method signature : `function approve(address spender, uint256 amount) external returns (bool)`

To do so, you need to first define the method via `ethabi` exported by `near2aurora`:
```rust
let approve = near2aurora::Function {
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
```

Then prepare the call args:
```rust
let args = vec![
    Token::Address("6105fa59830085aa0ce57dbf5063af2c1d766b6b".parse().unwrap()),
    Token::Uint(10000000000.into()),
]
```

And finally make the call:
```rust
near2aurora::call(
    &AccountId::new_unchecked("aurora".into()),
    &contract_address,
    &approve,
    &args,
    None,
    None
);
```
