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

use crate::helper::sdl::Rumble;
use crate::helper::settings::{AudioChannel, UserSettings};
use crate::helper::system::{ui_interact_just_pressed, ui_nav_back_just_pressed};
use crate::helper::ui::{Fade, text_button, vertical_spacer};
use crate::state::{AppState, ButtonFocusState};

#[derive(Component)]
struct SettingsMarker;

#[derive(Component)]
struct VolumeSettingsMarker(AudioChannel);

#[cfg(feature = "rumble")]
#[derive(Component)]
struct VibrationSettingsMarker;

fn spawn_settings(
    mut commands: Commands,
    settings: Res<UserSettings>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Fade::bundle());

    commands
        .spawn((
            SettingsMarker,
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.0),
                    top: Val::Px(0.0),
                    width: Val::Vw(100.0),
                    height: Val::Vh(100.0),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Settings",
                TextStyle {
                    font_size: 64.0,
                    font: asset_server.load("fonts/Ornatix.ttf"),
                    color: Color::WHITE,
                },
            ));

            vertical_spacer(parent, Val::VMin(4.0));

            text_button(parent, "Back", 0);

            vertical_spacer(parent, Val::VMin(2.0));

            text_button(parent, "+", 1);
            parent.spawn((
                VolumeSettingsMarker(AudioChannel::Master),
                TextBundle::from_section(
                    format!("Master Volume: {}", settings.master_volume),
                    TextStyle {
                        color: Color::WHITE,
                        ..default()
                    },
                ),
            ));
            text_button(parent, "-", 2);

            vertical_spacer(parent, Val::VMin(2.0));

            text_button(parent, "+", 3);
            parent.spawn((
                VolumeSettingsMarker(AudioChannel::Music),
                TextBundle::from_section(
                    format!("Music Volume: {}", settings.music_volume),
                    TextStyle {
                        color: Color::WHITE,
                        ..default()
                    },
                ),
            ));
            text_button(parent, "-", 4);

            vertical_spacer(parent, Val::VMin(2.0));

            text_button(parent, "+", 5);
            parent.spawn((
                VolumeSettingsMarker(AudioChannel::Sfx),
                TextBundle::from_section(
                    format!("Sfx Volume: {}", settings.sfx_volume),
                    TextStyle {
                        color: Color::WHITE,
                        ..default()
                    },
                ),
            ));
            text_button(parent, "-", 6);

            vertical_spacer(parent, Val::VMin(2.0));

            text_button(parent, "+", 7);
            parent.spawn((
                VolumeSettingsMarker(AudioChannel::Ui),
                TextBundle::from_section(
                    format!("Ui Volume: {}", settings.ui_volume),
                    TextStyle {
                        color: Color::WHITE,
                        ..default()
                    },
                ),
            ));
            text_button(parent, "-", 8);

            #[cfg(feature = "rumble")]
            {
                vertical_spacer(parent, Val::VMin(2.0));

                text_button(
                    parent,
                    format!(
                        "Gamepad Vibration: {}",
                        if settings.vibration { "On" } else { "Off" }
                    ),
                    9,
                )
                .insert(VibrationSettingsMarker);
            }
        });
}

fn update_volume_settings(
    mut volume_q: Query<(&VolumeSettingsMarker, &mut Text)>,
    settings: Res<UserSettings>,
) {
    for (marker, mut text) in volume_q.iter_mut() {
        match marker.0 {
            AudioChannel::Master => {
                text.sections[0].value =
                    format!("Master Volume: {:.0}%", settings.master_volume * 100.0);
            }
            AudioChannel::Music => {
                text.sections[0].value =
                    format!("Music Volume: {:.0}%", settings.music_volume * 100.0);
            }
            AudioChannel::Sfx => {
                text.sections[0].value = format!("Sfx Volume: {:.0}%", settings.sfx_volume * 100.0);
            }
            AudioChannel::Ui => {
                text.sections[0].value = format!("Ui Volume: {:.0}%", settings.ui_volume * 100.0);
            }
        }
    }
}

#[cfg(feature = "rumble")]
fn update_vibration_settings(
    vibration_button_q: Query<&Children, (With<Button>, With<VibrationSettingsMarker>)>,
    mut vibration_text_q: Query<&mut Text, (Without<Button>, Without<VibrationSettingsMarker>)>,
    settings: Res<UserSettings>,
) {
    for children in vibration_button_q.iter() {
        for child in children.iter() {
            for text in vibration_text_q.get_mut(*child).iter_mut() {
                text.sections[0].value = format!(
                    "Gamepad Vibration: {}",
                    if settings.vibration { "On" } else { "Off" }
                );
            }
        }
    }
}

fn despawn_settings(mut commands: Commands, query: Query<Entity, With<SettingsMarker>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn button(
    #[cfg(feature = "rumble")] mut commands: Commands,
    button_state: Res<State<ButtonFocusState>>,
    mut button_next_state: ResMut<NextState<ButtonFocusState>>,
    mut app_next_state: ResMut<NextState<AppState>>,
    mut settings: ResMut<UserSettings>,
) {
    if let ButtonFocusState::Id(focus_id) = button_state.get() {
        match focus_id {
            0 => {
                // Back
                app_next_state.set(AppState::Menu);
                button_next_state.set(ButtonFocusState::None);
            }
            1 => {
                // Master Volume +
                settings.master_volume = (settings.master_volume + 0.05).min(1.0);
            }
            2 => {
                // Master Volume -
                settings.master_volume = (settings.master_volume - 0.05).max(0.0);
            }
            3 => {
                // Music Volume +
                settings.music_volume = (settings.music_volume + 0.05).min(1.0);
            }
            4 => {
                // Music Volume -
                settings.music_volume = (settings.music_volume - 0.05).max(0.0);
            }
            5 => {
                // Sfx Volume +
                settings.sfx_volume = (settings.sfx_volume + 0.05).min(1.0);
            }
            6 => {
                // Sfx Volume -
                settings.sfx_volume = (settings.sfx_volume - 0.05).max(0.0);
            }
            7 => {
                // Ui Volume +
                settings.ui_volume = (settings.ui_volume + 0.05).min(1.0);
            }
            8 => {
                // Ui Volume -
                settings.ui_volume = (settings.ui_volume - 0.05).max(0.0);
            }
            #[cfg(feature = "rumble")]
            9 => {
                // Vibration
                settings.vibration = !settings.vibration;
                if settings.vibration {
                    commands.spawn(Rumble::new(0xffff, 0xffff, 0.2));
                }
            }
            _ => {}
        }
    }
}

fn to_menu(
    mut app_next_state: ResMut<NextState<AppState>>,
    mut button_next_state: ResMut<NextState<ButtonFocusState>>,
) {
    app_next_state.set(AppState::Menu);
    button_next_state.set(ButtonFocusState::None);
}

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainSettings), spawn_settings)
            .add_systems(
                Update,
                update_volume_settings.run_if(in_state(AppState::MainSettings)),
            );
        #[cfg(feature = "rumble")]
        app.add_systems(
            Update,
            update_vibration_settings.run_if(in_state(AppState::MainSettings)),
        );
        app.add_systems(OnExit(AppState::MainSettings), despawn_settings)
            .add_systems(
                Update,
                to_menu
                    .run_if(in_state(AppState::MainSettings))
                    .run_if(ui_nav_back_just_pressed()),
            )
            .add_systems(
                Update,
                button
                    .run_if(in_state(AppState::MainSettings))
                    .run_if(ui_interact_just_pressed()),
            );
    }
}
