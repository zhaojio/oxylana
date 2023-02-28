use anchor_client::{Client, Cluster, Program};
use anchor_lang::{__private::base64, system_program};
use oxylana::RustStation;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair, Signature},
    signer::Signer,
};

use solana_transaction_status::{
    EncodedTransaction, TransactionBinaryEncoding, UiTransactionEncoding, UiTransactionReturnData,
};

use std::{error::Error, rc::Rc};

#[test]
fn test_contract() -> Result<(), Box<dyn Error>> {
    println!("init");
    // Initialize client and program
    let key: Rc<Keypair> =
        Rc::new(read_keypair_file("/home/zhaojie/.config/solana/id.json").unwrap());
    let client: Client = Client::new(Cluster::Localnet, Rc::clone(&key) as Rc<dyn Signer>);
    let program: Program = client.program(oxylana::ID);

    // Build, sign, and send program instruction
    let rust_station: Pubkey = RustStation::get_pda(&key.pubkey());
    println!("{}", rust_station);
    let r = program.rpc().get_account(&rust_station);
    println!("{}", r.unwrap().lamports);
    let sig: Signature = program
        .request()
        .accounts(oxylana::accounts::Flip { rust_station })
        .args(oxylana::instruction::Flip {/* this ix has no args */})
        .payer(Rc::clone(&key) as Rc<dyn Signer>)
        .signer(&*key)
        .send()?;


    let tx = program
        .rpc()
        .get_transaction(&sig, UiTransactionEncoding::JsonParsed);

    let txx = tx.unwrap();

    let mete = txx.transaction.meta.unwrap();

    let _logs = mete.clone().log_messages;
    let _logs: Option<Vec<String>> = _logs.into();
    println!("{:?}", _logs.unwrap());

    let _res: Option<UiTransactionReturnData> = mete.clone().return_data.into();

    let res = base64::decode(_res.unwrap().data.0);
    // let res = String::from_utf8(res.unwrap());

    println!("return_data:{:?}", res);

    println!("demo sig: {sig}");

    // // Retrieve and validate state
    // let rust_station_account: RustStation = program.account(rust_station)?;
    assert!(false);

    println!("hello");
    Ok(())
}
