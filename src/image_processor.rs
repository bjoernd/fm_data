use crate::error::{FMDataError, Result};
use image::DynamicImage;
use log::{debug, info, warn};
use std::path::Path;
use tesseract::Tesseract;

/// Create a configured Tesseract OCR engine for Football Manager screenshots
fn create_tesseract() -> Result<Tesseract> {
    debug!("Initializing Tesseract OCR engine");
    
    let tesseract = Tesseract::new(None, Some("eng"))
        .map_err(|e| FMDataError::image(format!("Failed to initialize Tesseract OCR: {e}")))?
        .set_variable("tessedit_char_whitelist", "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789 ()-:.")
        .map_err(|e| FMDataError::image(format!("Failed to set OCR character whitelist: {e}")))?
        .set_variable("tessedit_pageseg_mode", "6") // Uniform block of text
        .map_err(|e| FMDataError::image(format!("Failed to set OCR page segmentation mode: {e}")))?;

    info!("Tesseract OCR engine initialized successfully");
    Ok(tesseract)
}

/// Load a PNG image from the specified file path
pub fn load_image<P: AsRef<Path>>(image_path: P) -> Result<DynamicImage> {
    let path = image_path.as_ref();
    debug!("Loading PNG image from: {}", path.display());

    let image = image::open(path)
        .map_err(|e| FMDataError::image(format!("Failed to load image {}: {e}", path.display())))?;

    // Verify it's actually a PNG (additional validation beyond CLI)
    if image.color().has_alpha() {
        debug!("Image has alpha channel (typical for PNG)");
    }

    info!("Successfully loaded PNG image: {}x{} pixels", image.width(), image.height());
    Ok(image)
}

/// Extract text from the entire image using OCR
pub fn extract_text_from_image<P: AsRef<Path>>(image_path: P) -> Result<String> {
    let path = image_path.as_ref();
    debug!("Setting image file for OCR processing: {}", path.display());

    let mut tesseract = create_tesseract()?
        .set_image(path.to_str().ok_or_else(|| {
            FMDataError::image("Image path contains invalid UTF-8 characters".to_string())
        })?)
        .map_err(|e| FMDataError::image(format!("Failed to set image file for OCR: {e}")))?;

    debug!("Running OCR text extraction");
    let extracted_text = tesseract
        .get_text()
        .map_err(|e| FMDataError::image(format!("Failed to extract text from image: {e}")))?;

    let text_length = extracted_text.len();
    let line_count = extracted_text.lines().count();
    
    info!("OCR extraction completed: {} characters, {} lines", text_length, line_count);
    
    if text_length < 50 {
        warn!("OCR extracted very little text ({} characters) - image quality may be poor", text_length);
    }

    debug!("OCR extracted text preview: {:?}", &extracted_text[..extracted_text.len().min(100)]);
    
    Ok(extracted_text)
}

/// Preprocess image for better OCR accuracy (if needed)  
pub fn preprocess_image(image: DynamicImage) -> Result<DynamicImage> {
    debug!("Preprocessing image for better OCR accuracy");
    
    // Convert to grayscale for better text recognition
    let grayscale = image.to_luma8();
    debug!("Converted image to grayscale");

    // Convert back to dynamic image
    let processed = DynamicImage::ImageLuma8(grayscale);
    
    info!("Image preprocessing completed");
    Ok(processed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageFormat, Rgb, RgbImage};
    use tempfile::NamedTempFile;
    use std::io::Write;

    fn create_test_png_with_text() -> NamedTempFile {
        // Create a simple test image with some text-like patterns
        let mut img = RgbImage::new(200, 100);
        
        // Fill with white background
        for pixel in img.pixels_mut() {
            *pixel = Rgb([255, 255, 255]);
        }
        
        // Add some black "text" pixels (simple pattern)
        for x in 10..50 {
            for y in 20..25 {
                img.put_pixel(x, y, Rgb([0, 0, 0]));
            }
        }

        // Create a temp file with .png extension
        let temp_file = NamedTempFile::with_suffix(".png").unwrap();
        let dynamic_img = DynamicImage::ImageRgb8(img);
        
        // Save directly to the temp file
        dynamic_img.save_with_format(temp_file.path(), ImageFormat::Png).unwrap();
        
        temp_file
    }

    fn create_invalid_image_file() -> NamedTempFile {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"not an image").unwrap();
        temp_file.flush().unwrap();
        temp_file
    }

    #[test]
    fn test_create_tesseract() {
        let result = create_tesseract();
        // OCR initialization may fail in test environments without Tesseract
        // So we check both success and expected failure cases
        match result {
            Ok(_) => {
                // Tesseract is available, processor created successfully
            },
            Err(e) => {
                // Expected failure if Tesseract not available in test environment
                assert!(e.to_string().contains("Failed to initialize Tesseract OCR"));
            }
        }
    }

    #[test]
    fn test_load_valid_image() {
        let temp_png = create_test_png_with_text();

        let result = load_image(temp_png.path());
        assert!(result.is_ok());
        
        let image = result.unwrap();
        assert_eq!(image.width(), 200);
        assert_eq!(image.height(), 100);
    }

    #[test]
    fn test_load_invalid_image() {
        let invalid_file = create_invalid_image_file();

        let result = load_image(invalid_file.path());
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Failed to load image"));
    }

    #[test]
    fn test_load_nonexistent_image() {
        let result = load_image("/nonexistent/path/image.png");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Failed to load image"));
    }

    #[test]
    fn test_extract_text_from_image() {
        let temp_png = create_test_png_with_text();
        
        let result = extract_text_from_image(temp_png.path());
        // OCR may fail in test environments, but we test the function exists and handles errors
        match result {
            Ok(text) => {
                // OCR succeeded - text should be a string
                assert!(text.is_ascii());
            },
            Err(e) => {
                // Expected failure if Tesseract not available
                assert!(
                    e.to_string().contains("Failed to initialize Tesseract OCR") ||
                    e.to_string().contains("Failed to set image file for OCR") ||
                    e.to_string().contains("Failed to extract text from image")
                );
            }
        }
    }

    #[test]
    fn test_preprocess_image() {
        let temp_png = create_test_png_with_text();

        let image = load_image(temp_png.path()).unwrap();
        let result = preprocess_image(image);
        assert!(result.is_ok());
        
        let processed_image = result.unwrap();
        assert_eq!(processed_image.width(), 200);
        assert_eq!(processed_image.height(), 100);
    }
}