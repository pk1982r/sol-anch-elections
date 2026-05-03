use std::str::FromStr;

use anchor_client::anchor_lang::solana_program;
use anchor_client::solana_sdk::signature::{Keypair, Signer};
use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig, pubkey::Pubkey, signature::read_keypair_file,
    },
    Client, Cluster,
};
use std::rc::Rc;

#[test]
fn test_initialize() {
    let program_id = "GaQ54Jta5EcPTbizuNAA6Cq1DZbxUUVfhTJoHSzC8roF";
    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    let payer = Rc::new(read_keypair_file(&anchor_wallet).unwrap());

    let client = Client::new_with_options(
        Cluster::Localnet,
        payer.clone(),
        CommitmentConfig::confirmed(),
    );

    let program_id = Pubkey::from_str(program_id).unwrap();
    let program = client.program(program_id).unwrap();

    let new_account = Keypair::new();

    let tx = program
        .request()
        .accounts(sol_anch_elections::accounts::Initialize {
            new_account: new_account.pubkey(),
            signer: payer.pubkey(),
            system_program: solana_program::system_program::ID,
        })
        .signer(&new_account)
        .signer(payer.as_ref())
        .send()
        .expect("");

    println!("Your transaction signature {}", tx);
}
