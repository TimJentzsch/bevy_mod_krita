use std::io::{Cursor, Read};

use bevy_asset::{AssetLoader, LoadedAsset};
use bevy_render::texture::{CompressedImageFormats, Image, ImageType};
use zip::ZipArchive;

#[derive(Debug, Clone)]
pub struct KritaDocumentLoader;

impl AssetLoader for KritaDocumentLoader {
    fn extensions(&self) -> &[&str] {
        &["kra"]
    }

    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy_asset::LoadContext,
    ) -> bevy_asset::BoxedFuture<'a, anyhow::Result<(), anyhow::Error>> {
        Box::pin(async move {
            // TODO: Improve error handling

            // `.kra` files are basically just `.zip` files, try to read the ZIP archive
            let reader = Cursor::new(bytes);
            let mut archive = ZipArchive::new(reader)?;

            // `.kra` files contain a file `mergedimage.png` which contains the rendered image
            // See <https://docs.krita.org/en/general_concepts/file_formats/file_kra.html>
            let mut merged_image = archive.by_name("mergedimage.png")?;

            let mut image_bytes = Vec::<u8>::new();
            merged_image.read_to_end(&mut image_bytes)?;

            let dyn_img = Image::from_buffer(
                &image_bytes,
                ImageType::Extension("png"),
                // FIXME: Figure out what this does
                CompressedImageFormats::empty(),
                true,
            )?;

            load_context.set_default_asset(LoadedAsset::new(dyn_img));
            Ok(())
        })
    }
}
