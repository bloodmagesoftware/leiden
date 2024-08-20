/*
 * Leiden - A video game
 * Copyright (C) 2024  Frank Mayer
 *
 * This file is part of Leiden.
 *
 * Fair Core License, Version 1.0, Apache 2.0 Future License
 * https://github.com/bloodmagesoftware/leiden/blob/main/LICENSE.md
 */

use bevy::prelude::{App, Plugin};

mod cursor;
mod ext;
pub mod sdl;
pub mod settings;
pub mod system;
pub mod ui;

pub struct HelperPlugin;

impl Plugin for HelperPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(cursor::CursorPlugin)
            .add_plugins(sdl::SdlPlugin)
            .add_plugins(settings::SettingsPlugin)
            .add_plugins(ui::UiPlugin);
    }
}
