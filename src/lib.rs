use hex_literal::hex;

use bevy::prelude::*;
use bevy::input::keyboard::KeyboardInput;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Press Any Key To Win".to_string(),
            width: 400.,
            height: 300.,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
	.add_system(anykey)
        .run();
    Ok(())
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(5.0),
                left: Val::Px(15.0),
                ..default()
            },
            ..default()
        },
        text: Text::with_section(
            "The Anykey Game!\nPress Any Key To Win!",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 20.0,
                color: Color::WHITE,
            },
            TextAlignment {
                horizontal: HorizontalAlign::Center,
                ..default()
            },
        ),
        ..default()
    });
}

macro_rules! print {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

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

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}



fn anykey(
    mut key_evr: EventReader<KeyboardInput>,
) {
    use bevy::input::ElementState;
    for ev in key_evr.iter() {
        match ev.state {
            ElementState::Pressed => {},
            ElementState::Released => {
		if let Some(key_code) = ev.key_code {
                    print!("{:?}", key_code);
		}
            }
        }
    }
}
