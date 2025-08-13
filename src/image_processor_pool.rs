use crate::error::{FMDataError, Result};
use crate::image_processor::{ImageProcessor, ProcessingConfig};
use std::sync::atomic::{AtomicUsize, Ordering};

/// A pool of image processors for concurrent processing with clear ownership semantics
pub struct ImageProcessorPool {
    processors: Vec<ImageProcessor>,
    current: AtomicUsize,
}

impl ImageProcessorPool {
    /// Create a new image processor pool with the specified size and configuration
    pub fn new(size: usize, config: ProcessingConfig) -> Result<Self> {
        if size == 0 {
            return Err(FMDataError::image("Pool size must be greater than 0"));
        }

        let processors: Result<Vec<_>, _> = (0..size)
            .map(|_| ImageProcessor::new(config.clone()))
            .collect();

        Ok(Self {
            processors: processors?,
            current: AtomicUsize::new(0),
        })
    }

    /// Get a processor from the pool using round-robin allocation
    pub fn get(&self) -> &ImageProcessor {
        let index = self.current.fetch_add(1, Ordering::Relaxed) % self.processors.len();
        &self.processors[index]
    }

    /// Get the number of processors in the pool
    pub fn size(&self) -> usize {
        self.processors.len()
    }

    /// Create a single-processor pool (for simple use cases)
    pub fn single(config: ProcessingConfig) -> Result<Self> {
        Self::new(1, config)
    }

    /// Create a pool sized based on the number of logical CPU cores
    pub fn sized_for_cpu(config: ProcessingConfig) -> Result<Self> {
        let size = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4) // Default to 4 if we can't determine
            .min(8); // Cap at 8 to avoid excessive resource usage
        
        Self::new(size, config)
    }
}

/// Builder for creating image processor pools with different configurations
pub struct ImageProcessorPoolBuilder {
    size: Option<usize>,
    config: ProcessingConfig,
}

impl ImageProcessorPoolBuilder {
    /// Create a new pool builder with default configuration
    pub fn new() -> Self {
        Self {
            size: None,
            config: ProcessingConfig::default(),
        }
    }

    /// Set the pool size
    pub fn with_size(mut self, size: usize) -> Self {
        self.size = Some(size);
        self
    }

    /// Set the processing configuration
    pub fn with_config(mut self, config: ProcessingConfig) -> Self {
        self.config = config;
        self
    }

    /// Set whether preprocessing is enabled
    pub fn enable_preprocessing(mut self, enable: bool) -> Self {
        self.config.enable_preprocessing = enable;
        self
    }

    /// Set the OCR language
    pub fn with_language(mut self, language: impl Into<String>) -> Self {
        self.config.ocr_language = language.into();
        self
    }

    /// Build the pool
    pub fn build(self) -> Result<ImageProcessorPool> {
        match self.size {
            Some(size) => ImageProcessorPool::new(size, self.config),
            None => ImageProcessorPool::sized_for_cpu(self.config),
        }
    }

    /// Build a single-processor pool
    pub fn build_single(self) -> Result<ImageProcessorPool> {
        ImageProcessorPool::single(self.config)
    }
}

impl Default for ImageProcessorPoolBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::image_processor::ProcessingConfig;

    #[test]
    fn test_pool_creation() {
        let config = ProcessingConfig::default();
        let pool = ImageProcessorPool::new(3, config).unwrap();
        
        assert_eq!(pool.size(), 3);
    }

    #[test]
    fn test_zero_size_pool_fails() {
        let config = ProcessingConfig::default();
        let result = ImageProcessorPool::new(0, config);
        
        assert!(result.is_err());
    }

    #[test]
    fn test_single_pool() {
        let config = ProcessingConfig::default();
        let pool = ImageProcessorPool::single(config).unwrap();
        
        assert_eq!(pool.size(), 1);
    }

    #[test]
    fn test_sized_for_cpu() {
        let config = ProcessingConfig::default();
        let pool = ImageProcessorPool::sized_for_cpu(config).unwrap();
        
        // Should be at least 1 and at most 8
        assert!(pool.size() >= 1);
        assert!(pool.size() <= 8);
    }

    #[test]
    fn test_round_robin_allocation() {
        let config = ProcessingConfig::default();
        let pool = ImageProcessorPool::new(3, config).unwrap();
        
        // Get processors multiple times and verify they cycle
        let processor1 = pool.get() as *const ImageProcessor;
        let processor2 = pool.get() as *const ImageProcessor;
        let processor3 = pool.get() as *const ImageProcessor;
        let processor4 = pool.get() as *const ImageProcessor; // Should wrap around
        
        // All processors should be different
        assert_ne!(processor1, processor2);
        assert_ne!(processor2, processor3);
        assert_ne!(processor1, processor3);
        
        // Fourth should be the same as first (round-robin)
        assert_eq!(processor1, processor4);
    }

    #[test]
    fn test_builder_default() {
        let pool = ImageProcessorPoolBuilder::new()
            .build_single()
            .unwrap();
        
        assert_eq!(pool.size(), 1);
    }

    #[test]
    fn test_builder_with_size() {
        let pool = ImageProcessorPoolBuilder::new()
            .with_size(5)
            .build()
            .unwrap();
        
        assert_eq!(pool.size(), 5);
    }

    #[test]
    fn test_builder_with_config() {
        let mut config = ProcessingConfig::default();
        config.enable_preprocessing = false;
        config.ocr_language = "deu".to_string();
        
        let pool = ImageProcessorPoolBuilder::new()
            .with_config(config)
            .build_single()
            .unwrap();
        
        assert_eq!(pool.size(), 1);
    }

    #[test]
    fn test_builder_fluent_interface() {
        let pool = ImageProcessorPoolBuilder::new()
            .with_size(2)
            .enable_preprocessing(false)
            .with_language("fra")
            .build()
            .unwrap();
        
        assert_eq!(pool.size(), 2);
    }

    #[test]
    fn test_concurrent_access() {
        use std::thread;
        
        let config = ProcessingConfig::default();
        let pool = ImageProcessorPool::new(2, config).unwrap();
        
        // Test that multiple threads can safely get processors
        let handles: Vec<_> = (0..4)
            .map(|_| {
                thread::spawn(|| {
                    // Each thread gets a processor (should not panic or deadlock)
                    let _processor = pool.get();
                    "success"
                })
            })
            .collect();
        
        for handle in handles {
            assert_eq!(handle.join().unwrap(), "success");
        }
    }
}