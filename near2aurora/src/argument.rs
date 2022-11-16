use hex;
use near_sdk::require;

pub enum Argument {
    Address(String),
    Uint256(u128),
}

impl Argument {
    pub fn encode(&self) -> Vec<u8> {
        match self {
            Argument::Address(address) => encode_address(address),
            Argument::Uint256(v) => encode_u256(v),
        }
    }
}

fn encode_address(address: &String) -> Vec<u8> {
    require!(address.len() == 40, "Invalid aurora address");
    hex::decode(address).unwrap().to_vec()
}

fn encode_u256(value: &u128) -> Vec<u8> {
    let amount_u256 = primitive_types::U256::from(*value);
    let mut encoded_amount = [0u8; 32];
    amount_u256.to_big_endian(&mut encoded_amount);
    encoded_amount.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_address() {
        let addr1 = Argument::Address("86538ca055e7fd992a26c5604f349e2ede3ce42d".to_string());
        assert_eq!(
            addr1.encode(),
            vec![134, 83, 140, 160, 85, 231, 253, 153, 42, 38, 197, 96, 79, 52, 158, 46, 222, 60, 228, 45]
        );
    }

    #[test]
    fn test_encode_u256() {
        let v1 = Argument::Uint256(1000000000000000000);
        assert_eq!(
            v1.encode(),
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 224, 182, 179, 167, 100, 0, 0]
        );
    }
}
