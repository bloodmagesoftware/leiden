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

use crate::helper::settings::UserSettings;

#[cfg(feature = "sdl")]
struct SdlWrapper {
    gamepad_subsystem: sdl2::GameControllerSubsystem,
}

#[cfg(feature = "sdl")]
#[derive(Resource)]
struct GlobalRumble {
    low_frequency: u16,
    high_frequency: u16,
}

#[cfg(feature = "sdl")]
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

#[cfg(feature = "sdl")]
static mut SDL_WRAPPER: Option<std::sync::Mutex<SdlWrapper>> = None;

#[cfg(feature = "sdl")]
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

#[cfg(feature = "sdl")]
#[derive(Component)]
pub struct Rumble {
    timer: Timer,
    low_frequency: u16,
    high_frequency: u16,
}

#[cfg(not(feature = "sdl"))]
#[derive(Component)]
pub struct Rumble {
    duration: std::time::Duration,
    low_frequency: f32,
    high_frequency: f32,
}

#[cfg(feature = "sdl")]
impl Rumble {
    pub fn new(low_frequency: f32, high_frequency: f32, duration_millis: u64) -> Self {
        Self {
            timer: Timer::from_seconds(duration_millis as f32 / 1000.0, TimerMode::Once),
            low_frequency: (low_frequency * 0xFFFF as f32) as u16,
            high_frequency: (high_frequency * 0xFFFF as f32) as u16,
        }
    }
}

#[cfg(not(feature = "sdl"))]
impl Rumble {
    pub fn new(low_frequency: f32, high_frequency: f32, duration_millis: u64) -> Self {
        Self {
            duration: std::time::Duration::from_millis(duration_millis),
            low_frequency,
            high_frequency,
        }
    }
}

#[cfg(feature = "sdl")]
fn update_rumbles(
    mut commands: Commands,
    time: Res<Time>,
    mut rumbles: ResMut<GlobalRumble>,
    mut rumble_query: Query<(Entity, &mut Rumble)>,
    user_settings: Res<UserSettings>,
) {
    if !user_settings.vibration {
        if rumbles.high_frequency != 0 || rumbles.low_frequency != 0 {
            with_sdl(|sdl| sdl.set_rumble(0, 0, 1000));
            rumbles.high_frequency = 0;
            rumbles.low_frequency = 0;
        }
        return;
    }

    let mut high_frequency = 0u16;
    let mut low_frequency = 0u16;

    for (e, mut rumble) in rumble_query.iter_mut() {
        rumble.timer.tick(time.delta());

        if rumble.timer.just_finished() {
            commands.entity(e).despawn();
        } else {
            if low_frequency < rumble.low_frequency {
                low_frequency = rumble.low_frequency;
            }
            if high_frequency < rumble.high_frequency {
                high_frequency = rumble.high_frequency;
            }
        }
    }

    if high_frequency != rumbles.high_frequency || low_frequency != rumbles.low_frequency {
        with_sdl(|sdl| sdl.set_rumble(low_frequency, high_frequency, 1000));
        rumbles.high_frequency = high_frequency;
        rumbles.low_frequency = low_frequency;
    }
}

#[cfg(not(feature = "sdl"))]
fn update_rumbles(
    mut commands: Commands,
    mut rumble_q: Query<(Entity, &mut Rumble)>,
    gamepads: Res<Gamepads>,
    mut evw_rumble: EventWriter<bevy::input::gamepad::GamepadRumbleRequest>,
    user_settings: Res<UserSettings>,
) {
    if !user_settings.vibration {
        for (e, _) in rumble_q.iter_mut() {
            commands.entity(e).despawn();
        }
        return;
    }

    for (e, rumble) in rumble_q.iter_mut() {
        for gamepad in gamepads.iter() {
            evw_rumble.send(bevy::input::gamepad::GamepadRumbleRequest::Add {
                gamepad,
                duration: rumble.duration,
                intensity: bevy::input::gamepad::GamepadRumbleIntensity {
                    strong_motor: rumble.high_frequency,
                    weak_motor: rumble.low_frequency,
                },
            });
        }
        commands.entity(e).despawn();
    }
}

pub struct SdlPlugin;

impl Plugin for SdlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_rumbles);

        #[cfg(feature = "sdl")]
        {
            match SdlWrapper::new() {
                Ok(sdl_wrapper) => unsafe {
                    SDL_WRAPPER = Some(std::sync::Mutex::new(sdl_wrapper));
                    app.insert_resource(GlobalRumble {
                        low_frequency: 0,
                        high_frequency: 0,
                    });
                },
                Err(err) => {
                    error!("Error initializing SDL: {}", err);
                }
            }
        }
    }
}
