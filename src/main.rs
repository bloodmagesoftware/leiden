/*
 * Leiden - A video game
 * Copyright (C) 2024  Frank Mayer
 *
 * This file is part of Leiden.
 *
 * Fair Core License, Version 1.0, Apache 2.0 Future License
 * https://github.com/bloodmagesoftware/leiden/blob/main/LICENSE.md
 */

use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::prelude::EntropyPlugin;

mod camera;
mod helper;
mod menu;
mod settings;
mod splash;
mod state;

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Leiden".into(),
                    name: Some("Leiden".into()),
                    present_mode: bevy_window::PresentMode::AutoVsync,
                    #[cfg(not(feature = "dev"))]
                    mode: bevy_window::WindowMode::BorderlessFullscreen,
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    window_theme: Some(bevy_window::WindowTheme::Dark),
                    ..default()
                }),
                ..default()
            }),
    )
    .insert_resource(ClearColor(Color::BLACK))
    .add_plugins(EntropyPlugin::<WyRand>::default())
    .add_plugins(camera::CameraPlugin)
    .add_plugins(helper::HelperPlugin)
    .add_plugins(menu::MenuPlugin)
    .add_plugins(settings::SettingsPlugin)
    .add_plugins(splash::SplashPlugin)
    .add_plugins(state::AppStatePlugin);

    app.run();
}
