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

use bevy::ecs::schedule::Condition;
use bevy::prelude::*;

use crate::state::ButtonFocusState;

pub fn ui_interact_just_pressed() -> impl Condition<()> {
    IntoSystem::into_system(
        |button_state: Res<State<ButtonFocusState>>,
         mouse_buttons: Res<ButtonInput<MouseButton>>,
         gamepads: Res<Gamepads>,
         gamepad_buttons: Res<ButtonInput<GamepadButton>>| {
            if button_state.is_none() {
                return false;
            }

            if mouse_buttons.just_pressed(MouseButton::Left) {
                return true;
            }

            for gamepad in gamepads.iter() {
                let south_button = GamepadButton {
                    gamepad,
                    button_type: GamepadButtonType::South,
                };
                if gamepad_buttons.just_pressed(south_button) {
                    return true;
                }
            }

            false
        },
    )
}

pub fn ui_nav_back_just_pressed() -> impl Condition<()> {
    IntoSystem::into_system(
        |keyboard_buttons: Res<ButtonInput<KeyCode>>,
         gamepads: Res<Gamepads>,
         gamepad_buttons: Res<ButtonInput<GamepadButton>>| {
            if keyboard_buttons.just_pressed(KeyCode::Escape) {
                return true;
            }

            for gamepad in gamepads.iter() {
                let east_button = GamepadButton {
                    gamepad,
                    button_type: GamepadButtonType::East,
                };
                if gamepad_buttons.just_pressed(east_button) {
                    return true;
                }
            }

            false
        },
    )
}
