mod text;

#[cfg(target_arch = "wasm32")]
use pdfium_render::prelude::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use web_sys::ImageData;

/// Downloads the given URL, opens it as a PDF document, then returns the ImageData for
/// the given page index using the given bitmap dimensions.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn get_image_data_for_page(
    url: String,
    index: PdfPageIndex,
    width: Pixels,
    height: Pixels,
) -> ImageData {
    let pdfium = Pdfium::default();
    let document = pdfium.load_pdf_from_fetch(url, None).await.unwrap();
    let x = document.pages()
        .get(index)
        .unwrap()
        .render_with_config(
            &PdfRenderConfig::new()
                .set_target_size(width, height)
                .render_form_data(true)
                .highlight_text_form_fields(PdfColor::YELLOW.with_alpha(128))
                .highlight_checkbox_form_fields(PdfColor::BLUE.with_alpha(128)),
        )
        .unwrap()
        .as_image_data()
        .unwrap();
    drop(x);
    let mut docs = vec!();
    for x in 0..3 {
        docs.push(text::generate_text_doc(&pdfium, "new test".to_string(), 12.0, false, false, text::StandardFont::TimesNewRoman).unwrap());
    };
    let x = docs[0].pages()
            .get(index)
            .unwrap()
            .render_with_config(
                &PdfRenderConfig::new()
                    .set_target_size(width, height)
                    .render_form_data(true)
                    .highlight_text_form_fields(PdfColor::YELLOW.with_alpha(128))
                    .highlight_checkbox_form_fields(PdfColor::BLUE.with_alpha(128)),
            )
            .unwrap()
            .as_image_data()
            .unwrap();
    x
}

// Source files in examples/ directory are expected to always have a main() entry-point.
// Since we're compiling to WASM, we'll never actually use this.
#[allow(dead_code)]
fn main() {}
