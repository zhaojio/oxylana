use solana_client_wasm::solana_sdk::{pubkey::Pubkey, transaction::Transaction};
use wasm_bindgen_futures::spawn_local;
use gloo_console::log;

fn main() {
    println!("init");
    let client = solana_client_wasm::WasmClient::new("http://localhost:8899");
    println!("{:?}", client.commitment());
    
}
