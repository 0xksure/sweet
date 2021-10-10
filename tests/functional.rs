#![cfg(feature = "test-bpf")]

use solana_program::pubkey::Pubkey;
use solana_program_test::{self::*, ProgramTest};
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::{Transaction, TransactionError},
};
use sweet::*;

fn program_test() -> ProgramTest {
    ProgramTest::new("sweet", id(), processor!(process_instruction))
}

#[tokio::test]
async fn test_sweet_program() {
    let data = "0hello".as_bytes();
    let (mut bank_client, payer, recent_blockhash) = program_test().start().await;
    let keypairs = vec![Keypair::new(), Keypair::new()];
    let pubkeys: Vec<Pubkey> = keypairs.iter().map(|keypair| keypair.pubkey()).collect();

    let signer_key_refs: Vec<&Pubkey> = pubkeys.iter().collect();
    let mut transaction = Transaction::new_with_payer(
        &[build_sweet_instruction(data, &signer_key_refs)],
        Some(&payer.pubkey()),
    );

    let mut signers = vec![&payer];
    for keypair in keypairs.iter() {
        signers.push(keypair);
    }
}
