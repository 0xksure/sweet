use std::mem;

use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use byteorder::{ByteOrder, LittleEndian};
pub use solana_program;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    decode_error::DecodeError,
    entrypoint,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use thiserror::Error;

solana_program::declare_id!("SweetS4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr");

entrypoint!(process_instruction);

// Error
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum TokenError {
    #[error("Invalid instruction")]
    InvalidInstruction,
}
impl From<TokenError> for ProgramError {
    fn from(e: TokenError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for TokenError {
    fn type_of() -> &'static str {
        "TokenError"
    }
}

//WriteAccount is the data needed to write data to an account
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, BorshSchema, PartialEq)]
struct WriteAccount {
    offset: u64,
    // The data contained by the account
    data: Vec<u8>,
}

// Instruction ---------------------
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum TokenInstruction {
    Hello,
    // Input
    // 0: process
    // Accounts:
    // 1. account to be created
    WriteAccount { offset: u64, data: Vec<u8> },
}

// Processor --------------------------
pub struct Processor {}

impl Processor {
    fn process_hello(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();
        let account = next_account_info(accounts_iter)?;
        if account.owner != program_id {
            msg!("The provided acount is not owned by the program");
            return Err(ProgramError::IncorrectProgramId);
        }
        if account.try_data_len()? < mem::size_of::<u32>() {
            msg!("Data length of account must be larger than u32");
            return Err(ProgramError::InvalidAccountData);
        }
        let mut data = account.try_borrow_mut_data()?;
        let mut num_greets = LittleEndian::read_u32(&data);
        num_greets += 1;
        LittleEndian::write_u32(&mut data[0..], num_greets);
        msg!("hello");
        Ok(())
    }

    // create_account creates
    fn write_account(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        offset: u64,
        data: Vec<u8>,
    ) -> ProgramResult {
        let account_iter = &mut accounts.iter();
        let new_account = next_account_info(account_iter)?;

        if new_account.owner != program_id {
            msg!("the provided account is not owned by the program");
            return Err(ProgramError::IncorrectProgramId);
        }

        if new_account.try_data_len()? < mem::size_of::<u32>() {
            msg!("Data length of account must be larger than u32");
            return Err(ProgramError::InvalidAccountData);
        }

        msg!("create account");

        Ok(())
    }

    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
        msg!("process instruction");
        let instruction = TokenInstruction::try_from_slice(input)?;
        match instruction {
            TokenInstruction::Hello => {
                msg!("Hello hello hello");
                Self::process_hello(program_id, accounts)
            }
            TokenInstruction::WriteAccount { offset, data } => {
                msg!("create account");
                Self::write_account(program_id, accounts, offset, data)
            }
        }
    }
}

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    if let Err(error) = Processor::process(program_id, accounts, _instruction_data) {
        return Err(error);
    }
    Ok(())
}

pub fn create_sweet_instruction(data: &[u8], signer_pubkeys: &[&Pubkey]) -> Instruction {
    Instruction {
        program_id: id(),
        accounts: signer_pubkeys
            .iter()
            .map(|&pubkey| AccountMeta::new_readonly(*pubkey, true))
            .collect(),
        data: data.to_vec(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use byteorder::LittleEndian;
    use solana_program::{account_info::AccountInfo, clock::Epoch, pubkey::Pubkey};

    #[test]
    fn test_sanity() {
        let program_id = Pubkey::new_unique();
        let mut data = vec![0; mem::size_of::<u64>()];
        LittleEndian::write_u64(&mut data, 0);

        let key = Pubkey::new_unique();
        let mut lamports = 0;
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &program_id,
            false,
            Epoch::default(),
        );
        let mut _instruction_data: Vec<u8> = Vec::new();
        _instruction_data.push(0);
        let accounts = vec![account];
        let read_u64 = LittleEndian::read_u64(&accounts[0].data.borrow());
        assert_eq!(read_u64, 0);
        process_instruction(&program_id, &accounts, &_instruction_data).unwrap();
        let read_u64 = LittleEndian::read_u64(&accounts[0].data.borrow());
        assert_eq!(read_u64, 1);
        process_instruction(&program_id, &accounts, &_instruction_data).unwrap();
        let read_u64 = LittleEndian::read_u64(&accounts[0].data.borrow());
        assert_eq!(read_u64, 2);
    }

    #[test]
    fn test_create_account() {
        let program_id = Pubkey::new_unique();
        let mut data = vec![0; mem::size_of::<u64>()];
        LittleEndian::write_u64(&mut data, 0);

        let key = Pubkey::new_unique();
        let mut lamports = 0;
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &program_id,
            false,
            Epoch::default(),
        );

        let mut _instruction_data: Vec<u8> = Vec::new();
        _instruction_data.push(1);
        let accounts = [account];
        process_instruction(&program_id, &accounts, &_instruction_data);
    }
}
