use bevy_app::Plugin;
use bevy_asset::AssetApp;
use bevy_render::texture::Image;

use crate::loader::KritaDocumentLoader;

/// A plugin to load `.kra` Krita documents directly as [`Image`] assets.
#[derive(Debug, Default)]
pub struct KritaPlugin;

impl Plugin for KritaPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.init_asset_loader::<KritaDocumentLoader>()
            // This is also done in `bevy_render`, we do it just to be sure
            .register_type::<Image>();
    }
}
