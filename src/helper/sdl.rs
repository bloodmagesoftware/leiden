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

use std::sync::Mutex;

use bevy::app::App;
use bevy::prelude::{
    Commands, Component, Entity, Plugin, Query, Res, ResMut, Resource, Time, Update,
};
use bevy::time::{Timer, TimerMode};
use log::{error, info};

struct SdlWrapper {
    gamepad_subsystem: sdl2::GameControllerSubsystem,
}

#[derive(Resource)]
struct GlobalRumble {
    low_frequency: u16,
    high_frequency: u16,
}

impl SdlWrapper {
    pub fn new() -> Result<Self, String> {
        sdl2::hint::set("SDL_JOYSTICK_THREAD", "1");

        let sdl_context = sdl2::init()?;
        let gamepad_subsystem = sdl_context.game_controller()?;

        Ok(Self { gamepad_subsystem })
    }

    pub fn set_rumble(&self, low_frequency: u16, high_frequency: u16, duration: u32) {
        let available = match self.gamepad_subsystem.num_joysticks() {
            Ok(available) => available,
            Err(err) => {
                error!("Error getting number of gamepads: {}", err);
                return;
            }
        };

        for i in 0..available {
            let mut game_controller = match self.gamepad_subsystem.open(i) {
                Ok(game_controller) => game_controller,
                Err(err) => {
                    error!("Error opening game controller {}: {}", i, err);
                    continue;
                }
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
            } else {
                info!("No rumble for {:?}", game_controller.name());
            }
        }
    }
}

static mut SDL_WRAPPER: Option<Mutex<SdlWrapper>> = None;

fn with_sdl<F>(f: F)
where
    F: FnOnce(&SdlWrapper),
{
    unsafe {
        if let Some(sdl_wrapper) = &SDL_WRAPPER {
            if let Ok(sdl_wrapper) = sdl_wrapper.lock() {
                f(&sdl_wrapper);
            } else {
                error!("Error locking SDL wrapper");
            }
        } else {
            error!("SDL wrapper not initialized");
        }
    }
}

#[derive(Component)]
pub struct Rumble {
    pub timer: Timer,
    pub low_frequency: u16,
    pub high_frequency: u16,
}

impl Rumble {
    pub fn new(low_frequency: u16, high_frequency: u16, duration: f32) -> Self {
        Self {
            timer: Timer::from_seconds(duration, TimerMode::Once),
            low_frequency,
            high_frequency,
        }
    }
}

fn update_rumbles(
    mut commands: Commands,
    time: Res<Time>,
    mut rumbles: ResMut<GlobalRumble>,
    mut rumble_query: Query<(Entity, &mut Rumble)>,
) {
    let mut high_frequency = 0u16;
    let mut low_frequency = 0u16;

    for (e, mut rumble) in rumble_query.iter_mut() {
        rumble.timer.tick(time.delta());

        if rumble.timer.just_finished() {
            commands.entity(e).despawn();
        } else {
            low_frequency += rumble.low_frequency;
            high_frequency += rumble.high_frequency;
        }
    }

    if high_frequency != rumbles.high_frequency || low_frequency != rumbles.low_frequency {
        with_sdl(|sdl| sdl.set_rumble(low_frequency, high_frequency, 1000));
        rumbles.high_frequency = high_frequency;
        rumbles.low_frequency = low_frequency;
    }
}

pub struct SdlPlugin;

impl Plugin for SdlPlugin {
    fn build(&self, app: &mut App) {
        match SdlWrapper::new() {
            Ok(sdl_wrapper) => unsafe {
                SDL_WRAPPER = Some(Mutex::new(sdl_wrapper));
                app.insert_resource(GlobalRumble {
                    low_frequency: 0,
                    high_frequency: 0,
                })
                .add_systems(Update, update_rumbles);
            },
            Err(err) => {
                error!("Error initializing SDL: {}", err);
            }
        }
    }
}
