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

use crate::helper::settings::{AudioChannel, UserSettings};
use crate::helper::system::ui_interact_just_pressed;
use crate::state::{ButtonFocusState, InputState};

#[derive(Component)]
pub struct ButtonIdentifier {
    pub id: i8,
}

impl ButtonIdentifier {
    pub fn new(id: i8) -> Self {
        Self { id }
    }
}

pub fn text_button(parent: &mut ChildBuilder, content: impl Into<String>, id: i8) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Auto,
                    min_width: Val::Vw(20.0),
                    height: Val::Auto,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            ButtonIdentifier::new(id),
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                content,
                TextStyle {
                    font_size: 32.0,
                    ..default()
                },
            ));
        });
}

pub fn vertical_spacer(parent: &mut ChildBuilder, height: Val) {
    parent.spawn(NodeBundle {
        style: Style {
            height,
            ..default()
        },
        ..default()
    });
}

const BUTTON_NORMAL: Color = Color::WHITE;
const BUTTON_HOVER: Color = Color::hsl(267.0, 0.84, 0.81);

fn update_button(
    button_q: Query<(&Children, &ButtonIdentifier)>,
    mut text_q: Query<&mut Text>,
    button_focus_state: Res<State<ButtonFocusState>>,
) {
    for (children, id) in button_q.iter() {
        let c = if button_focus_state.is_id(id.id) {
            BUTTON_HOVER
        } else {
            BUTTON_NORMAL
        };

        for child in children.iter() {
            if let Ok(mut text) = text_q.get_mut(*child) {
                for section in text.sections.iter_mut() {
                    section.style.color = c;
                }
            }
        }
    }
}

fn cursor_update_button_focus(
    button_q: Query<(&Interaction, &ButtonIdentifier), Changed<Interaction>>,
    mut next_state: ResMut<NextState<ButtonFocusState>>,
) {
    let mut some = false;

    for (interaction, id) in button_q.iter() {
        if interaction != &Interaction::None {
            next_state.set(ButtonFocusState::Id(id.id));
            return;
        }
        some = true;
    }

    if some {
        next_state.set(ButtonFocusState::None);
    }
}

fn gamepad_update_button_focus(
    button_q: Query<&ButtonIdentifier>,
    gamepad: Res<Gamepads>,
    buttons: Res<ButtonInput<GamepadButton>>,
    mut next_state: ResMut<NextState<ButtonFocusState>>,
    state: Res<State<ButtonFocusState>>,
) {
    for gamepad in gamepad.iter() {
        let d_pad_down = GamepadButton {
            gamepad,
            button_type: GamepadButtonType::DPadDown,
        };

        if buttons.just_pressed(d_pad_down) {
            next_state.set(ButtonFocusState::Id(find_next_button(&button_q, &state)));
            return;
        }

        let d_pad_up = GamepadButton {
            gamepad,
            button_type: GamepadButtonType::DPadUp,
        };

        if buttons.just_pressed(d_pad_up) {
            next_state.set(ButtonFocusState::Id(find_prev_button(&button_q, &state)));
            return;
        }
    }
    if state.is_none() {
        if let Some(id) = find_first_button(&button_q) {
            next_state.set(ButtonFocusState::Id(id));
        }
    }
}

fn find_first_button(button_q: &Query<&ButtonIdentifier>) -> Option<i8> {
    let mut min = i8::MAX;
    for button in button_q.iter() {
        if button.id < min {
            min = button.id;
        }
    }

    if min == i8::MAX {
        None
    } else {
        Some(min)
    }
}

fn find_next_button(button_q: &Query<&ButtonIdentifier>, current: &ButtonFocusState) -> i8 {
    match current {
        ButtonFocusState::None => {
            // select first button
            let mut min = i8::MAX;
            for button in button_q.iter() {
                if button.id < min {
                    min = button.id;
                }
            }
            min
        }
        ButtonFocusState::Id(current) => {
            let mut min = i8::MAX;
            for button in button_q.iter() {
                if button.id > *current && button.id < min {
                    min = button.id;
                }
            }
            if min != i8::MAX {
                return min;
            }
            // overflow
            min = *current;
            for button in button_q.iter() {
                if button.id < min {
                    min = button.id;
                }
            }
            min
        }
    }
}

fn find_prev_button(button_q: &Query<&ButtonIdentifier>, current: &ButtonFocusState) -> i8 {
    match current {
        ButtonFocusState::None => {
            // select last button
            let mut max = i8::MIN;
            for button in button_q.iter() {
                if button.id > max {
                    max = button.id;
                }
            }
            max
        }
        ButtonFocusState::Id(current) => {
            let mut max = i8::MIN;
            for button in button_q.iter() {
                if button.id < *current && button.id > max {
                    max = button.id;
                }
            }
            if max != i8::MIN {
                return max;
            }
            // overflow
            max = *current;
            for button in button_q.iter() {
                if button.id > max {
                    max = button.id;
                }
            }
            max
        }
    }
}

#[derive(Component)]
pub struct Fade {
    timer: Timer,
}

impl Fade {
    pub fn bundle() -> impl Bundle {
        (
            NodeBundle {
                z_index: ZIndex::Global(100),
                style: Style {
                    position_type: PositionType::Absolute,
                    display: Display::Block,
                    width: Val::Vw(100.0),
                    height: Val::Vh(100.0),
                    left: Val::Px(0.0),
                    top: Val::Px(0.0),
                    ..default()
                },
                background_color: BackgroundColor(Color::srgba(0., 0., 0., 1.)),
                ..default()
            },
            Self {
                timer: Timer::from_seconds(2.0, TimerMode::Once),
            },
        )
    }
}

fn fade(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Fade, &mut BackgroundColor)>,
) {
    for (e, mut fade_in, mut background_color) in query.iter_mut() {
        fade_in.timer.tick(time.delta());
        if fade_in.timer.finished() {
            commands.entity(e).despawn_recursive();
        } else {
            background_color
                .0
                .set_alpha(fade_in.timer.fraction_remaining().powi(2));
        }
    }
}

fn play_ui_sound(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    settings: Res<UserSettings>,
) {
    commands.spawn(AudioBundle {
        source: asset_server.load("audio/ui/wood_hit.wav"),
        settings: PlaybackSettings {
            mode: PlaybackMode::Despawn,
            volume: settings.volume_by_channel(AudioChannel::Ui),
            ..default()
        },
    });
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_button)
            .add_systems(
                Update,
                cursor_update_button_focus.run_if(in_state(InputState::MouseAndKeyboard)),
            )
            .add_systems(
                Update,
                gamepad_update_button_focus.run_if(in_state(InputState::Gamepad)),
            )
            .add_systems(Update, play_ui_sound.run_if(ui_interact_just_pressed()))
            .add_systems(Update, fade);
    }
}
