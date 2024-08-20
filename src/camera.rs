/*
 * Leiden - A video game
 * Copyright (C) 2024  Frank Mayer
 *
 * This file is part of Leiden.
 *
 * Fair Core License, Version 1.0, Apache 2.0 Future License
 * https://github.com/bloodmagesoftware/leiden/blob/main/LICENSE.md
 */

use bevy::app::PreStartup;
use bevy::prelude::{App, Camera2dBundle, Commands};

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub struct CameraPlugin;

impl bevy::app::Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, spawn_camera);
    }
}
