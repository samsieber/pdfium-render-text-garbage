use image::DynamicImage;
use pdfium_render::prelude::{PdfPage, PdfPageObjectCommon, PdfiumError};

pub fn extract_page_image(page: &PdfPage) -> Result<Option<DynamicImage>, PdfiumError> {
    log::info!("HERE");
    if page.text().unwrap().is_empty() && page.annotations().is_empty() {
        log::info!("No text no annotations");
        let images = page.objects().create_group(|_o|  true)?;
        log::info!("Images len: {}", images.len());
        if images.len() == 1 {
            let obj = images.iter().next();
            log::info!("Iter status: {}", obj.is_some());
            if let Some(image) =  &obj {
                if let Some(image) = image.as_image_object() {
                    let bounds = image.bounds()?;
                    let media_box = page.boundaries().media()?;
                    log::info!("{:?}, {:?}", media_box.bounds, bounds);
                    // if media_box.bounds == bounds {
                        let image = image.get_raw_image()?;

                        log::info!("Found an image of type: {}", match image {
                            DynamicImage::ImageLuma8(_) => "ImageLuma8",
                            DynamicImage::ImageLumaA8(_) => "ImageLumaA8",
                            DynamicImage::ImageRgb8(_) => "ImageRgb8",
                            DynamicImage::ImageRgba8(_) => "ImageRgba8",
                            DynamicImage::ImageLuma16(_) => "ImageLuma16",
                            DynamicImage::ImageLumaA16(_) => "ImageLumaA16",
                            DynamicImage::ImageRgb16(_) => "ImageRgb16",
                            DynamicImage::ImageRgba16(_) => "ImageRgba16",
                            DynamicImage::ImageRgb32F(_) => "ImageRgb32F",
                            DynamicImage::ImageRgba32F(_) => "ImageRgba32F",
                            _ => todo!(),
                        });
                        return Ok(Some(image))
                    // }
                } else {
                    log::info!("Object type: {:?}",  image.object_type());
                }
            } 
        }
    }
    Ok(None)
}