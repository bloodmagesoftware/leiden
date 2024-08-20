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

use crate::state::AppState;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum SplashState {
    Bms,
    Bevy,
}

#[derive(Component)]
struct Splash {
    timer: Timer,
    state: SplashState,
}

#[derive(Component)]
struct SplashImageMarker;

impl Splash {
    fn new() -> Self {
        Self {
            timer: Timer::from_seconds(4.0, TimerMode::Once),
            state: SplashState::Bms,
        }
    }
}

#[derive(Component)]
struct SplashOverlayMarker;

fn spawn_splash_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SplashOverlayMarker,
        NodeBundle {
            z_index: ZIndex::Global(1),
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                width: Val::Vw(100.0),
                height: Val::Vh(100.0),
                display: Display::Block,
                ..default()
            },
            background_color: BackgroundColor(Color::srgba(0., 0., 0., 1.)),
            ..default()
        },
    ));

    commands
        .spawn((
            Splash::new(),
            NodeBundle {
                style: Style {
                    width: Val::Vw(100.0),
                    height: Val::Vh(100.0),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::BLACK),
                z_index: ZIndex::Global(0),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                SplashImageMarker,
                ImageBundle {
                    image: UiImage {
                        texture: asset_server.load("textures/splash/bms.png"),
                        ..default()
                    },
                    style: Style {
                        width: Val::VMin(50.0),
                        height: Val::Auto,
                        ..default()
                    },
                    ..default()
                },
            ));
        });
}

fn despawn_splash(
    mut commands: Commands,
    splash_overlay_q: Query<Entity, With<SplashOverlayMarker>>,
    splash_q: Query<Entity, With<Splash>>,
) {
    for entity in splash_overlay_q.iter() {
        commands.entity(entity).despawn_recursive();
    }

    for entity in splash_q.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn update_splash_content(
    time: Res<Time>,
    mut splash_q: Query<&mut Splash>,
    mut image_q: Query<&mut UiImage, With<SplashImageMarker>>,
    mut next_state: ResMut<NextState<AppState>>,
    asset_server: Res<AssetServer>,
) {
    let mut splash = match splash_q.get_single_mut() {
        Ok(x) => x,
        Err(_) => return,
    };

    splash.timer.tick(time.delta());

    if splash.timer.just_finished() {
        let mut ui_image = match image_q.get_single_mut() {
            Ok(x) => x,
            Err(_) => return,
        };

        match splash.state {
            SplashState::Bms => {
                splash.state = SplashState::Bevy;
                ui_image.texture = asset_server.load("textures/splash/bevy.png");
                splash.timer.reset();
            }
            SplashState::Bevy => {
                next_state.set(AppState::Menu);
            }
        }
    }
}

fn update_splash_opacity(
    mut splash_q: Query<&Splash>,
    mut overlay_q: Query<&mut BackgroundColor, With<SplashOverlayMarker>>,
) {
    let splash = match splash_q.get_single_mut() {
        Ok(x) => x,
        Err(_) => return,
    };

    let mut background_color = match overlay_q.get_single_mut() {
        Ok(x) => x,
        Err(_) => return,
    };

    let t = splash.timer.fraction();
    let opacity = (-0.75 + (2. * t - 1.).powf(2.) * 2.).clamp(0.0, 1.0);
    background_color.0.set_alpha(opacity);
}

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Splash), spawn_splash_ui)
            .add_systems(
                Update,
                (update_splash_content, update_splash_opacity).run_if(in_state(AppState::Splash)),
            )
            .add_systems(OnExit(AppState::Splash), despawn_splash);
    }
}
