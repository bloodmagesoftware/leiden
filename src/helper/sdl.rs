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
use bevy::app::App;
use bevy::log::error;
use bevy::prelude::{Plugin, Resource};

#[derive(Resource)]
pub struct SdlWrapper {
    sdl_context: sdl3::Sdl,
    gamepad_subsystem: sdl3::GamepadSubsystem,
}

impl SdlWrapper {
    pub fn new() -> Result<Self, String> {
        sdl3::hint::set("SDL_JOYSTICK_THREAD", "1");

        let sdl_context = sdl3::init()?;
        let gamepad_subsystem = sdl_context.gamepad()?;

        Ok(Self {
            sdl_context,
            gamepad_subsystem,
        })
    }

    pub fn rumble(&self, low_frequency: u16, high_frequency: u16, duration: u32) {
        let available = match self.gamepad_subsystem.num_gamepads() {
            Ok(available) => available,
            Err(_) => return,
        };

        for i in 0..available {
            let mut game_controller = match self.gamepad_subsystem.open(i) {
                Ok(game_controller) => game_controller,
                Err(_) => continue,
            };

            if game_controller.has_rumble() {
                match game_controller.set_rumble(low_frequency, high_frequency, duration) {
                    Ok(_) => {}
                    Err(err) => {
                        error!(
                            "Error setting rumble for {:?} : {}",
                            game_controller.name(),
                            err
                        );
                    }
                }
            }
        }
    }
}

pub struct SdlPlugin;

impl Plugin for SdlPlugin {
    fn build(&self, app: &mut App) {
        match SdlWrapper::new() {
            Ok(sdl_wrapper) => {
                app.insert_resource(sdl_wrapper);
            }
            Err(err) => {
                error!("Error initializing SDL: {}", err);
            }
        }
    }
}
