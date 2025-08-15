use crate::error::{FMDataError, Result};
use crate::error_helpers::ConfigResult;
use arboard::{Clipboard, ImageData};
use image::{DynamicImage, ImageBuffer, Rgba};
use std::io::{stdin, stdout, Write};
use tempfile::NamedTempFile;

/// Clipboard manager for handling image paste operations
pub struct ClipboardManager {
    clipboard: Clipboard,
}

impl ClipboardManager {
    /// Create a new clipboard manager
    pub fn new() -> Result<Self> {
        let clipboard = Clipboard::new().config_context("initialize clipboard")?;

        Ok(Self { clipboard })
    }

    /// Wait for user to paste an image from clipboard and save it to a temporary file
    /// Returns the path to the temporary file containing the pasted image
    pub fn wait_for_image_paste(&mut self) -> Result<NamedTempFile> {
        println!("📋 Waiting for image from clipboard...");
        println!("💡 Copy an image (Cmd+C) and press Enter to paste it, or Ctrl+C to cancel");

        print!("Press Enter when ready to paste: ");
        stdout().flush().config_context("flush stdout")?;

        // Wait for user to press Enter
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .config_context("read from stdin")?;

        // Try to get image from clipboard
        let image_data = self.clipboard.get_image().config_context("get image from clipboard. Make sure you have copied an image (Cmd+C) before pressing Enter")?;

        // Convert arboard ImageData to image crate format
        let dynamic_image = self.convert_clipboard_image_to_dynamic_image(&image_data)?;

        // Save to temporary file
        let temp_file =
            NamedTempFile::with_suffix(".png").config_context("create temporary file")?;

        dynamic_image
            .save(temp_file.path())
            .config_context("save image to temporary file")?;

        println!("✅ Successfully pasted image from clipboard");
        Ok(temp_file)
    }

    /// Convert arboard ImageData to image crate DynamicImage
    fn convert_clipboard_image_to_dynamic_image(
        &self,
        image_data: &ImageData,
    ) -> Result<DynamicImage> {
        let ImageData {
            width,
            height,
            bytes,
        } = image_data;

        // arboard returns RGBA data, convert to Vec<u8> for image crate compatibility
        let rgba_buffer = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(
            *width as u32,
            *height as u32,
            bytes.to_vec(),
        )
        .ok_or_else(|| {
            FMDataError::config("Failed to create image buffer from clipboard data".to_string())
        })?;

        Ok(DynamicImage::ImageRgba8(rgba_buffer))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clipboard_manager_creation() {
        // Note: This test may fail in CI environments without a GUI
        // We'll test that the creation doesn't panic, but clipboard access might fail
        let result = ClipboardManager::new();
        // On headless systems, this might fail, which is expected
        if result.is_err() {
            println!("Clipboard initialization failed (expected in headless environments)");
        }
    }

    #[test]
    fn test_convert_clipboard_image_to_dynamic_image() {
        if let Ok(manager) = ClipboardManager::new() {
            // Create test image data (2x2 RGBA image)
            let test_data = vec![
                255, 0, 0, 255, // Red pixel
                0, 255, 0, 255, // Green pixel
                0, 0, 255, 255, // Blue pixel
                255, 255, 255, 255, // White pixel
            ];

            let image_data = ImageData {
                width: 2,
                height: 2,
                bytes: test_data.into(),
            };

            let result = manager.convert_clipboard_image_to_dynamic_image(&image_data);
            assert!(result.is_ok());

            let dynamic_image = result.expect("Failed to convert clipboard image to dynamic image");
            assert_eq!(dynamic_image.width(), 2);
            assert_eq!(dynamic_image.height(), 2);
        }
    }

    #[test]
    fn test_convert_clipboard_image_invalid_dimensions() {
        if let Ok(manager) = ClipboardManager::new() {
            // Create invalid test data (not enough bytes for dimensions)
            let test_data = vec![255, 0, 0, 255]; // Only 1 pixel worth of data for 2x2 image

            let image_data = ImageData {
                width: 2,
                height: 2,
                bytes: test_data.into(),
            };

            let result = manager.convert_clipboard_image_to_dynamic_image(&image_data);
            assert!(result.is_err());
            assert!(result
                .unwrap_err()
                .to_string()
                .contains("Failed to create image buffer"));
        }
    }

    #[test]
    fn test_convert_clipboard_image_empty_data() {
        if let Ok(manager) = ClipboardManager::new() {
            // Create empty test data
            let image_data = ImageData {
                width: 0,
                height: 0,
                bytes: Vec::new().into(),
            };

            let result = manager.convert_clipboard_image_to_dynamic_image(&image_data);
            // This should succeed with 0x0 image
            assert!(result.is_ok());

            let dynamic_image = result.expect("Failed to convert clipboard image to dynamic image");
            assert_eq!(dynamic_image.width(), 0);
            assert_eq!(dynamic_image.height(), 0);
        }
    }
}
