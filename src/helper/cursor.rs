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

use bevy::app::FixedMain;
use bevy::prelude::*;
use bevy_window::PrimaryWindow;

use crate::helper::ext::SetVal;
use crate::state::InputState;

#[derive(Component)]
pub struct Cursor;

fn cursor_grab(mut window_q: Query<&mut Window, With<PrimaryWindow>>) {
    let mut primary_window = window_q.single_mut();
    primary_window.cursor.visible = false;
}

fn spawn_custom_cursor(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Cursor,
        ImageBundle {
            image: UiImage {
                texture: asset_server.load("textures/ui/cursor.png"),
                ..default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Px(32.),
                height: Val::Px(32.),
                left: Val::Px(10000.),
                top: Val::Px(10000.),
                ..default()
            },
            z_index: ZIndex::Global(15),
            ..default()
        },
    ));
}

fn despawn_custom_cursor(mut commands: Commands, query: Query<Entity, With<Cursor>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn update_custom_cursor_position(
    mut cursor_q: Query<&mut Style, With<Cursor>>,
    mut evr_cursor: EventReader<CursorMoved>,
    mut evr_cursor_entered: EventReader<CursorEntered>,
    mut evr_cursor_left: EventReader<CursorLeft>,
) {
    for _ in evr_cursor_left.read() {
        for mut style in cursor_q.iter_mut() {
            style.display = Display::None;
        }
    }

    for _ in evr_cursor_entered.read() {
        for mut style in cursor_q.iter_mut() {
            style.display = Display::Block;
        }
    }

    for ev in evr_cursor.read() {
        for mut style in cursor_q.iter_mut() {
            style.left.set_val(ev.position.x);
            style.top.set_val(ev.position.y);
        }
    }
}

pub struct CursorPlugin;

impl bevy::app::Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, cursor_grab)
            .add_systems(OnEnter(InputState::MouseAndKeyboard), spawn_custom_cursor)
            .add_systems(
                FixedMain,
                update_custom_cursor_position.run_if(in_state(InputState::MouseAndKeyboard)),
            )
            .add_systems(OnExit(InputState::MouseAndKeyboard), despawn_custom_cursor);
    }
}
