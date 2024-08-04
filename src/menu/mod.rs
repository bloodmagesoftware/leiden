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
use bevy::audio::PlaybackMode;
use bevy::prelude::*;

use crate::helper::settings::{AudioChannel, DynamicAudio, DynamicAudioBundle};
use crate::helper::system::ui_interact_just_pressed;
use crate::helper::ui::{Fade, text_button, vertical_spacer};
use crate::state::{AppState, ButtonFocusState};

#[derive(Component)]
pub struct MenuMarker;

fn spawn_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Fade::bundle());

    commands
        .spawn((
            MenuMarker,
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(8.0),
                    top: Val::Px(8.0),
                    width: Val::Auto,
                    height: Val::Auto,
                    display: Display::Block,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                format!("Version {}", env!("CARGO_PKG_VERSION")),
                TextStyle {
                    font_size: 16.0,
                    color: Color::linear_rgba(0.8, 0.8, 0.8, 0.8),
                    ..default()
                },
            ));
        });

    commands
        .spawn((
            MenuMarker,
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
                    row_gap: Val::VMin(5.0),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Leiden",
                TextStyle {
                    font_size: 128.0,
                    font: asset_server.load("fonts/Ornatix.ttf"),
                    color: Color::WHITE,
                },
            ));

            vertical_spacer(parent, Val::Vh(10.0));

            text_button(parent, "Continue", 0);
            text_button(parent, "Options", 1);
            #[cfg(feature = "exit")]
            text_button(parent, "Quit", 2);
        });
}

fn despawn_menu(mut commands: Commands, node_q: Query<Entity, With<MenuMarker>>) {
    for entity in node_q.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn button(
    button_state: Res<State<ButtonFocusState>>,
    mut button_next_state: ResMut<NextState<ButtonFocusState>>,
    mut app_next_state: ResMut<NextState<AppState>>,
    #[cfg(feature = "exit")] mut exit: EventWriter<AppExit>,
) {
    if let ButtonFocusState::Id(focus_id) = button_state.get() {
        match focus_id {
            0 => {
                // Continue
            }
            1 => {
                // Options
                app_next_state.set(AppState::MainSettings);
            }
            #[cfg(feature = "exit")]
            2 => {
                // Quit
                exit.send(AppExit::Success);
            }
            _ => {}
        }
        button_next_state.set(ButtonFocusState::None);
    }
}

#[derive(Component)]
struct MenuMusicMarker;

fn spawn_menu_music(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(DynamicAudioBundle {
        source: asset_server.load("audio/music/The Detective - Christoffer Moe Ditlevsen.mp3"),
        settings: PlaybackSettings {
            mode: PlaybackMode::Loop,
            ..default()
        },
        dynamic_audio: DynamicAudio::new(AudioChannel::Music),
    });
}

fn despawn_menu_music(mut commands: Commands, query: Query<Entity, With<MenuMusicMarker>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Menu), spawn_menu)
            .add_systems(OnExit(AppState::Menu), despawn_menu)
            .add_systems(
                Update,
                button
                    .run_if(in_state(AppState::Menu))
                    .run_if(ui_interact_just_pressed()),
            );

        app.add_systems(Startup, spawn_menu_music)
            .add_systems(OnEnter(AppState::Game), despawn_menu_music)
            .add_systems(OnExit(AppState::Game), spawn_menu_music);
    }
}
