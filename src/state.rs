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
use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    Splash,
    Menu,
    MainSettings,
    Game,
}

impl Default for AppState {
    fn default() -> Self {
        #[cfg(feature = "dev")]
        {
            AppState::Menu
        }
        #[cfg(not(feature = "dev"))]
        {
            AppState::Splash
        }
    }
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum InputState {
    #[default]
    MouseAndKeyboard,
    Gamepad,
}

fn update_input_state(mut next_state: ResMut<NextState<InputState>>, gamepads: Res<Gamepads>) {
    if gamepads.iter().next().is_some() {
        next_state.set(InputState::Gamepad);
    } else {
        next_state.set(InputState::MouseAndKeyboard);
    }
}

#[derive(States, Debug, Default, PartialEq, Eq, Hash, Clone)]
pub enum ButtonFocusState {
    #[default]
    None,
    Id(i8),
}

impl ButtonFocusState {
    pub fn set(&mut self, id: i8) {
        *self = ButtonFocusState::Id(id);
    }

    pub fn clear(&mut self) {
        *self = ButtonFocusState::None;
    }

    pub fn is_id(&self, id: i8) -> bool {
        match self {
            ButtonFocusState::Id(focus_id) => *focus_id == id,
            _ => false,
        }
    }

    pub fn is_none(&self) -> bool {
        matches!(self, ButtonFocusState::None)
    }

    pub fn get_id(&self) -> Option<i8> {
        match self {
            ButtonFocusState::Id(focus_id) => Some(*focus_id),
            _ => None,
        }
    }
}

pub struct AppStatePlugin;

impl Plugin for AppStatePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(AppState::default());

        app.insert_state(InputState::default())
            .add_systems(PreUpdate, update_input_state);

        app.insert_state(ButtonFocusState::default());
    }
}
