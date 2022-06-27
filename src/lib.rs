use hex_literal::hex;

use wasm_bindgen::prelude::*;

macro_rules! print {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    let _ = env_logger::try_init();
    let transport = web3::transports::Http::new("http://localhost:8545").unwrap();
    let web3 = web3::Web3::new(transport);

    print!("Calling accounts.");
    let mut accounts = web3.eth().accounts().await.unwrap();
    print!("Accounts: {:?}", accounts);
    accounts.push(hex!("00a329c0648769a73afac7f9381e08fb43dbea72").into());

    print!("Calling balance.");
    for account in accounts {
        let balance = web3.eth().balance(account, None).await.unwrap();
        print!("Balance of {:?}: {}", account, balance);
    }
    Ok(())
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
