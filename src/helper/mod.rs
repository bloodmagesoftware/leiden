/*
 * Leiden - A video game
 * Copyright (C) 2024  Frank Mayer
 *
 * This file is part of Leiden.
 *
 * Leiden is licensed under the terms of the custom license available at:
 * https://github.com/bloodmagesoftware/leiden/blob/main/LICENSE
 *
 * Unauthorized copying, modification, distribution, or use of this software, via any medium, is strictly prohibited.
 */

use bevy::prelude::{App, Plugin};

mod cursor;
mod ext;
pub mod settings;
pub mod system;
pub mod ui;

pub struct HelperPlugin;

impl Plugin for HelperPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(cursor::CursorPlugin)
            .add_plugins(settings::SettingsPlugin)
            .add_plugins(ui::UiPlugin);
    }
}
