#[cfg(target_arch = "wasm32")]
use wasm_bindgen::Clamped;
#[cfg(target_arch = "wasm32")]
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};

mod extract;
use crate::extract::extract_page_image;
use image::DynamicImage;

#[cfg(target_arch = "wasm32")]
use pdfium_render::prelude::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use web_sys::ImageData;

#[cfg(target_arch = "wasm32")]
pub async fn internal_write_image_data_for_page(
    pdfium: &Pdfium,
    url: String,
    index: PdfPageIndex
) {
    let document = pdfium.load_pdf_from_fetch(&url, None).await.unwrap();
    let page = document.pages()
        .get(index)
        .unwrap();
    let dynamic_image = extract_page_image(&page).unwrap().unwrap();
}

#[cfg(target_arch = "wasm32")]
pub async fn draw(
    dynamic_image: DynamicImage,
    canvas1: HtmlCanvasElement
) {
    let width = dynamic_image.width();
    let height = dynamic_image.height();
    let raw_pixels = dynamic_image.clone().into_rgba8().into_raw();
    // let raw_pixels2 = dynamic_image.into_rgba8().into_raw();
    let new_img_data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&raw_pixels),
        width,
        height,
    ).unwrap();

    let ctx = canvas1
        .get_context("2d")
            .unwrap()
            .expect("Could not get 2d rendering context for OffscreenCanvas")
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

    canvas1.set_width(width);
    canvas1.set_height(height);

    ctx.put_image_data(&new_img_data, 0.0, 0.0);
}

/// Downloads the given URL, opens it as a PDF document, then returns the ImageData for
/// the given page index using the given bitmap dimensions.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn write_image_data_for_page(
    url: String,
    index: PdfPageIndex,
    canvas1: HtmlCanvasElement
) {

    let pdfium = Pdfium::default();
    let document1 = pdfium.load_pdf_from_fetch(&url, None).await.unwrap();
    let page1 = document1.pages()
        .get(index)
        .unwrap();
    let dynamic_image = extract_page_image(&page1).unwrap().unwrap();

    let width = dynamic_image.width();
    let height = dynamic_image.height();
    let raw_pixels = dynamic_image.into_rgba8().into_raw();

    let new_img_data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&raw_pixels),
        width,
        height,
    ).unwrap();

    let ctx = canvas1
        .get_context("2d")
            .unwrap()
            .expect("Could not get 2d rendering context for OffscreenCanvas")
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

    canvas1.set_width(width);
    canvas1.set_height(height);

    ctx.put_image_data(&new_img_data, 0.0, 0.0);
}

// Source files in examples/ directory are expected to always have a main() entry-point.
// Since we're compiling to WASM, we'll never actually use this.
#[allow(dead_code)]
fn main() {}
