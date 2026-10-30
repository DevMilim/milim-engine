use std::{
    fs,
    io::{self, Cursor},
    num::NonZero,
    sync::Mutex,
};

use engine_core::{
    core::{AssetCache, Handler},
    plugin::Plugin,
};
use rodio::{
    Decoder, DeviceSinkBuilder, MixerDeviceSink, Source,
    mixer::{self, Mixer},
};

pub use rodio::Player;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&mut self, app: &mut engine_core::core::AppBuilder) {
        app.set_resource(AudioAssets::default());
        app.set_resource(Audio::default());
    }
}

pub struct AudioAssets {
    sounds: AssetCache<SoundAsset>,
}

impl Default for AudioAssets {
    fn default() -> Self {
        Self {
            sounds: Default::default(),
        }
    }
}

impl AudioAssets {
    pub fn load_audio(&mut self, path: &str) -> io::Result<()> {
        let sound = SoundAsset::load_audio(path)?;
        self.sounds.insert(path, sound);
        Ok(())
    }
    pub fn clear(&mut self) {
        self.sounds.clear();
    }
}

#[derive(Debug)]
pub struct SoundAsset {
    pub bytes: Vec<u8>,
}

impl SoundAsset {
    pub fn load_audio(path: &str) -> io::Result<Self> {
        Ok(Self {
            bytes: fs::read(path)?,
        })
    }
}

pub struct Audio {
    sink: MixerDeviceSink,
    players: Mutex<Vec<Player>>,
    mixer_controller: Mixer,
}

impl Default for Audio {
    fn default() -> Self {
        Self::new()
    }
}

impl Audio {
    pub fn new() -> Self {
        let (controller, mixer) =
            mixer::mixer(NonZero::new(2).unwrap(), NonZero::new(44_100).unwrap());

        let mut sink = DeviceSinkBuilder::open_default_sink().unwrap();
        sink.log_on_drop(false);
        let player = Player::connect_new(sink.mixer());
        player.set_volume(1.0);

        player.append(mixer);

        Self {
            sink,
            players: Mutex::new(Vec::new()),
            mixer_controller: controller,
        }
    }
    pub fn play_controled(
        &self,
        resources: &AudioAssets,
        sound: Handler<SoundAsset>,
        looped: bool,
    ) -> Player {
        let sound_bytes = &resources.sounds.get(sound).unwrap().bytes;
        let source = Decoder::try_from(Cursor::new(sound_bytes.to_vec())).unwrap();

        let player = Player::connect_new(&self.mixer_controller);
        if looped {
            player.append(source.repeat_infinite());
        } else {
            player.append(source);
        }
        player
    }

    pub fn play_and_forget(&self, resources: &AudioAssets, sound: Handler<SoundAsset>) {
        let sound_bytes = &resources.sounds.get(sound).unwrap().bytes;
        let source = Decoder::try_from(Cursor::new(sound_bytes.to_vec())).unwrap();

        let mixer = self.sink.mixer();

        let player = Player::connect_new(mixer);
        player.append(source);

        match self.players.lock() {
            Ok(mut players) => {
                players.retain(|p| !p.empty());
                players.push(player);
            }
            Err(e) => eprintln!("AudioSys: mutex poisoned, som pode não persistir: {e}"),
        }
    }
}
