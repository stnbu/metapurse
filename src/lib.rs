use bevy::prelude::*;

use bevy_egui::{egui, EguiContext, EguiPlugin};
use metamask_bevy::*;

use wasm_bindgen::prelude::*;

#[macro_use]
mod console;

//extern crate console;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let mut app = App::new();

    #[cfg(target_arch = "wasm32")]
    app.add_system(handle_browser_resize);

    app.add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(MetaMaskPlugin)
        .add_system(ui_example)
        //.add_system(anykey)
        .run();
    Ok(())
}

fn ui_example(
    mut egui_context: ResMut<EguiContext>,
    metamask_ch: ResMut<MetamaskChannel>,
    app_data: Res<AppData>,
    mut app_state: ResMut<State<AppState>>,
) {
    egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
        let addr_tx = metamask_ch.addr_tx.clone();
        let sign_tx = metamask_ch.sign_tx.clone();

        ui.label("Hello. <tap-tap>. Is this thing on? World?");

        if app_data.no_metamask {
            panic!("You have no Metamask. I'm a bird.");
        }
        if ui.button("metamask").clicked() {
            app_state.set(AppState::LoadingAddr).unwrap();
            wasm_bindgen_futures::spawn_local(async move {
                request_account(&addr_tx).await;
            });
        }
        match app_data.user_wallet_addr {
            Some(_addr) => {
                ui.label("yes wallet");
            }
            None => {
                ui.label("no wallet");
            }
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
