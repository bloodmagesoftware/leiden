/*
 * Leiden - A video game
 * Copyright (C) 2024  Frank Mayer
 *
 * This file is part of Leiden.
 *
 * Fair Core License, Version 1.0, Apache 2.0 Future License
 * https://github.com/bloodmagesoftware/leiden/blob/main/LICENSE.md
 */

use crate::state::AppState;
use bevy::audio::Volume;
use bevy::prelude::*;
use bevy_persistent::{Persistent, StorageFormat};
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};
use std::path::Path;

#[derive(Resource, Serialize, Deserialize, Debug, Clone, Copy)]
pub struct UserSettings {
    pub music_volume: f32,
    pub sfx_volume: f32,
    pub ui_volume: f32,
    pub master_volume: f32,
    pub vibration: bool,
}

impl PartialEq for UserSettings {
    fn eq(&self, other: &Self) -> bool {
        self.music_volume == other.music_volume
            && self.sfx_volume == other.sfx_volume
            && self.ui_volume == other.ui_volume
            && self.master_volume == other.master_volume
            && self.vibration == other.vibration
    }
}

impl Eq for UserSettings {}

impl Hash for UserSettings {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.music_volume.to_bits().hash(state);
        self.sfx_volume.to_bits().hash(state);
        self.ui_volume.to_bits().hash(state);
        self.master_volume.to_bits().hash(state);
    }
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            music_volume: 1.,
            sfx_volume: 1.,
            ui_volume: 1.,
            master_volume: 0.5,
            #[cfg(feature = "rumble")]
            vibration: true,
            #[cfg(not(feature = "rumble"))]
            vibration: false,
        }
    }
}

impl UserSettings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn value_by_channel(&self, channel: AudioChannel) -> f32 {
        match channel {
            AudioChannel::Music => self.music_volume * self.master_volume,
            AudioChannel::Sfx => self.sfx_volume * self.master_volume,
            AudioChannel::Ui => self.ui_volume * self.master_volume,
            AudioChannel::Master => self.master_volume,
        }
    }

    pub fn value_by_channel_scaled(&self, channel: AudioChannel, scale: f32) -> f32 {
        self.value_by_channel(channel) * scale
    }

    pub fn volume_by_channel(&self, channel: AudioChannel) -> Volume {
        Volume::new(self.value_by_channel(channel))
    }

    pub fn volume_by_channel_scaled(&self, channel: AudioChannel, scale: f32) -> Volume {
        Volume::new(self.value_by_channel_scaled(channel, scale))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AudioChannel {
    Music,
    Sfx,
    Ui,
    Master,
}

#[derive(Component)]
pub struct DynamicAudio {
    pub channel: AudioChannel,
}
impl DynamicAudio {
    pub fn new(channel: AudioChannel) -> Self {
        Self { channel }
    }
}

#[derive(Bundle)]
pub struct DynamicAudioBundle {
    pub source: Handle<AudioSource>,
    pub settings: PlaybackSettings,
    pub dynamic_audio: DynamicAudio,
}

fn update_dynamic_audio(
    settings: Res<UserSettings>,
    audio_q: Query<(&DynamicAudio, &AudioSink, &PlaybackSettings)>,
) {
    for (dynamic_audio, audio_sink, playback_settings) in audio_q.iter() {
        let next_volume =
            playback_settings.volume.get() * settings.value_by_channel(dynamic_audio.channel);

        let current_volume = audio_sink.volume();

        if next_volume != current_volume {
            audio_sink.set_volume(next_volume);
        }
    }
}

fn apply_persistent_user_settings(
    mut user_settings: ResMut<UserSettings>,
    persistent_user_settings: Res<Persistent<UserSettings>>,
) {
    {
        *user_settings = *persistent_user_settings.get();
    }
}

fn persist_user_settings(
    user_settings: Res<UserSettings>,
    mut persistent_user_settings: ResMut<Persistent<UserSettings>>,
) {
    persistent_user_settings
        .set(*user_settings)
        .expect("failed to persist user settings");
}

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_dynamic_audio);

        let config_dir = dirs::config_dir()
            .map(|native_config_dir| {
                native_config_dir
                    .join("bloodmagesoftware")
                    .join("leiden")
                    .join("config")
            })
            .unwrap_or(Path::new("local").join("config"));

        let persistent_user_settings_resource = Persistent::<UserSettings>::builder()
            .name("user_settings")
            .format(StorageFormat::Bincode)
            .path(config_dir.join("user_settings"))
            .default(UserSettings::default())
            .revertible(true)
            .revert_to_default_on_deserialization_errors(true)
            .build()
            .expect("failed to initialize user settings resource");

        app.insert_resource(persistent_user_settings_resource)
            .insert_resource(UserSettings::default())
            .add_systems(PreStartup, apply_persistent_user_settings)
            .add_systems(OnExit(AppState::MainSettings), persist_user_settings);
    }
}
