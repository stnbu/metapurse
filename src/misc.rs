// TBD's
//
// I don't want to fish through git history, so I put it here.

pub async fn _start() -> Result<(), JsValue> {
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

fn _anykey(mut key_evr: EventReader<KeyboardInput>, audio: Res<Audio>, sound: Res<ChaChing>) {
    use bevy::input::ElementState;
    for ev in key_evr.iter() {
        match ev.state {
            ElementState::Pressed => {}
            ElementState::Released => {
                if let Some(key_code) = ev.key_code {
                    print!("{:?}", key_code);
                    audio.play(sound.0.clone());
                }
            }
        }
    }
}

struct ChaChing(Handle<AudioSource>);

macro_rules! print {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
