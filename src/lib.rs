use hex_literal::hex;

use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;

use bevy_egui::{egui, EguiContext, EguiPlugin};
use metamask_bevy::*;

use wasm_bindgen::prelude::*;

// #[wasm_bindgen(start)]
// pub fn start() -> Result<(), JsValue> {
//     App::new()
//         .insert_resource(WindowDescriptor {
//             title: "Press Any Key To Win".to_string(),
//             width: 400.,
//             height: 300.,
//             ..default()
//         })
//         .add_plugins(DefaultPlugins)
//         .add_plugin(MetaMaskPlugin)
//         .add_startup_system(setup)
//         .add_system(anykey)
//         .run();
//     Ok(())
// }

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let mut app = App::new();

    #[cfg(target_arch = "wasm32")]
    app.add_system(handle_browser_resize);

    app.add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(MetaMaskPlugin)
        .add_system(ui_example)
        .add_system(anykey)
        .run();
    Ok(())
}

// ----
// fn main() {
//     let mut app = App::new();
//     #[cfg(target_arch = "wasm32")]
//     app.add_system(handle_browser_resize);
//     app.add_plugins(DefaultPlugins)
//         .add_plugin(EguiPlugin)
//         .add_plugin(metamask::MetaMaskPlugin)
//         .add_system(ui_example)
//         .run();
// }
// ----

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let chaching = asset_server.load("sounds/chaching.ogg");
    commands.insert_resource(ChaChing(chaching));

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

fn anykey(mut key_evr: EventReader<KeyboardInput>, audio: Res<Audio>, sound: Res<ChaChing>) {
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

// ------------------

fn ui_example(
    mut egui_context: ResMut<EguiContext>,
    metamask_ch: ResMut<MetamaskChannel>,
    app_data: Res<AppData>,
    mut app_state: ResMut<State<AppState>>,
) {
    egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
        let addr_tx = metamask_ch.addr_tx.clone();
        let sign_tx = metamask_ch.sign_tx.clone();

        if !app_data.no_metamask {
            if ui.button("metamask").clicked() {
                app_state.set(AppState::LoadingAddr).unwrap();
                wasm_bindgen_futures::spawn_local(async move {
                    request_account(&addr_tx).await;
                });
            }
            if let Some(addr) = &app_data.user_wallet_addr {
                let addr = addr.clone();
                ui.label(addr.to_string());
                if ui.button("Sign a text").clicked() {
                    app_state.set(AppState::LoadingSign).unwrap();
                    wasm_bindgen_futures::spawn_local(async move {
                        sign_a_string(&sign_tx, &addr).await;
                    })
                }
            }

            if let Some(signed) = &app_data.signed {
                ui.label(signed);
            }
        } else {
            ui.label("no metamask");
        }
    });
}

#[cfg(target_arch = "wasm32")]
fn handle_browser_resize(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    let wasm_window = web_sys::window().unwrap();
    let (target_width, target_height) = (
        wasm_window.inner_width().unwrap().as_f64().unwrap() as f32,
        wasm_window.inner_height().unwrap().as_f64().unwrap() as f32,
    );

    if window.width() != target_width || window.height() != target_height {
        window.set_resolution(target_width, target_height);
    }
}
