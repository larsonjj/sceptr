use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

// the following asset collections will be loaded during the State `AppState::Loading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/font_space/pixeloid_mono.ttf")]
    pub pixeliod_mono: Handle<Font>,
    #[asset(path = "fonts/font_space/pixeloid_sans_bold.ttf")]
    pub pixeliod_bold: Handle<Font>,
    #[asset(path = "fonts/font_space/pixeloid_sans.ttf")]
    pub pixeliod_sans: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    // #[asset(path = "audio/chosic/fluffing_a_duck.ogg")]
    // pub background_music: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/player_proto.png")]
    pub player_proto: Handle<Image>,
    #[asset(path = "textures/enemy_proto.png")]
    pub enemy_proto: Handle<Image>,
}
