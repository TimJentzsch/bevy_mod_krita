use std::io::{Cursor, Read};

use bevy_asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext};
use bevy_ecs::world::{FromWorld, World};
use bevy_render::{
    render_asset::RenderAssetUsages,
    texture::{CompressedImageFormats, Image, ImageSampler, ImageType},
};
use zip::ZipArchive;

/// An asset loader to load Krita's `.kra` files.
#[derive(Debug, Clone)]
pub struct KritaDocumentLoader;

impl AssetLoader for KritaDocumentLoader {
    type Asset = Image;
    type Settings = ();
    type Error = anyhow::Error;

    fn extensions(&self) -> &[&str] {
        &["kra"]
    }

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        #[allow(unused_variables)] load_context: &'a mut LoadContext,
    ) -> bevy_asset::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            // TODO: Improve error handling

            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            // `.kra` files are basically just `.zip` files, try to read the ZIP archive
            let reader = Cursor::new(bytes);
            let mut archive = ZipArchive::new(reader)?;

            // `.kra` files contain a file `mergedimage.png` which contains the rendered image
            // See <https://docs.krita.org/en/general_concepts/file_formats/file_kra.html>
            let mut merged_image = archive.by_name("mergedimage.png")?;

            let mut image_bytes = Vec::<u8>::new();
            merged_image.read_to_end(&mut image_bytes)?;

            Ok(Image::from_buffer(
                #[cfg(all(debug_assertions, feature = "dds"))]
                load_context.path().display().to_string(),
                &image_bytes,
                ImageType::Extension("png"),
                // TODO: Check what to put here
                CompressedImageFormats::all(),
                // TODO: Check if it's srgb
                true,
                ImageSampler::Default,
                // TODO: Make this configurable
                RenderAssetUsages::default(),
            )?)
        })
    }
}

impl FromWorld for KritaDocumentLoader {
    fn from_world(_world: &mut World) -> Self {
        Self
    }
}
