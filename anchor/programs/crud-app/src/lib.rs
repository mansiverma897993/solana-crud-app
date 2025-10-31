#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("Count3AcZucFDPSFBAeHkQ6AvttieKUkyJ8HiQGhQwe");

#[program]
pub mod counter {
    use super::*;

   
}

#[account]
#[derive(InitSpace)]

pub struct JournalEntryState {\
    pub  owner: Pubkey,
    pub title: String
    pub title: String,
    pub message: String,
}

