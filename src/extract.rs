use image::{DynamicImage, GenericImageView};
use pdfium_render::prelude::{PdfPage, PdfPageObjectCommon, PdfiumError};

pub fn extract_page_image(page: &PdfPage) -> Result<Option<DynamicImage>, PdfiumError> {
    if page.text().unwrap().is_empty() && page.annotations().is_empty() {
        let images = page.objects().create_group(|_o|  true)?;
        if images.len() == 1 {
            let obj = images.iter().next();
            if let Some(image) =  &obj {
                if let Some(image) = image.as_image_object() {
                    log::debug!("Before bounds");
                    let bounds = image.bounds()?; 
                    log::debug!("Before bounds");
                    let media_box = page.boundaries().media()?;
                    log::debug!("Going for bitmap");
                    let image = image.get_raw_image()?;
                    let data : Vec<u8> = image.pixels().map(|p| p.2.0[0]).collect();

                    process_data(&data[0..2000]);

                    return Ok(Some(image))
                }
            } 
        }
    }
    Ok(None)
}

#[derive(PartialEq, Eq)]
enum LastPush {
    ASCII,
    U8,
    NUL,
}

/// Prints out bytes as contiguous runs of valid ascii or integer codes of individual bytes, treating NUL as special case
fn process_data(data: &[u8]) {
    let mut current_run: Vec<u8> = Vec::new();
    let mut last_push = LastPush::NUL;

    for &byte in data {
        if byte == 0 {
            if last_push == LastPush::ASCII  {
                log::info!("{:?}", String::from_utf8_lossy(&current_run));
            } else if last_push == LastPush::U8  {
                log::info!("{:?} ", &current_run);
            }
            last_push = LastPush::NUL;
            current_run.clear();
            log::info!("NUL");
        } else if byte < 128 {
            if last_push == LastPush::U8 {
                log::info!("{:?}", &current_run);
                current_run.clear();
            }
            last_push = LastPush::ASCII;
            current_run.push(byte);
        } else {
            if last_push == LastPush::ASCII  {
                log::info!("{:?}", String::from_utf8_lossy(&current_run));
                current_run.clear();
            } 
            last_push = LastPush::U8;
            current_run.push(byte);
        }
    }

    if last_push == LastPush::ASCII  {
        log::info!("{:?} ", String::from_utf8_lossy(&current_run))
    } else if last_push == LastPush::U8  {
        log::info!("{:?} ", &current_run);
    }
}
