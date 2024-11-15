#[cfg(target_arch = "wasm32")]
use pdfium_render::prelude::*;

pub fn generate_text_doc<'a>(pdfium: &'a Pdfium, text: String, size: f32,) -> Result<PdfDocument<'a>, PdfiumError> {
    // Create a new blank PDF document
    let mut document = pdfium.create_new_pdf()?;
    let max_chars = text.lines().map(|l| l.chars().count()).max().unwrap_or(1);
    let max_point_width = max_chars as f32 * size * 0.8;
    let lines = text.lines().count();
    let point_height = lines as f32 * size;

    // Add a new page with a specific size (e.g., A4)
    let mut page = document.pages_mut().create_page_at_start(PdfPagePaperSize::new_custom(
        PdfPoints::new(max_point_width as f32),
        PdfPoints::new(point_height * 1.4)
    ))?;

    // Set the font and font size
    let fonts = document.fonts_mut();
    let font = fonts.times_roman();
    let font_size = PdfPoints::new(size); // Replace with your desired font size

    // Define text and starting coordinates
    let mut y_offset = page.height() - font_size; // Subtract `PdfPoints` directly

    // Add text line-by-line, handling newlines
    for line in text.lines() {
        log::info!("Creating text line: '{}'", line);
        let mut text_object = PdfPageTextObject::new(&document, line, font, font_size)?;
        text_object.set_fill_color(PdfColor::new(0, 0, 0, 255))?; // Set text color to black

        // Position the text on the page
        text_object.translate(PdfPoints::new(0.0), y_offset)?;
        y_offset -= PdfPoints::new(font_size.value * 1.4); // Adjust `y_offset` with `PdfPoints`

        // Add the text object to the page
        page.objects_mut().add_text_object(text_object)?;
    }
    for obj in page.objects().iter() {
        match &obj {
            PdfPageObject::Text(text) => {
                log::info!("Retreived text line: {}", text.text());
            }
            PdfPageObject::Path(_) => {}
            PdfPageObject::Image(_) => {}
            PdfPageObject::Shading(_) => {}
            PdfPageObject::XObjectForm(_) => {}
            PdfPageObject::Unsupported(_) => {}
        }
    }

    Ok(document)
}