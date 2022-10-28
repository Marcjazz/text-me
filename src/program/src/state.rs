use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Contact {
    address: Pubkey,
    is_verified: bool
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct User {
    username: String,
    contacts: Vec<Contact>
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Message {
    message: String,
    is_read: bool,
    send_at: u32,
    recieved_at: u32,
    send_by: Pubkey,
    send_to: Pubkey
}