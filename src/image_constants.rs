//! Constants for image processing and OCR configuration
//!
//! This module centralizes all magic numbers used in image processing,
//! footedness detection, and OCR configuration to improve maintainability.

/// OCR Configuration Constants
pub mod ocr {
    /// Page segmentation mode for OCR (uniform text blocks)
    pub const PAGE_SEG_MODE: &str = "6";

    /// OCR engine mode (Classic + LSTM)
    pub const ENGINE_MODE: &str = "1";

    /// Numeric mode classification setting
    pub const NUMERIC_MODE: &str = "1";

    /// Default OCR language
    pub const DEFAULT_LANGUAGE: &str = "eng";

    /// Character whitelist for OCR (alphanumeric and common punctuation)
    pub const CHAR_WHITELIST: &str =
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789 ()";
}

/// Text Processing Constants
pub mod text {
    /// Minimum expected text length for successful OCR
    pub const MIN_EXPECTED_TEXT_LENGTH: usize = 50;

    /// Maximum characters to show in text preview logs
    pub const TEXT_PREVIEW_LENGTH: usize = 100;
}

/// Image Preprocessing Constants
pub mod preprocessing {
    /// Brightness threshold for contrast enhancement (0-255)
    pub const BRIGHTNESS_THRESHOLD: u8 = 128;

    /// Contrast multiplier for dark pixels (makes them darker)
    pub const DARK_PIXEL_MULTIPLIER: f32 = 0.7;

    /// Contrast multiplier for light pixels (makes them lighter)  
    pub const LIGHT_PIXEL_MULTIPLIER: f32 = 0.7;
}

/// Footedness Detection Constants
pub mod footedness {
    /// Search region size for finding circles near foot indicators
    pub const SEARCH_REGION_SIZE: u32 = 40;

    /// Search region width for finding text patterns (horizontal dark streaks)
    pub const TEXT_SEARCH_WIDTH: u32 = 100;

    /// Minimum dark pixels required to consider a region as text
    pub const MIN_DARK_PIXELS_FOR_TEXT: u32 = 30;

    /// Minimum colored pixels required to classify a region as having that color
    pub const MIN_COLORED_PIXELS: u32 = 5;
}

/// Color Classification Constants
pub mod color {
    /// Minimum brightness value for color classification (exclude very dark pixels)
    pub const MIN_BRIGHTNESS: u32 = 50;

    /// Maximum brightness value for color classification (exclude very light pixels)
    pub const MAX_BRIGHTNESS: u32 = 240;

    /// Minimum green channel value for green color detection
    pub const GREEN_MIN_VALUE: u8 = 120;

    /// Green channel advantage required over red/blue for green detection
    pub const GREEN_CHANNEL_ADVANTAGE: u16 = 30;

    /// Minimum red channel value for yellow color detection
    pub const YELLOW_MIN_RED: u8 = 150;

    /// Minimum green channel value for yellow color detection  
    pub const YELLOW_MIN_GREEN: u8 = 150;

    /// Maximum blue channel value for yellow color detection
    pub const YELLOW_MAX_BLUE: u8 = 100;
}

/// Image Layout Constants
pub mod layout {
    /// Fraction of image height to start searching for footedness indicators (2/3 from top)
    pub const FOOTEDNESS_SEARCH_START_FRACTION: u32 = 2;
    pub const FOOTEDNESS_SEARCH_TOTAL_FRACTION: u32 = 3;

    /// Circle radius for footedness indicator circles (squared for distance calculation)
    pub const CIRCLE_RADIUS_SQUARED: i32 = 100;

    /// Circle drawing radius offset for creating test circles
    pub const CIRCLE_DRAW_RADIUS: i32 = 10;
}

/// Test Image Constants
pub mod test {
    /// Test image dimensions
    pub const TEST_IMAGE_WIDTH: u32 = 200;
    pub const TEST_IMAGE_HEIGHT: u32 = 100;

    /// Larger test image dimensions for footedness testing
    pub const LARGE_TEST_IMAGE_WIDTH: u32 = 400;
    pub const LARGE_TEST_IMAGE_HEIGHT: u32 = 300;

    /// RGB values for test colors
    pub const WHITE_RGB: [u8; 3] = [255, 255, 255];
    pub const BLACK_RGB: [u8; 3] = [0, 0, 0];
    pub const GREEN_RGB: [u8; 3] = [50, 200, 50];
    pub const YELLOW_RGB: [u8; 3] = [200, 200, 50];
    pub const GRAY_RGB: [u8; 3] = [128, 128, 128];

    /// Test positions and sizes
    pub const LEFT_FOOT_TEXT_START: u32 = 50;
    pub const LEFT_FOOT_TEXT_END: u32 = 150;
    pub const RIGHT_FOOT_TEXT_START: u32 = 250;
    pub const RIGHT_FOOT_TEXT_END: u32 = 350;
    pub const FOOT_TEXT_Y_START: u32 = 250;
    pub const FOOT_TEXT_Y_END: u32 = 255;

    /// Test circle positions
    pub const LEFT_CIRCLE_X: i32 = 100;
    pub const LEFT_CIRCLE_Y: i32 = 230;
    pub const RIGHT_CIRCLE_X: i32 = 300;
    pub const RIGHT_CIRCLE_Y: i32 = 230;
}

/// Age and Name Processing Constants
pub mod age_name {
    /// Minimum valid age for players
    pub const MIN_PLAYER_AGE: u8 = 15;

    /// Maximum valid age for players  
    pub const MAX_PLAYER_AGE: u8 = 45;

    /// Minimum length for potential player names
    pub const MIN_NAME_LENGTH: usize = 2;

    /// Minimum fraction of alphabetic characters required in a name part
    pub const MIN_ALPHABETIC_FRACTION: usize = 2; // 1/2 = 50%

    /// Minimum length for cleaned name after removing digits
    pub const MIN_CLEANED_NAME_LENGTH: usize = 2;

    /// Maximum number of words allowed in a player name
    pub const MAX_NAME_WORDS: usize = 4;

    /// Minimum number of words required in a player name
    pub const MIN_NAME_WORDS: usize = 2;

    /// Maximum length for connector words (like "van", "de", "la")
    pub const MAX_CONNECTOR_WORD_LENGTH: usize = 3;
}
