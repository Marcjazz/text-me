use borsh::{BorshSerialize, BorshDeserialize};
use solana_program::borsh::try_from_slice_unchecked;

#[derive(BorshSerialize, BorshDeserialize)]
pub enum TextMeInstruction {
    InviteUser,
    Register,
    Message(String),
}

impl TextMeInstruction {
    pub fn decode(data: &[u8]) -> Self {
        try_from_slice_unchecked(data).unwrap()
    }
}